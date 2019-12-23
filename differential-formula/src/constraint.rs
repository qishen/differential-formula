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

use enum_dispatch::enum_dispatch;

use crate::term::*;
use crate::rule::*;
use crate::expression::*;


#[derive(Clone, Debug)]
pub struct Predicate {
    pub negated: bool,
    pub term: Term,
    pub alias: Option<Term>, // Must be a variable term.
}

impl Display for Predicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut term_str = format!("{}", self.term);
        let alias_str = match &self.alias {
            Some(a) => {
                format!("{} is ", a)
            },
            None => "".to_string()
        };

        term_str = alias_str + &term_str;

        if self.negated {
            term_str = "no ".to_string() + &term_str;
        }
        write!(f, "{}", term_str)
    }
}

impl Predicate {
    pub fn variables(&self) -> HashSet<Term> {
        let mut var_set = HashSet::new();
        let vars = self.term.variables();
        for var in vars.into_iter() {
            var_set.insert(var);
        }
        
        if self.alias != None {
            var_set.insert(self.alias.clone().unwrap());
        }

        var_set
    }
}


#[derive(Clone, PartialEq, Debug)]
pub enum BinOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            BinOp::Eq => "=",
            BinOp::Ne => "!=",
            BinOp::Lt => "<",
            BinOp::Le => "<=",
            BinOp::Gt => ">",
            BinOp::Ge => ">=",
        };
        write!(f, "{}", op_str)
    }
}


#[derive(Clone, Debug)]
pub struct Binary {
    pub op: BinOp,
    pub left: Expr,
    pub right: Expr,
}

impl Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.op, self.right)
    }
}

impl Binary {
    pub fn has_set_comprehension(&self) -> bool {
        return self.left.has_set_comprehension() || self.right.has_set_comprehension(); 
    }

    pub fn evaluate(&self, binding: &HashMap<Term, Term>) -> Option<bool> {
        // Cannot not directly handle set comprehension in evaluation of binary constraint.
        if self.has_set_comprehension() { 
            return None; 
        }
        else {
            let lvalue = self.left.evaluate(binding).unwrap();
            let rvalue = self.right.evaluate(binding).unwrap();
            let satisfied = match self.op {
                BinOp::Eq => { if lvalue == rvalue { true } else { false } },
                BinOp::Ge => { if lvalue >= rvalue { true } else { false } },
                BinOp::Gt => { if lvalue >  rvalue { true } else { false } },
                BinOp::Le => { if lvalue <= rvalue { true } else { false } },
                BinOp::Lt => { if lvalue <  rvalue { true } else { false } },
                BinOp::Ne => { if lvalue != rvalue { true } else { false } },
            };

            Some(satisfied)
        }
    }
}

#[enum_dispatch]
#[derive(Clone, Debug)]
pub enum Constraint {
    Predicate,
    Binary,
}

impl Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let con_str = match self {
            Constraint::Predicate(p) => { format!("{}", p) },
            Constraint::Binary(b) => { format!("{}", b) },
        };

        write!(f, "{}", con_str) 
    }
}


#[enum_dispatch(Constraint)]
pub trait ConstraintBehavior {}
