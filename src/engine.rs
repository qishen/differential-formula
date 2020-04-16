extern crate rand;
extern crate timely;
extern crate differential_dataflow;
extern crate serde;

use std::borrow::Borrow;
use std::sync::Arc;
use std::iter::*;
use std::vec::Vec;
use std::convert::TryInto;
use std::string::String;

use im::{OrdMap, OrdSet};

use timely::dataflow::Scope;
use timely::dataflow::operators::*;

use differential_dataflow::{Collection, ExchangeData};
use differential_dataflow::input::{Input, InputSession};
use differential_dataflow::operators::join::{Join, JoinCore};
use differential_dataflow::operators::*;

use crate::constraint::*;
use crate::term::*;
use crate::expression::*;
use crate::rule::*;
use crate::type_system::*;
use crate::parser::combinator::*;
use crate::util::GenericMap;



pub struct Session {
    worker: timely::worker::Worker<timely::communication::allocator::Thread>,
    input: InputSession<i32, Arc<Term>, isize>,
    probe: timely::dataflow::ProbeHandle<i32>,
    env: Option<Env>,
    domain_name: String,
    model_name: Option<String>,
    step_count: i32,
}

impl Session {
    fn new(input: InputSession<i32, Arc<Term>, isize>, 
            probe: timely::dataflow::ProbeHandle<i32>, 
            worker: timely::worker::Worker<timely::communication::allocator::Thread>,
            env: Option<Env>,
            domain_name: String,
            model_name: Option<String>,
        ) -> Self {

        Session {
            worker,
            input,
            probe,
            env,
            domain_name,
            model_name,
            step_count: 1,
        }
    }

    pub fn parse_term_str(&self, term_str: &str) -> Option<Arc<Term>> {
        // Call function from parser.
        parse_into_term(&self.env, self.domain_name.clone(), self.model_name.clone(), term_str)
    }

    fn _advance(&mut self) {
        self.input.advance_to(self.step_count);
        self.input.flush();
        while self.probe.less_than(&self.input.time()) {
            self.worker.step();
        }
        self.step_count += 1;
    }

    pub fn add_term(&mut self, term: Arc<Term>) {
        self.input.insert(term);
        self._advance();
    }

    pub fn add_terms(&mut self, terms: Vec<Arc<Term>>) {
        for term in terms {
            self.input.insert(term);
        }
        self._advance();
    }

    pub fn remove_term(&mut self, term: Arc<Term>) {
        self.input.remove(term);
        self._advance();
    }

    pub fn remove_terms(&mut self, terms: Vec<Arc<Term>>) {
        for term in terms {
            self.input.remove(term);
        }
        self._advance();
    }

    pub fn load_model(&mut self, model: Model) {
        self.add_terms(model.models);
    }
}


pub struct DDEngine {
    pub env: Option<Env>,
    pub inspect: bool,
}

impl DDEngine {
    pub fn new() -> Self {
        let engine = DDEngine {
            env: None,
            inspect: false,
        };

        engine
    }

    pub fn install(&mut self, env: Env) {
        self.env = Some(env);
    }

    pub fn parse_string(content: String) -> Env {
        let content_eof = format!("{} {}", content, " EOF");
        let env = parse_str(content_eof);
        env
    }


    pub fn parse_file() {

    }

    pub fn get_domain(&self, name: String) -> Option<&Domain> {
        let domain = match &self.env {
            None => None,
            Some(env) => env.get_domain_by_name(name),
        };

        domain
    }

    pub fn get_model(&self, name: String) -> Option<&Model> {
        let model = match &self.env {
            None => None,
            Some(env) => env.get_model_by_name(name),
        };

        model
    }

    pub fn create_session(&mut self, domain_name: &str, model_name: Option<&str>) -> Session {
        // Create a single thread worker.
        let allocator = timely::communication::allocator::Thread::new();
        let mut worker = timely::worker::Worker::new(allocator);

        let domain = self.get_domain(domain_name.to_string()).unwrap();
        let (mut input, probe) = self.create_dataflow(&domain.clone(), &mut worker);

        // TODO: Need to remove deep clone by adding lifetime to field with reference in Session.
        Session::new(
            input, 
            probe, 
            worker, 
            self.env.clone(), 
            domain_name.to_string(), 
            model_name.map(|x| { x.to_string() }),
        )
    }

    pub fn dataflow_filtered_by_type<G>(
        &self,
        terms: &Collection<G, Arc<Term>>,
        pred_term: Term,
    ) -> Collection<G, Arc<Term>>
    where
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice + Ord,
    {
        let c: Composite = pred_term.try_into().unwrap();
        terms
            .filter(move |term| {
                let term_ref: &Term = term.borrow();
                match term_ref {
                    Term::Composite(composite) => {
                        if c.sort == composite.sort { true } 
                        else { false }
                    }
                    _ => { false }
                }
            })
    }


