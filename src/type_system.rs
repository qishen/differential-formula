use std::borrow::*;
use std::hash::Hash;
use std::sync::Arc;
use std::vec::Vec;
use std::fmt::*;
use std::string::String;

use serde::{Serialize, Deserialize};

use crate::term::*;
use crate::util::wrapper::*;

/// A supertrait that includes Borrow<Type>, From<Type> and a bunch of basic traits that allow
/// the type to be compariable, cloneable, displayable and be able to transmit between threads.
pub trait BorrowedType: 
    Borrow<Type> +       // Able to be borrowed as a native type.
    From<Type> +         // Able to be created from a native type.
    UniqueForm<String> + // Have an unique form as string and can be accessed as reference.
    differential_dataflow::ExchangeData +
    Eq + Clone + Debug + Display + Hash + Ord {}


impl From<Type> for Arc<UniqueFormWrapper<String, Type>> {} 
impl Borrow<Type> for Arc<UniqueFormWrapper<String, Type>> {}

pub trait FormulaType {}
impl<T> FormulaType for T where T: BorrowedType {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Serialize, Deserialize)]
pub enum Type {
    BaseType(BaseType),
    CompositeType(CompositeType),
    EnumType(EnumType),
    RangeType(RangeType),
    RenamedType(RenamedType),
    UnionType(UnionType),
    Undefined(Undefined)
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.derive_unique_form())
    }
}

/// Each type has an unique form as string.
impl HasUniqueForm<String> for Type {
    fn derive_unique_form(&self) -> String {
        match self {
            Type::BaseType(t) => t.derive_unique_form(),
            Type::CompositeType(t) => t.derive_unique_form(),
            Type::EnumType(t) => t.derive_unique_form(),
            Type::RangeType(t) => t.derive_unique_form(),
            Type::RenamedType(t) => t.derive_unique_form(),
            Type::UnionType(t) => t.derive_unique_form(),
            Type::Undefined(t) => t.derive_unique_form(),
        }
    }
}

impl Type {
    /// Wrap the base type to create a new type with an additional prefix.
    pub fn rename_type(&self, scope: String) -> Type {
        let base = self.clone();
        Type::RenamedType(
            RenamedType { 
                scope, 
                base: Arc::new(base.into()) 
            }
        )
    }

    /// Unroll RenamedType recursively to find the base type that is not a RenamedType and 
    /// return a clone if the type is not a RenamedType.
    pub fn base_type(&self) -> &Type {
        match self {
            Type::RenamedType(rtype) => {
                // Peel off Arc and UniqueFormWrapper.
                let type_ref: &Type = rtype.base.borrow();
                type_ref.base_type()
            },
            _ => { self }
        }
    }

    /// Create an empty undefined type as pure Type.
    pub fn undefined() -> Type {
        Type::Undefined(Undefined{})
    }

    /// Create an empty undefined type as AtomicStrType.
    pub fn atomic_undefined() -> AtomicStrType {
        Type::undefined().into()
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct EnumType {
    pub name: String,
    // Use AtomicTerm by default.
    pub items: Vec<AtomicTerm>,
}

impl FormulaType for EnumType {}
impl HasUniqueForm<String> for EnumType {
    fn derive_unique_form(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct RenamedType {
    pub scope: String,
    pub base: Arc<UniqueFormWrapper<String, Type>>,
}

impl FormulaType for RenamedType {}
impl HasUniqueForm<String> for RenamedType {
    fn derive_unique_form(&self) -> String {
        let wrapper: &UniqueFormWrapper<String, Type> = self.base.borrow();
        format!("{:?}.{:?}", self.scope, wrapper.derive_unique_form())
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Undefined {}

impl FormulaType for Undefined {}
impl HasUniqueForm<String> for Undefined {
    fn derive_unique_form(&self) -> String {
        "Undefined".to_string()
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct RangeType {
    low: AtomEnum,
    high: AtomEnum,
}

impl FormulaType for RangeType {}
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

impl FormulaType for BaseType {}
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

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct CompositeType {
    pub name: String,
    pub arguments: Vec<(Option<String>, AtomicStrType)>
}

impl FormulaType for CompositeType {}
impl HasUniqueForm<String> for CompositeType {
    fn derive_unique_form(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct UnionType {
    pub name: String,
    pub subtypes: Vec<AtomicStrType>,
}

impl FormulaType for UnionType {}
impl HasUniqueForm<String> for UnionType {
    fn derive_unique_form(&self) -> String {
        self.name.clone()
    }
}