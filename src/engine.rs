use std::borrow::Borrow;
use std::convert::*;
use std::sync::*;
use std::iter::*;
use std::vec::Vec;
use std::collections::{BTreeMap, HashSet, HashMap};

use im::OrdSet;
use timely::dataflow::*;
use differential_dataflow::*;
use differential_dataflow::input::InputSession;
use differential_dataflow::operators::join::*;
use differential_dataflow::operators::*;
use differential_dataflow::hashable::*;

use crate::constraint::*;
use crate::term::*;
use crate::expression::*;
use crate::module::*;
use crate::rule::*;
use crate::parser::combinator::*;
use crate::util::*;
use crate::util::map::*;


pub struct Session<FM, T> where FM: FormulaModule<T>, T: TermStructure {
    module: Arc<RwLock<FM>>,
    worker: timely::worker::Worker<timely::communication::allocator::Thread>,
    input: InputSession<i32, T, isize>,
    probe: timely::dataflow::ProbeHandle<i32>,
    step_count: i32,
}

impl<FM, T> Session<FM, T> where FM: FormulaModule<T>, T: TermStructure
{
    pub fn new(module: FM, engine: &DDEngine<T>) -> Self {
        // Create a single thread worker.
        let allocator = timely::communication::allocator::Thread::new();
        let mut worker = timely::worker::Worker::new(allocator);
        let thread_safe_module = Arc::new(RwLock::new(module));
        let (mut input, probe) = engine.create_dataflow(thread_safe_module.clone(), &mut worker);

        input.advance_to(0);

        Session {
            module: thread_safe_module,
            worker,
            input,
            probe,
            step_count: 1,
        }
    }

    fn _advance(&mut self) {
        self.input.advance_to(self.step_count);
        self.input.flush();
        while self.probe.less_than(&self.input.time()) {
            // println!("Current timestamp: {:?}", self.input.time());
            self.worker.step();
        }
        self.step_count += 1;
    }

    pub fn create_term(&self, term_str: &str) -> Option<T> {
        // Add an ending to avoid Incomplete Error in the parser.
        let term_ast = term(&format!("{}{}", term_str, "~")[..]).unwrap().1;
        let reader = self.module.read().unwrap();
        let metainfo = reader.meta_info();
        let term = T::from_term_ast(&term_ast, metainfo.type_map());
        Some(term)
    }

    /// Add wrapped term into input with its hash and ordering pre-computed.
    pub fn add_term(&mut self, term: T) {
        self.input.insert(term);
        self._advance();
    }

    pub fn add_terms<Iter>(&mut self, terms: Iter) 
    where
        Iter: IntoIterator<Item=T>
    {
        for term in terms {
            self.input.insert(term.into());
        }
        self._advance();
    }

    pub fn remove_term(&mut self, term: T) {
        self.input.remove(term.into());
        self._advance();
    }

    pub fn remove_terms<Iter>(&mut self, terms: Iter) 
    where
        Iter: IntoIterator<Item=T>
    {
        for term in terms {
            self.input.remove(term);
        }
        self._advance();
    }

    pub fn load(&mut self) {
        let mut terms = vec![];
        for term in self.module.read().unwrap().terms().into_iter() {
            terms.push(term.clone());
        }
        self.add_terms(terms.into_iter());
    }
}

pub struct StreamIndex<G, T> 
where
    G: Scope,
    G::Timestamp: differential_dataflow::lattice::Lattice + Ord,
    T: TermStructure,
{
    pub indexes: HashMap<T, Collection<G, QuickHashOrdMap<T, T>>>,
}

impl<G, T> StreamIndex<G, T> 
where
    G: Scope,
    G::Timestamp: differential_dataflow::lattice::Lattice + Ord,
    T: TermStructure,
{
    pub fn find_stream_by_pattern(&self, term: &T) -> Collection<G, QuickHashOrdMap<T, T>> {
        let (normalized_term, _) = term.normalize();
        self.indexes.get(normalized_term.borrow()).unwrap().clone()
    } 
}

pub struct DDEngine<T> where T: TermStructure {
    pub env: Env<T>,
    pub inspect: bool,
}

impl<T> DDEngine<T> where T: TermStructure {
    /// Create a new engine instance given an Env.
    pub fn new(env: Env<T>) -> Self {
        DDEngine {
            env,
            inspect: false,
        }
    }

    /// Build an environment with generic term specify in the engine from FORMULA program text.
    pub fn build_env(&mut self, text: String) -> Env<T> {
        let env = T::load_program(text + " EOF");
        return env;
    }

