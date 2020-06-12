use std::cmp::*;
use std::hash::*;

use differential_dataflow::hashable::*;
use crate::term::*;

pub trait HasUniqueForm {
    type Form;
    /// Generate an unique form.
    fn derive_unique_form(&self) -> Self::Form;
}

pub trait UniqueForm<U> {
    /// This method provides easy access to the unique form as reference.
    fn unique_form(&self) -> &U;

    /// The item may not be immutable and the unique form needs to be updated 
    /// when it is internally mutated.
    fn update_unique_form(&mut self);
}

pub struct UniqueFormWrapper<U, T> {
    unique_form: U,
    item: T,
}

impl<U, T> UniqueForm<U> for UniqueFormWrapper<U, T> 
where T: HasUniqueForm<Form=U>
{
    fn unique_form(&self) -> &U {
        &self.unique_form
    }

    fn update_unique_form(&mut self) {
        let new_form = self.item.derive_unique_form();
        self.unique_form = new_form;
    }
}

impl<U: Eq, T> Eq for UniqueFormWrapper<U, T> {} 

impl<U: Eq, T> PartialEq for UniqueFormWrapper<U, T> {
    fn eq(&self, other: &Self) -> bool {
        self.unique_form == other.unique_form
    }
}

impl<U: Ord, T> Ord for UniqueFormWrapper<U, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.unique_form.cmp(&other.unique_form)
    }
}

impl<U: Ord, T> PartialOrd for UniqueFormWrapper<U, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<U: Hash, T> Hash for UniqueFormWrapper<U, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.unique_form.hash(state);
    }
}

// A wrapped Term that cache the hash and use hash to compare ordering first.
pub type HashedTerm = OrdHashableWrapper<Term>;