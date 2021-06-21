use std::collections::{HashMap, HashSet};

use crate::term::*;
use crate::type_system::*;
use crate::util::*;

mod setcompre;
mod expr;

pub use setcompre::*;
pub use expr::*;

/// This trait applies to Formula expressions like terms, rules, constraints, setcompre, etc.
/// It provides some functions to return variables in the expressions or replace some variables.
pub trait BasicExprOps {
    /// Return all variables in the expression.
    fn variables(&self) -> HashSet<AtomicTerm>;

    /// Find a term with certain pattern in the expression and replace it with another term.
    /// The pattern can be any generic term that implemets `TermStructure` trait.
    fn replace_pattern(&mut self, pattern: &AtomicTerm, replacement: &AtomicTerm);
}

pub trait SetCompreOps: BasicExprOps {
    // Check if the expression has set comprehension inside.
    fn has_set_comprehension(&self) -> bool;

    // Return all set comprehensions as references.
    fn set_comprehensions(&self) -> Vec<&SetComprehension>;

    /// Find set comprehension in the expression and replace it with a don't-care variable to 
    /// represent it. The method will return a hash map mapping don't-care variable term to set 
    /// comprehension and there is a counter that is used to generate variable name.
    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) 
    -> HashMap<AtomicTerm, SetComprehension>;
}

impl<E> BasicExprOps for Option<E> where E: BasicExprOps {
    fn variables(&self) -> HashSet<AtomicTerm> {
        match self {
            Some(expr) => {
                expr.variables()
            },
            None => { HashSet::new() }
        }
    }

    fn replace_pattern(&mut self, pattern: &AtomicTerm, replacement: &AtomicTerm) {
        match self {
            Some(expr) => {
                expr.replace_pattern(pattern, replacement);
            },
            None => {},
        };
    }
}

impl<E> SetCompreOps for Option<E> where E: SetCompreOps {
    fn has_set_comprehension(&self) -> bool {
        match self {
            Some(expr) => {
                expr.has_set_comprehension()
            },
            None => false
        }
    }

    fn set_comprehensions(&self) -> Vec<&SetComprehension> {
        match self {
            Some(expr) => {
                return expr.set_comprehensions();
            },
            None => { return Vec::new(); },
        };
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) 
    -> HashMap<AtomicTerm, SetComprehension> {
        match self {
            Some(expr) => {
                return expr.replace_set_comprehension(generator);
            },
            None => { return HashMap::new(); },
        };
    }
}

impl<E> BasicExprOps for Vec<E> where E: BasicExprOps {
    fn variables(&self) -> HashSet<AtomicTerm> {
        let mut vars = HashSet::new();
        for element in self.iter() {
            let sub_vars = element.variables();
            vars.extend(sub_vars);
        }
        vars
    }

    fn replace_pattern(&mut self, pattern: &AtomicTerm, replacement: &AtomicTerm) {
        for element in self.iter_mut() {
            element.replace_pattern(pattern, replacement);
        }
    }
}

impl<E> SetCompreOps for Vec<E> where E: SetCompreOps {
    fn has_set_comprehension(&self) -> bool {
        self.iter().any(|expr| {
            expr.has_set_comprehension()
        })
    }

    fn set_comprehensions(&self) -> Vec<&SetComprehension> {
        self.iter().fold(Vec::new(), |mut acc, expr| {
            let setcompres = expr.set_comprehensions();
            acc.extend(setcompres);
            return acc;
        })
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) 
    -> HashMap<AtomicTerm, SetComprehension> {
        let mut map = HashMap::new();
        for element in self.iter_mut() {
            let sub_map = element.replace_set_comprehension(generator);
            map.extend(sub_map);
        }
        map
    }

}