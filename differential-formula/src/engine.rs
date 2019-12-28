extern crate rand;
extern crate timely;
extern crate differential_dataflow;
extern crate serde;

use core::hash::Hash;
use core::borrow::Borrow;

use im::OrdMap;
use rand::{Rng, SeedableRng, StdRng};
use std::iter::*;
use std::any::Any;
use std::rc::Rc;
use std::sync::Arc;
use std::vec::Vec;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::string::String;

use enum_dispatch::enum_dispatch;
use abomonation::Abomonation;
use serde::*;
use num::*;

use timely::dataflow::Scope;
use timely::dataflow::operators::*;
use timely::dataflow::operators::capture::Extract;
use timely::dataflow::operators::feedback::*;
use timely::Configuration;

use differential_dataflow::{Collection, ExchangeData};
use differential_dataflow::input::{Input, InputSession};
use differential_dataflow::difference::Monoid;
use differential_dataflow::operators::join::{Join, JoinCore};
use differential_dataflow::operators::*;
use differential_dataflow::collection::AsCollection;

use crate::constraint::*;
use crate::term::*;
use crate::expression::*;
use crate::rule::*;
use crate::type_system::*;
use crate::parser::parse_str;



pub struct Session {
    worker: timely::worker::Worker<timely::communication::allocator::Thread>,
    input: InputSession<i32, Term, isize>,
    probe: timely::dataflow::ProbeHandle<i32>,
    domain: Domain,
    step_count: i32,
}

impl Session {
    fn new(input: InputSession<i32, Term, isize>, 
            probe: timely::dataflow::ProbeHandle<i32>, 
            domain: Domain,
            worker: timely::worker::Worker<timely::communication::allocator::Thread>
        ) -> Self {

        Session {
            worker,
            input,
            probe,
            domain,
            step_count: 1,
        }
    }

    fn parse_term(term_str: String) {

    }

    fn _advance(&mut self) {
        self.input.advance_to(self.step_count);
        self.input.flush();
        while self.probe.less_than(&self.input.time()) {
            self.worker.step();
        }
        self.step_count += 1;
    }

    pub fn add_term(&mut self, term: Term) {
        self.input.insert(term);
        self._advance();
    }

    pub fn add_terms(&mut self, terms: Vec<Term>) {
        for term in terms {
            self.input.insert(term);
        }
        self._advance();
    }

    pub fn remove_term(&mut self, term: Term) {
        self.input.remove(term);
        self._advance();
    }

    pub fn remove_terms(&mut self, terms: Vec<Term>) {
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
}

impl DDEngine {
    pub fn new() -> Self {
        let engine = DDEngine {
            env: None,
        };

        engine
    }

    pub fn install(&mut self, env: Env) {
        self.env = Some(env);
    }

    pub fn parse_string(content: &str) -> Env {
        let content_eof = format!("{} {}", content, " EOF");
        let env = parse_str(&content_eof[..]);
        env
    }


    pub fn parse_file() {

    }

    pub fn get_domain(&self, name: String) -> Option<Domain> {
        let domain = match &self.env {
            None => None,
            Some(env) => env.get_domain_by_name(name),
        };

        domain
    }

    pub fn get_model(&self, name: String) -> Option<Model> {
        let model = match &self.env {
            None => None,
            Some(env) => env.get_model_by_name(name),
        };

        model
    }

    pub fn create_session(&mut self, domain_name: String) -> Session {
        // Create a single thread worker.
        let allocator = timely::communication::allocator::Thread::new();
        let mut worker = timely::worker::Worker::new(allocator);

        let domain = self.get_domain(domain_name).unwrap();
        let (mut input, probe) = self.create_dataflow(&domain, &mut worker);

        Session::new(input, probe, domain, worker)
    }

