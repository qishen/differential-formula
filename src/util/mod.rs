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
}