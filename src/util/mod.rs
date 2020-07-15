pub mod wrapper;
pub mod map;

use std::fmt::*;
use std::hash::Hash;

use im::*;

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
        let name = format!("{}{}", self.prefix, self.counter);
        self.counter += 1;
        name
    }

    // Cannot decide what's the exact type of Term<S, T>
    // pub fn generate_dc_term(&mut self) -> Term {
    //     let var: Term = Variable::new(format!("{}{}", self.prefix, self.counter), vec![]).into();
    //     self.counter += 1;
    //     var
    // }
}


pub fn ldiff_intersection_rdiff<T: Eq+Hash+Clone+Ord>(one: &OrdSet<T>, two: &OrdSet<T>) 
-> (OrdSet<T>, OrdSet<T>, OrdSet<T>) 
{
    let left: OrdSet<T> = one.clone().difference(two.clone()).into_iter().map(|x| x.clone()).collect();
    let middle: OrdSet<T> = one.clone().intersection(two.clone()).into_iter().map(|x| x.clone()).collect();
    let right: OrdSet<T> = two.clone().difference(one.clone()).into_iter().map(|x| x.clone()).collect();
    (left, middle, right)
}