    pub fn dataflow_filtered_by_type<G>(
        terms: &Collection<G, Term>,
        pred_term: Term,
    ) -> Collection<G, Term>
    where
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice + Ord,
    {
        let c: Composite = pred_term.try_into().unwrap();
        terms
            .map(|term| {
                let composite: Composite = term.try_into().unwrap();
                composite
            })
            .filter(move |composite| {
                // Filter step to get all models with specific type.
                c.sort == composite.sort
            })
            .map(|composite| composite.into())
    }


    pub fn dataflow_filtered_by_positive_predicate_constraint<G>(
        terms: &Collection<G, Term>, 
        pos_pred_constraint: Constraint,
    ) -> Collection<G, OrdMap<Term, Term>> 
    where 
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice + Ord,
    {
        let predicate: Predicate = pos_pred_constraint.try_into().unwrap();
        let pred_alias = predicate.alias;
        let pred_term = predicate.term.clone();
        let pred_term_copy = pred_term.clone();

        DDEngine::dataflow_filtered_by_type(terms, pred_term_copy) 
            .map(move |term| {
                // Get a hash map mapping variable to term based on matching term. 
                // TODO: Handle situations when no binding is found and return None.
                let mut binding = pred_term.get_ordered_bindings(&term).unwrap();

                // Predicate may have alias, if so add itself to existing binding.
                // TODO: what if alias variable is already in existing variables.
                if let Some(vterm) = &pred_alias {
                    binding.insert(vterm.clone(), term);
                }

                binding
            })
            //.inspect(|x| { println!("Initial bindings for the first constraint is {:?}", x); });
    }

    pub fn dataflow_from_term_bindings_split<G>(
        bindings: &Collection<G, OrdMap<Term, Term>>, 
        keys1: Vec<Term>,
        keys2: Vec<Term>,
    ) -> Collection<G, (OrdMap<Term, Term>, OrdMap<Term, Term>)>
    where 
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice,
    {
        let collection_of_two_vectors = bindings.map(move |mut binding| {
            let mut first = OrdMap::new();
            let mut second = OrdMap::new();

            for var in keys1.iter() {
                if let Some((k, v)) = binding.remove_with_key(var) {
                    first.insert(k, v);
                }
            }

            for var in keys2.iter() {
                if let Some((k, v)) = binding.remove_with_key(var) {
                    second.insert(k, v);
                }
            }

            (first, second)
        });

        collection_of_two_vectors
    }

