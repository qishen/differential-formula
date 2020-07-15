use std::collections::*;

use crate::util::map::*;

mod atomic;
mod generic;
mod indexed_term;

pub use atomic::*;
pub use generic::*;
pub use indexed_term::*;
pub use crate::type_system::*;

/// `TermStructure` trait must be implemented before implementing `VisitTerm` trait because
/// it only works for those types that look like a Formula term with tree data structure to traverse.
pub trait VisitTerm: TermStructure {
    
    /// Traverse the term recursively to find the pattern without mutating the found term.
    fn traverse<F1, F2>(&self, pattern: &F1, logic: &F2) where F1: Fn(&Self) -> bool, F2: Fn(&Self);

    /// Traverse the term recursively from root to find the term that satifies the pattern
    /// and then apply the logic to the mutable term.
    fn traverse_mut<F1, F2>(&mut self, pattern: &F1, logic: &mut F2) where F1: Fn(&Self) -> bool, F2: FnMut(&mut Self);

    /// Convert non-ground term into a normalized form and return a hash map that maps original variable into
    /// normalized variable. The normalized variables start with `~p`.
    fn normalize(&self) -> (Self, HashMap<Self, Self>);

    /// Compare two Formula terms and return a binding if every variable in the first term `a` including 
    /// itself can match to the terms in the second term `b` at exact the same position. Fail if a conflict
    /// is detected or two terms have different types. This methods can be called by any borrowed term like
    /// Box<Term>, Rc<Term> or Arc<Term> that implementes Borrow<Term> and the binding map accepts those
    /// types too. `K` and `V` must implement trait `From<Term>` because a clone is made and then automatically
    /// converted to the instance of `K` or `V`.
    fn get_bindings_in_place<M>(&self, binding: &mut M, term: &Self) -> bool where M: GenericMap<Self, Self>;
    
    /// Simply return a hash map that maps variables to terms.
    fn get_bindings(&self, term: &Self) -> Option<HashMap<Self, Self>>;

    /// Use `BTreeMap` when there is additional requirement that the map needs to implement `Ord` trait.
    fn get_ordered_bindings(&self, term: &Self) -> Option<BTreeMap<Self, Self>>;

    /// The same `BTreeMap` wrapped in `OrdHashableWrappr` that stashs the hash of the map and use the
    /// cached hash to decide the ordering of two maps. Only decide the ordering of two maps by recursively
    /// digging into two maps when the two hashes are equal in case of hash collision.
    fn get_cached_bindings(&self, term: &Self) -> Option<QuickHashOrdMap<Self, Self>>;
    
    /// Clone itself and mutate the cloned term by replacing variables with terms in the map.
    fn propagate_bindings<M>(&self, map: &M) -> Self where M: GenericMap<Self, Self>;
        
    /// Update the binidng if the term (in borrowed form) is the subterm of one of the variable in the binding,
    /// e.g. `x.y.z` wants to update the binding with variable `x.y` as key in the binding. A subterm in the term
    /// that `x.y` points to will be derived and `x.y.z` -> `subterm` will be added into the current binding.
    fn update_binding<M>(&self, binding: &mut M) -> bool where M: GenericMap<Self, Self>;

    /// Check if the term has variable(s) inside it.
    fn is_groundterm(&self) -> bool;

    /// Compare the variables in two terms and find the intersection part.
    fn intersect(&self, other: &Self) -> (HashSet<Self>, HashSet<Self>, HashSet<Self>);

    /// Compare two iterators of terms and check if they share some terms or a term in one list is the subterm
    /// of a term from the other list.
    fn has_deep_intersection<'a, I>(a: I, b: I) -> bool where I: Iterator<Item=&'a Self>;

    /// Find the subterm given a variable term with fragments.
    fn find_subterm(&self, var: &Self) -> Option<&Self>;

    // Check if two binding map has conflits in variable mappings.
    // fn has_conflict<M>(outer: &M, inner: &M) -> bool where M: GenericMap<Self, Self>;
}