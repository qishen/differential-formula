use std::collections::*;

use crate::util::map::*;

mod atomic;
mod generic;
mod matches;

pub use atomic::*;
pub use generic::*;
pub use matches::*;
pub use crate::type_system::*;

/// `TermStructure` trait must be implemented before implementing `VisitTerm` trait because
/// it only works for those types that look like a Formula term with tree data structure to traverse.
pub trait TermTraversal: TermStructure {
    
    /// Traverse the term recursively to find the pattern without mutating the found term.
    fn traverse<F1, F2>(&self, pattern: &F1, logic: &F2) 
    where 
        F1: Fn(&Self) -> bool, 
        F2: Fn(&Self);

    /// Traverse the term recursively from root to find the term that satifies the pattern
    /// and then apply the logic to the mutable term.
    fn traverse_mut<F1, F2>(&mut self, pattern: &F1, logic: &mut F2) 
    where 
        F1: Fn(&Self) -> bool, 
        F2: FnMut(&mut Self);

    // / Convert non-ground term into a normalized form and return a hash map that maps original variable into
    // / normalized variable. The normalized variables start with `~p`.
    fn normalize(&self) -> (Self, HashMap<Self, Self>);

    // / Compare two Formula terms and return a binding if every variable in the first term `a` including 
    // / itself can match to the terms in the second term `b` at exact the same position. Fail if a conflict
    // / is detected or two terms have different types. This methods can be called by any borrowed term like
    // / Box<Term>, Rc<Term> or Arc<Term> that implementes Borrow<Term> and the binding map accepts those
    // / types too. `K` and `V` must implement trait `From<Term>` because a clone is made and then automatically
    // / converted to the instance of `K` or `V`.
    // fn get_bindings_in_place<M>(&self, binding: &mut M, term: &Self) -> bool where M: GenericMap<Self, Self>;
    // fn get_bindings_in_place(&self, binding: &mut HashMap<&Self, &Self>, term: &Self) -> bool;
    fn match_in_place<'a>(&'a self, binding: &mut HashMap<&'a Self, &'a Self>, term: &'a Self) -> bool;
    
    // / Simply return a hash map that maps variables to terms.
    // fn get_bindings(&self, term: &Self) -> Option<HashMap<&Self, &Self>>;
    fn match_to<'a>(&'a self, term: &'a Self) -> Option<HashMap<&'a Self, &'a Self>>;

    // / Clone itself and mutate the cloned term by replacing variables with terms in the map.
    fn propagate(&self, map: &HashMap<&Self, &Self>) -> Self;
}