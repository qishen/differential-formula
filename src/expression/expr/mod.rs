use std::borrow::*;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Display};
use std::sync::Arc;
use std::vec::Vec;

use enum_dispatch::enum_dispatch;
use num::*;

use crate::expression::{FormulaExprTrait, SetComprehension};
use crate::term::*;
use crate::util::*;
use crate::util::map::*;

mod arith_expr;
mod base_expr;
pub use arith_expr::*;
pub use base_expr::*;


#[enum_dispatch(ExprTrait)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expr {
    BaseExpr,
    ArithExpr,
}

/// `ExprTrait` is used to evaluate expression with concrete values.
#[enum_dispatch]
pub trait ExprTrait {
    /// Check if expression contains set comprehension.
    fn has_set_comprehension(&self) -> bool;

    // Return all set comprehension in the expression.
    fn set_comprehensions(&self) -> Vec<SetComprehension>;

    // Evaluate the expression given the variable binding map.
    fn evaluate<M, K, V>(&self, binding: &M) -> Option<BigInt> 
    where 
        M: GenericMap<K, V>,
        K: Borrow<Term>,
        V: Borrow<Term>;
}

impl<T> FormulaExprTrait for T 
where 
    T: Borrow<Expr> + BorrowMut<Expr>
{
    type Output = Term;

    fn variables(&self) -> HashSet<Self::Output> {
        match self.borrow() {
            Expr::BaseExpr(b) => b.variables(),
            Expr::ArithExpr(a) => a.variables(),
        }
    }

    fn replace_pattern<P: Borrow<Term>>(&mut self, pattern: &P, replacement: &Self::Output) {
        match self.borrow_mut() {
            Expr::BaseExpr(b) => b.replace_pattern(pattern, replacement),
            Expr::ArithExpr(a) => a.replace_pattern(pattern, replacement),
        }
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Self::Output, SetComprehension> {
        match self.borrow_mut() {
            Expr::BaseExpr(b) => { return b.replace_set_comprehension(generator); },
            Expr::ArithExpr(a) => { return a.replace_set_comprehension(generator); }
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::BaseExpr(b) => write!(f, "{}", b),
            Expr::ArithExpr(a) => write!(f, "{}", a),
        }
    }
}