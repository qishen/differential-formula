use std::borrow::*;
use std::vec::Vec;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Display};

use num::*;

use crate::expression::{Expression, SetComprehension};
use crate::expression::expr::ExprTrait;
use crate::term::*;
use crate::util::*;
use crate::util::map::*;


// #[enum_dispatch]
pub trait BaseExprTrait {}
impl<T> BaseExprTrait for SetComprehension<T> where T: BorrowedTerm {}
impl<T> BaseExprTrait for T where T: BorrowedTerm {}

// #[enum_dispatch(BaseExprTrait)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BaseExpr<T> where T: BorrowedTerm {
    SetComprehension(SetComprehension<T>),
    Term(T),
}

impl<T> ExprTrait for BaseExpr<T> where T: BorrowedTerm {

    type TermOutput = T;

    fn has_set_comprehension(&self) -> bool {
        let has_setcompre = match self {
            BaseExpr::SetComprehension(s) => true,
            _ => false,
        };
        has_setcompre
    }

    fn set_comprehensions(&self) -> Vec<SetComprehension<Self::TermOutput>> {
        let mut setcompres = vec![];
        match self {
            BaseExpr::SetComprehension(s) => {
                setcompres.push(s.clone());
            },
            _ => {},
        };
        setcompres
    }

    fn evaluate<M>(&self, binding: &M) -> Option<BigInt> where M: GenericMap<Self::TermOutput, Self::TermOutput> {
        match self {
            BaseExpr::Term(term) => {
                match term.borrow() {
                    Term::Atom(atom) => {
                        // The expression is a term of integer type.
                        match &atom.val {
                            AtomEnum::Int(num) => {
                                return Some(num.clone());
                            },
                            _ => { return None; },
                        }
                    },
                    Term::Variable(variable) => {
                        // The expression is a variable and find the value in hash map by that variable
                        let root_var = term.var_root().unwrap();
                        let term_str: &String = term.borrow();
                        let val_term = match root_var == term_str {
                            true => { 
                                binding.gget(term_str).unwrap().clone() 
                            },
                            false => {
                                // x.y.z does not exist in the binding but x exists.
                                let val_term = binding.gget(root_var).unwrap();
                                val_term.find_subterm(&term).unwrap()
                            }
                        };

                        // val_term must be an atom term for arithmetic evaluation.
                        match val_term.borrow() {
                            Term::Atom(atom) => {
                                match &atom.val {
                                    AtomEnum::Int(num) => {
                                        return Some(num.clone())
                                    },
                                    _ => { None }
                                }
                            },
                            _ => { None }
                        }
                    },
                    _ => { return None; }
                }
            },
            _ => { return None; } // Can't directly evaluate set comprehension.
        }
    }
}

impl<T> Display for BaseExpr<T> where T: BorrowedTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BaseExpr::SetComprehension(s) => write!(f, "{}", s),
            BaseExpr::Term(t) => write!(f, "{}", t),
        }
    }
}

impl<T> Expression for BaseExpr<T> where T: BorrowedTerm {

    type TermOutput = T;

    fn variables(&self) -> HashSet<Self::TermOutput> {
        match self {
            BaseExpr::Term(t) => {
                t.variables()
            },
            BaseExpr::SetComprehension(s) => s.variables(),
        }
    }

    fn replace_pattern(&mut self, pattern: &Self::TermOutput, replacement: &Self::TermOutput) {
        match self {
            BaseExpr::SetComprehension(s) => s.replace_pattern(pattern, replacement),
            BaseExpr::Term(t) => t.replace_pattern(pattern, replacement),
        };
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Self::TermOutput, SetComprehension<Self::TermOutput>> {
        let mut map = HashMap::new();
        match self {
            BaseExpr::SetComprehension(setcompre) => {
                // Recursively replace setcompres in the conditions of current setcompre.
                setcompre.replace_set_comprehension(generator);
                // Make a deep copy to avoid ugly try_into().
                let replaced_setcompre = setcompre.clone(); 
                let dc_name = generator.generate_name();
                let dc_var: T = Term::Variable(Variable::new(None, dc_name, vec![])).into();
                let mut base_expr: BaseExpr<T> = BaseExpr::Term(dc_var.clone());
                std::mem::swap(self, &mut base_expr);
                map.insert(dc_var, replaced_setcompre); 
            },
            BaseExpr::Term(_) => {},
        };
        return map;
    }
}