    // Return a filtered binding collection by considering negative predicate.
    pub fn dataflow_filtered_by_negative_predicate_constraint<G>(
        models: &Collection<G, Term>, 
        prev_collection: &Collection<G, OrdMap<Term, Term>>,
        prev_vars: Vec<Term>,
        neg_pred_constraint: Constraint,
    ) -> Collection<G, OrdMap<Term, Term>>
    where
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice,
    {
        let neg_pred: Predicate = neg_pred_constraint.try_into().unwrap();
        let mut neg_pred_term = neg_pred.term.clone();
        let neg_term_composite: Composite = neg_pred.term.clone().try_into().unwrap();
        let mut neg_term_vars = neg_pred.term.variables();

        // Temporarily derive all terms for negative predicate.
        let neg_term_collection = prev_collection.map(move |binding| {
            neg_pred_term.propagate_bindings(&binding)
        });

        // We only care about models of the type of negative predicate.
        neg_pred_term = neg_pred.term.clone();
        let filtered_old_models = DDEngine::dataflow_filtered_by_type(models, neg_pred_term); 

        // Substract existing models from the derived terms 
        // of negative predicate.
        let derived_terms_with_old_models_substracted = neg_term_collection
            .map(|x| (x, true))
            .antijoin(&filtered_old_models)
            .map(|(x,_)| x);
        
        // Refill the moved variable to be used in closure again.
        neg_pred_term = neg_pred.term.clone();
        neg_term_vars = neg_term_vars.clone();
        // Turn binding into vector in the order of term variables.
        let vector_from_neg_bindings = derived_terms_with_old_models_substracted
            .map(move |x| {
                let mut binding = neg_pred_term.get_bindings(&x);
                binding
            })
            .filter(|binding| {
                // Filter out terms that cannot be binded to negative term.
                binding != &None
            })
            .map(move |mut binding_or_none| {
                let mut list = vec![];
                let mut binding = binding_or_none.unwrap();
                for var in neg_term_vars.iter() {
                    let val = binding.remove(var).unwrap();
                    list.push(val);
                }
                list
            });
        
        // split vars into two vectors, one with all vars in negative term, the other one with the rest.
        let mut other_vars_set: HashSet<Term> = HashSet::from_iter(prev_vars.clone());
        for var in neg_pred.term.variables().iter() {
            other_vars_set.remove(var);
        }

        let other_vars = Vec::from_iter(other_vars_set);
        let other_vars_copy = other_vars.clone();
        
        let neg_vars_set = neg_pred.term.variables();
        let neg_vars = Vec::from_iter(neg_vars_set);
        let neg_vars_copy = neg_vars.clone();

        // Filter binding collection to exclude those bindings that derive some negative predicate terms 
        // that already exist in previous model term collection.
        let filtered_bindings = prev_collection
            .map(move |mut binding| {
                let mut key = vec![];
                let mut other = vec![];

                for var in neg_vars.iter() {
                    let val = binding.remove(var).unwrap();
                    key.push(val);
                }

                for var in other_vars.iter() {
                    let val = binding.remove(var).unwrap();
                    other.push(val);
                }
                
                // `key` contains terms mapped from all variables in negative term, 
                // `other` contains terms mapped from the rest variables excluding those in negative term.
                (key, other)
            })
            .join(&vector_from_neg_bindings.map(|x| (x, true)))
            .map(move |(key, (other, _))| {
                // Combine two vectors back into a single binding after join operation.
                let mut binding = OrdMap::new();
                let map_from_two_lists = |map: &mut OrdMap<Term, Term>, key: Vec<Term>, value: Vec<Term>| {
                    let mut value_iter = value.into_iter();
                    for var in key.into_iter() {
                        map.insert(var, value_iter.next().unwrap());
                    }
                };
                map_from_two_lists(&mut binding, neg_vars_copy.clone(), key);
                map_from_two_lists(&mut binding, other_vars_copy.clone(), other);
                binding
            });
        
        filtered_bindings
    }


