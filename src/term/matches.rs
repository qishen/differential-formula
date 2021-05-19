use std::borrow::*;
use std::collections::*;
use std::hash::*;
use std::sync::*;
use std::hash::Hash;
use std::fmt::*;

use serde::{Serialize, Deserialize};
use num::*;
use weak_table::*;
use im::OrdMap;

use super::generic::*;
use crate::module::*;
use crate::term::*;
use crate::util::map::*;
use crate::util::wrapper::*;

/// AtomicPtrMatch matches only check pointer equality.
// pub type AtomicPtrMatch<T> = AtomicPtrWrapper<Match<T>>;

/// A match is a hash map mapping variable term to another term.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Match<T> where T: Ord { // where T: TermStructure {
    map: BTreeMap<T, T>
}

impl<T> From<BTreeMap<T, T>> for Match<T> where T: TermStructure {
    fn from(item: BTreeMap<T, T>) -> Self {
        Match { map: item }
    }
}

// pub struct AtomicPtrMatchStore<T> where T: TermStructure {
//     matches: HashSet<Arc<Match<T>>>
// }

// impl<T> AtomicPtrMatchStore<T> where T: TermStructure {
//     /// Create an empty hashset for storing all matches.
//     pub fn new() -> Self {
//         AtomicPtrMatchStore {
//             matches: HashSet::new()
//         }
//     }

//     /// Intern a native Match and return a match wrapped with atomic reference wrapper. 
//     pub fn intern(&mut self, term_match: Match<T>) -> AtomicPtrMatch<T> {
//         if let Some(arc_match) = self.matches.get(&term_match) {
//             // println!("Found a match {:?}", term_match);
//             return AtomicPtrWrapper { ptr: arc_match.clone() };
//         } else {
//             // println!("Need to create a new allocation.");
//             let new_arc_match = Arc::new(term_match.clone());
//             let arc_match = new_arc_match.clone();
//             self.matches.insert(new_arc_match);
//             return AtomicPtrWrapper { ptr: arc_match };
//         }
//     }
// }

// impl<T> GenericMap<T, T> for Match<T> where T: TermStructure {
//     fn gkeys(&self) -> Vec<&T> {
//         let mut list = vec![];
//         let keys = BTreeMap::keys(&self.map);
//         for key in keys {
//             list.push(key);
//         }
//         list
//     }

//     fn contains_gkey<Q>(&self, k: &Q) -> bool
//     where
//         T: Borrow<Q>,
//         Q: Hash + Eq + Ord,
//     {
//         BTreeMap::contains_key(&self.map, k)
//     }

//     fn gget<Q>(&self, k: &Q) -> Option<&T>
//     where
//         T: Borrow<Q>,
//         Q: Hash + Eq + Ord,
//     {
//         BTreeMap::get(&self.map, k)
//     }

//     fn ginsert(&mut self, k: T, v: T) -> Option<T> {
//         BTreeMap::insert(&mut self.map, k, v)
//     }
// }

// impl<T> GenericMap<T, T> for AtomicPtrMatch<T> where T: TermStructure {
//     fn gkeys(&self) -> Vec<&T> {
//         let mut list = vec![];
//         let keys = BTreeMap::keys(&self.ptr.as_ref().map);
//         for key in keys {
//             list.push(key);
//         }
//         list
//     }

//     fn contains_gkey<Q>(&self, k: &Q) -> bool
//     where
//         T: Borrow<Q>,
//         Q: Hash + Eq + Ord,
//     {
//         BTreeMap::contains_key(&self.ptr.as_ref().map, k)
//     }

//     fn gget<Q>(&self, k: &Q) -> Option<&T>
//     where
//         T: Borrow<Q>,
//         Q: Hash + Eq + Ord,
//     {
//         BTreeMap::get(&self.ptr.as_ref().map, k)
//     }

//     fn ginsert(&mut self, k: T, v: T) -> Option<T> {
//         let term_match = Arc::make_mut(&mut self.ptr);
//         BTreeMap::insert(&mut term_match.map, k, v)
//     }
// }

// #[test]
// fn test_ord_map() {
//     let mut map = OrdMap::new();
//     let a: AtomicPtrWrapper<usize> = 1.into();
//     let b: AtomicPtrWrapper<usize> = 2.into();
//     map.insert(a, 100);
//     map.insert(b, 200);
//     assert_eq!(map.len(), 1);
//     println!("{:?}", map);
// }

// #[test]
// fn test_matches() {
//     let mut term_store = AtomicPtrTermStore::new(HashSet::new(), HashMap::new());
//     let mut match_store = AtomicPtrMatchStore::new();

//     let a: AtomicTerm = "a".into();
//     let b: AtomicTerm = "b".into();
//     let ptr_a = term_store.intern(a).clone();
//     let ptr_b = term_store.intern(b).clone();

//     let one: AtomicTerm = 1.into();
//     let two: AtomicTerm = 2.into();
//     let ptr_one = term_store.intern(one).clone();
//     let ptr_two = term_store.intern(two).clone();

//     assert_eq!(ptr_one, ptr_one.clone());

//     let mut mapx = BTreeMap::new();
//     mapx.insert(ptr_a.clone(), ptr_one.clone());
//     mapx.insert(ptr_b.clone(), ptr_two.clone());

//     let mut mapx1 = BTreeMap::new();
//     mapx1.insert(ptr_a.clone(), ptr_one.clone());
//     mapx1.insert(ptr_b.clone(), ptr_two.clone());

//     assert_eq!(mapx, mapx1);

//     let matchx: Match<AtomicPtrTerm> = mapx.into();
//     let matchx1: Match<AtomicPtrTerm> = mapx1.into();

//     assert_eq!(matchx, matchx1);

//     let ptr_matchx = match_store.intern(matchx);
//     let ptr_matchx1 = match_store.intern(matchx1);
    
//     assert_eq!(ptr_matchx, ptr_matchx.clone());
//     assert_eq!(match_store.matches.len(), 1);
//     assert_eq!(ptr_matchx.ptr, ptr_matchx1.ptr); // Compare two Arc<T>.
//     assert_eq!(ptr_matchx.ptr.as_ref(), ptr_matchx1.ptr.as_ref()); // Compare two T.
//     assert_eq!(ptr_matchx, ptr_matchx1); // Compare the location of allocation.

// }