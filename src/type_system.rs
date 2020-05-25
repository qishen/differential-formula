use std::sync::Arc;
use std::vec::Vec;
use std::fmt::*;
use std::string::String;
use enum_dispatch::*;
use serde::{Serialize, Deserialize};
use crate::term::*;


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
    /// Wrap the base type to create a new type with an additional prefix.
    pub fn rename_type(&self, scope: String) -> Type {
        let base = self.clone();
        let new_name = format!("{}.{}", scope, base.name());
        RenamedType { 
            name: new_name, 
            scope, 
            base: Arc::new(base) 
        }.into()
    }

    /// Unroll RenamedType recursively to find the base type that is not a RenamedType and 
    /// return a clone if the type is not a RenamedType.
    pub fn base_type(&self) -> &Type {
        match self {
            Type::RenamedType(rtype) => {
                rtype.base.base_type()
            },
            _ => { self }
        }
    }
}

#[enum_dispatch(Type)]
pub trait FormulaType {
    fn name(&self) -> &String;
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct EnumType {
    pub name: String,
    pub items: Vec<Term>,
}

impl FormulaType for EnumType {
    fn name(&self) -> &String {
        return &self.name;
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct RenamedType {
    pub name: String,
    pub scope: String,
    pub base: Arc<Type>,
}

impl FormulaType for RenamedType {
    fn name(&self) -> &String {
        return &self.name;
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Undefined {
    pub name: String,
}

impl FormulaType for Undefined {
    fn name(&self) -> &String {
        //"undefined".to_string()
        return &self.name;
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct RangeType {
    pub name: String,
    low: Term,
    high: Term,
}

impl FormulaType for RangeType {
    fn name(&self) -> &String {
        //return format!("({} .. {})", self.low, self.high);
        return &self.name;
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum BaseTypeEnum {
    Boolean,
    String,
    Integer,
    PosInteger,
    NegInteger,
    Rational,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct BaseType {
    pub name: String,
    pub base: BaseTypeEnum,
}

impl BaseType {
    pub fn new(name: &str) -> Self {
        match name {
            "Boolean" => BaseType { name: "Boolean".to_string(), base: BaseTypeEnum::Boolean },
            "String" => BaseType { name: "String".to_string(), base: BaseTypeEnum::String },
            "Integer" => BaseType { name: "Integer".to_string(), base: BaseTypeEnum::Integer },
            "PosInteger" => BaseType { name: "PosInteger".to_string(), base: BaseTypeEnum::PosInteger },
            "NegInteger" => BaseType { name: "NegInteger".to_string(), base: BaseTypeEnum::NegInteger },
            _ => BaseType { name: "Rational".to_string(), base: BaseTypeEnum::Rational },
        }
    }
}

impl FormulaType for BaseType {
    fn name(&self) -> &String {
        // return format!("{:?}", self);
        return &self.name;
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct CompositeType {
    pub name: String,
    pub arguments: Vec<(Option<String>, Arc<Type>)>
}

impl FormulaType for CompositeType {
    fn name(&self) -> &String {
        return &self.name;
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct UnionType {
    pub name: String,
    pub subtypes: Vec<Arc<Type>>,
}

impl FormulaType for UnionType {
    fn name(&self) -> &String {
        return &self.name;
    }
}