    pub fn dataflow_from_constraints<G>(models: &Collection<G, Term>, constraints: &Vec<Constraint>) 
        -> (Vec<Term>, Collection<G, OrdMap<Term, Term>>)
    where
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice,
    {
        // Construct a headless rule from a list of constraints.
        let rule = Rule { head: vec![], body: constraints.clone() };
        let pos_preds = rule.pos_preds();
        let neg_preds = rule.neg_preds();
        let mut pos_preds_iterator = pos_preds.into_iter();

        // Rule execution needs at least one positive predicate to start with.
        let initial_pred_constraint = pos_preds_iterator.next().unwrap();
        let initial_pred: Predicate = initial_pred_constraint.clone().try_into().unwrap();
        let initial_term = initial_pred.term;

        // A list of binded variables.
        let mut prev_vars: Vec<Term> = Vec::new();
        prev_vars.extend(initial_term.variables());

        // Don't forget to add alias variable for the first predicate constraint.
        if let Some(vterm) = initial_pred.alias {
            prev_vars.push(vterm);
        }
        prev_vars.sort();

        // Filter models by its type to match the positive predicate term and the output of dataflow
        // is not models but a collection of bindings mapping each variable to a ground term.
        let mut prev_collection = DDEngine::dataflow_filtered_by_positive_predicate_constraint(
            models, 
            initial_pred_constraint,
        );

        while let Some(pos_constraint) = pos_preds_iterator.next() {
            // Take constraint in default order and further extends existing bindings in the collection.
            let rule_pred: Predicate = pos_constraint.clone().try_into().unwrap();
            let rule_pred_term = rule_pred.term.clone();
            let rule_pred_alias = rule_pred.alias;
            let mut rule_pred_vars = rule_pred_term.variables();

            // alias of positive predicate needs to be considered as a variable.
            if let Some(alias) = rule_pred_alias {
                rule_pred_vars.insert(alias);
            }

            // List of variables are sorted in default order.
            let (left_diff_vars, intersection_vars, right_diff_vars) = 
                Term::terms_intersection(
                    HashSet::from_iter(prev_vars.clone()), 
                    rule_pred_vars,
                );

            let left_diff_vars_copy = left_diff_vars.clone();
            let mut right_diff_vars_copy = right_diff_vars.clone();
            let intersection_vars_copy = intersection_vars.clone();
            
            // Filter term collection by predicate constraint.
            let filtered_collection = DDEngine::dataflow_filtered_by_positive_predicate_constraint(
                models, 
                pos_constraint,
            );

            // Split bindings of current term into two vectors for join operation later.
            let split_collection = DDEngine::dataflow_from_term_bindings_split(
                &filtered_collection, 
                intersection_vars.clone(),
                right_diff_vars.clone(),
            );

            // Split bindings of previous terms into two vectors for join operation later.
            let split_prev_collection = DDEngine::dataflow_from_term_bindings_split(
                &prev_collection, 
                intersection_vars.clone(), 
                left_diff_vars.clone(),
            );

            // Join operation cannot be performed on HashMap of terms, we use vec of terms instead
            // and have to turn three lists back into HashMap for the next iteration.
            prev_collection = split_prev_collection
                .join(&split_collection)
                .map(move |(inter, (left, right))| {
                    let mut binding = OrdMap::new();
                    binding.extend(left);
                    binding.extend(right);
                    binding.extend(inter);
                    binding
                });
                //.inspect(|x| { print!("Join result is {:?}\n", x); });

            // Filter out binding with conflict on variables with fragments like x.y.z
            prev_collection = prev_collection.filter(|mut binding| {
                for var in binding.keys() {
                    let variable: Variable = var.clone().try_into().unwrap();
                    let root_var = var.root_var();
                    // Check if variable has fragments and root variable exists in binding.
                    if var != &root_var && binding.contains_key(&root_var) {
                        // TODO: too much clones here.
                        let root_value = binding.get(&root_var).unwrap().clone();
                        let composite: Composite = root_value.try_into().unwrap();
                        let sub_value = composite.get_subterm_by_labels(&variable.fragments).unwrap();
                        if &sub_value == binding.get(var).unwrap() { return true; } 
                        else { return false; }
                    }
                }

                true
            });

            // Refill variable after its value was moved and update existing binded variable list.
            right_diff_vars_copy = right_diff_vars.clone();
            prev_vars.extend(right_diff_vars_copy);
        } 

        // Filter binding collection by negative predicate constraints.
        for neg_pred_constraint in neg_preds.into_iter() {
            prev_collection = DDEngine::dataflow_filtered_by_negative_predicate_constraint(
                &models, 
                &prev_collection, 
                prev_vars.clone(), 
                neg_pred_constraint
            );
        } 


        for bin_constraint in rule.binary_constraints_with_derived_term().into_iter() {
            let binary: Binary = bin_constraint.try_into().unwrap();
            // Let's assume every set comprehension must be explicitly declared with a variable on the left side of binary constraint.
            // e.g. x = count({a| a is A(b, c)}) before the aggregation result is used else where like x + 2 = 3.

            // Every derived variable must be declared once before used in other constraints 
            // e.g. y = (x + x) * x.
            let left_base_expr: BaseExpr = binary.left.try_into().unwrap();
            let var_term: Term = left_base_expr.try_into().unwrap();

            match binary.right {
                Expr::BaseExpr(right_base_expr) => {
                    match right_base_expr {
                        BaseExpr::SetComprehension(setcompre) => {
                            prev_collection = DDEngine::dataflow_from_set_comprehension(
                                var_term,
                                &prev_collection, 
                                models, 
                                &setcompre
                            );
                        },
                        _ => {}, // Ignored because it does not make sense to derive new term from single variable term.
                    }
                },
                Expr::ArithExpr(right_base_expr) => {
                    // Evaluate arithmetic expression to derive new term and add it to existing binding.
                    prev_collection = prev_collection.map(move |mut binding| {
                        let num = right_base_expr.evaluate(&binding).unwrap();
                        let atom_term: Term = Atom::Int(num).into();
                        binding.insert(var_term.clone(), atom_term);
                        binding
                    });
                }
            }
        }

        // Since there are no derived terms then directly evaluate the expression to filter binding collection.
        for bin_constraint in rule.binary_constraints_without_derived_term().into_iter() {
            let binary: Binary = bin_constraint.try_into().unwrap();
            prev_collection = prev_collection.filter(move |binding| {
                binary.evaluate(binding).unwrap()
            });
        }

        (prev_vars.clone(), prev_collection)
            //.inspect(|x| {
            //    println!("binding is {:?}", x);  
            //})
    }

    
    pub fn dataflow_convert_map_to_vec<G>(
        vars: Vec<Term>,
        collection: &Collection<G, HashMap<Term, Term>>
    ) -> Collection<G, Vec<Term>> 
    where
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice,
    {
        // Map hash map to vector because cannot do join on hash map.
        collection.map(move |mut x| {
            let mut list = vec![];
            for var in vars.clone().into_iter() {
                if x.contains_key(&var) {
                    let value = x.remove(&var).unwrap();
                    list.push(value);
                }
            }

            list
        })
    }


