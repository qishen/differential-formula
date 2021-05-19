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
pub enum Expr<T> where T: TermStructure {
    BaseExpr(BaseExpr<T>),
    ArithExpr(ArithExpr<T>),
}

impl<T> TryFrom<Expr<T>> for BaseExpr<T> where T: TermStructure {
    type Error = &'static str;

    fn try_from(value: Expr<T>) -> Result<Self, Self::Error> {
        match value {
            Expr::BaseExpr(base_expr) => {
                Ok(base_expr)
            },
            _ => { Err("It's not a BaseExpr.") }
        }
    }
}

impl<T> TryFrom<Expr<T>> for ArithExpr<T> where T: TermStructure {
    type Error = &'static str;

    fn try_from(value: Expr<T>) -> Result<Self, Self::Error> {
        match value {
            Expr::ArithExpr(arith_expr) => {
                Ok(arith_expr)
            },
            _ => { Err("It's not a ArithExpr.") }
        }
    }
}

impl<T> fmt::Display for Expr<T> where T: TermStructure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::BaseExpr(b) => write!(f, "{}", b),
            Expr::ArithExpr(a) => write!(f, "{}", a),
        }
    }
}

// #[enum_dispatch]
pub trait ExprTrait {

    type TermOutput: TermStructure;

    /// Evaluate the expression given the variable binding map.
    fn evaluate<M>(&self, binding: &M) -> Option<BigInt> where M: GenericMap<Self::TermOutput, Self::TermOutput>;
}

impl<T> ExprTrait for Expr<T> where T: TermStructure
{
    type TermOutput = T;

    fn evaluate<M>(&self, binding: &M) -> Option<BigInt> where M: GenericMap<Self::TermOutput, Self::TermOutput> {
        match self.borrow() {
            Expr::BaseExpr(be) => be.evaluate(binding),
            Expr::ArithExpr(ae) => ae.evaluate(binding),
        }
    }
}

impl<T> BasicExprOps for Expr<T> where T: TermStructure
{
    type TermOutput = T;

    fn variables(&self) -> HashSet<Self::TermOutput> {
        match self.borrow() {
            Expr::BaseExpr(b) => b.variables(),
            Expr::ArithExpr(a) => a.variables(),
        }
    }

    fn replace_pattern(&mut self, pattern: &Self::TermOutput, replacement: &Self::TermOutput) {
        match self.borrow_mut() {
            Expr::BaseExpr(b) => b.replace_pattern(pattern, replacement),
            Expr::ArithExpr(a) => a.replace_pattern(pattern, replacement),
        }
    }
}

impl<T> SetCompreOps for Expr<T> where T: TermStructure {
    fn has_set_comprehension(&self) -> bool {
        match self {
            Expr::BaseExpr(be) => be.has_set_comprehension(),
            Expr::ArithExpr(ae) => ae.has_set_comprehension(),
        }
    }

    fn set_comprehensions(&self) -> Vec<&SetComprehension<Self::TermOutput>> {
        match self {
            Expr::BaseExpr(be) => be.set_comprehensions(),
            Expr::ArithExpr(ae) => ae.set_comprehensions(),
        }
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Self::TermOutput, SetComprehension<Self::TermOutput>> {
        match self {
            Expr::BaseExpr(be) => be.replace_set_comprehension(generator),
            Expr::ArithExpr(ae) => ae.replace_set_comprehension(generator),
        }
    }
}