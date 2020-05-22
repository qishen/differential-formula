use std::hash::{Hash, Hasher};
use std::iter::*;
use std::borrow::Borrow;
use std::collections::{BTreeMap, HashMap};
use std::collections::btree_map::*;
use im::OrdMap;
use differential_dataflow::hashable::*;
use serde::*;
use fnv;


use crate::term::*;

#[derive(Debug, Clone)]
pub struct NameGenerator {
    prefix: String,
    counter: i64
}

impl NameGenerator {
    pub fn new(prefix: &str) -> Self {
        NameGenerator { 
            prefix: prefix.to_string(), 
            counter: 0 
        }
    }

    pub fn generate_name(&mut self) -> String {
        format!("{}{}", self.prefix, self.counter)
    }

    pub fn generate_dc_term(&mut self) -> Term {
        let var: Term = Variable::new(format!("{}{}", self.prefix, self.counter), vec![]).into();
        self.counter += 1;
        var
    }
}

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

impl<K, V> GenericMap<K, V> for OrdMap<K, V>
where
    K: Eq + Hash + Ord + Clone,
    V: Clone,
{
    fn gkeys(&self) -> Vec<&K> {
        let mut list = vec![];
        let keys = OrdMap::keys(self);
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
        OrdMap::contains_key(self, k)
    }

    fn gget<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord,
    {
        OrdMap::get(self, k)
    }

    fn ginsert(&mut self, k: K, v: V) -> Option<V> 
    {
        OrdMap::insert(self, k, v)
    }
}

/// `QuickHashOrdMap is actually an `OrdMap` wrapped twice to stash the hash of the map
/// and compare the hash first when deciding the ordering of two maps.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct QuickHashOrdMap<K, V> 
where 
    K: Hash + Ord, 
    V: Hash + Ord,
{
    map: OrdWrapper<HashableWrapper<BTreeMap<K, V>>>
}

impl<K, V> Hashable for QuickHashOrdMap<K, V> 
where
    K: Hash + Ord,
    V: Hash + Ord,
{
    type Output = u64;
    #[inline]
    fn hashed(&self) -> Self::Output { 
        let mut h: fnv::FnvHasher = Default::default();
        self.map.item.item.hash(&mut h);
        h.finish()
    }
}

impl<K, V> From<QuickHashOrdMap<K, V>> for BTreeMap<K, V>
where
    K: Hash + Ord,
    V: Hash + Ord
{
    fn from(item: QuickHashOrdMap<K, V>) -> BTreeMap<K, V> {
        item.map.item.item
    }
}

impl<K, V> From<BTreeMap<K, V>> for QuickHashOrdMap<K, V> 
where
    K: Hash + Ord,
    V: Hash + Ord
{
    fn from(item: BTreeMap<K, V>) -> QuickHashOrdMap<K, V> {
        let map_with_hash: HashableWrapper<BTreeMap<K, V>> = item.into();
        QuickHashOrdMap {
            map: OrdWrapper { item: map_with_hash }
        }
    }
}

// impl<K, V> Serialize for QuickHashOrdMap<K, V> 
// where 
//     K: Clone + Hash + Ord + Serialize, 
//     V: Clone + Hash + Ord + Serialize
// {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut s = serializer.serialize_struct("QuickHashOrdMap", 3)?;
//         s.serialize_field("map", &**self.map)?;
//         s.end()
//     }
// }

impl<K, V> GenericMap<K, V> for QuickHashOrdMap<K, V>
where
    K: Eq + Hash + Ord + Clone,
    V: Clone + Hash + Ord,
{
    fn gkeys(&self) -> Vec<&K> {
        let mut list = vec![];
        let keys = BTreeMap::keys(&self.map.item.item);
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
        BTreeMap::contains_key(&self.map.item.item, k)
    }

    fn gget<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord,
    {
        BTreeMap::get(&self.map.item.item, k)
    }

    fn ginsert(&mut self, k: K, v: V) -> Option<V> 
    {
        BTreeMap::insert(&mut self.map.item.item, k, v)
    }
}