    /// Find module in environment and add a new rule to the module
    /// Only support Domain and Transform module.
    pub fn add_rule(&mut self, module_name: &str, rule_text: &str) -> bool {
        let rule_ast = rule(rule_text).unwrap().1;
        if self.env.transform_map.contains_key(module_name) {
            let mut transform = self.env.transform_map.get_mut(module_name).unwrap();
            let rule = rule_ast.to_rule(&transform.meta_info());
            transform.add_rule(rule);
        } else if self.env.domain_map.contains_key(module_name) {
            let mut domain = self.env.domain_map.get_mut(module_name).unwrap();
            let rule = rule_ast.to_rule(&domain.meta_info());
            domain.add_rule(rule);
        } else if self.env.model_map.contains_key(module_name) {
            // Every model has a deep copy of the domain so models are actually detached from
            // the original domain and it is safe to modify the domain owned by the model.
            let mut model = self.env.model_map.get_mut(module_name).unwrap();
            let rule = rule_ast.to_rule(&model.meta_info());
            model.add_rule(rule);
        } else {
            return false;
        }
        true
    }

    pub fn install_model(&mut self, module: Model<T>) {
        self.env.model_map.insert(module.name.clone(), module); 
    }

    pub fn create_empty_model(&self, model_name: &str, domain_name: &str) -> Model<T> {
        let domain = self.env.get_domain_by_name(domain_name).unwrap();
        Model::new(
            model_name.to_string(),
            domain,
            HashSet::new(),
            HashMap::new(),
        )
    }

    pub fn create_model_transformation(&mut self, cmd: &str) -> Transformation<T> {
        // tast is TransformationAst.
        let tast = transformation(cmd).unwrap().1;
        let transform = self.env.get_transform_by_name(&tast.transform_name).unwrap();
        let mut input_model_map = HashMap::new();
        let mut input_term_map = HashMap::new();

        for (i, param) in tast.params.iter().enumerate() {
            let raw_term = T::from_term_ast(param, transform.meta_info().type_map());
            let id = transform.get_id(i).unwrap().clone();
            if raw_term.term_type() == TermType::Variable {
                // Looks like a variable term but actually is a model name.
                let model_name = format!("{}", raw_term);
                let model = self.env.get_model_by_name(&model_name[..]).unwrap().clone();
                input_model_map.insert(id, model);
            } else {
                input_term_map.insert(id, raw_term);
            }
        }

        Transformation::new(transform.clone(), input_term_map, input_model_map)
    }

    pub fn dataflow_filtered_by_type<G>(&self, terms: &Collection<G, T>, pred_term: T) -> Collection<G, T>
    where
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice + Ord,
    {
        terms
            .filter(move |term| {
                term.sort() == pred_term.sort()
            })
    }

    pub fn dataflow_filtered_by_pattern<FM, G>(
        &self,
        model: Arc<RwLock<FM>>,
        terms: &Collection<G, T>, 
        pattern: &T,
    ) -> Collection<G, QuickHashOrdMap<T, T>>
    where 
        FM: FormulaModule<T>,
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice + Ord,
    {
        let (normalized_pattern_term, _) = pattern.normalize();
        let binding_collection = self.dataflow_filtered_by_type(terms, pattern.clone()) 
            .map(move |term| {
                let binding_opt = normalized_pattern_term.get_cached_bindings(&term);
                (term, binding_opt)
            })
            .filter(|(_, binding_opt)| {
                match binding_opt {
                    None => false,
                    _ => true,
                }
            })
            .map(move |(term, binding_opt)| {
                // Create a variable to represent term itself.
                // TODO: what if alias variable is already in existing variables.
                let mut binding = binding_opt.unwrap();
                // let self_var: T = self_term.clone().into_with_index(model.clone());
                let self_var = T::create_variable_term(None, "~self".to_string(), vec![]);
                binding.ginsert(self_var, term); // A deep clone occurs here.
                return binding;
            });
            //.inspect(|x| { println!("Initial bindings for the first constraint is {:?}", x); });
        binding_collection
    }

