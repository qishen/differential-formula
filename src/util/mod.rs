pub mod wrapper;
pub mod map;


use std::cmp::Ordering;
use std::fmt::*;
use std::hash::{Hash, Hasher};
use std::iter::Iterator;
use std::borrow::Borrow;
use std::collections::{BTreeMap, HashMap};
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
