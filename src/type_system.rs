use std::borrow::*;
use std::hash::Hash;
use std::vec::Vec;
use std::fmt::*;
use std::string::String;

use serde::{Serialize, Deserialize};

use crate::term::*;
use crate::util::wrapper::*;

/// A supertrait that includes Borrow<Type>, From<Type> and a bunch of basic traits that allow
/// the type to be compariable, cloneable, displayable and be able to transmit between threads.
// pub trait BorrowedType: Borrow<Type> + 
//                         From<Type> + 
//                         UniqueForm<String> + 
//                         differential_dataflow::ExchangeData + 
//                         Eq + 
//                         Clone + 
//                         Debug + 
//                         Display + 
//                         Hash + 
//                         Ord {}

pub trait FormulaTypeTrait {}

/// `RawType` is the type created directly from type definition in the domain without optimization. 
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Serialize, Deserialize)]
pub enum RawType {
    // Built-in primitive type e.g. integer, float, string. 
    BaseType(BaseType),
    // A ::= new (src: B, dst: C). A term with variables can be used to represent the type
    // A(m, n) where m is of type `B` and n is of type `C`.
    CompositeType(CompositeType),
    // A set of atoms e.g. B ::= {1.0, 100, "hello", -3.14}
    EnumType(EnumType),
    // A range of integers e.g. C ::= {1..100}
    RangeType(RangeType),
    // Prefixed types that are reused in other domains by inheritance.
    RenamedType(RenamedType),
    // A union of several types e.g. D ::= A + B + {1, 2, "hi"}
    UnionType(UnionType),
    // Undefined type for variables without context.
    Undefined(Undefined)
}

impl Display for RawType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.derive_unique_form())
    }
}

/// Each type has an unique form as string.
/// TODO: implement HasUniqueForm<RegularType>
impl HasUniqueForm<String> for RawType {
    fn derive_unique_form(&self) -> String {
        match self {
            RawType::BaseType(t) => t.derive_unique_form(),
            RawType::CompositeType(t) => t.derive_unique_form(),
            RawType::EnumType(t) => t.derive_unique_form(),
            RawType::RangeType(t) => t.derive_unique_form(),
            RawType::RenamedType(t) => t.derive_unique_form(),
            RawType::UnionType(t) => t.derive_unique_form(),
            RawType::Undefined(t) => t.derive_unique_form(),
        }
    }
}

impl RawType {
    /// Wrap the base type to create a new type with an additional prefix.
    pub fn rename_type(&self, scope: String) -> RawType {
        let base = self.clone();
        RawType::RenamedType(
            RenamedType { 
                scope, 
                base: base.into() 
            }
        )
    }

    /// Unroll RenamedType to return the base type for only one level.
    pub fn unrename_type(&self) -> &RawType {
        match self {
            RawType::RenamedType(rtype) => {
                rtype.base.borrow()
            },
            _ => { self }
        }
    }

    /// Unroll RenamedType recursively to find the base type that is not a RenamedType.
    pub fn base_type(&self) -> &RawType {
        match self {
            RawType::RenamedType(rtype) => {
                // Peel off Arc and UniqueFormWrapper.
                let type_ref: &RawType = rtype.base.borrow();
                type_ref.base_type()
            },
            _ => { self }
        }
    }

    /// Create an empty undefined type as pure Type.
    pub fn undefined() -> RawType {
        RawType::Undefined(Undefined{})
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct EnumType {
    pub name: String,
    pub items: Vec<AtomicTerm>,
}

impl FormulaTypeTrait for EnumType {}
impl HasUniqueForm<String> for EnumType {
    fn derive_unique_form(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct RenamedType {
    pub scope: String,
    pub base: Box<RawType>,
}

impl FormulaTypeTrait for RenamedType {}
impl HasUniqueForm<String> for RenamedType {
    fn derive_unique_form(&self) -> String {
        format!("{:?}.{:?}", self.scope, self.base.derive_unique_form())
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Undefined {}

impl FormulaTypeTrait for Undefined {}
impl HasUniqueForm<String> for Undefined {
    fn derive_unique_form(&self) -> String {
        "Undefined".to_string()
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct RangeType {
    low: AtomEnum, // low has to be a integer
    high: AtomEnum, // high has to be a integer
}

impl FormulaTypeTrait for RangeType {}
impl HasUniqueForm<String> for RangeType {
    fn derive_unique_form(&self) -> String {
        format!("{:?} .. {:?}", self.low, self.high)
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

impl FormulaTypeTrait for BaseType {}
impl HasUniqueForm<String> for BaseType {
    fn derive_unique_form(&self) -> String {
        let base_type_str = match self {
            BaseType::Boolean => "Boolean",
            BaseType::String => "String",
            BaseType::Integer => "Integer",
            BaseType::PosInteger => "PosInteger",
            BaseType::NegInteger => "NegInteger",
            BaseType::Rational => "Rational"
        };
        base_type_str.to_string()
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct CompositeType {
    pub name: String,
    pub arguments: Vec<(Option<String>, RawType)>
}

impl Debug for CompositeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut arg_strs: Vec<String> = vec![];
        for (tag, raw_type) in self.arguments.iter() {
            let arg_str = match tag {
                Some(name) => { format!("{}: {}", name, raw_type.derive_unique_form()) },
                None => { raw_type.derive_unique_form() }
            };
            arg_strs.push(arg_str);
        }
        write!(f, "{} ::= ({})", self.name, arg_strs.join(", "))
    }
}

impl FormulaTypeTrait for CompositeType {}
impl HasUniqueForm<String> for CompositeType {
    fn derive_unique_form(&self) -> String {
        self.name.clone()
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct UnionType {
    pub name: String,
    pub subtypes: Vec<RawType>,
}

impl Debug for UnionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let names: Vec<String> = self.subtypes.iter().map(|raw_type| {
            raw_type.derive_unique_form()
        }).collect();
        write!(f, "{} ::= {}", self.name, names.join(" + "))
    }
}

impl FormulaTypeTrait for UnionType {}
impl HasUniqueForm<String> for UnionType {
    fn derive_unique_form(&self) -> String {
        self.name.clone()
    }
}