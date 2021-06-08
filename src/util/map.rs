use std::cmp::Ordering;
use std::fmt::*;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;
use std::collections::{BTreeMap, HashMap};
use differential_dataflow::hashable::*;
use serde::*;
use fnv;

/// A Map trait to provide some basic common map operations.
/// Rust standard lib does not provide a generic map interface like C# does after v1.0 because 
/// it's user's responsibility to define what is a generic map and what functions should be included
/// since different users have different needs for generic map interface.
pub trait GenericMap<K, V> {
    fn gkeys(&self) -> Vec<&K>;

    fn contains_gkey<Q>(&self, k: &Q) -> bool 
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord;

    fn gget<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord;

    fn ginsert(&mut self, k: K, v: V) -> Option<V>;
}

impl<K, V> GenericMap<K, V> for BTreeMap<K, V>
where
    K: Eq + Hash + Ord + Clone,
    V: Clone,
{
    fn gkeys(&self) -> Vec<&K> {
        let mut list = vec![];
        let keys = BTreeMap::keys(self);
        for key in keys {
            list.push(key);
        }
        list
    }

    fn contains_gkey<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord,
    {
        BTreeMap::contains_key(self, k)
    }

    fn gget<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord,
    {
        BTreeMap::get(self, k)
    }

    fn ginsert(&mut self, k: K, v: V) -> Option<V> {
        BTreeMap::insert(self, k, v)
    }
}

impl<K, V> GenericMap<K, V> for HashMap<K, V>
where
    K: Eq + Hash + Ord + Clone,
    V: Clone,
{
    fn gkeys(&self) -> Vec<&K> {
        let mut list = vec![];
        let keys = HashMap::keys(self);
        for key in keys {
            list.push(key);
        }
        list
    }

    fn contains_gkey<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord,
    {
        HashMap::contains_key(self, k)
    }

    fn gget<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord,
    {
        HashMap::get(self, k)
    }

    fn ginsert(&mut self, k: K, v: V) -> Option<V> {
        HashMap::insert(self, k, v)
    }
}

#[derive(Debug, Clone, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct PtrHashMap<K, V> where K: Ord {
    map: BTreeMap<K, V>
}

impl<K, V> Hash for PtrHashMap<K, V> where K: Ord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self, state);
    }
}

impl<K, V> PartialEq for PtrHashMap<K, V> where K: Ord {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl<K,V> PtrHashMap<K, V> where K: Ord {
    pub fn new(map: BTreeMap<K, V>) -> Self {
        PtrHashMap { map }
    }
}

impl<K, V> GenericMap<K, V> for PtrHashMap<K, V>
where
    K: Eq + Hash + Ord + Clone,
    V: Clone + Hash + Ord,
{
    fn gkeys(&self) -> Vec<&K> {
        let mut list = vec![];
        let keys = BTreeMap::keys(&self.map);
        for key in keys {
            list.push(key);
        }
        list
    }

    fn contains_gkey<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord,
    {
        BTreeMap::contains_key(&self.map, k)
    }

    fn gget<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord,
    {
        BTreeMap::get(&self.map, k)
    }

    fn ginsert(&mut self, k: K, v: V) -> Option<V> 
    {
        BTreeMap::insert(&mut self.map, k, v)
    }
}