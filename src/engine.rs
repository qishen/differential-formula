extern crate rand;
extern crate timely;
extern crate differential_dataflow;
extern crate serde;

use std::iter::*;
use std::vec::Vec;
use std::collections::*;
use std::convert::TryInto;
use std::string::String;
use std::cmp::Reverse;

use num::*;
use im::OrdMap;
use num_iter::range;

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
use crate::parser::*;



pub struct Session {
    worker: timely::worker::Worker<timely::communication::allocator::Thread>,
    input: InputSession<i32, Term, isize>,
    probe: timely::dataflow::ProbeHandle<i32>,
    env: Option<Env>,
    domain_name: String,
    model_name: Option<String>,
    step_count: i32,
}

impl Session {
    fn new(input: InputSession<i32, Term, isize>, 
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

    pub fn parse_term_str(&self, term_str: &str) -> Option<Term> {
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
        &self,
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

        let binding_collection = self.dataflow_filtered_by_type(terms, pred_term.clone()) 
            .map(move |term| {
                let binding_opt = pred_term.get_ordered_bindings(&term);
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
                if let Some(vterm) = &pred_alias {
                    binding.insert(vterm.clone(), term);
                }

                binding
            });
            //.inspect(|x| { println!("Initial bindings for the first constraint is {:?}", x); });

        binding_collection
    }

    pub fn dataflow_from_term_bindings_split<G>(
        &self,
        bindings: &Collection<G, OrdMap<Term, Term>>, 
        keys1: Vec<Term>,
        keys2: Vec<Term>,
    ) -> Collection<G, (OrdMap<Term, Term>, OrdMap<Term, Term>)>
    where 
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice,
    {
        let map_tuple_collection = bindings.map(move |mut binding| {
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

        map_tuple_collection
            //.inspect(|x| { println!("Split binding {:?}", x); })
    }

    pub fn dataflow_from_constraints<G>(&self, models: &Collection<G, Term>, constraints: &Vec<Constraint>) 
        -> Collection<G, OrdMap<Term, Term>>
    where
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice,
    {
        // Construct a headless rule from a list of constraints.
        let temp_rule = Rule::new(vec![], constraints.clone());
        let pos_preds = temp_rule.pos_preds();
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
        let mut prev_collection = self.dataflow_filtered_by_positive_predicate_constraint(
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
            let filtered_collection = self.dataflow_filtered_by_positive_predicate_constraint(
                models, 
                pos_constraint,
            );

            // Split bindings of current term into two vectors for join operation later.
            let split_collection = self.dataflow_from_term_bindings_split(
                &filtered_collection, 
                intersection_vars.clone(),
                right_diff_vars.clone(),
            );

            // Split bindings of previous terms into two vectors for join operation later.
            let split_prev_collection = self.dataflow_from_term_bindings_split(
                &prev_collection, 
                intersection_vars.clone(), 
                left_diff_vars.clone(),
            );            ;

            prev_collection = split_prev_collection
                .join(&split_collection)
                .map(move |(inter, (left, right))| {
                    let mut binding = OrdMap::new();
                    binding.extend(left);
                    binding.extend(right);
                    binding.extend(inter);
                    binding
                })
                //.inspect(|x| { print!("Join result is {:?}\n", x); })
                ;

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

        for bin_constraint in temp_rule.ordered_definition_constraints().into_iter() {
            let binary: Binary = bin_constraint.clone().try_into().unwrap();
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
                            prev_collection = self.dataflow_from_set_comprehension(
                                var_term,
                                &prev_collection, 
                                models, 
                                &setcompre
                            )
                            //.inspect(|x| { println!("Check it here {:?}", x); })
                            ;
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
        for bin_constraint in temp_rule.pure_constraints().into_iter() {
            let binary: Binary = bin_constraint.clone().try_into().unwrap();
            prev_collection = prev_collection.filter(move |binding| {
                binary.evaluate(binding).unwrap()
            });
        }

        prev_collection
    }

    
    pub fn dataflow_from_set_comprehension<G>(
        &self,
        var: Term,
        ordered_outer_collection: &Collection<G, OrdMap<Term, Term>>, // Binding collectionf from outer scope of set comprehension.
        models: &Collection<G, Term>, // Existing model collection.
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
        let mut setcompre_default = setcompre.default.clone();
        let constraints = &setcompre.condition;
        let mut setcompre_var = var.clone();
        
        // Evaluate constraints in set comprehension and return ordered binding collection.
        let mut ordered_collection = self.dataflow_from_constraints(models, constraints);

        /*
        In case the production (outer, inner) could be empty set if inner binding collection is empty,
        directly add default set comprehension value to the outer binding and concatenate the new stream into
        production stream later on.
        */
        let ordered_outer_collection_plus_default = ordered_outer_collection.map(move |mut outer| {
            let num_term: Term = Atom::Int(setcompre_default.clone()).into();
            outer.insert(setcompre_var.clone(), num_term);
            outer
        });

        setcompre_var = var.clone();
        setcompre_default = setcompre.default.clone();
        // Make a production of inner binding and outer binding collection but it could be empty.
        let production_stream = ordered_outer_collection.map(|x| (true, x))
            .join(&ordered_collection.map(|x| (true, x)))
            .map(move |(_, (outer, inner))| { (outer, inner) });
        
        setcompre_var = var.clone();
        setcompre_default = setcompre.default.clone();
        let binding_and_aggregation_stream = production_stream 
            .filter(|(outer, inner)| {
                /*
                If outer and inner binding does not have variable confilct on keys then do a reduce operation to 
                group binding tuples by its first element outer binding.
                e.g. rule :- Path(a, b), dc0 = count({dc | Path(u, u)}), dc0 = 0. 
                (Path(1, 2), Path(0, 0)), (Path(1, 2), Path(1, 1)) reduces to Path(1, 2) -> [Path(0, 0), Path(1, 1)]
                Finally aggregate over the list of bindings that belong to the outer binding.
                */
                let has_conflit = Term::has_conflit(outer, inner);
                !has_conflit
            })
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
                        output.push((vec![num], 1));
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
                        output.push((vec![sum], 1));
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
                        output.push((vec![max], 1));
                    },
                    SetCompreOp::MinAll => {
                        let mut min = BigInt::from_i64(std::isize::MAX as i64).unwrap();
                        for (term, count) in terms {
                            let atom: Atom = term.clone().clone().try_into().unwrap();
                            match atom {
                                Atom::Int(i) => { if i < min { min = i; } },
                                _ => {},
                            };
                        }
                        output.push((vec![min], 1));
                    },
                    SetCompreOp::TopK => {
                        let k = setcompre_default.clone();
                        let mut max_heap = BinaryHeap::new();
                        for (term, count) in terms {
                            let atom: Atom = term.clone().clone().try_into().unwrap();
                            match atom {
                                Atom::Int(i) => { 
                                    max_heap.push(i);
                                },
                                _ => {},
                            };
                        }

                        let mut topk = vec![];
                        for i in num_iter::range(BigInt::zero(), k) {
                            if !max_heap.is_empty() {
                                topk.push(max_heap.pop().unwrap());
                            }
                        }

                        output.push((topk, 1));
                    },
                    _ => {
                        let k = setcompre_default.clone();
                        let mut min_heap = BinaryHeap::new();
                        for (term, count) in terms {
                            let atom: Atom = term.clone().clone().try_into().unwrap();
                            match atom {
                                Atom::Int(i) => { 
                                    min_heap.push(Reverse(i));
                                },
                                _ => {},
                            };
                        }

                        let mut bottomk = vec![];
                        for i in num_iter::range(BigInt::zero(), k) {
                            if !min_heap.is_empty() {
                                let r = min_heap.pop().unwrap().0;
                                bottomk.push(r);
                            }
                        }

                        output.push((bottomk, 1));
                    }
                };

            });
        
        // The stream of bindings that does contribution to the aggregation result.
        let remained_binding_after_aggregation = binding_and_aggregation_stream.map(|(x, aggregation)| { x });
        let binding_with_aggregation_stream = binding_and_aggregation_stream
            .map(move |(mut binding, nums)| {
                // Take the first element in num list when operator is count, sum, maxAll, minAll.
                let num_term: Term = Atom::Int(nums.get(0).unwrap().clone()).into();
                binding.insert(setcompre_var.clone(), num_term);
                binding

                // When operator is topk or bottomk.
                // TODO: return a list of numeric values but how?
            });

        setcompre_var = var.clone();
        setcompre_default = setcompre.default.clone();
        let binding_with_default_stream = ordered_outer_collection.map(|x| (x, true))
            .antijoin(&remained_binding_after_aggregation)
            .map(move |(mut outer, _)| {
                // Add default value of set comprehension to each binding.
                let num_term: Term = Atom::Int(setcompre_default.clone()).into();
                outer.insert(setcompre_var.clone(), num_term);
                outer 
            });

        let final_binding_stream = binding_with_aggregation_stream.concat(&binding_with_default_stream);

        final_binding_stream
            //.inspect(|x| { println!("Aggregation result added into binding {:?}", x); })
    }

    pub fn dataflow_from_single_rule<G>(&self, models: &Collection<G, Term>, rule: &Rule) -> Collection<G, Term>
    where 
        G: Scope,
        G::Timestamp: differential_dataflow::lattice::Lattice,
    {
        let head_terms = rule.get_head();

        // Iteratively add new derived models to dataflow until fix point is reached.
        let models_after_rule_execution = models
        //.inspect(|x| { println!("Beginning check out {:?}", x)})
        .iterate(|transitive_models| {
            let constraints = rule.get_body();
            let binding_collection = self.dataflow_from_constraints(&transitive_models, &constraints);

            let mut combined_models = transitive_models.map(|x| x);
            for term in head_terms.into_iter() {
                let headterm_stream = binding_collection
                    .map(move |binding| {
                        term.propagate_bindings(&binding)
                    });

                combined_models = combined_models.concat(&headterm_stream);
            }

            combined_models.distinct()
        });

        models_after_rule_execution
    }


    pub fn create_dataflow(
        &mut self, 
        domain: &Domain, 
        worker: &mut timely::worker::Worker<timely::communication::allocator::Thread>
    ) -> (InputSession<i32, Term, isize>, timely::dataflow::ProbeHandle<i32>)
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

