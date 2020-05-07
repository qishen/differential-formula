use std::borrow::*;
use std::sync::Arc;
use std::vec::Vec;
use std::collections::*;
use std::convert::TryInto;
use std::fmt::*;
use std::string::String;

use enum_dispatch::enum_dispatch;
use serde::{Serialize, Deserialize};

use crate::expression::*;
use crate::term::*;
use crate::rule::*;
use crate::util::*;


#[enum_dispatch]
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Type {
    BaseType,
    CompositeType,
    EnumType,
    RangeType,
    RenamedType,
    UnionType,
    Undefined
}

impl Type {
    pub fn find_subterm(&self, term: &Arc<Term>, labels: &Vec<String>) -> Option<Arc<Term>> 
    {   
        // Note that self could be a renamed type too.
        match self.base_type() {
            Type::CompositeType(c) => {
                let mut ctype = c;
                let aggregated_term = labels.iter().fold(Some(term.clone()), |subterm_opt, label| {
                    subterm_opt.map(|subterm| {
                        // Find the first match or return None when none of them are matched.
                        ctype.arguments.iter().enumerate().find_map(|(i, (l_opt, t))| {
                            match l_opt {
                                Some(l) => {
                                    if label == l {
                                        match subterm.borrow() {
                                            Term::Composite(cterm) => {
                                                // Update the composite type for the next round.
                                                // Note that `t` could be a renamed type wrapping composite type.
                                                match t.base_type() {
                                                    Type::CompositeType(tc) => {
                                                        ctype = tc;
                                                    },
                                                    _ => {}
                                                }
                                                let cterm_arc = cterm.arguments.get(i).unwrap();
                                                Some(cterm_arc.clone())
                                            },
                                            _ => { None }
                                        }
                                    }
                                    else { None }
                                },
                                _ => { None }
                            }
                        })
                    }).unwrap()
                });
                
                aggregated_term
            },
            // This function only applies to composite type.
            _ => { None }
        }
    }

    // Wrap the base type to create a new type with an additional prefix.
    pub fn rename_type(&self, scope: String) -> Type {
        let base = self.clone();
        RenamedType {
            scope,
            base: Arc::new(base),
        }.into()
    }

    /// Unroll RenamedType recursively to find the base type that is not a RenamedType and 
    /// return a clone if the type is not a RenamedType.
    pub fn base_type(&self) -> &Type {
        match self {
            Type::RenamedType(rtype) => {
                rtype.base.base_type()
            },
            _ => {
                self
            }
        }
    }
}

#[enum_dispatch(Type)]
pub trait FormulaType {
    fn name(&self) -> String;
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct EnumType {
    pub name: String,
    pub items: Vec<Term>,
}

impl FormulaType for EnumType {
    fn name(&self) -> String {
        return format!("{:?}", self.name);
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct RenamedType {
    pub scope: String,
    pub base: Arc<Type>,
}

impl FormulaType for RenamedType {
    fn name(&self) -> String {
        return format!("{}.{}", self.scope, self.base.name());
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Undefined {}

impl FormulaType for Undefined {
    fn name(&self) -> String {
        "undefined".to_string()
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct RangeType {
    pub low: Term,
    pub high: Term,
}

impl FormulaType for RangeType {
    fn name(&self) -> String {
        return format!("({} .. {})", self.low, self.high);
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

impl FormulaType for BaseType {
    fn name(&self) -> String {
        return format!("{:?}", self);
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct CompositeType {
    pub name: String,
    pub arguments: Vec<(Option<String>, Arc<Type>)>
}

impl FormulaType for CompositeType {
    fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct UnionType {
    pub name: String,
    pub subtypes: Vec<Arc<Type>>,
}

impl FormulaType for UnionType {
    fn name(&self) -> String {
        self.name.clone()
    }
}