    pub fn dataflow_filtered_by_pattern() {

    }


    pub fn dataflow_filtered_by_positive_predicate_constraint<G>(
        &self,
        terms: &Collection<G, Arc<Term>>, 
        pos_pred_constraint: Constraint,
    ) -> Collection<G, OrdMap<Arc<Term>, Arc<Term>>> 
    where 
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice + Ord,
    {
        let predicate: Predicate = pos_pred_constraint.try_into().unwrap();
        let pred_alias_arc = predicate.alias.map(|x| { Arc::new(x) });
        let pred_term = predicate.term.clone();

        let binding_collection = self.dataflow_filtered_by_type(terms, pred_term.clone()) 
            .map(move |term_arc| {
                let binding_opt = pred_term.get_ordered_bindings(&term_arc);
                (term_arc, binding_opt)
            })
            .filter(|(_, binding_opt)| {
                match binding_opt {
                    None => false,
                    _ => true,
                }
            })
            .map(move |(term_arc, binding_opt)| {
                // If predicate may have alias then add itself to existing binding.
                // TODO: what if alias variable is already in existing variables.
                let mut binding = binding_opt.unwrap();
                if let Some(vterm_arc) = &pred_alias_arc {
                    //binding.insert(vterm, &term);
                    binding.ginsert(vterm_arc.clone(), term_arc);
                }

                binding
            });
            //.inspect(|x| { println!("Initial bindings for the first constraint is {:?}", x); });

        binding_collection
    }

    pub fn split_binding<G>(
        &self,
        bindings: &Collection<G, OrdMap<Arc<Term>, Arc<Term>>>, 
        left_keys:  OrdSet<Term>,
        right_keys: OrdSet<Term>,
    ) -> Collection<G, (OrdMap<Arc<Term>, Arc<Term>>, OrdMap<Arc<Term>, Arc<Term>>)>
    where 
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice,
    {
        let map_tuple_collection = bindings.map(move |mut binding| {
            let mut first = OrdMap::new();
            let mut second = OrdMap::new();

            for var in left_keys.clone().iter() {
                if let Some((k, v)) = binding.remove_with_key(var) {
                    first.insert(k, v);
                }
            }

            for var in right_keys.clone().iter() {
                if let Some((k, v)) = binding.remove_with_key(var) {
                    second.insert(k, v);
                }
            }

            (first, second)
        });

        map_tuple_collection
    }

    /// Join two binding (OrdMap) collections by considering the same or related variable terms. Split each binding into
    /// two bindings: One with the shared variables and the other with non-shared variable. Before the splitting we have to
    /// do an extra step to extend the current binding with the variable subterms from the other collection of binding.
    /// if binding1 has key `x.y` and binding2 has key `x.y.z` and `x`, then binding1 must have `x.y.z` as well but won't 
    /// have `x` because you can't derive parent term from subterm backwards. On the other hand binding2 must have `x.y` too.
    pub fn join_two_bindings<G>(&self, 
        prev_vars: OrdSet<Term>,
        prev_col: &Collection<G, OrdMap<Arc<Term>, Arc<Term>>>, 
        new_vars: OrdSet<Term>,
        new_col: &Collection<G, OrdMap<Arc<Term>, Arc<Term>>>
    ) 
    -> (OrdSet<Term>, Collection<G, OrdMap<Arc<Term>, Arc<Term>>>)
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

        // Turn collection of [binding] into collection of [(middle, left)] for joins.
        let m_l_col = self.split_binding(&updated_prev_col, mvars.clone(), lvars.clone());

        let joint_collection = m_l_col
                        .join(&m_r_col)
            .map(move |(inter, (left, right))| {
                let mut binding = OrdMap::new();
                binding.extend(left);
                binding.extend(right);
                binding.extend(inter);
                binding
            });

