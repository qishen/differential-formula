use std::borrow::*;
use std::collections::*;
use std::convert::*;
use std::fmt;
use std::sync::*;
use std::vec::Vec;

use num::*;

use crate::expression::*;
use crate::term::*;
use crate::type_system::*;
use crate::util::map::*;
use crate::util::NameGenerator;

mod arith_expr;
mod base_expr;
pub use arith_expr::*;
pub use base_expr::*;


/// enum_dispatch does not support trait with associated types.
// #[enum_dispatch(ExprTrait)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expr {
    BaseExpr(BaseExpr),
    ArithExpr(ArithExpr),
}

impl TryFrom<Expr> for BaseExpr {
    type Error = &'static str;

    fn try_from(value: Expr) -> Result<Self, Self::Error> {
        match value {
            Expr::BaseExpr(base_expr) => {
                Ok(base_expr)
            },
            _ => { Err("It's not a BaseExpr.") }
        }
    }
}

impl TryFrom<Expr> for ArithExpr {
    type Error = &'static str;

    fn try_from(value: Expr) -> Result<Self, Self::Error> {
        match value {
            Expr::ArithExpr(arith_expr) => {
                Ok(arith_expr)
            },
            _ => { Err("It's not a ArithExpr.") }
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::BaseExpr(b) => write!(f, "{}", b),
            Expr::ArithExpr(a) => write!(f, "{}", a),
        }
    }
}

// #[enum_dispatch]
pub trait ExprTrait {
    /// Evaluate the expression given the variable binding map.
    fn evaluate<M>(&self, binding: &M) -> Option<BigInt> where M: GenericMap<AtomicTerm, AtomicTerm>;
}

impl ExprTrait for Expr {
    fn evaluate<M>(&self, binding: &M) -> Option<BigInt> where M: GenericMap<AtomicTerm, AtomicTerm> {
        match self.borrow() {
            Expr::BaseExpr(be) => be.evaluate(binding),
            Expr::ArithExpr(ae) => ae.evaluate(binding),
        }
    }
}

impl BasicExprOps for Expr {
    fn variables(&self) -> HashSet<AtomicTerm> {
        match self.borrow() {
            Expr::BaseExpr(b) => b.variables(),
            Expr::ArithExpr(a) => a.variables(),
        }
    }

    fn replace_pattern(&mut self, pattern: &AtomicTerm, replacement: &AtomicTerm) {
        match self.borrow_mut() {
            Expr::BaseExpr(b) => b.replace_pattern(pattern, replacement),
            Expr::ArithExpr(a) => a.replace_pattern(pattern, replacement),
        }
    }
}

impl SetCompreOps for Expr {
    fn has_set_comprehension(&self) -> bool {
        match self {
            Expr::BaseExpr(be) => be.has_set_comprehension(),
            Expr::ArithExpr(ae) => ae.has_set_comprehension(),
        }
    }

    fn set_comprehensions(&self) -> Vec<&SetComprehension> {
        match self {
            Expr::BaseExpr(be) => be.set_comprehensions(),
            Expr::ArithExpr(ae) => ae.set_comprehensions(),
        }
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) 
    -> HashMap<AtomicTerm, SetComprehension> {
        match self {
            Expr::BaseExpr(be) => be.replace_set_comprehension(generator),
            Expr::ArithExpr(ae) => ae.replace_set_comprehension(generator),
        }
    }
}