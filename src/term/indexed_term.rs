use std::borrow::*;
use std::sync::*;
use std::collections::*;
use std::hash::*;

use serde::{Serialize, Deserialize};
use derivative::*;

use crate::module::*;
use crate::util::map::*;
use crate::util::wrapper::*;
use crate::term::{FormulaTerm, Term};


#[derive(Derivative)]
#[derivative(Default, Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub struct IndexedTerm {
    // An unique id for the term and can't be modified once created.
    id: usize,

    // Skip index for some derived traits.
    #[derivative(Debug="ignore")]
    #[derivative(Hash="ignore")]
    #[derivative(PartialOrd="ignore")]
    #[derivative(PartialEq="ignore")]
    #[derivative(Ord="ignore")]
    #[serde(skip)]
    // Use `RwLock` to maintain a thread safe model store.
    index: Arc<RwLock<Model>>
}

impl Borrow<Term> for IndexedTerm {
    fn borrow(&self) -> &Term {
        // Check the model store with id and return the term reference.
        self.get_term()
    }
}

/// Get a reference from model store and make a deep copy.
impl From<IndexedTerm> for Term {
    fn from(iterm: IndexedTerm) -> Self {
        iterm.get_term().clone()
    }
}

/// Convert from IndexedTerm to `Arc<Term>` needs a deep copy.
impl From<IndexedTerm> for Arc<Term> {
    fn from(iterm: IndexedTerm) -> Self {
        Arc::new(iterm.get_term().clone())
    }
}

impl HasUniqueForm for IndexedTerm {
    type Form = usize;
    fn derive_unique_form(&self) -> Self::Form {
        // Use the usize id as unique form for IndexedTerm.
        self.id
    }
}

impl IndexedTerm {
    pub fn new(term: &Term, index: Arc<RwLock<Model>>) -> IndexedTerm {
        let readable_model = index.read().unwrap();
        if readable_model.contains_term(term) {
            let id = readable_model.get_id_by_term(term).unwrap();
            return IndexedTerm { id, index };
        }

        // Write a new term into model store and return its id to create an IndexedTerm.
        let writable_model = index.write().unwrap();
        let id = writable_model.add_term(term.clone());
        return IndexedTerm { id, index };
    }

    pub fn get_read_model(&self) -> RwLockReadGuard<Model> {
        let read_guard = self.index.read().unwrap();
        return read_guard;
    }

    pub fn get_write_model(&mut self) -> RwLockWriteGuard<Model> {
        let write_guard = self.index.write().unwrap();
        return write_guard;
    }

    pub fn get_term(&self) -> &Term {
        let term_ref = self.index.as_ref().read().unwrap().get_term_by_id(self.id).unwrap();
        return term_ref;
    }

    pub fn get_id(&self) -> usize {
        return self.id.clone();
    }

    pub fn remove_self(&mut self) -> Term {
        let (id, term) = self.index.write().unwrap().remove_term_by_id(self.id).unwrap();
        return term;
    }
}

impl FormulaTerm for IndexedTerm {

    type Output = IndexedTerm;

    fn traverse<F1, F2>(&self, pattern: &F1, logic: &F2)
    where F1: Fn(&Self::Output) -> bool, F2: Fn(&Self::Output)
    {
        if pattern(self) {
            logic(self);
        }

        match self.borrow() {
            Term::Composite(c) => {
                for arg in c.arguments.iter() {
                    let arg_ref: &Term = arg.borrow();
                    // It may insert new term into model store if the argument term is not found
                    // in the store even though all the subterms should already exist in model store.
                    let indexed_arg = IndexedTerm::new(arg_ref, self.index.clone());
                    indexed_arg.traverse(pattern, logic);
                }
            },
            _ => {}
        };
    }

    fn traverse_mut<F1, F2>(&mut self, pattern: &F1, logic: &mut F2) 
    where F1: Fn(&Self::Output) -> bool, F2: FnMut(&mut Self::Output)
    {
        if pattern(self) {
            logic(self); // Replace `self` with a new IndexedTerm.
        }

        // Clone term and mutate its arguments then add it into model store and mutate original
        // IndexedTerm by simply replace it with a new IndexedTerm.
        let mut new_term = self.get_term().clone();
        if let Term::Composite(c) = new_term {
            for arg in c.arguments.iter() {
                let arg_ref: &Term = arg.borrow();
                let mut indexed_arg = IndexedTerm::new(arg_ref, self.index.clone());
                indexed_arg.traverse_mut(pattern, logic);
                // Convert IndexedTerm back into Arc<Term>.
                // TODO: I have no choice but to make a clone because I can't access Arc<Term> from model store.
                let new_arg = Arc::new(indexed_arg.get_term().clone());
                *arg = new_arg;
            }
        }
        let new_indexed_term = IndexedTerm::new(&new_term, self.index.clone());
        *self = new_indexed_term;
    }

