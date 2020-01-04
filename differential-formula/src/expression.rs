extern crate num;
extern crate rand;
extern crate timely;
extern crate differential_dataflow;

use std::iter::*;
use std::sync::Arc;
use std::vec::Vec;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::string::String;

use enum_dispatch::enum_dispatch;
use num::*;

use crate::term::*;
use crate::rule::*;
use crate::constraint::*;
use crate::util::GenericMap;

#[readonly::make]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SetComprehension {
    pub vars: Vec<Term>,
    pub condition: Vec<Constraint>,
    pub op: SetCompreOp,
    pub default: BigInt,
}

// Turn SetComprehension into a headless rule.
impl From<SetComprehension> for Rule {
    fn from(setcompre: SetComprehension) -> Self {
        Rule::new(vec![], setcompre.condition.clone())
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SetCompreOp {
    Sum,
    Count,
    MinAll,
    MaxAll,
}

impl Display for SetCompreOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            SetCompreOp::Sum => { "sum" },
            SetCompreOp::Count => { "count" },
            SetCompreOp::MinAll => { "minAll" },
            SetCompreOp::MaxAll => { "maxAll" },
        };

        write!(f, "{}", op_str)
    }
}

impl Display for SetComprehension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let headterm_strs: Vec<String> = self.vars.iter().map(|x| {
            let term_str = format!("{}", x);
            term_str
        }).collect();

        let constraint_strs: Vec<String> = self.condition.iter().map(|x| {
            let con_str = format!("{}", x);
            con_str
        }).collect();

        let head_str = headterm_strs.join(", ");
        let body_str = constraint_strs.join(", ");
        let mut setcompre_str = format!("{}({{ {} | {} }})", self.op, head_str, body_str); 
        write!(f, "{}", setcompre_str)
    }
}

impl SetComprehension {
    pub fn new(vars: Vec<Term>, condition: Vec<Constraint>, op: SetCompreOp, default: BigInt) -> Self {
        SetComprehension {
            vars,
            condition,
            op,
            default,
        }
    }
    pub fn variables(&self) -> HashSet<Term> {
        let rule: Rule = self.clone().into();
        rule.variables()
    }

    pub fn matched_variables(&self) -> HashSet<Term> {
        let rule: Rule = self.clone().into();
        rule.matched_variables()
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ArithmeticOp {
    Add,
    Min,
    Mul,
    Div,
}

impl Display for ArithmeticOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            ArithmeticOp::Add => "+",
            ArithmeticOp::Min => "-",
            ArithmeticOp::Mul => "*",
            ArithmeticOp::Div => "/",
        };

        write!(f, "{}", op_str)
    }
}

#[enum_dispatch(BaseExpr)]
pub trait BaseExprBehavior {}
impl BaseExprBehavior for SetComprehension {}
impl BaseExprBehavior for Term {}

#[enum_dispatch]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BaseExpr {
    SetComprehension,
    Term,
}

// TODO: put them separately into methods in the BaseExprBehavior trait.
impl ExprBehavior for BaseExpr {
    fn variables(&self) -> HashSet<Term> {
        let mut term_set: HashSet<Term> = HashSet::new();
        match self {
            BaseExpr::SetComprehension(setcompre) => {
                // Turn it into a rule and find all rule variables.
                let rule: Rule = setcompre.clone().into();
                let vars = rule.variables();
                term_set.extend(vars);
            },
            BaseExpr::Term(term) => {
                match term {
                    Term::Variable(v) => {
                        let var_term: Term = v.clone().into();
                        term_set.insert(var_term);
                    },
                    _ => {},
                }
            },
        };

        term_set
    }

    fn has_set_comprehension(&self) -> bool {
        let has_setcompre = match self {
            BaseExpr::SetComprehension(s) => true,
            _ => false,
        };

        has_setcompre
    }

    // A Expr could have multiple set comprehensions.
    fn set_comprehensions(&self) -> Vec<SetComprehension> {
        let mut setcompres = vec![];
        match self {
            BaseExpr::SetComprehension(s) => {
                setcompres.push(s.clone());
            },
            _ => {},
        };

        setcompres
    }

    fn evaluate<T>(&self, binding: &T) -> Option<BigInt> where T: GenericMap<Term, Term> {
        match self {
            BaseExpr::Term(term) => {
                match term {
                    Term::Atom(atom) => {
                        // The expression is a term of integer type.
                        match atom {
                            Atom::Int(num) => {
                                return Some(num.clone());
                            },
                            _ => { return None; },
                        }
                    },
                    Term::Variable(variable) => {
                        // The expression is a variable and find the value in hash map by that variable
                        let root_var = term.root_var();
                        let val_term = match &root_var == term {
                            true => { 
                                binding.get(term).unwrap().clone() 
                            },
                            false => {
                                // x.y.z does not exist in the binding but x exists.
                                let val_term = binding.get(&root_var).unwrap();
                                let val_subterm = val_term.get_subterm_by_labels(&variable.fragments).unwrap();
                                val_subterm
                            }
                        };

                        // val_term must be an atom term for arithmetic evaluation.
                        let atom: Atom = val_term.try_into().unwrap();
                        match atom {
                            Atom::Int(num) => { 
                                return Some(num); 
                            },
                            _ => { return None; },
                        }
                    },
                    _ => { return None; }
                }

            },
            _ => { return None; } // Can't directly evaluate set comprehension.
        }
    }
}

impl Display for BaseExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BaseExpr::SetComprehension(s) => write!(f, "{}", s),
            BaseExpr::Term(t) => write!(f, "{}", t),
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArithExpr {
    pub op: ArithmeticOp,
    pub left: Arc<Expr>,
    pub right: Arc<Expr>,
}

impl Display for ArithExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.left, self.op, self.right)
    }
}

impl ExprBehavior for ArithExpr {
    fn variables(&self) -> HashSet<Term> {
        let mut var_set = HashSet::new();
        let mut left_set = self.left.variables();
        let mut right_set = self.right.variables();
        var_set.extend(left_set);
        var_set.extend(right_set);
        var_set
    }

    fn has_set_comprehension(&self) -> bool {
        return self.left.has_set_comprehension() || self.right.has_set_comprehension();
    }

    fn set_comprehensions(&self) -> Vec<SetComprehension> {
        let mut list = vec![];
        let mut left_vec = self.left.set_comprehensions();
        let mut right_vec = self.right.set_comprehensions();
        list.append(&mut left_vec);
        list.append(&mut right_vec);
        list
    }

    fn evaluate<T>(&self, binding: &T) -> Option<BigInt> where T: GenericMap<Term, Term> {
        let lvalue = self.left.evaluate(binding).unwrap();
        let rvalue = self.right.evaluate(binding).unwrap();
        let result = match self.op {
            ArithmeticOp::Add => { lvalue + rvalue },
            ArithmeticOp::Div => { lvalue / rvalue },
            ArithmeticOp::Min => { lvalue - rvalue },
            ArithmeticOp::Mul => { lvalue * rvalue },
        };

        Some(result)
    } 
}


#[enum_dispatch(Expr)]
pub trait ExprBehavior {
    fn variables(&self) -> HashSet<Term>;
    fn has_set_comprehension(&self) -> bool;
    fn set_comprehensions(&self) -> Vec<SetComprehension>;
    fn evaluate<T>(&self, binding: &T) -> Option<BigInt> where T: GenericMap<Term, Term>; 
}

#[enum_dispatch]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expr {
    BaseExpr,
    ArithExpr,
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::BaseExpr(b) => write!(f, "{}", b),
            Expr::ArithExpr(a) => write!(f, "{}", a),
        }
    }
}