    pub fn dataflow_from_set_comprehension<G>(
        var: Term,
        outer_collection: &Collection<G, OrdMap<Term, Term>>,
        models: &Collection<G, Term>, 
        setcompre: &SetComprehension,
    ) -> Collection<G, OrdMap<Term, Term>>
    where 
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice,
    {
        // Each binding from the input will enter into a separate scope when evaluating set comprehension.
        // e.g. B(A(a, b), k) :- A(a, b), k = count({x | x is A(a, b)}). k is always evaluated to 1 no matter 
        // how many terms of type A the current program has.
        let head_terms = setcompre.vars.clone();
        let setcompre_op = setcompre.op.clone();
        let constraints = &setcompre.condition;
        let (vars, mut collection) = DDEngine::dataflow_from_constraints(models, constraints);

        let ordered_outer_collection = outer_collection.map(|x| {
            let ordered_map: OrdMap<Term, Term> = OrdMap::from_iter(x.into_iter());
            (true, ordered_map)
        });

        let ordered_collection = collection.map(|x| {
            let ordered_map: OrdMap<Term, Term> = OrdMap::from_iter(x.into_iter());
            (true, ordered_map)
        });

        let aggregation_stream = ordered_outer_collection.join(&ordered_collection)
            .map(move |(_, (outer, inner))| { (outer, inner) })
            .reduce(move |key, input, output| {
                // Collect all derived terms in set comprehension.
                let mut terms = vec![];
                for (binding, count) in input.iter() {
                    for head_term in head_terms.clone() {
                        // TODO: Deep clone may slow down performance.
                        let term = head_term.propagate_bindings(&binding.clone().clone());
                        terms.push((term, count));
                    }
                }

                match setcompre_op {
                    SetCompreOp::Count => {
                        let mut num = BigInt::from_i64(0 as i64).unwrap();
                        for (term, count) in terms {
                            num += count.clone() as i64;
                        }                        
                        output.push((num, 1));
                    },
                    SetCompreOp::Sum => {
                        let mut sum = BigInt::from_i64(0).unwrap();
                        for (term, count) in terms {
                            let atom: Atom = term.clone().clone().try_into().unwrap();
                            match atom {
                                Atom::Int(i) => { sum += i * count; },
                                _ => {},
                            };
                        }
                        output.push((sum, 1));
                    },
                    SetCompreOp::MaxAll => {
                        let mut max = BigInt::from_i64(std::isize::MIN as i64).unwrap();
                        for (term, count) in terms {
                            let atom: Atom = term.clone().clone().try_into().unwrap();
                            match atom {
                                Atom::Int(i) => { if i > max { max = i; } },
                                _ => {},
                            };
                        }
                        output.push((max, 1));
                    },
                    _ => {
                        let mut min = BigInt::from_i64(std::isize::MAX as i64).unwrap();
                        for (term, count) in terms {
                            let atom: Atom = term.clone().clone().try_into().unwrap();
                            match atom {
                                Atom::Int(i) => { if i < min { min = i; } },
                                _ => {},
                            };
                        }
                        output.push((min, 1));
                    },
                };

            }).map(move |(mut binding, num)| {
                let num_term: Term = Atom::Int(num).into();
                binding.insert(var.clone(), num_term);
                binding
            });

        aggregation_stream
            .inspect(|x| { println!("Aggregation result added into binding {:?}", x); })
    }

