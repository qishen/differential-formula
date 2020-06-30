use std::borrow::*;
use std::cmp::*;
use std::hash::*;
use serde::*;

/// A trait that defines how to convert into an unique form in type U, the trait has a generic
/// param so you could implement multiple versions that convert some instances into different
/// unique forms like in string or integer.
pub trait HasUniqueForm<U> {
    /// Generate an unique form.
    fn derive_unique_form(&self) -> U;
}

/// Provides methods to access unique form reference and update unique form when item mutated.
/// The item needs to have a field to hold the unique form in order to return a reference.
pub trait UniqueForm<U>: HasUniqueForm<U> {
    /// This method provides easy access to the unique form as reference.
    fn unique_form(&self) -> &U;

    /// The item may not be immutable and the unique form needs to be updated 
    /// when it is internally mutated.
    fn update_unique_form(&mut self);
}

/// A wrapper for items of any type T that can be converted into an unique form of type U
/// and store the unique form inside the wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueFormWrapper<U, T> where T: HasUniqueForm<U> {
    unique_form: U,
    item: T,
}

impl<U, T> Borrow<T> for UniqueFormWrapper<U, T> {
    fn borrow(&self) -> &T {
        &self.item
    }
}

impl<U, T: HasUniqueForm<U>> From<T> for UniqueFormWrapper<U, T> where T: HasUniqueForm<U> {
    fn from(item: T) -> Self {
        UniqueFormWrapper {
            unique_form: item.derive_unique_form(),
            item
        }
    }
}

impl<U, T> UniqueForm<U> for UniqueFormWrapper<U, T> where T: HasUniqueForm<U> {
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

impl<U: Ord, T> PartialOrd for UniqueFormWrapper<U, T> where T: HasUniqueForm<U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<U: Hash, T> Hash for UniqueFormWrapper<U, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.unique_form.hash(state);
    }
}