        (all_vars, joint_collection)
    } 

    pub fn dataflow_from_constraints<G>(
        &self, 
        models: &Collection<G, Arc<Term>>, 
        constraints: &Vec<Constraint>
    ) 
    -> (OrdSet<Term>, Collection<G, OrdMap<Arc<Term>, Arc<Term>>>)
    where
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice,
    {
        // Construct a headless rule from a list of constraints.
        let temp_rule = Rule::new(vec![], constraints.clone());
        let pos_preds = temp_rule.pos_preds();

        let default_vars: OrdSet<Term> = OrdSet::new();
        // TODO: find a better way to create an empty collection.
        let default_col = models
            .filter(|x| false)
            .map(|x| {
                OrdMap::new()
            });

        // Join all positive predicate terms by their shared variables one by one in order.
        let (mut vars, mut collection) = pos_preds.iter().fold((default_vars, default_col), |(prev_vars, prev_col), pred_constraint| {
            let pred: Predicate = pred_constraint.clone().try_into().unwrap();
            let term = pred.term.clone();
            let mut vars: OrdSet<Term> = OrdSet::new();
            vars.extend(term.variables().into_iter().map(|x| x.clone()));

            // Don't forget to add alias variable for the predicate constraint.
            if let Some(vterm) = pred.alias.clone() {
                vars.insert(vterm);
            }
            
            let col = self.dataflow_filtered_by_positive_predicate_constraint(
                models, 
                pred_constraint.clone(),
            );

            if prev_vars.len() != 0 {
                return self.join_two_bindings(prev_vars, &prev_col, vars, &col);
            }
            else {
                return (vars, col);
            }

        });

        for bin_constraint in temp_rule.ordered_definition_constraints().into_iter() {
            let binary: Binary = bin_constraint.clone().try_into().unwrap();
            // Let's assume every set comprehension must be explicitly declared with a variable on the left side of binary constraint.
            // e.g. x = count({a| a is A(b, c)}) before the aggregation result is used else where like x + 2 = 3.

            // Every derived variable must be declared once before used in other constraints 
            // e.g. y = (x + x) * x.
            let left_base_expr: BaseExpr = binary.left.try_into().unwrap();
            let var_term: Term = left_base_expr.try_into().unwrap();
            let var_term_arc = Arc::new(var_term.clone());

            // Add the definition term into the list of all variable terms in current rule.
            vars.insert(var_term.clone());

            match binary.right {
                Expr::BaseExpr(right_base_expr) => {
                    match right_base_expr {
                        BaseExpr::SetComprehension(setcompre) => {
                            collection = self.dataflow_from_set_comprehension(
                                var_term.clone(),
                                vars.clone(),
                                &collection, 
                                models, 
                                &setcompre
                            );
                        },
                        _ => {}, // Ignored because it does not make sense to derive new term from single variable term.
                    }
                },
                Expr::ArithExpr(right_base_expr) => {
                    // Evaluate arithmetic expression to derive new term and add it to existing binding.
                    collection = collection.map(move |mut binding| {
                        let num = right_base_expr.evaluate(&binding).unwrap();
                        let atom_term: Term = Atom::Int(num).into();
                        binding.insert(var_term_arc.clone(), Arc::new(atom_term));
                        binding
                    });
                }
            }
        }

        // Since there are no derived terms then directly evaluate the expression to filter binding collection.
        for bin_constraint in temp_rule.pure_constraints().into_iter() {
            let binary: Binary = bin_constraint.clone().try_into().unwrap();
            collection = collection.filter(move |binding| {
                binary.evaluate(binding).unwrap()
            });
        }

        (vars, collection)
    }

    
    pub fn dataflow_from_set_comprehension<G>(
        &self,
        var: Term,
        outer_vars: OrdSet<Term>,
        ordered_outer_collection: &Collection<G, OrdMap<Arc<Term>, Arc<Term>>>, // Binding collection from outer scope of set comprehension.
        models: &Collection<G, Arc<Term>>, // Existing model collection.
        setcompre: &SetComprehension,
    ) -> Collection<G, OrdMap<Arc<Term>, Arc<Term>>>
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
        let mut setcompre_var = Arc::new(var.clone());
        
        // Evaluate constraints in set comprehension and return ordered binding collection.
        let (inner_vars, ordered_inner_collection) = self.dataflow_from_constraints(models, constraints);

        // If inner scope and outer scope don't have shared variables then it means they can be handled separately.
        match Term::has_deep_intersection(inner_vars.iter(), outer_vars.iter()) {
            false => {
                let aggregation_stream = ordered_inner_collection
                    .map(|x| { ((), x) })
                    .reduce(move |key, input, output| {
                        let mut terms = vec![];
                        for (binding, count) in input.iter() {
                            for head_term in head_terms.iter() {
                                let term = match head_term {
                                    Term::Composite(c) => { head_term.propagate_bindings(*binding).unwrap() },
                                    Term::Variable(v) => { binding.gget(head_term).unwrap().clone() },
                                    Term::Atom(a) => { Arc::new(head_term.clone()) }
                                };
                                terms.push((term, count));
                            }
                        }

                        let aggregated_result = setcompre_op.aggregate(terms);
                        output.push((vec![aggregated_result], 1));
                    });

                ordered_outer_collection
                    .map(|x| { ((), x) })
                    .join(&aggregation_stream)
                    .map(move |(_, (mut binding, nums))| {
                        // Take the first element in num list when operator is count, sum, maxAll, minAll.
                        let num_term: Term = Atom::Int(nums.get(0).unwrap().clone()).into();
                        binding.insert(setcompre_var.clone(), Arc::new(num_term));
                        binding
                    })
            },
            true => {
                setcompre_var = Arc::new(var.clone());
                setcompre_default = setcompre.default.clone();

                let (all_vars, join_stream) = self.join_two_bindings(
                    outer_vars.clone(), 
                    ordered_outer_collection, 
                    inner_vars.clone(), 
                    &ordered_inner_collection
                );

                // Take binding in the outer scope as key and bindings in inner scope are grouped by the key.
                let binding_and_aggregation_stream = self.split_binding(&join_stream, outer_vars.clone(), inner_vars.clone())
                    .reduce(move |key, input, output| {
                        let mut terms = vec![];
                        for (binding, count) in input.iter() {
                            for head_term in head_terms.iter() {
                                let term = match head_term {
                                    Term::Composite(c) => { head_term.propagate_bindings(*binding).unwrap() },
                                    Term::Variable(v) => { binding.gget(head_term).unwrap().clone() },
                                    Term::Atom(a) => { Arc::new(head_term.clone()) }
                                };
                                terms.push((term, count));
                            }
                        }

                        let aggregated_result = setcompre_op.aggregate(terms);
                        output.push((vec![aggregated_result], 1));
                    });

                let binding_with_aggregation_stream = binding_and_aggregation_stream
                    .map(move |(mut binding, nums)| {
                        // Take the first element in num list when operator is count, sum, maxAll, minAll.
                        let num_term: Term = Atom::Int(nums.get(0).unwrap().clone()).into();
                        binding.insert(setcompre_var.clone(), Arc::new(num_term));
                        binding

                        // When operator is topk or bottomk.
                        // TODO: return a list of numeric values but how?
                    });

                setcompre_var = Arc::new(var.clone());
                setcompre_default = setcompre.default.clone();
                // Find the stream of binding in outer scope that does no contribution to the aggregation result
                // Then simply add default aggregation value into the binding.
                let binding_with_default_stream = ordered_outer_collection.map(|x| (x, true))
                    .antijoin(&binding_and_aggregation_stream.map(|(x, aggregation)| { x }))
                    .map(move |(mut outer, _)| {
                        // Add default value of set comprehension to each binding.
                        let num_term: Term = Atom::Int(setcompre_default.clone()).into();
                        outer.insert(setcompre_var.clone(), Arc::new(num_term));
                        outer 
                    });

                binding_with_aggregation_stream.concat(&binding_with_default_stream)
            }
        }
    }

    pub fn dataflow_from_single_rule<G>(&self, models: &Collection<G, Arc<Term>>, rule: &Rule) 
    -> Collection<G, Arc<Term>>
    where 
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice,
    {
        let head_terms = rule.get_head();
        let constraints = rule.get_body();

        models
            .iterate(|inner| {
                //let models = models.enter(&inner.scope());
                let (_, binding_collection) = self.dataflow_from_constraints(&inner, &constraints);
                let derived_terms = binding_collection
                    .map(move |binding| {
                        let mut new_terms: Vec<Arc<Term>> = vec![];
                        for head_term in head_terms.iter() {
                            let mut new_term = head_term.propagate_bindings(&binding).unwrap();
                            new_terms.push(new_term);
                        }
                        new_terms
                    })
                    .flat_map(|x| x);

                inner.concat(&derived_terms).distinct()
            })
    }

    pub fn create_dataflow(
        &mut self, 
        domain: &Domain, 
        worker: &mut timely::worker::Worker<timely::communication::allocator::Thread>
    ) 
    -> (InputSession<i32, Arc<Term>, isize>, timely::dataflow::ProbeHandle<i32>)
    {
        let mut input = InputSession::<i32, Arc<Term>, isize>::new();
        let stratified_rules = domain.stratified_rules();

        let probe = worker.dataflow(|scope| {
            // models are updated after execution of rules from each stratum.
            let models = input.to_collection(scope).distinct();
            let mut new_models = models.map(|x| x);

            for (i, stratum) in stratified_rules.into_iter().enumerate() {
                // Rules to be executed are from the same stratum and independent from each other.
                for rule in stratum.iter() {
                    if self.inspect {
                        println!("Stratum {}: {}", i, rule);
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

