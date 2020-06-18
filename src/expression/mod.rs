use std::hash::Hash;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};

use crate::term::*;
use crate::util::*;

mod setcompre;
mod expr;

pub use setcompre::*;
pub use expr::*;

/// This trait applies to Formula expressions like terms, rules, constraints, setcompre, etc.
/// It provides some functions to return variables in the expressions or replace some variables.
pub trait FormulaExprTrait {
    
    type Output;

    // Return all variables in the expression.
    fn variables(&self) -> HashSet<Self::Output>;

    /// Find a term with certain pattern in the expression and replace it with another term.
    fn replace_pattern<P: Borrow<Term>>(&mut self, pattern: &P, replacement: &Self::Output);

    /// Find set comprehension in the expression and replace it with a don't-care variable to 
    /// represent it. The method will return a hash map mapping don't-care variable term to set 
    /// comprehension and there is a counter that is used to generate variable name.
    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Self::Output, SetComprehension>;
}

impl<T, O> FormulaExprTrait for Option<T> 
where 
    T: FormulaExprTrait<Output=O>
{
    type Output = O;

    fn variables(&self) -> HashSet<Self::Output> {
        match self {
            Some(expr) => {
                expr.variables()
            },
            None => { HashSet::new() }
        }
    }

    fn replace_pattern<P: Borrow<Term>>(&mut self, pattern: &P, replacement: &Self::Output) {
        match self {
            Some(expr) => {
                expr.replace_pattern(pattern, replacement);
            },
            None => {},
        };
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Self::Output, SetComprehension> {
        match self {
            Some(expr) => {
                return expr.replace_set_comprehension(generator);
            },
            None => { return HashMap::new(); },
        };
    }
}

impl<T, O> FormulaExprTrait for Vec<T> 
where
    T: FormulaExprTrait<Output=O>,
    O: Eq + Hash,
{
    type Output = O;

    fn variables(&self) -> HashSet<Self::Output> {
        let mut vars = HashSet::new();
        for element in self.iter() {
            let sub_vars = element.variables();
            vars.extend(sub_vars);
        }
        vars
    }

    fn replace_pattern<P: Borrow<Term>>(&mut self, pattern: &P, replacement: &Self::Output) {
        for element in self.iter_mut() {
            element.replace_pattern(pattern, replacement);
        }
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Self::Output, SetComprehension> {
        let mut map = HashMap::new();
        for element in self.iter_mut() {
            let sub_map = element.replace_set_comprehension(generator);
            map.extend(sub_map);
        }
        map
    }
}