    /// Match a stream of terms to predicate constraint and return a stream of hash maps
    /// that map variables to terms as the result of matching.
    pub fn dataflow_filtered_by_positive_predicate_constraint<G>(
        &self,
        model: Arc<RwLock<Model<T>>>,
        terms: &Collection<G, T>, 
        pos_pred_constraint: Constraint<T>,
    ) -> Collection<G, QuickHashOrdMap<T, T>> 
    where 
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice + Ord,
    {
        let predicate: Predicate<T> = pos_pred_constraint.try_into().unwrap();
        let pred_term = predicate.term.clone();
        let alias: Option<T> = predicate.alias.clone();

        let binding_collection = self.dataflow_filtered_by_type(terms, pred_term.clone()) 
            .map(move |term| {
                let binding_opt = pred_term.get_cached_bindings(&term);
                (term, binding_opt)
            })
            .filter(|(_, binding_opt)| {
                match binding_opt {
                    None => false,
                    _ => true,
                }
            })
            .map(move |(term, binding_opt)| {
                // If predicate may have alias then add itself to existing binding.
                // TODO: what if alias variable is already in existing variables.
                let mut binding = binding_opt.unwrap();
                if let Some(vterm) = &alias {
                    binding.ginsert(vterm.clone(), term); // A deep clone occurs here.
                }
                binding
            });
            //.inspect(|x| { println!("Initial bindings for the first constraint is {:?}", x); });

        binding_collection
    }
    
    /// Split the stream of hash maps into two streams of hash maps that one has "left_keys"
    /// and the other one has "right_keys".
    pub fn split_binding<FM, G>(
        &self,
        model: Arc<RwLock<FM>>,
        bindings: &Collection<G, QuickHashOrdMap<T, T>>, 
        left_keys:  OrdSet<T>,
        right_keys: OrdSet<T>,
    ) -> Collection<G, (QuickHashOrdMap<T, T>, QuickHashOrdMap<T, T>)>
    where 
        FM: FormulaModule<T>,
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice,
    {
        // let left: OrdSet<T> = left_keys.into_iter().map(|x| x.into_with_index(model.clone())).collect();
        // let right: OrdSet<T> = right_keys.into_iter().map(|x| x.into_with_index(model.clone())).collect();

        bindings.map(move |binding| {
            let mut first = BTreeMap::new();
            let mut second = BTreeMap::new();
            let mut btree_map: BTreeMap<T, T> = binding.into();

            for k in left_keys.iter() {
                if let Some(v) = btree_map.remove(k.borrow()) {
                    first.insert(k.clone(), v);
                }
            }

            for k in right_keys.iter() {
                if let Some(v) = btree_map.remove(k.borrow()) {
                    second.insert(k.clone(), v);
                }
            }

            (first.into(), second.into())
        })
    }

    /// Join two binding (OrdMap) collections by considering the same or related variable terms. Split each binding into
    /// two bindings: One with the shared variables and the other with non-shared variable. Before the splitting we have to
    /// do an extra step to extend the current binding with the variable subterms from the other collection of binding.
    /// if binding1 has key `x.y` and binding2 has key `x.y.z` and `x`, then binding1 must have `x.y.z` as well but won't 
    /// have `x` because you can't derive parent term from subterm backwards. On the other hand binding2 must have `x.y` too.
    pub fn join_two_bindings<FM, G>(&self, 
        model: Arc<RwLock<FM>>,
        prev_vars: OrdSet<T>,
        prev_col: &Collection<G, QuickHashOrdMap<T, T>>, 
        new_vars: OrdSet<T>,
        new_col: &Collection<G, QuickHashOrdMap<T, T>>
    ) 
    -> (OrdSet<T>, Collection<G, QuickHashOrdMap<T, T>>)
    where
        FM: FormulaModule<T>,
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice
    {
        // let prev_vars: OrdSet<T> = prev_vars.into_iter().map(|x| x.into_with_index(model.clone())).collect();
        // let new_vars: OrdSet<T> = new_vars.into_iter().map(|x| x.into_with_index(model.clone())).collect();

        let mut prev_vars_extra = OrdSet::new();
        let mut new_vars_extra = OrdSet::new();

        // When binding one has `x.y` and binding two has `x.y.z` update binding one with `x.y.z` and vice versa.
        for prev_var in prev_vars.iter() {
            for new_var in new_vars.iter() {
                if new_var.is_direct_subterm_of(prev_var) && prev_var != new_var {
                    prev_vars_extra.insert(new_var.clone());
                }
                else if prev_var.is_direct_subterm_of(new_var) && new_var != prev_var {
                    new_vars_extra.insert(prev_var.clone());
                }
            }
        }

        // Update stream of hashmaps to add variables with fragments as the key.
        let prev_vars_extra_copy = prev_vars_extra.clone();
        let updated_prev_col = prev_col.map(move |mut binding| {
            for prev_var in prev_vars_extra_copy.iter() {
                prev_var.update_binding(&mut binding);
            }
            binding
        });

        let new_vars_extra_copy = new_vars_extra.clone();
        let updated_new_col = new_col.map(move |mut binding| {
            for new_var in new_vars_extra_copy.iter() {
                new_var.update_binding(&mut binding);
            }
            binding
        });

        let mut updated_prev_vars: OrdSet<T> = OrdSet::new();
        updated_prev_vars.extend(prev_vars);
        updated_prev_vars.extend(prev_vars_extra);
        
        let mut updated_new_vars = OrdSet::new();
        updated_new_vars.extend(new_vars);
        updated_new_vars.extend(new_vars_extra);

        let mut all_vars: OrdSet<T> = OrdSet::new();
        all_vars.extend(updated_prev_vars.clone());
        all_vars.extend(updated_new_vars.clone());

        let (lvars, mvars, rvars) = ldiff_intersection_rdiff(&updated_prev_vars, &updated_new_vars);

        // Turn collection of [binding] into collection of [(middle, right)] for joins.
        let m_r_col = self.split_binding(
            model.clone(),
            &updated_new_col, 
            mvars.clone().into_iter().map(|x| x.borrow().clone()).collect(),
            rvars.clone().into_iter().map(|x| x.borrow().clone()).collect()
        );
        // m_r_col.inspect(|x| println!("m_r_col: {:?}", x));

        // Turn collection of [binding] into collection of [(middle, left)] for joins.
        let m_l_col = self.split_binding(
            model.clone(),
            &updated_prev_col, 
            mvars.clone().into_iter().map(|x| x.borrow().clone()).collect(), 
            lvars.clone().into_iter().map(|x| x.borrow().clone()).collect()
        );
        // m_l_col.inspect(|x| println!("m_l_col: {:?}", x));

        let joint_collection = m_l_col
                        .join(&m_r_col)
            .map(move |(inter, (left, right))| {
                let mut binding = BTreeMap::new();
                let btree_inter: BTreeMap<T, T> = inter.into();
                let btree_left: BTreeMap<T, T> = left.into();
                let btree_right: BTreeMap<T, T> = right.into();
                binding.extend(btree_inter);
                binding.extend(btree_left);
                binding.extend(btree_right);
                binding.into()
            });

        // joint_collection.inspect(|x| println!("Join result: {:?}", x));
        (all_vars.into_iter().map(|x| x.borrow().clone()).collect(), joint_collection)
    } 