    pub fn dataflow_from_single_rule<G>(models: &Collection<G, Term>, rule: &Rule) -> Collection<G, Term>
    where 
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice,
    {
        let head_terms = rule.head.clone();

        // Iteratively add new derived models to dataflow until fix point is reached.
        let models_after_rule_execution = models
        .iterate(|transitive_models| {
            // Put models into subscope without changing.
            // let models = models.enter(&transitive_models.scope());
            
            // Filter models by constraints and return a collection of variable binding.
            let constraints = &rule.body;
            let (vars, binding_collection) = DDEngine::dataflow_from_constraints(transitive_models, constraints);

            // Now we have a collection of bindings for all variables in the rule body, then we use each 
            // binding in the collection to derive new terms from head by propagating bindings. 
            let mut combined_models = transitive_models.map(|x| x);
            for term in head_terms.into_iter() {
                let headterm_stream = binding_collection
                    .map(move |binding| {
                        term.propagate_bindings(&binding)
                    })
                    .inspect(|x| {
                        println!("Updates on new derived terms are {:?}.", x);
                    });

                combined_models = combined_models.concat(&headterm_stream);
            }

            // Iteratively loop the dataflow until all models are different.
            combined_models.distinct()
        });
        //.inspect(|x| { println!("Final output {:?}", x); });

        models_after_rule_execution
    }


    pub fn create_dataflow(&mut self, domain: &Domain, worker: &mut timely::worker::Worker<timely::communication::allocator::Thread>) -> (
        InputSession<i32, Term, isize>, 
        timely::dataflow::ProbeHandle<i32>,
    )
    {
        let mut input = InputSession::<i32, Term, isize>::new();
        let stratified_rules = domain.stratified_rules();

        let probe = worker.dataflow(|scope| {
            // models are updated after execution of rules from each stratum.
            let models = input.to_collection(scope).distinct();
            let mut new_models = models.map(|x| x);

            for (i, stratum) in stratified_rules.into_iter().enumerate() {
                // Rules to be executed are from the same stratum and independent from each other.
                for rule in stratum.iter() {
                    println!("Stratum {}: {}", i, rule);

                    let models_after_rule_execution = DDEngine::dataflow_from_single_rule(
                        &new_models, 
                        rule
                    );

                    new_models = new_models
                        .concat(&models_after_rule_execution)
                        .distinct();
                }

                new_models
                    .inspect(move |x| { println!("Stratum {}: {:?}", &i, x); });
            }

            new_models.probe()
        });

        (input, probe)
    }

    pub fn add_fact() {
        unimplemented!();
    }

    pub fn remove_fact() {
        unimplemented!();
    }

    pub fn add_rule() {
        unimplemented!();
    }

    pub fn remove_rule() {
        unimplemented!();
    }

}

