use std::borrow::Borrow;
use std::sync::Arc;
use std::iter::*;
use std::vec::Vec;
use std::convert::*;
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


pub struct Session<FM: FormulaModule> {
    worker: timely::worker::Worker<timely::communication::allocator::Thread>,
    input: InputSession<i32, HashedTerm, isize>,
    probe: timely::dataflow::ProbeHandle<i32>,
    module: FM,
    step_count: i32,
}

impl<FM: FormulaModule> Session<FM> {
    pub fn new(module: FM, engine: &DDEngine) -> Self {
        // Create a single thread worker.
        let allocator = timely::communication::allocator::Thread::new();
        let mut worker = timely::worker::Worker::new(allocator);
        let (mut input, probe) = engine.create_dataflow(&module, &mut worker);

        Session {
            worker,
            input,
            probe,
            module,
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

    pub fn create_term(&self, term_str: &str) -> Option<Term> {
        // Add an ending to avoid Incomplete Error in the parser.
        let term_ast = term(&format!("{}{}", term_str, "~")[..]).unwrap().1;
        let term = term_ast.to_term(&self.module);
        Some(term)
    }

    /// Add wrapped term into input with its hash and ordering pre-computed.
    pub fn add_term(&mut self, term: Term) {
        self.input.insert(term.into());
        self._advance();
    }

    pub fn add_terms<Iter: IntoIterator<Item=Term>>(&mut self, terms: Iter) {
        for term in terms {
            self.input.insert(term.into());
        }
        self._advance();
    }

    pub fn remove_term(&mut self, term: Term) {
        self.input.remove(term.into());
        self._advance();
    }

    pub fn remove_terms<Iter: IntoIterator<Item=Term>>(&mut self, terms: Iter) {
        for term in terms {
            self.input.remove(term.into());
        }
        self._advance();
    }

    pub fn load(&mut self) {
        let terms = self.module.terms();
        self.add_terms(terms);
    }
}


pub struct DDEngine {
    pub env: Env,
    pub inspect: bool,
}

impl DDEngine {
    pub fn new() -> Self {
        DDEngine {
            env: Env::new(),
            inspect: false,
        }
    }

    // TODO: Enable installation of multiple programs and detect conflicts between programs.
    pub fn install(&mut self, program_text: String) {
        let env = load_program(program_text + " EOF");
        self.env = env;
    }

    /// Find module in environment and add a new rule to the module
    /// Only support Domain and Transform module.
    pub fn add_rule(&mut self, module_name: &str, rule_text: &str) -> bool {
        let rule_ast = rule(rule_text).unwrap().1;
        if self.env.transform_map.contains_key(module_name) {
            let transform = self.env.transform_map.get_mut(module_name).unwrap();
            let rule = rule_ast.to_rule(transform);
            transform.rules.push(rule);
        } else if self.env.domain_map.contains_key(module_name) {
            let domain = self.env.domain_map.get_mut(module_name).unwrap();
            let rule = rule_ast.to_rule(domain);
            domain.rules.push(rule);
        } else if self.env.model_map.contains_key(module_name) {
            // Every model has a deep copy of the domain so models are actually detached from
            // the original domain and it is safe to modify the domain owned by the model.
            let model = self.env.model_map.get_mut(module_name).unwrap();
            let rule = rule_ast.to_rule(&model.domain);
            model.domain.rules.push(rule);
        } else {
            return false;
        }
        true
    }

    pub fn install_model(&mut self, module: Model) {
        self.env.model_map.insert(module.model_name.clone(), module); 
    }

    pub fn create_empty_model(&mut self, model_name: &str, domain_name: &str) -> Model {
        let domain = self.env.get_domain_by_name(domain_name).unwrap().clone();
        Model {
            model_name: model_name.to_string(),
            domain,
            terms: HashSet::new(),
            alias_map: HashMap::new(),
            reverse_alias_map: HashMap::new()
        }
    }

    pub fn create_model_transformation(&mut self, cmd: &str) -> Transformation {
        // tast is TransformationAst.
        let tast = transformation(cmd).unwrap().1;
        let transform = self.env.get_transform_by_name(&tast.transform_name).unwrap();

        let mut input_model_map = HashMap::new();
        let mut input_term_map = HashMap::new();

        for (i, param) in tast.params.iter().enumerate() {
            let raw_term = param.to_term(transform);
            let id = transform.get_id(i).unwrap().clone();

            if let Term::Variable(v) = raw_term {
                // Looks like a variable term but actually is a model name.
                let model_name = v.root;
                let model = self.env.get_model_by_name(&model_name[..]).unwrap().clone();
                input_model_map.insert(id, model);
            } else {
                input_term_map.insert(id, raw_term);
            }
        }

        Transformation::new(transform.clone(), input_term_map, input_model_map)
    }

    pub fn dataflow_filtered_by_type<G>(
        &self,
        terms: &Collection<G, HashedTerm>,
        pred_term: Term,
    ) -> Collection<G, HashedTerm>
    where
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice + Ord,
    {
        let c: Composite = pred_term.try_into().unwrap();
        terms
            .filter(move |wrapped_term| {
                let term_ref: &Term = wrapped_term.borrow();
                match term_ref {
                    Term::Composite(composite) => {
                        if c.sort == composite.sort { true } 
                        else { false }
                    }
                    _ => { false }
                }
            })
    }

    // TODO: Some predicates in the rule may have the same pattern.
    pub fn dataflow_filtered_by_pattern() {

    }

    pub fn dataflow_filtered_by_positive_predicate_constraint<G>(
        &self,
        terms: &Collection<G, HashedTerm>, 
        pos_pred_constraint: Constraint,
    ) -> Collection<G, QuickHashOrdMap<Term, Term>> 
    where 
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice + Ord,
    {
        let predicate: Predicate = pos_pred_constraint.try_into().unwrap();
        let pred_term = predicate.term.clone();

        let binding_collection = self.dataflow_filtered_by_type(terms, pred_term.clone()) 
            .map(move |wrapped_term| {
                let binding_opt = pred_term.get_cached_bindings((**wrapped_term).borrow());
                (wrapped_term, binding_opt)
            })
            .filter(|(_, binding_opt)| {
                match binding_opt {
                    None => false,
                    _ => true,
                }
            })
            .map(move |(wrapped_term, binding_opt)| {
                // If predicate may have alias then add itself to existing binding.
                // TODO: what if alias variable is already in existing variables.
                let mut binding = binding_opt.unwrap();
                if let Some(vterm) = &predicate.alias {
                    let term = &**wrapped_term;
                    binding.ginsert(vterm.clone(), term.clone()); // A deep clone occurs here.
                }
                binding
            });
            //.inspect(|x| { println!("Initial bindings for the first constraint is {:?}", x); });

        binding_collection
    }
    
    pub fn split_binding<G>(
        &self,
        bindings: &Collection<G, QuickHashOrdMap<Term, Term>>, 
        left_keys:  OrdSet<Term>,
        right_keys: OrdSet<Term>,
    ) -> Collection<G, (QuickHashOrdMap<Term, Term>, QuickHashOrdMap<Term, Term>)>
    where 
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice,
    {
        bindings.map(move |mut binding| {
            let mut first = BTreeMap::new();
            let mut second = BTreeMap::new();
            let mut btree_map: BTreeMap<Term, Term> = binding.into();

            for k in left_keys.clone().into_iter() {
                if let Some(v) = btree_map.remove(&k) {
                    first.insert(k, v);
                }
            }

            for k in right_keys.clone().into_iter() {
                if let Some(v) = btree_map.remove(&k) {
                    second.insert(k, v);
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
    pub fn join_two_bindings<G>(&self, 
        prev_vars: OrdSet<Term>,
        prev_col: &Collection<G, QuickHashOrdMap<Term, Term>>, 
        new_vars: OrdSet<Term>,
        new_col: &Collection<G, QuickHashOrdMap<Term, Term>>
    ) 
    -> (OrdSet<Term>, Collection<G, QuickHashOrdMap<Term, Term>>)
    where
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice,
    {
        let mut prev_vars_extra = OrdSet::new();
        let mut new_vars_extra = OrdSet::new();

        // When binding one has `x.y` and binding two has `x.y.z` update binding one with `x.y.z` and vice versa.
        for prev_var in prev_vars.iter() {
            for new_var in new_vars.iter() {
                if prev_var.has_subterm(new_var).unwrap() && prev_var != new_var {
                    prev_vars_extra.insert(Arc::new(new_var.clone()));
                }
                else if new_var.has_subterm(prev_var).unwrap() && new_var != prev_var {
                    new_vars_extra.insert(Arc::new(prev_var.clone()));
                }
            }
        }

        let prev_vars_extra_copy = prev_vars_extra.clone();
        let updated_prev_col = prev_col.map(move |mut binding| {
            for prev_var in prev_vars_extra_copy.iter() {
                Term::update_binding(&prev_var.clone(), &mut binding);
            }
            binding
        });

        let new_vars_extra_copy = new_vars_extra.clone();
        let updated_new_col = new_col.map(move |mut binding| {
            for new_var in new_vars_extra_copy.iter() {
                Term::update_binding(&new_var.clone(), &mut binding);
            }
            binding
        });

        let mut updated_prev_vars: OrdSet<Term> = OrdSet::new();
        updated_prev_vars.extend(prev_vars);
        updated_prev_vars.extend(prev_vars_extra.into_iter().map(|x| { 
            let term: &Term = x.borrow();
            term.clone() 
        }));
        
        let mut updated_new_vars = OrdSet::new();
        updated_new_vars.extend(new_vars);
        updated_new_vars.extend(new_vars_extra.into_iter().map(|x| {
            let term: &Term = x.borrow();
            term.clone()
        }));

        let mut all_vars = OrdSet::new();
        all_vars.extend(updated_prev_vars.clone());
        all_vars.extend(updated_new_vars.clone());

        let (lvars, mvars, rvars) = Term::two_sets_intersection(updated_prev_vars, updated_new_vars);


        // Turn collection of [binding] into collection of [(middle, right)] for joins.
        let m_r_col = self.split_binding(&updated_new_col, mvars.clone(), rvars.clone());
        // m_r_col.inspect(|x| println!("m_r_col: {:?}", x));

        // Turn collection of [binding] into collection of [(middle, left)] for joins.
        let m_l_col = self.split_binding(&updated_prev_col, mvars.clone(), lvars.clone());
        // m_l_col.inspect(|x| println!("m_l_col: {:?}", x));

        let joint_collection = m_l_col
                        .join(&m_r_col)
            .map(move |(inter, (left, right))| {
                let mut binding = BTreeMap::new();
                let btree_inter: BTreeMap<Term, Term> = inter.into();
                let btree_left: BTreeMap<Term, Term> = left.into();
                let btree_right: BTreeMap<Term, Term> = right.into();
                binding.extend(btree_inter);
                binding.extend(btree_left);
                binding.extend(btree_right);
                binding.into()
            });

        // joint_collection.inspect(|x| println!("Join result: {:?}", x));
        (all_vars, joint_collection)
    } 

    pub fn dataflow_from_constraints<G>(
        &self, 
        models: &Collection<G, HashedTerm>, 
        constraints: &Vec<Constraint>
    ) 
    -> (OrdSet<Term>, Collection<G, QuickHashOrdMap<Term, Term>>)
    where
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice,
    {
        // Construct a headless rule from a list of constraints.
        let temp_rule = Rule::new(vec![], constraints.clone());
        let pos_preds = temp_rule.predicate_constraints();

        let default_vars: OrdSet<Term> = OrdSet::new();

        //let temp_rule2 = temp_rule.clone();
        //models.inspect(move |x| println!("Model for rule {}: {:?}", temp_rule2, x));

        // TODO: find a better way to create an empty collection.
        let default_col = models
            .filter(|_wrapped_term| false)
            .map(|_wrapped_term| {
                let empty_map = BTreeMap::new();
                let quick_hash_map: QuickHashOrdMap<Term, Term> = empty_map.into();
                quick_hash_map
            });

        // Join all positive predicate terms by their shared variables one by one in order.
        let (mut vars, collection) = pos_preds.into_iter()
            .fold((default_vars, default_col), |(prev_vars, prev_col), pred_constraint| {
                let pred: Predicate = pred_constraint.clone().try_into().unwrap();
                let term = pred.term.clone();
                let mut vars: OrdSet<Term> = OrdSet::new();
                vars.extend(term.variables().into_iter().map(|x| x.clone()));

                // Don't forget to add alias variable for the predicate constraint.
                if let Some(vterm) = pred.alias.clone() {
                    vars.insert(vterm);
                }
                
                // let constraint = pred_constraint.clone();
                // models.inspect(move |x| println!("Models before enter {}: {:?}", constraint, x));

                let col = self.dataflow_filtered_by_positive_predicate_constraint(
                    models, 
                    pred_constraint.clone(),
                );
                
                // col.inspect(move |x| println!("Changed matches for {}: {:?}", constraint, x));

                if prev_vars.len() != 0 {
                    return self.join_two_bindings(prev_vars, &prev_col, vars, &col);
                }
                else {
                    return (vars, col);
                }
            });

        // Let's assume every set comprehension must be explicitly declared with a variable on the left side 
        // of binary constraint.
        // e.g. x = count({a| a is A(b, c)}) before the aggregation result is used else where like x + 2 = 3.
        // Every derived variable must be declared once before used in other constraints 
        // e.g. y = (x + x) * x.
        let updated_collection = temp_rule.ordered_declaration_constraints().into_iter()
            .fold(collection, |outer_col, constraint| {
                let binary: Binary = constraint.clone().try_into().unwrap();
                let left_base_expr: BaseExpr = binary.left.try_into().unwrap();
                let var_term: Term = left_base_expr.try_into().unwrap();
                let var_term_copy = var_term.clone();

                match binary.right {
                    Expr::BaseExpr(right_base_expr) => {
                        match right_base_expr {
                            BaseExpr::SetComprehension(setcompre) => {
                                let updated_col = self.dataflow_from_set_comprehension(
                                    var_term.clone(),
                                    vars.clone(),
                                    &outer_col, 
                                    &models, 
                                    &setcompre
                                );
                                // Add declaration variable into the set of all vars after evaluation of setcompre
                                vars.insert(var_term.clone());
                                return updated_col;
                            },
                            _ => {}, // Ignored because it does not make sense to derive new term from single variable term.
                        }
                    },
                    Expr::ArithExpr(right_base_expr) => {
                        // Evaluate arithmetic expression to derive new term and add it to existing binding.
                        let updated_col = outer_col.map(move |binding_wrapper| {
                            let mut binding: BTreeMap<Term, Term> = binding_wrapper.into();
                            let num = right_base_expr.evaluate(&binding).unwrap();
                            let atom_term: Term = Atom::Int(num).into();
                            binding.insert(var_term.clone(), atom_term);
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
            let binary: Binary = constraint.clone().try_into().unwrap();
            let updated_col = col.filter(move |binding| {
                binary.evaluate(binding).unwrap()
            });
            return updated_col;
        });

        (vars, final_collection)
    }

    
    pub fn dataflow_from_set_comprehension<G>(
        &self,
        var: Term,
        outer_vars: OrdSet<Term>,
        ordered_outer_collection: &Collection<G, QuickHashOrdMap<Term, Term>>, // Binding collection from outer scope of set comprehension.
        models: &Collection<G, HashedTerm>, // Existing model collection.
        setcompre: &SetComprehension,
    ) -> Collection<G, QuickHashOrdMap<Term, Term>>
    where 
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice,
    {
        // Each binding from the input will enter into a separate scope when evaluating set comprehension.
        // e.g. B(A(a, b), k) :- A(a, b), k = count({x | x is A(a, b)}). k is always evaluated to 1 no matter 
        // how many terms of type A the current program has.
        let head_terms = setcompre.vars.clone();
        let setcompre_op = setcompre.op.clone();
        let mut setcompre_default = setcompre.default.clone();
        let constraints = &setcompre.condition;
        let mut setcompre_var = var.clone();
        
        // Evaluate constraints in set comprehension and return ordered binding collection.
        let (inner_vars, ordered_inner_collection) = self.dataflow_from_constraints(models, constraints);
        
        // models.inspect(|x| println!("Models for inner constraints: {:?}", x));
        // ordered_outer_collection.inspect(|x| println!("Outer collection {:?}", x));
        //ordered_inner_collection.inspect(|x| println!("Inner collection {:?}", x));
        //ordered_outer_collection.inspect(|x| println!("Outer collection {:?}", x));

        // If inner scope and outer scope don't have shared variables then it means they can be handled separately.
        match Term::has_deep_intersection(inner_vars.iter(), outer_vars.iter()) {
            false => {
                // Inner scope and outer scope have no shared variables then use the stream produced by set
                // comprehension to aggregate the terms in the set and return an integer.
                let aggregation_stream = ordered_inner_collection
                    .map(move |binding| {
                        let mut terms = vec![];
                        for head_term in head_terms.iter() {
                            let term = match head_term {
                                Term::Composite(c) => { head_term.propagate_bindings(&binding) },
                                Term::Variable(v) => { binding.gget(head_term).unwrap().clone() },
                                Term::Atom(a) => { head_term.clone() }
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
                            let mut binding = BTreeMap::new();
                            let num_term: Term = Atom::Int(nums.get(0).unwrap().clone().into()).into();
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
                                let mut binding: BTreeMap<Term, Term> = binding_wrapper.into();
                                let num_term: Term = Atom::Int(nums.get(0).unwrap().clone().into()).into();
                                binding.insert(setcompre_var.clone(), num_term);
                                binding.into()
                            })
                    }
                }
                //.inspect(|x| { println!("No variable sharing after reduce: {:?}", x); })
            },
            true => {
                // When inner scope and outer scope share some same variables.
                setcompre_var = var.clone();
                setcompre_default = setcompre.default.clone();

                let (all_vars, join_stream) = self.join_two_bindings(
                    outer_vars.clone(), 
                    ordered_outer_collection, 
                    inner_vars.clone(), 
                    &ordered_inner_collection
                );

                //join_stream.inspect(|x| println!("Joins result: {:?}", x));

                // println!("Split: outer_vars={:?}, inner_vars={:?}", outer_vars, inner_vars);

                // Take binding in the outer scope as key and bindings in inner scope are grouped by the key.
                let binding_and_aggregation_stream = self.split_binding(
                        &join_stream, outer_vars.clone(), inner_vars.clone()
                    )
                    //.inspect(|x| println!("Before reduce: {:?}", x))
                    .reduce(move |key, input, output| {
                        // Each outer binidng as key has points a list of derived terms from inner scope.
                        for (binding, count) in input.into_iter() {
                            for head_term in head_terms.iter() {
                                let term = match head_term {
                                    Term::Composite(_) => { head_term.propagate_bindings(*binding) },
                                    Term::Variable(_) => { binding.gget(head_term).unwrap().clone() },
                                    Term::Atom(_) => { head_term.clone() }
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
                        let mut binding: BTreeMap<Term, Term> = binding_wrapper.into();
                        // Take the first element in num list when operator is count, sum, maxAll, minAll.
                        let num_term: Term = Atom::Int(nums.get(0).unwrap().clone()).into();
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
                        let mut outer: BTreeMap<Term, Term> = outer_wrapper.into();
                        let num_term: Term = Atom::Int(setcompre_default.clone()).into();
                        outer.insert(setcompre_var.clone(), num_term);
                        outer.into()
                    });

                binding_with_aggregation_stream.concat(&binding_with_default_stream)
            }
        }
        //.inspect(|x| { println!("Aggregation result: {:?}", x); })
    }

    pub fn dataflow_from_single_rule<G>(&self, models: &Collection<G, HashedTerm>, rule: &Rule) 
    -> Collection<G, HashedTerm>
    where 
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice,
    {
        let head_terms = rule.get_head();
        let constraints = rule.get_body();

        models
            //.inspect(|x| println!("Model changes: {:?}", x))
            .iterate(|inner| {
                // inner.inspect(|x| println!("Inner model changes: {:?}", x));
                // let models = models.enter(&inner.scope());
                let (_, binding_collection) = self.dataflow_from_constraints(&inner, &constraints);
                let derived_terms = binding_collection
                    //.inspect(|x| println!("A binding map derived from constraints: {:?}", x))
                    .map(move |binding| {
                        let mut new_terms: Vec<HashedTerm> = vec![];
                        for head_term in head_terms.iter() {
                            // If head term has variable term then it means that's a constant.
                            if let Term::Variable(var) = head_term {
                                let constant_name = var.root.clone();
                                let constant = Term::create_constant(constant_name);
                                new_terms.push(constant.into());
                            } else {
                                let new_term = head_term.propagate_bindings(&binding);
                                new_terms.push(new_term.into());
                            }
                        }
                        new_terms
                    })
                    .flat_map(|x| x)
                    // .inspect(|x| println!("Derived head term: {:?}", x))
                    ;

                inner.concat(&derived_terms).distinct()
            })
            // .inspect(|x| println!("All terms derived in rule: {:?}", x))
    }

    pub fn create_dataflow<FM: FormulaModule>(
        &self, 
        module: &FM, 
        worker: &mut timely::worker::Worker<timely::communication::allocator::Thread>
    ) 
    -> (InputSession<i32, HashedTerm, isize>, timely::dataflow::ProbeHandle<i32>)
    {
        let mut input = InputSession::<i32, HashedTerm, isize>::new();
        let stratified_rules = module.stratified_rules();

        let probe = worker.dataflow(|scope| {
            // Make sure there are no model duplicates.
            let models = input.to_collection(scope)
                //.inspect(|x| println!("Model Input: {:?}", x))
                .distinct();

            let mut new_models = models.map(|x| x);

            for (i, stratum) in stratified_rules.into_iter().enumerate() {
                // Rules to be executed are from the same stratum and independent from each other.
                for rule in stratum.iter() {
                    if self.inspect {
                        println!("Rule at Stratum {}: {}", i, rule);
                    }
                    
                    let models_after_rule_execution = self.dataflow_from_single_rule(
                        &new_models, 
                        rule
                    );

                    new_models = new_models
                        .concat(&models_after_rule_execution)
                        .distinct();
                }

                if self.inspect {
                    new_models.inspect(move |x| { println!("Stratum {}: {:?}", &i, x); });
                }
            }

            new_models.probe()
        });

        (input, probe)
    }

}