    pub fn dataflow_from_constraints<FM, G>(
        &self, 
        model: Arc<RwLock<FM>>,
        terms: &Collection<G, T>, 
        constraints: &Vec<Constraint<T>>,
        cache: &mut Option<StreamIndex<G, T>>,
    ) 
    -> (OrdSet<T>, Collection<G, QuickHashOrdMap<T, T>>)
    where
        FM: FormulaModule<T>,
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice
    {
        // Construct a headless rule from a list of constraints.
        let temp_rule = Rule::new(vec![], constraints.clone());
        let pos_preds = temp_rule.predicate_constraints();
        let init_vars: OrdSet<T> = OrdSet::new();

        // TODO: find a better way to create an empty collection.
        let init_col = terms
            .filter(|_wrapped_term| false)
            .map(|_term| {
                let empty_map = BTreeMap::new();
                let quick_hash_map: QuickHashOrdMap<T, T> = empty_map.into();
                quick_hash_map
            });

        // Join all positive predicate terms by their shared variables one by one in order.
        let (mut vars, collection) = pos_preds.into_iter()
            .fold((init_vars, init_col), |(prev_vars, prev_col), pred_constraint| {
                let pred: Predicate<T> = pred_constraint.clone().try_into().unwrap();
                let term: T = pred.term.clone();
                let mut vars: OrdSet<T> = OrdSet::new();
                vars.extend(term.variables().into_iter().map(|x| x.clone()));

                // Don't forget to add alias variable for the predicate constraint.
                if let Some(vterm) = pred.alias.clone() {
                    vars.insert(vterm);
                }
                
                let (normalized_pattern, vmap) = term.normalize();

                // Get bindings derived from all terms or use the cached stream, suppose all the streams
                // use the normalized variables as keys in the hashmaps.
                let mut model_ref = model.clone();
                
                let col = cache.as_ref().map_or_else(
                    move || {
                        // Sometimes you don't want to use cache like when in the inner iteration dealing
                        // with new derived terms.
                        self.dataflow_filtered_by_pattern(model_ref.clone(), terms, &term)
                    }, 
                    |cache| {
                        let col = match cache.indexes.contains_key(normalized_pattern.borrow()) {
                            true => {
                                cache.indexes.get(normalized_pattern.borrow()).unwrap().clone()
                            },
                            false => {
                                // self.dataflow_filtered_by_positive_predicate_constraint(
                                //     models, 
                                //     pred_constraint.clone(),
                                // )
                                println!("The pattern {} is not found in {:?}", normalized_pattern, cache.indexes.keys());
                                unimplemented!()
                            }
                        };
                        return col;
                    }
                )
                .map(move |binding| {
                    let btree_map: BTreeMap<T, T> = binding.into();
                    let mut updated_btree_map: BTreeMap<T, T> = BTreeMap::new();
                    for (k, v) in btree_map.into_iter() {
                        // Change the normalized variables back into original ones.
                        if vmap.contains_key(k.borrow()) { 
                            let k2 = vmap.get(k.borrow()).unwrap().clone();
                            updated_btree_map.insert(k2, v);
                        }
                    }
                    updated_btree_map.into()
                })
                //.inspect(|x| println!("col: {:?}", x))
                ;

                // col.inspect(move |x| println!("Changed matches for {}: {:?}", constraint, x));

                if prev_vars.len() != 0 {
                    return self.join_two_bindings(model.clone(), prev_vars, &prev_col, vars, &col);
                }
                else { return (vars, col); }
                // return (vars, col);
            });
        
        let vars_ref = vars.clone();
        // Let's assume every set comprehension must be explicitly declared with a variable on the left side 
        // of binary constraint.
        // e.g. x = count({a| a is A(b, c)}) before the aggregation result is used else where like x + 2 = 3.
        // Every derived variable must be declared once before used in other constraints 
        // e.g. y = (x + x) * x.
        let updated_collection = temp_rule.ordered_declaration_constraints().into_iter()
            .fold(collection, move |outer_col, constraint| {
                let mut model_ref = model.clone();
                let binary: Binary<T> = constraint.clone().try_into().unwrap();
                let left_base_expr: BaseExpr<T> = binary.left.try_into().unwrap();
                let var_term = match left_base_expr {
                    BaseExpr::Term(term) => Some(term),
                    _ => None
                }.unwrap();
                let var_term_copy = var_term.clone();

                match binary.right {
                    Expr::BaseExpr(right_base_expr) => {
                        match right_base_expr {
                            BaseExpr::SetComprehension(setcompre) => {
                                model_ref = model.clone();
                                let updated_col = self.dataflow_from_set_comprehension(
                                    model_ref.clone(),
                                    var_term.clone(),
                                    vars.clone(),
                                    &outer_col, 
                                    &terms, 
                                    &setcompre,
                                    cache
                                );
                                // Add declaration variable into the set of all vars after evaluation of setcompre
                                vars.insert(var_term.clone());
                                return updated_col;
                            },
                            _ => {}, // Ignored because it does not make sense to derive new term from single variable term.
                        }
                    },
                    Expr::ArithExpr(right_base_expr) => {
                        model_ref = model.clone();
                        // Evaluate arithmetic expression to derive new term and add it to existing binding.
                        let updated_col = outer_col.map(move |binding_wrapper| {
                            let mut binding: BTreeMap<T, T> = binding_wrapper.into();
                            let num = right_base_expr.evaluate(&binding).unwrap();
                            let atom_enum = AtomEnum::Int(num);
                            let num_term = T::create_atom_term(None, atom_enum);
                            // binding.insert(var_term.clone(), num_term.into_with_index(model_ref.clone()));
                            binding.insert(var_term.clone(), num_term);
                            binding.into()
                        });

                        vars.insert(var_term_copy);
                        return updated_col;
                    }
                }

                return outer_col;
            });
            
        // Since there are no derived terms then directly evaluate the expression to filter binding collection.
        let final_collection = temp_rule.pure_constraints().into_iter().fold(updated_collection, |col, constraint| {
            let binary: Binary<T> = constraint.clone().try_into().unwrap();
            let updated_col = col.filter(move |binding| {
                binary.evaluate(binding).unwrap()
            });
            return updated_col;
        });

        (vars_ref, final_collection)
    }

