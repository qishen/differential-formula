use std::borrow::*;
use std::sync::Arc;
use std::vec::Vec;
use std::collections::*;
use std::convert::TryInto;
use std::fmt::{Debug, Display, Formatter};
use std::string::String;

use enum_dispatch::enum_dispatch;
use serde::{Serialize, Deserialize};

use crate::term::*;
use crate::rule::*;


#[enum_dispatch]
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Type {
    BaseType,
    CompositeType,
    RangeType,
    UnionType,
    Undefined
}

impl Type {
    pub fn find_subterm<'a, T>(&self, term: &'a T, labels: &Vec<String>) -> Option<&'a Term> 
    where T: Borrow<Term>,
    {
        match self {
            Type::CompositeType(ctype) => {
                let aggregated_term = labels.iter().fold(Some(term.borrow()), |subterm_opt, label| {
                    match subterm_opt {
                        Some(subterm) => {
                            // Find the first match or return None when none of them are matched.
                            ctype.arguments.iter().enumerate().find_map(|(i, (l_opt, t))| {
                                match l_opt {
                                    Some(l) => {
                                        if label == l {
                                            match subterm {
                                                Term::Composite(cterm) => {
                                                    let cterm_arc = cterm.arguments.get(i).unwrap();
                                                    // Impl Borrow<Q> where Q is infered as Term for Arc<Term> exists. 
                                                    Some(cterm_arc.borrow())
                                                },
                                                _ => { None }
                                            }
                                        }
                                        else { None }
                                    },
                                    None => { None },
                                }
                            })
                        },
                        None => { None }
                    }
                });

                aggregated_term
            },
            // Only apply to composite type.
            _ => { None }
        }
    }
}

#[enum_dispatch(Type)]
pub trait TypeBehavior {
    fn name(&self) -> String;
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Undefined {}

impl TypeBehavior for Undefined {
    fn name(&self) -> String {
        "undefined".to_string()
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct RangeType {
    pub low: Term,
    pub high: Term,
}

impl TypeBehavior for RangeType {
    fn name(&self) -> String {
        return format!("({}, {})", self.low, self.high);
    }
}


#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum BaseType {
    Boolean,
    String,
    Integer,
    PosInteger,
    NegInteger,
    Rational,
}

impl TypeBehavior for BaseType {
    fn name(&self) -> String {
        return format!("{:?}", self);
    }
}


#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct CompositeType {
    pub name: String,
    pub arguments: Vec<(Option<String>, Type)>
}

impl TypeBehavior for CompositeType {
    fn name(&self) -> String {
        self.name.clone()
    }
}


#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct UnionType {
    pub name: String,
    pub subtypes: Vec<Type>,
}

impl TypeBehavior for UnionType {
    fn name(&self) -> String {
        self.name.clone()
    }
}



#[enum_dispatch]
#[derive(Debug, Clone)]
pub enum Program {
    Domain,
    Model,
}


#[derive(Debug, Clone)]
pub struct Domain {
    pub name: String,
    pub type_map: HashMap<String, Arc<Type>>,
    pub rules: Vec<Rule>,
}

impl Domain {
    pub fn get_type(&self, name: &String) -> Arc<Type> {
        self.type_map.get(name).unwrap().clone()
    }
}

impl Domain {
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn stratified_rules(&self) -> Vec<Vec<Rule>> {
        // TODO:
        vec![self.rules.clone()]
    }
}


#[derive(Debug, Clone)]
pub struct Model {
    pub model_name: String,
    pub domain_name: String,
    pub models: Vec<Term>,
    // variable term to composite term mapping.
    pub alias_map: HashMap<Term, Term>, 
    // composite term to string alias mapping and the composite can't have alias built in.
    pub reverse_alias_map: HashMap<Term, String>, 
}

impl Model {
    pub fn new(model_name: String, 
        domain_name: String, 
        models: Vec<Term>, 
        alias_map: HashMap<Term, Term>) -> Self {
        
        let mut reverse_alias_map = HashMap::new();
        for key in alias_map.keys() {
            let variable: Variable = key.clone().try_into().unwrap();
            let var_str = variable.root;
            
            /* 
            Clone each entry in the alias map to create reverse alias map mapping composite term to 
            alias in the format of string and the alias is removed for each key.
            */
            let mut val_composite: Composite = alias_map.get(key).unwrap().clone().try_into().unwrap();
            val_composite.alias = None;
            reverse_alias_map.insert(val_composite.into(), var_str);
        }

        let model = Model {
            model_name,
            domain_name,
            models,
            alias_map,
            reverse_alias_map,
        };

        model
    }
}


#[derive(Debug, Clone)]
pub struct Env {
    pub domain_map: HashMap<String, Domain>,
    pub model_map: HashMap<String, Model>,
}


impl Env {
    pub fn get_model_by_name(&self, name: String) -> Option<&Model> {
        // Make a clone of the model in Env and return it.
        match self.model_map.get(&name) {
            None => None,
            Some(model) => Some(model),
        }
    }

    pub fn get_domain_by_name(&self, name: String) -> Option<&Domain> {
        // Make a clone of the domain in Env and return it.
        match self.domain_map.get(&name) {
            None => None,
            Some(model) => Some(model),
        }
    }
}