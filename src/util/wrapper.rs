use std::borrow::*;
use std::cmp::*;
use std::fmt::*;
use std::hash::*;
use std::sync::Arc;
use serde::*;

/// A trait that defines how to convert into an unique form in type U, the trait has a generic
/// param so you could implement multiple versions that convert some instances into different
/// unique forms like in string or integer.
pub trait HasUniqueForm<U> {
    /// Generate an unique form.
    fn derive_unique_form(&self) -> U;
}

/// implement trait for atomic wrapper type too.
impl<T, U> HasUniqueForm<U> for Arc<T> where T: HasUniqueForm<U> {
    fn derive_unique_form(&self) -> U {
        let reference: &T = self.borrow();
        reference.derive_unique_form()
    }
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

/// Implement trait for atomic wrapper type too.
impl<T, U> UniqueForm<U> for Arc<T> where T: UniqueForm<U> {
    fn unique_form(&self) -> &U {
        let reference: &T = self.borrow();
        reference.unique_form()
    }

    fn update_unique_form(&mut self) {
        // It's a bad idea to modify the inner value of an atomic reference.
        unimplemented!()
    }

}

/// A wrapper for items of any type T that can be converted into an unique form of type U
/// and store the unique form inside the wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueFormWrapper<U, T> where T: HasUniqueForm<U> {
    unique_form: U,
    item: T,
}

impl<U, T> UniqueFormWrapper<U, T> where T: HasUniqueForm<U> {
    pub fn unique_form(&self) -> &U {
        &self.unique_form
    }

    pub fn item(&self) -> &T {
        &self.item
    }
}

impl<T> Borrow<String> for UniqueFormWrapper<String, T> where T: HasUniqueForm<String> {
    fn borrow(&self) -> &String {
        self.unique_form()
    }
}

/// The wrapper should be able to be borrowed as
// impl<U, T> Borrow<T> for UniqueFormWrapper<U, T> where T: HasUniqueForm<U> {
//     fn borrow(&self) -> &T {
//         &self.item
//     }
// }

// TODO: A good question here that how should I tell the difference when I want to have two different
// generic parameters in Borrow<X> trait with each one refers to different fields in the struct.
// impl<U, T> Borrow<U> for UniqueFormWrapper<U, T> where T: HasUniqueForm<U> {
//     fn borrow(&self) -> &U {
//         &self.unique_form
//     }
// }

impl<U, T> From<T> for UniqueFormWrapper<U, T> where T: HasUniqueForm<U> {
    fn from(item: T) -> Self {
        UniqueFormWrapper {
            unique_form: item.derive_unique_form(),
            item
        }
    }
}

/// Explicitly implement `HasUniqueForm` for UniqueFormWrapper where the wrapped value
/// has unique form.
impl<U, T> HasUniqueForm<U> for UniqueFormWrapper<U, T> where T: HasUniqueForm<U> {
    fn derive_unique_form(&self) -> U {
        self.item.derive_unique_form()
    }
}

// UniqueFormWrapper has the field to store the unique form and provide access to it.
impl<U, T> UniqueForm<U> for UniqueFormWrapper<U, T> 
where 
    T: HasUniqueForm<U>,
{
    fn unique_form(&self) -> &U {
        &self.unique_form
    }

    fn update_unique_form(&mut self) {
        let new_form = self.item.derive_unique_form();
        self.unique_form = new_form;
    }
}

impl<U: Display, T> Display for UniqueFormWrapper<U, T> where T: HasUniqueForm<U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.unique_form)
    }
}

impl<U: Eq, T> Eq for UniqueFormWrapper<U, T> where T: HasUniqueForm<U> {} 

impl<U: Eq, T> PartialEq for UniqueFormWrapper<U, T> where T: HasUniqueForm<U> {
    fn eq(&self, other: &Self) -> bool {
        self.unique_form == other.unique_form
    }
}

impl<U: Ord, T> Ord for UniqueFormWrapper<U, T> where T: HasUniqueForm<U> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.unique_form.cmp(&other.unique_form)
    }
}

impl<U: Ord, T> PartialOrd for UniqueFormWrapper<U, T> where T: HasUniqueForm<U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<U: Hash, T> Hash for UniqueFormWrapper<U, T> where T: HasUniqueForm<U> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.unique_form.hash(state);
    }
}

/// `AtomicPtrWrapper` is a simple wrapper over atomic reference of certain type.
#[derive(PartialOrd, Ord, Eq, Clone, Serialize, Deserialize)]
pub struct AtomicPtrWrapper<T> {
    pub ptr: Arc<T>
}

impl<T> Display for AtomicPtrWrapper<T> where T: Display{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.ptr.as_ref())
    }
}

impl<T> Debug for AtomicPtrWrapper<T> where T: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Rewrite Debug trait in the same way as Display.
        write!(f, "{:?}", self.ptr.as_ref())
    }
}

// Compute hash on the pointer address rather than the value.
impl<T> Hash for AtomicPtrWrapper<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(&self.ptr, state)
    }
}

// Decide equality by pointer rather than the value.
impl<T> PartialEq for AtomicPtrWrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.ptr, &other.ptr)
    }
}

// Convert into atomic pointer wrapped value.
impl<T> From<T> for AtomicPtrWrapper<T> {
    fn from(item: T) -> AtomicPtrWrapper<T> {
        AtomicPtrWrapper { ptr: Arc::new(item) }
    }
}