    pub fn dataflow_from_set_comprehension<FM, G>(
        &self,
        model: Arc<RwLock<FM>>,
        var: T,
        outer_vars: OrdSet<T>,
        ordered_outer_collection: &Collection<G, QuickHashOrdMap<T, T>>, // Binding collection from outer scope of set comprehension.
        terms: &Collection<G, T>, // Existing model collection.
        setcompre: &SetComprehension<T>,
        cache: &mut Option<StreamIndex<G, T>>,
    ) -> Collection<G, QuickHashOrdMap<T, T>>
    where 
        FM: FormulaModule<T>,
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice,
    {
        // Each binding from the input will enter into a separate scope when evaluating set comprehension.
        // e.g. B(A(a, b), k) :- A(a, b), k = count({x | x is A(a, b)}). k is always evaluated to 1 no matter 
        // how many terms of type A the current program has.
        let head_terms: Vec<T> = setcompre.vars.clone();
        let setcompre_op = setcompre.op.clone();
        let mut setcompre_default = setcompre.default.clone();
        let constraints = &setcompre.condition;
        let mut setcompre_var: T = var.clone();
        
        // Evaluate constraints in set comprehension and return ordered binding collection.
        let (inner_vars, ordered_inner_collection) = self.dataflow_from_constraints(model.clone(), terms, constraints, cache);
        
        // models.inspect(|x| println!("Models for inner constraints: {:?}", x));
        // ordered_outer_collection.inspect(|x| println!("Outer collection {:?}", x));
        // ordered_inner_collection.inspect(|x| println!("Inner collection {:?}", x));
        // ordered_outer_collection.inspect(|x| println!("Outer collection {:?}", x));

        // If inner scope and outer scope don't have shared variables then it means they can be handled separately.
        match T::has_deep_intersection(inner_vars.iter(), outer_vars.iter()) {
            false => {
                // Inner scope and outer scope have no shared variables then use the stream produced by set
                // comprehension to aggregate the terms in the set and return an integer.
                let aggregation_stream = ordered_inner_collection
                    .map(move |binding| {
                        let mut terms = vec![];
                        for head_term in head_terms.iter() {
                            let term = match head_term.term_type() {
                                TermType::Composite => { head_term.propagate_bindings(&binding) },
                                TermType::Variable => { binding.gget(head_term.borrow()).unwrap().clone() },
                                TermType::Atom => { head_term.clone() }
                            };
                            terms.push(term);
                        }
                        terms
                    })
                    .flat_map(|term_list| term_list)
                    .distinct() // Consolidate the terms derived in the head and remove duplicates.
                    .map(|x| ((), x))
                    .reduce(move |_key, input, output| {
                        let input_iter = input.iter();
                        let aggre_result = setcompre_op.aggregate(input_iter);
                        output.push((vec![aggre_result], 1));
                    });
                
                // Don't do joins if the outer scope has no constraints or doesn't generate a binding
                // because joins with empty collection always return empty.
                match outer_vars.len() == 0 {
                    true => {
                        aggregation_stream.map(move |(_, nums)| {
                            let mut binding: BTreeMap<T, T> = BTreeMap::new();
                            let atom_enum = AtomEnum::Int(nums.get(0).unwrap().clone());
                            let num_term = T::create_atom_term(None, atom_enum);
                            // binding.insert(setcompre_var.clone(), num_term.into_with_index(model.clone()));
                            binding.insert(setcompre_var.clone(), num_term);
                            binding.into()
                        })
                    },
                    false => {
                        ordered_outer_collection
                            .map(|x| { ((), x) })
                            .join(&aggregation_stream)
                            .map(move |(_, (binding_wrapper, nums))| {
                                // Take the first element in num list when operator is count, sum, maxAll, minAll.
                                let mut binding: BTreeMap<T, T> = binding_wrapper.into();
                                let atom_enum = AtomEnum::Int(nums.get(0).unwrap().clone());
                                let num_term = T::create_atom_term(None, atom_enum);
                                // binding.insert(setcompre_var.clone(), num_term.into_with_index(model.clone()));
                                binding.insert(setcompre_var.clone(), num_term);
                                binding.into()
                            })
                    }
                }
                //.inspect(|x| { println!("No variable sharing after reduce: {:?}", x); })
            },
            true => {
                // When inner scope and outer scope share some same variables.
                let mut model_ref = model.clone();
                setcompre_var = var.clone();
                setcompre_default = setcompre.default.clone();
                let (all_vars, join_stream) = self.join_two_bindings(
                    model.clone(),
                    outer_vars.clone(), 
                    ordered_outer_collection, 
                    inner_vars.clone(), 
                    &ordered_inner_collection
                );

                //join_stream.inspect(|x| println!("Joins result: {:?}", x));

                // println!("Split: outer_vars={:?}, inner_vars={:?}", outer_vars, inner_vars);

                // Take binding in the outer scope as key and bindings in inner scope are grouped by the key.
                let binding_and_aggregation_stream = self.split_binding(
                        model.clone(),
                        &join_stream, 
                        outer_vars.clone(), 
                        inner_vars.clone()
                    )
                    //.inspect(|x| println!("Before reduce: {:?}", x))
                    .reduce(move |key, input, output| {
                        // Each outer binidng as key has points a list of derived terms from inner scope.
                        for (binding, count) in input.into_iter() {
                            for head_term in head_terms.iter() {
                                let term = match head_term.term_type() {
                                    TermType::Composite => { head_term.propagate_bindings(*binding) },
                                    TermType::Variable => { binding.gget(head_term.borrow()).unwrap().clone() },
                                    TermType::Atom => { head_term.clone() }
                                };
                                output.push((term, *count));
                            }
                        }
                        // Reduce to tuples of (outer_binding, head_term)
                    })
                    //.inspect(|x| { println!("Before distinct: {:?}", x); })
                    // TODO: Fix the trait bound of Hashable for tuples.
                    //.distinct() 
                    //.consolidate()
                    //.inspect(|x| { println!("After distinct: {:?}", x); })
                    .reduce(move |_key, input, output| {
                        let input_iter = input.iter();
                        let aggre_result = setcompre_op.aggregate(input_iter);
                        output.push((vec![aggre_result], 1));
                        // Reduce to tuples of (outer_binding, aggregation_result).
                    })
                    //.inspect(|x| { println!("Sharing variable after reduce: {:?}", x); })
                    ;

                let binding_with_aggregation_stream = binding_and_aggregation_stream
                    .map(move |(binding_wrapper, nums)| {
                        let mut binding: BTreeMap<T, T> = binding_wrapper.into();
                        // Take the first element in num list when operator is count, sum, maxAll, minAll.
                        let atom_enum = AtomEnum::Int(nums.get(0).unwrap().clone());
                        let num_term = T::create_atom_term(None, atom_enum);
                        binding.insert(setcompre_var.clone(), num_term);
                        binding.into()

                        // When operator is top_k or bottom_k.
                        // TODO: return a list of numeric values but how?
                    });

                setcompre_var = var.clone();
                setcompre_default = setcompre.default.clone();
                // Find the stream of binding in outer scope that does no contribution to the aggregation result
                // Then simply add default aggregation value into the binding.
                let binding_with_default_stream = ordered_outer_collection.map(|x| (x, true))
                    .antijoin(&binding_and_aggregation_stream.map(|(x, aggregation)| { x }))
                    .map(move |(mut outer_wrapper, _)| {
                        // Add default value of set comprehension to each binding.
                        let mut outer: BTreeMap<T, T> = outer_wrapper.into();
                        let atom_enum = AtomEnum::Int(setcompre_default.clone());
                        let num_term = T::create_atom_term(None, atom_enum);
                        outer.insert(setcompre_var.clone(), num_term);
                        // outer.insert(setcompre_var.clone(), num_term.into_with_index(model_ref.clone()));
                        outer.into()
                    });

                binding_with_aggregation_stream.concat(&binding_with_default_stream)
            }
        }
        //.inspect(|x| { println!("Aggregation result: {:?}", x); })
    }

