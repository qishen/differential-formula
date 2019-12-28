use core::hash::Hash;
use core::borrow::Borrow;

use std::collections::HashMap;
use im::OrdMap;


/*
Rust standard lib does not provide a generic map interface like C# does after v1.0 because 
it's user's responsibility to define what is a generic map and what functions should be included
since different users have different needs for generic map interface. Zhong Kou Nan Tiao.
*/
pub trait GenericMap<K, V> {
    fn contains_key<Q>(&self, k: &Q) -> bool 
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord;

    fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord;

    fn insert(&mut self, k: K, v: V) -> Option<V>;
}

impl<K, V> GenericMap<K, V> for HashMap<K, V>
where
    K: Eq + Hash + Ord + Clone,
    V: Clone,
{
    fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord,
    {
        HashMap::contains_key(self, k)
    }

    fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord,
    {
        HashMap::get(self, k)
    }

    fn insert(&mut self, k: K, v: V) -> Option<V> {
        HashMap::insert(self, k, v)
    }
}

impl<K, V> GenericMap<K, V> for OrdMap<K, V>
where
    K: Eq + Hash + Ord + Clone,
    V: Clone,
{
    fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord,
    {
        OrdMap::contains_key(self, k)
    }

    fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord,
    {
        OrdMap::get(self, k)
    }

    fn insert(&mut self, k: K, v: V) -> Option<V> 
    where
    {
        OrdMap::insert(self, k, v)
    }
}
