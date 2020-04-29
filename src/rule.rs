extern crate rand;
extern crate timely;
extern crate differential_dataflow;
extern crate abomonation_derive;
extern crate abomonation;

use std::iter::*;
use std::vec::Vec;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::string::String;

use petgraph::graph::*;
use petgraph::algo::*;

use crate::term::*;
use crate::expression::*;
use crate::constraint::*;


#[derive(Clone, Debug)]
pub struct Rule {
    head: Vec<Term>,
    body: Vec<Constraint>,
    dc_var_counter: i64,
}

impl FormulaExpr for Rule {
    fn replace(&mut self, pattern: &Term, replacement: &Term) {
        self.head.replace(pattern, replacement);
        self.body.replace(pattern, replacement);
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let term_strs: Vec<String> = self.head.iter().map(|x| {
            let term_str = format!("{}", x);
            term_str
        }).collect();

        let constraint_strs: Vec<String> = self.body.iter().map(|x| {
            let con_str = format!("{}", x);
            con_str
        }).collect();

        let head_str = term_strs.join(", ");
        let body_str = constraint_strs.join(", ");
        write!(f, "{}", head_str + " :- " + &body_str)
    }
}


impl Rule {
    pub fn new(head: Vec<Term>, body: Vec<Constraint>) -> Self {
        let mut rule = Rule {
            head,
            body,
            dc_var_counter: 0,
        };

        rule.convert_negative_predicate();
        rule
    }

    pub fn is_conformance_rule(&self) -> bool {
        self.head.len() == 0
    }

    pub fn get_head(&self) -> Vec<Term> {
        self.head.clone()
    }

    pub fn get_body(&self) -> Vec<Constraint> {
        self.body.clone()
    }

    fn convert_negative_predicate(&mut self) {
        let mut constraints = vec![];
        // Empty all constraints in the rule body.
        let clist: Vec<Constraint> = self.body.drain(..).collect(); 
        for constraint in clist.into_iter() {
            match constraint {
                Constraint::Predicate(predicate) => {
                    if predicate.negated {
                        let varname = format!("~dc{}", self.dc_var_counter);
                        self.dc_var_counter += 1;
                        let introduced_var: Term = Variable::new(varname, vec![]).into();
                        let (b1, b2) = predicate.to_binary_constraints(introduced_var).unwrap();
                        constraints.push(b1);
                        constraints.push(b2);
                    }
                    else {
                        constraints.push(predicate.into());
                    }
                },
                _ => {
                    constraints.push(constraint);
                }
            }
        }

        self.body = constraints;
    }

    // Return all existing variables in the body of a rule.
    pub fn variables(&self) -> HashSet<Term> {
        let mut var_set = HashSet::new();
        for constraint in self.body.iter() {
            match constraint {
                Constraint::Predicate(predicate) => {
                    let vars = predicate.variables();
                    var_set.extend(vars);
                },
                Constraint::Binary(binary) => {
                    var_set.extend(binary.left.variables());
                    var_set.extend(binary.right.variables());
                },
                Constraint::TypeConstraint(type_constraint) => {
                    var_set.insert(type_constraint.var.clone());
                }
            }
        }

        var_set
    }

    // Return derived variables by simply deducting matched variables from all variables.
    pub fn derived_variables(&self) -> HashSet<Term> {
        /*
        If variable with fragments like x.y.z = a + b is mistaken as derived variable, 
        then remove it from derived variable list.
        */
        let mut diff_vars = HashSet::new();
        let matched_vars = self.matched_variables();
        let all_vars = self.variables();

        for var in all_vars.into_iter() {
            // var cannot have fragments as a derived variable.
            if !matched_vars.contains(&var) && &var == var.root() {
                diff_vars.insert(var);
            }
        }

        diff_vars
    }

    pub fn matched_variables(&self) -> HashSet<Term> {
        let mut var_set = HashSet::new();
        for constraint in self.body.iter() {
            match constraint {
                Constraint::Predicate(predicate) => {
                    // All variables in predicate and its alias are matched variables.
                    let mut vars = predicate.variables();
                    var_set.extend(vars);
                },
                Constraint::Binary(binary) => {
                    // For binary constraint only set comprehension has matched variables in its condition,
                    // other variables are either derived variables or ones already matched in other predicates.
                    let mut setcompres = binary.left.set_comprehensions();
                    setcompres.append(&mut binary.right.set_comprehensions());

                    for setcompre in setcompres.iter() {
                        let vars = setcompre.matched_variables();
                        var_set.extend(vars);
                    }
                },
                Constraint::TypeConstraint(type_constraint) => {
                    // The single variable here must be already matched else where.
                }
            }
        }

        var_set
    }