    pub fn dataflow_from_single_rule<FM, G>(
        &self, 
        model: Arc<RwLock<FM>>,
        terms: &Collection<G, T>, 
        rule: &Rule<T>,
        cache: &mut Option<StreamIndex<G, T>>,
    ) -> Collection<G, T>
    where 
        FM: FormulaModule<T>,
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice,
    {
        // let head_terms: Vec<Term> = rule.get_head().into_iter().map(|term| {
        //     let (pattern, _) = term.normalize();
        //     return pattern;
        // }).collect();

        let head_terms = rule.get_head();
        let head_terms_copy = head_terms.clone();
        let constraints = rule.get_body();

        let (_, binding_collection) = self.dataflow_from_constraints(model, &terms, &constraints, cache);

        terms.clone()
        
        // let fixpoint_collection = binding_collection
        //     .iterate(|inner_collection| {
        //         let derived_terms = inner_collection
        //             //.inspect(|x| println!("A binding map derived from constraints: {:#?}", x))
        //             .map(move |binding| {
        //                 let mut new_terms: Vec<HashedTerm> = vec![];
        //                 for head_term in head_terms.iter() {
        //                     // If head term has variable term then it means that's a constant.
        //                     if let Term::Variable(var) = head_term {
        //                         let constant_name = var.root.clone();
        //                         let constant = Term::create_constant(constant_name);
        //                         new_terms.push(constant.into());
        //                     } else {
        //                         // println!("Binding for head term: {:?}", binding);
        //                         let new_term = head_term.propagate_bindings(&binding);
        //                         new_terms.push(new_term.into());
        //                     }
        //                 }
        //                 new_terms
        //             })
        //             .flat_map(|x| x)
        //             //.inspect(|x| println!("Add some new terms: {:?}", x))
        //             ;

        //         let (_, additional_binding_collection) = self.dataflow_from_constraints(
        //             &derived_terms, 
        //             &constraints, 
        //             &mut None,
        //         );

        //         inner_collection.concat(&additional_binding_collection).distinct()
        //     });
        
        // fixpoint_collection.map(move |binding| {
        //     let mut new_terms: Vec<HashedTerm> = vec![];
        //     for head_term in head_terms_copy.iter() {
        //         // If head term has variable term then it means that's a constant.
        //         if let Term::Variable(var) = head_term {
        //             let constant_name = var.root.clone();
        //             let constant = Term::create_constant(constant_name);
        //             new_terms.push(constant.into());
        //         } else {
        //             // println!("Binding for head term: {:?}", binding);
        //             let new_term = head_term.propagate_bindings(&binding);
        //             new_terms.push(new_term.into());
        //         }
        //     }
        //     new_terms
        // })
        // .flat_map(|x| x)

        //.inspect(|x| println!("Add some new terms: {:?}", x))
        
        // terms
        //     .iterate(|inner| {
        //         let mut inner_binding_collection = binding_collection.enter(&inner.scope());
        //         let derived_terms = inner_binding_collection
        //             //.inspect(|x| println!("A binding map derived from constraints: {:#?}", x))
        //             .map(move |binding| {
        //                 let mut new_terms: Vec<HashedTerm> = vec![];
        //                 for head_term in head_terms.iter() {
        //                     // If head term has variable term then it means that's a constant.
        //                     if let Term::Variable(var) = head_term {
        //                         let constant_name = var.root.clone();
        //                         let constant = Term::create_constant(constant_name);
        //                         new_terms.push(constant.into());
        //                     } else {
        //                         // println!("Binding for head term: {:?}", binding);
        //                         let new_term = head_term.propagate_bindings(&binding);
        //                         new_terms.push(new_term.into());
        //                     }
        //                 }
        //                 new_terms
        //             })
        //             .flat_map(|x| x)
        //             //.inspect(|x| println!("Add some new terms: {:?}", x))
        //             ;

        //         let (_, additional_binding_collection) = self.dataflow_from_constraints(
        //             &derived_terms, 
        //             &constraints, 
        //             &mut None,
        //         );

        //         inner_binding_collection = inner_binding_collection.concat(&additional_binding_collection);
        //         inner.concat(&derived_terms).distinct()
        //     })
            // .inspect(|x| println!("All terms derived in rule: {:?}", x))
    }

