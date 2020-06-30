use std::vec::Vec;
use std::collections::*;
use std::string::String;
use std::hash::Hash;

use crate::util::map::*;
use crate::module::MetaInfo;
use crate::type_system::*;

mod atomic;
mod generic;
mod indexed_term;

pub use atomic::*;
pub use generic::*;
pub use indexed_term::*;

/// `BorrowedTerm` trait must be implemented before implementing `VisitTerm` trait because
/// it only works for type that looks like a Formula term with tree data structure to traverse.
pub trait VisitTerm {
    
    // By default the Output type should be the same type that implement `VisitTerm` trait.
    type Output: Eq+Ord+Hash+Clone;

    /// Traverse the term recursively to find the pattern without mutating the found term.
    fn traverse<F1, F2>(&self, pattern: &F1, logic: &F2)
    where F1: Fn(&Self::Output) -> bool, F2: Fn(&Self::Output);

    /// Traverse the term recursively from root to find the term that satifies the pattern
    /// and then apply the logic to the mutable term.
    fn traverse_mut<F1, F2>(&mut self, pattern: &F1, logic: &mut F2) 
    where F1: Fn(&Self::Output) -> bool, F2: FnMut(&mut Self::Output);

    /// Convert non-ground term into a normalized form.
    fn normalize(&self) -> (Self::Output, HashMap<Self::Output, Self::Output>);

    /// Compare two Formula terms and return a binding if every variable in the first term `a` including 
    /// itself can match to the terms in the second term `b` at exact the same position. Fail if a conflict
    /// is detected or two terms have different types. This methods can be called by any borrowed term like
    /// Box<Term>, Rc<Term> or Arc<Term> that implementes Borrow<Term> and the binding map accepts those
    /// types too. `K` and `V` must implement trait `From<Term>` because a clone is made and then automatically
    /// converted to the instance of `K` or `V`.
    fn get_bindings_in_place<M>(&self, binding: &mut M, term: &Self::Output) -> bool 
    where M: GenericMap<Self::Output, Self::Output>;
    
    /// Simply return a hash map that maps variables to terms.
    fn get_bindings(&self, term: &Self::Output) -> Option<HashMap<Self::Output, Self::Output>>;

    /// Use `BTreeMap` when there is additional requirement that the map needs to implement `Ord` trait.
    fn get_ordered_bindings(&self, term: &Self::Output) -> Option<BTreeMap<Self::Output, Self::Output>>;

    /// The same `BTreeMap` wrapped in `OrdHashableWrappr` that stashs the hash of the map and use the
    /// cached hash to decide the ordering of two maps. Only decide the ordering of two maps by recursively
    /// digging into two maps when the two hashes are equal in case of hash collision.
    fn get_cached_bindings(&self, term: &Self::Output) -> Option<QuickHashOrdMap<Self::Output, Self::Output>>;
    
    /// Clone itself and mutate the cloned term by replacing variables with terms in the map.
    fn propagate_bindings<M>(&self, map: &M) -> Self::Output 
    where M: GenericMap<Self::Output, Self::Output>;
        
    /// Find the subterm of a composite term when given a variable term with fragments.
    fn find_subterm(&self, var_term: &Self::Output) -> Option<Self::Output>;

    /// A similar method as find_subterm() method but given a list of labels as the argument to derive subterm.
    fn find_subterm_by_labels(&self, labels: &Vec<String>) -> Option<Self::Output>;

    /// Update the binidng if the term (in borrowed form) is the subterm of one of the variable in the binding,
    /// e.g. `x.y.z` wants to update the binding with variable `x.y` as key in the binding. A subterm in the term
    /// that `x.y` points to will be derived and `x.y.z` -> `subterm` will be added into the current binding.
    fn update_binding<M>(&self, binding: &mut M) -> bool where M: GenericMap<Self::Output, Self::Output>;

    /// Immutable version of `rename_mut` that return a new term with everything renamed.
    fn rename<BS, BT>(&self, scope: String, metainfo: &MetaInfo<BS, BT>) -> Self
    where BS: BorrowedType, BT: BorrowedTerm;

    /// Extend the root of variable term with scope or add additional scope to the type of a composite term.
    /// atom term is skipped.
    fn rename_mut<BS, BT>(&mut self, scope: String, metainfo: &MetaInfo<BS, BT>) 
    where BS: BorrowedType, BT: BorrowedTerm;

    /// Check if the term has variable(s) inside it.
    fn is_groundterm(&self) -> bool;

    /// Only apply to variable term to return the root term.
    fn root(&self) -> Self::Output;

    /// Check if the term is a don't-care variable with variable root name as '_'.
    fn is_dc_variable(&self) -> bool;

    /// Compare the variables in two terms and find the intersection part.
    fn intersect(&self, other: &Self::Output) -> (HashSet<Self::Output>, HashSet<Self::Output>, HashSet<Self::Output>);

    // Check if two binding map has conflits in variable mappings.
    fn has_conflict<M>(outer: &M, inner: &M) -> bool 
    where M: GenericMap<Self::Output, Self::Output>;

    /// Check if a variable term is the subterm of another variable term. 
    /// Variable with longer fragments is the subterm of variable with shorter fragments.
    fn has_subterm(&self, term: &Self::Output) -> Option<bool>;

    /// If one variable term starts with another variable term, then return their difference in the fragments.
    fn fragments_difference(&self, term: &Self::Output) -> Option<Vec<String>>;
}