    pub fn pos_preds(&self) -> Vec<Constraint> {
        let preds: Vec<Constraint> = self.body.iter().filter(|x| {
            match x {
                Constraint::Predicate(p) => {
                    if p.negated { return false; }
                    else { return true; }
                },
                _ => false,
            }
        }).map(|x| x.clone()).collect();     

        preds
    }

    /* 
    Simply check if the left side is a single variable term in the list of derived variables.
    */
    pub fn is_constraint_with_derived_term(&self, constraint: Constraint) -> bool {
        let derived_vars = self.derived_variables();
        match constraint {
            Constraint::Binary(b) => {
                match b.left {
                    Expr::BaseExpr(base_expr) => {
                        match base_expr {
                            BaseExpr::Term(term) => {
                                match term {
                                    Term::Variable(v) => { 
                                        let var_term: Term = v.into();
                                        if derived_vars.contains(&var_term) { true } 
                                        else { false }
                                    },
                                    _ => { false },
                                }
                            },
                            _ => { false },
                        }
                    },
                    _ => { false },
                }
            },
            _ => { false },
        }
    }


    /*
    A list of constraints in which every variable inside it can be directly evaluated by binding map.
    Simply return all binary constraints that are not definition constraints.
    */
    pub fn pure_constraints(&self) -> Vec<&Constraint> {
        let mut pure_constraints = vec![];
        let definition_constraint_set: HashSet<&Constraint> = HashSet::from_iter(self.ordered_definition_constraints());
        for constraint in self.binary_constraints() {
            if !definition_constraint_set.contains(constraint) {
                pure_constraints.push(constraint);
            }
        }
        pure_constraints
    }


    /* 
    Definition constraints are declarations of new derived variable and they were never matched
    in other predicates in the same rule.
    e.g. var = [set comprehension] or var = [Expr]. Expr can not be an atom.
    Re-arrange the order of definition constraints to make sure c = count({..}) is executed
    before val = c * c.
    */
    pub fn ordered_definition_constraints(&self) -> Vec<&Constraint> {
        let mut definition_constraints = vec![];
        let declared_vars = self.derived_variables();
        for declared_var in declared_vars.iter() {
            let cons = self.definition_constraints_of_variable(declared_var);
            /* 
            Assume set comprehension is always put on the top before other types of expression.
            Only the first one is considered as definition constraint and rest of them are pure
            constraints.
            e.g. [c = count({..}), c = a + 1, c = 1], the last two are pure constraints.
            */
            let first = cons.get(0).unwrap();
            definition_constraints.push(first.clone());
        }

        let mut map = HashMap::new();
        let mut graph = Graph::new();
        for constraint in definition_constraints.clone() {
            // Each node is indexed and associated with a weight in which you can store some data.
            let node = graph.add_node(constraint);
            map.insert(constraint, node);
        }

        for constraint in definition_constraints.clone() {
            let &node = map.get(constraint).unwrap();
            // left of binary must be a variable term.
            let binary: Binary = constraint.clone().try_into().unwrap();
            let base_expr: BaseExpr = binary.left.try_into().unwrap();
            let left_var: Term = base_expr.try_into().unwrap(); 

            for other_constraint in definition_constraints.clone() {
                let other_binary: Binary = other_constraint.clone().try_into().unwrap();
                // Check if the right side of the other definition constraint contain this definition variable.
                if other_binary.right.variables().contains(&left_var) {
                    let &other_node = map.get(other_constraint).unwrap();
                    graph.add_edge(node, other_node, 1);
                }
            }
        }

        let mut ordered_definition_constraints = vec![];
        let indexes = toposort(&graph, None).unwrap();

        for index in indexes {
            let &constraint = graph.node_weight(index).unwrap();
            ordered_definition_constraints.push(constraint);
        }
         
        ordered_definition_constraints
    }

    /*
    Return all binary constraints in which the left side is the specified variable.
    e.g. result = [a = count({...}), a = b + c, a = 1]
    */
    fn definition_constraints_of_variable(&self, variable: &Term) -> Vec<&Constraint> {
        let mut matched_constraints = vec![];
        for constraint in self.body.iter() {
            match constraint {
                Constraint::Binary(b) => {
                    match &b.left {
                        Expr::BaseExpr(be) => {
                            match be {
                                BaseExpr::Term(t) => {
                                    if t == variable {
                                        matched_constraints.push(constraint);
                                    }
                                },
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                },
                _ => {},
            }
        }        

        matched_constraints
    }
   
    // Return all binary constraints.
    pub fn binary_constraints(&self) -> Vec<&Constraint> {
        self.body.iter().filter(|x| {
            match x {
                Constraint::Binary(b) => true,
                _ => false,
            }
        }).collect()
    }
}