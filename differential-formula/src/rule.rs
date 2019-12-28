extern crate rand;
extern crate timely;
extern crate differential_dataflow;
extern crate abomonation_derive;
extern crate abomonation;

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

use crate::term::*;
use crate::expression::*;
use crate::constraint::*;

#[derive(Clone, Debug)]
pub struct Rule {
    pub head: Vec<Term>,
    pub body: Vec<Constraint>
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
    // Return all variables in the body of a rule.
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

    pub fn derived_variables(&self) -> HashSet<Term> {
        // Return derived variables by simply deducting matched variables from all variables.
        let mut diff_vars = HashSet::new();
        let matched_vars = self.matched_variables();
        let all_vars = self.variables();
        for var in all_vars.into_iter() {
            if !matched_vars.contains(&var) {
                diff_vars.insert(var);
            }
        }

        diff_vars
    }

    pub fn matched_variables(&self) -> HashSet<Term> {
        let mut var_set = HashSet::new();
        for constraint in self.body.iter() {
            match constraint {
                // All variables in predicate and its alias are matched variables.
                Constraint::Predicate(predicate) => {
                    let mut vars = predicate.variables();
                    var_set.extend(vars);
                },
                // if it is a binary constraint then only set comprehension has matched variables,
                // other variables are either derived variables or variables already matched in predicates.
                Constraint::Binary(binary) => {
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

    pub fn neg_preds(&self) -> Vec<Constraint> {
        let preds: Vec<Constraint> = self.body.iter().filter(|x| {
            match x {
                Constraint::Predicate(p) => {
                    if p.negated { return true; }
                    else { return false; }
                },
                _ => false,
            }
        }).map(|x| x.clone()).collect();     

        preds
    }

    // Simply check if the left side is a single variable term in the list of derived variables.
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

    // Return all constraints that are declaration of new derived terms 
    // in the form of var = [set comprehension] or var = [Expr].
    pub fn binary_constraints_with_derived_term(&self) -> Vec<Constraint> {
        let constraints: Vec<Constraint> = self.body.clone().into_iter().filter(|x| {
            self.is_constraint_with_derived_term(x.clone())
        }).map(|x| x.clone()).collect();

        constraints
    }

    // Return all constraints that are not declaration of derived terms.
    pub fn binary_constraints_without_derived_term(&self) -> Vec<Constraint> {
        let constraints: Vec<Constraint> = self.body.clone().into_iter().filter(|x| {
            match x {
                Constraint::Binary(b) => {
                    !self.is_constraint_with_derived_term(x.clone())
                },
                _ => { false }
            }
        }).map(|x| x.clone()).collect();

        constraints
    }
   
    // Return all binary constraints.
    pub fn binary_constraints(&self) -> Vec<Constraint> {
        let bins: Vec<Constraint> = self.body.iter().filter(|x| {
            match x {
                Constraint::Binary(b) => true,
                _ => false,
            }
        }).map(|x| x.clone()).collect();

        bins
    }
}