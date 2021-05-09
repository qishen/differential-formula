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

/// `QuickHashOrdMap is actually an `BTreeMap` wrapped twice to stash the hash of the map
/// and compare the hash first when deciding the ordering of two maps. When two maps have
/// the same hash, compare the unique forms of the maps instead of recursively
#[derive(Debug, Clone)]
pub struct QuickHashOrdMap<K, V> 
where 
    K: Hash + Ord, 
    V: Hash + Ord,
{
    // A string to represent the content in the hash map.
    unique_form: String,
    map: OrdWrapper<HashableWrapper<BTreeMap<K, V>>>
}

// impl<K, V> Hash for QuickHashOrdMap<K, V> {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.id.hash(state);
//         self.phone.hash(state);
//     }
// }

impl<K: Ord+Hash, V: Ord+Hash> Display for QuickHashOrdMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.unique_form)
    }
}

impl<K: Ord+Hash, V: Ord+Hash> Eq for QuickHashOrdMap<K, V> {} 

impl<K: Ord+Hash, V: Ord+Hash> PartialEq for QuickHashOrdMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.unique_form == other.unique_form
    }
}

impl<K: Ord+Hash, V: Ord+Hash> Ord for QuickHashOrdMap<K, V> {
    /// Use the unique name to compare ordering because that's fast!
    fn cmp(&self, other: &Self) -> Ordering {
        self.unique_form.cmp(&other.unique_form)
    }
}

impl<K: Hash+Ord, V: Hash+Ord> PartialOrd for QuickHashOrdMap<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Use default FNV hasher to implement `Hashable` trait given `Hash` is implemented for BTreeMap.
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

// Convert QuickHashOrdMap back into BTreeMap.
impl<K, V> From<QuickHashOrdMap<K, V>> for BTreeMap<K, V>
where
    K: Hash + Ord,
    V: Hash + Ord
{
    fn from(item: QuickHashOrdMap<K, V>) -> BTreeMap<K, V> {
        item.map.item.item
    }
}

/// Convert a BTreeMap into QuickHashOrdMap by adding unique form and hash.
impl<K, V> From<BTreeMap<K, V>> for QuickHashOrdMap<K, V> 
where
    K: Hash + Ord + Display,
    V: Hash + Ord + Display,
{
    fn from(item: BTreeMap<K, V>) -> QuickHashOrdMap<K, V> {
        let mut pairs = vec![];
        for (k, v) in item.iter() {
            pairs.push(format!("{}: {}", k, v));
        }
        let unique_form = format!("{{ {} }}", pairs.join(", "));
        let map_with_hash: HashableWrapper<BTreeMap<K, V>> = item.into();
        QuickHashOrdMap {
            unique_form,
            map: OrdWrapper { item: map_with_hash }
        }
    }
}

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