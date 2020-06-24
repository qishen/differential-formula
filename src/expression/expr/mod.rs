use std::borrow::*;
use std::collections::*;
use std::fmt::*;
use std::vec::Vec;

// use enum_dispatch::enum_dispatch;
use num::*;

use crate::expression::{FormulaExprTrait, SetComprehension};
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
pub enum Expr<S, T> where S: BorrowedType, T: BorrowedTerm<S, T> {
    BaseExpr(BaseExpr<S, T>),
    ArithExpr(ArithExpr<S, T>),
}

impl<S, T> Display for Expr<S, T> where S: BorrowedType, T: BorrowedTerm<S, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Expr::BaseExpr(b) => write!(f, "{}", b),
            Expr::ArithExpr(a) => write!(f, "{}", a),
        }
    }
}

// #[enum_dispatch]
pub trait ExprTrait {

    type SortOutput;
    type TermOutput;

    /// Check if expression contains set comprehension.
    fn has_set_comprehension(&self) -> bool;

    /// Return all set comprehension in the expression.
    fn set_comprehensions(&self) -> Vec<SetComprehension<Self::SortOutput, Self::TermOutput>>;

    /// Evaluate the expression given the variable binding map.
    fn evaluate<M>(&self, binding: &M) -> Option<BigInt> where M: GenericMap<Self::TermOutput, Self::TermOutput>;
}


impl<E, S, T> ExprTrait for E
where 
    E: Borrow<Expr<S, T>> + BorrowMut<Expr<S, T>>,
    S: BorrowedType,
    T: BorrowedTerm<S, T>
{

    type SortOutput = S;
    type TermOutput = T;

    fn has_set_comprehension(&self) -> bool {
        match self.borrow() {
            Expr::BaseExpr(be) => be.has_set_comprehension(),
            Expr::ArithExpr(ae) => ae.has_set_comprehension(),
        }
    }

    fn set_comprehensions(&self) -> Vec<SetComprehension<Self::SortOutput, Self::TermOutput>> {
        match self.borrow() {
            Expr::BaseExpr(be) => be.set_comprehensions(),
            Expr::ArithExpr(ae) => ae.set_comprehensions(),
        }
    }

    fn evaluate<M>(&self, binding: &M) -> Option<BigInt> where M: GenericMap<Self::TermOutput, Self::TermOutput> {
        match self.borrow() {
            Expr::BaseExpr(be) => be.evaluate(binding),
            Expr::ArithExpr(ae) => ae.evaluate(binding),
        }
    }
}

impl<E, S, T> FormulaExprTrait for E
where 
    E: Borrow<Expr<S, T>> + BorrowMut<Expr<S, T>>,
    S: BorrowedType,
    T: BorrowedTerm<S, T>
{
    type SortOutput = S;
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

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) 
    -> HashMap<Self::TermOutput, SetComprehension<Self::SortOutput, Self::TermOutput>> 
    {
        match self.borrow_mut() {
            Expr::BaseExpr(b) => { return b.replace_set_comprehension(generator); },
            Expr::ArithExpr(a) => { return a.replace_set_comprehension(generator); }
        }
    }
}