    pub fn create_dataflow<FM>(
        &self, 
        module: Arc<RwLock<FM>>,
        worker: &mut timely::worker::Worker<timely::communication::allocator::Thread>
    ) 
    -> (InputSession<i32, T, isize>, timely::dataflow::ProbeHandle<i32>) 
    where FM: FormulaModule<T>
    {
        let mut input = InputSession::<i32, T, isize>::new();
        let probe = worker.dataflow(|scope| {
            // Make sure there are no model duplicates.
            let terms = input.to_collection(scope)
                //.inspect(|x| println!("Model Input: {:?}", x))
                .distinct();

            let mut new_terms = terms.map(|x| x);
            
            let mut stream_cache = Some(StreamIndex { indexes: HashMap::new() });
            let stratified_rules = module.read().unwrap().meta_info().stratified_rules();
            for (i, stratum) in stratified_rules.into_iter().enumerate() {
                // Rules to be executed are from the same stratum and independent from each other.
                for rule in stratum.iter() {
                    if self.inspect {
                        println!("Rule at Stratum {}: {}", i, rule);
                    }
                    
                    if let Some(mut cache) = stream_cache {
                        for constraint in rule.get_body().iter() {
                            if let Constraint::Predicate(predicate) = constraint {
                                let (normalized_term, _) = predicate.term.normalize();
                                if !cache.indexes.contains_key(&normalized_term) {
                                    let stream = self.dataflow_filtered_by_pattern(module.clone(), &new_terms, &predicate.term);
                                    // stream.inspect(|x| println!("Cached stream: {:?}", x));
                                    cache.indexes.insert(normalized_term, stream);
                                }
                            }
                        }
                        stream_cache = Some(cache);
                    }
                }

                for rule in stratum.iter() {
                    let models_after_rule_execution = self.dataflow_from_single_rule(
                        module.clone(),
                        &new_terms, 
                        rule,
                        &mut stream_cache,
                    );

                    new_terms = new_terms
                        .concat(&models_after_rule_execution)
                        .distinct();
                }

                if self.inspect {
                    new_terms.inspect(move |x| { println!("Stratum {}: {:?}", &i, x); });
                }
            }

            new_terms.probe()
        });

        (input, probe)
    }

}

