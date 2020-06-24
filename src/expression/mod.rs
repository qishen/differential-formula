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
pub trait FormulaExprTrait {
    
    type SortOutput;
    type TermOutput;

    /// Return all variables in the expression.
    fn variables(&self) -> HashSet<Self::TermOutput>;

    /// Find a term with certain pattern in the expression and replace it with another term.
    fn replace_pattern(&mut self, pattern: &Self::TermOutput, replacement: &Self::TermOutput);

    /// Find set comprehension in the expression and replace it with a don't-care variable to 
    /// represent it. The method will return a hash map mapping don't-care variable term to set 
    /// comprehension and there is a counter that is used to generate variable name.
    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) 
    -> HashMap<Self::TermOutput, SetComprehension<Self::SortOutput, Self::TermOutput>>;
}

impl<E, S, T> FormulaExprTrait for Option<E> 
where 
    S: BorrowedType,
    T: BorrowedTerm<S, T>,
    E: FormulaExprTrait<SortOutput=S, TermOutput=T>
{
    type SortOutput = S;
    type TermOutput = T;

    fn variables(&self) -> HashSet<Self::TermOutput> {
        match self {
            Some(expr) => {
                expr.variables()
            },
            None => { HashSet::new() }
        }
    }

    fn replace_pattern(&mut self, pattern: &Self::TermOutput, replacement: &Self::TermOutput) {
        match self {
            Some(expr) => {
                expr.replace_pattern(pattern, replacement);
            },
            None => {},
        };
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) 
    -> HashMap<Self::TermOutput, SetComprehension<Self::SortOutput, Self::TermOutput>> 
    {
        match self {
            Some(expr) => {
                return expr.replace_set_comprehension(generator);
            },
            None => { return HashMap::new(); },
        };
    }
}

impl<E, S, T> FormulaExprTrait for Vec<E> 
where
    S: BorrowedType,
    T: BorrowedTerm<S, T>,
    E: FormulaExprTrait<SortOutput=S, TermOutput=T>
{
    type SortOutput = S;
    type TermOutput = T;

    fn variables(&self) -> HashSet<Self::TermOutput> {
        let mut vars = HashSet::new();
        for element in self.iter() {
            let sub_vars = element.variables();
            vars.extend(sub_vars);
        }
        vars
    }

    fn replace_pattern(&mut self, pattern: &Self::TermOutput, replacement: &Self::TermOutput) {
        for element in self.iter_mut() {
            element.replace_pattern(pattern, replacement);
        }
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) 
    -> HashMap<Self::TermOutput, SetComprehension<Self::SortOutput, Self::TermOutput>> 
    {
        let mut map = HashMap::new();
        for element in self.iter_mut() {
            let sub_map = element.replace_set_comprehension(generator);
            map.extend(sub_map);
        }
        map
    }
}
