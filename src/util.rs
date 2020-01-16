use core::hash::Hash;
use core::borrow::Borrow;

use std::iter::*;
use std::collections::HashMap;
use std::collections::hash_map::Keys;
use im::OrdMap;


/*
Rust standard lib does not provide a generic map interface like C# does after v1.0 because 
it's user's responsibility to define what is a generic map and what functions should be included
since different users have different needs for generic map interface. Zhong Kou Nan Tiao.
*/
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
    where
    {
        OrdMap::insert(self, k, v)
    }
}