    fn normalize(&self) -> (Self::Output, HashMap<Self::Output, Self::Output>)
    {
        // Call the same `normalize` method on &Term that has many deep clones and then convert
        // all terms into IndexedTerm. Only use this method when performance is not a concern.
        let (term, vmap) = self.get_term().normalize();
        let indexed_term = IndexedTerm::new(&term, self.index.clone());
        let mut indexed_vmap = HashMap::new();
        for (k, v) in vmap.iter() {
            let ik = IndexedTerm::new(k, self.index.clone());
            let iv = IndexedTerm::new(v, self.index.clone());
            indexed_vmap.insert(ik, iv);
        }

        (indexed_term, indexed_vmap)
    }

    fn get_bindings_in_place<M>(&self, binding: &mut M, term: &Self::Output) -> bool 
    where M: GenericMap<Self::Output, Self::Output>
    {
        let term_ref: &Term = self.borrow();
        match term_ref {
            Term::Atom(sa) => { false }, // Atom cannot be a pattern.
            Term::Variable(sv) => { 
                // Detect a conflict in variable binding and return false.
                if binding.contains_gkey(self) && 
                   binding.gget(self).unwrap() != term {
                    return false;
                } 

                // Skip the do-not-care variable represented by underscore.
                if !self.is_dc_variable() {
                    // Clone IndexTerm is fine which only copies an integer and an atomic reference.
                    binding.ginsert(self.clone(), term.clone());
                }

                return true;
            },
            Term::Composite(sc) => {
                match term.borrow() {
                    Term::Composite(tc) => {
                        if sc.sort != tc.sort || sc.arguments.len() != tc.arguments.len() {
                            return false;
                        }

                        for i in 0..sc.arguments.len() {
                            // Get term reference from arguments and convert it into IndexTerm by finding
                            // it in the model store.
                            let x_ref = sc.arguments.get(i).unwrap();
                            let y_ref = tc.arguments.get(i).unwrap();
                            let x: Self::Output = IndexedTerm::new(x_ref, self.index);
                            let y: Self::Output = IndexedTerm::new(y_ref, self.index);
    
                            match x.borrow() {
                                Term::Atom(xa) => {
                                    // Atom arguments need to be equal otherwise fail.
                                    if x != y { return false; }
                                },
                                _ => {
                                    let has_binding = x.get_bindings_in_place(binding, &y);
                                    if !has_binding { return false; }
                                }
                            }
                        }
                    },
                    _ => { return false; } // Composite pattern won't match atom or variable.
                };
        
                return true;
            },
        }
    }
    
    fn get_bindings(&self, term: &Self::Output) -> Option<HashMap<Self::Output, Self::Output>>
    {
        unimplemented!()
    }

    fn get_ordered_bindings(&self, term: &Self::Output) -> Option<BTreeMap<Self::Output, Self::Output>>
    {

        unimplemented!()
    }

    fn get_cached_bindings(&self, term: &Self::Output) -> Option<QuickHashOrdMap<Self::Output, Self::Output>>
    {
        unimplemented!()

    }
    
    fn propagate_bindings<M>(&self, map: &M) -> Self::Output 
    where M: GenericMap<Self::Output, Self::Output>
    {

        unimplemented!()
    }
        
    fn find_subterm<T: Borrow<Term>>(&self, var_term: &T) -> Option<Self::Output>
    {

        unimplemented!()
    }

    fn find_subterm_by_labels(&self, labels: &Vec<String>) -> Option<Self::Output>
    {
        unimplemented!()

    }

    fn update_binding<M>(&self, binding: &mut M) -> bool where M: GenericMap<Self::Output, Self::Output>
    {

        unimplemented!()
    }

    fn rename(&mut self, scope: String)
    {

        unimplemented!()
    }

    fn is_groundterm(&self) -> bool
    {
        let term: &Term = self.borrow();
        term.is_groundterm()
    }

    fn root(&self) -> Self::Output
    {
        unimplemented!()
    }

    fn is_dc_variable(&self) -> bool
    {
        let term: &Term = self.borrow();
        term.is_dc_variable()
    }

    fn intersect(&self, other: &Self::Output) -> (HashSet<Self::Output>, HashSet<Self::Output>, HashSet<Self::Output>)
    {

        unimplemented!()
    }

    fn has_conflict<M>(outer: &M, inner: &M) -> bool 
    where M: GenericMap<Self::Output, Self::Output>
    {

        unimplemented!()
    }

    fn has_subterm(&self, term: &Self::Output) -> Option<bool>
    {

        unimplemented!()
    }

    fn fragments_difference(&self, term: &Term) -> Option<Vec<String>>
    {

        unimplemented!()
    }
}