use std::borrow::*;
use std::convert::*;
use std::sync::Arc;
use std::vec::Vec;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Display};

use enum_dispatch::enum_dispatch;
use num::*;

use crate::expression::{FormulaExprTrait, SetComprehension};
use crate::expression::expr::ExprTrait;
use crate::term::*;
use crate::util::*;
use crate::util::map::*;


#[enum_dispatch]
pub trait BaseExprTrait {}
impl BaseExprTrait for SetComprehension {}
impl BaseExprTrait for Term {}

#[enum_dispatch(BaseExprTrait)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BaseExpr {
    SetComprehension,
    Term,
}

impl ExprTrait for BaseExpr {
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

    fn evaluate<M, K, V>(&self, binding: &M) -> Option<BigInt> 
    where 
        M: GenericMap<K, V>,
        K: Borrow<Term>,
        V: Borrow<Term>
    {
        match self {
            BaseExpr::Term(term) => {
                match term {
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
                        let root_var = term.root();
                        let val_term = match root_var.borrow() == term {
                            true => { 
                                binding.gget(term).unwrap().borrow().clone() 
                            },
                            false => {
                                // x.y.z does not exist in the binding but x exists.
                                let val_term: &Term = binding.gget(root_var.borrow()).unwrap().borrow();
                                val_term.find_subterm(term).unwrap()
                            }
                        };

                        // val_term must be an atom term for arithmetic evaluation.
                        let val_term_ref: &Term = val_term.borrow();
                        match val_term_ref {
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

impl Display for BaseExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BaseExpr::SetComprehension(s) => write!(f, "{}", s),
            BaseExpr::Term(t) => write!(f, "{}", t),
        }
    }
}

impl FormulaExprTrait for BaseExpr {

    type Output = Term;

    fn variables(&self) -> HashSet<Self::Output> {
        match self {
            BaseExpr::Term(t) => t.variables(),
            BaseExpr::SetComprehension(s) => s.variables(),
        }
    }

    fn replace_pattern<P: Borrow<Term>>(&mut self, pattern: &P, replacement: &Self::Output) {
        match self {
            BaseExpr::SetComprehension(s) => s.replace_pattern(pattern, replacement),
            BaseExpr::Term(t) => t.replace_pattern(pattern, replacement),
        };
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Self::Output, SetComprehension> {
        let mut map = HashMap::new();
        match self {
            BaseExpr::SetComprehension(setcompre) => {
                // It won't return anything but do some conversion if setcompre has setcompre inside itself.
                setcompre.replace_set_comprehension(generator);
            },
            BaseExpr::Term(t) => {
                return map;
            },
        };
        let introduced_var = generator.generate_dc_term();
        let mut base_expr: BaseExpr = BaseExpr::Term(introduced_var.clone());
        std::mem::swap(self, &mut base_expr);
        map.insert(introduced_var, base_expr.try_into().unwrap()); 
        return map;
    }
}
