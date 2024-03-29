
use std::iter::*;
use std::sync::Arc;
use std::vec::Vec;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Display};

use num::*;

use crate::expression::*;
use crate::term::*;
use crate::util::*;
use crate::util::map::*;

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

impl BasicExprOps for ArithExpr {
    fn variables(&self) -> HashSet<AtomicTerm> {
        let mut vars = HashSet::new();
        let left_vars = self.left.variables();
        let right_vars = self.right.variables();
        vars.extend(left_vars);
        vars.extend(right_vars);
        vars
    }

    fn replace_pattern(&mut self, pattern: &AtomicTerm, replacement: &AtomicTerm) {
        let left = Arc::make_mut(&mut self.left);
        left.replace_pattern(pattern, replacement);
        let right = Arc::make_mut(&mut self.right);
        right.replace_pattern(pattern, replacement);
    }
}

impl SetCompreOps for ArithExpr {
    fn has_set_comprehension(&self) -> bool {
        self.left.has_set_comprehension() || self.right.has_set_comprehension()
    }

    fn set_comprehensions(&self) -> Vec<&SetComprehension> {
        let mut list = vec![];
        let mut left_vec = self.left.set_comprehensions();
        let mut right_vec = self.right.set_comprehensions();
        list.append(&mut left_vec);
        list.append(&mut right_vec);
        list
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) 
    -> HashMap<AtomicTerm, SetComprehension> {
        let mut map = HashMap::new();
        let left_map = Arc::make_mut(&mut self.left).replace_set_comprehension(generator);
        let right_map = Arc::make_mut(&mut self.right).replace_set_comprehension(generator);
        map.extend(left_map);
        map.extend(right_map);
        map
    }
}

impl ExprTrait for ArithExpr {
    fn evaluate<M>(&self, binding: &M) -> Option<BigInt> where M: GenericMap<AtomicTerm, AtomicTerm> {
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

