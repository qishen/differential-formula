use std::borrow::*;
use std::hash::Hash;
use std::vec::Vec;
use std::fmt::*;
use std::string::String;

use serde::{Serialize, Deserialize};

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::term::*;
use crate::util::wrapper::*;

pub trait FormulaTypeTrait {}

#[derive(Eq, Ord, Debug, Clone, Serialize, Deserialize)]
pub enum RawType {
    TypeId(Cow<'static, str>),
    Type(FormulaTypeEnum),
    // TODO: Change to `Any` type which is a union of all types in FORMULA.
    Undefined,
}

// impl<'source> pyo3::FromPyObject<'source> for RawType {
//     fn extract(ob: &'source PyAny) -> PyResult<Self> {
//         todo!()
//     }
// }

impl PartialEq for RawType {
    fn eq(&self, other: &Self) -> bool {
        self.type_id() == other.type_id()
    }
}

impl PartialOrd for RawType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.type_id().partial_cmp(&other.type_id())
    }
}

impl Hash for RawType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.type_id().hash(state);
    }
}

impl HasUniqueForm<String> for RawType{
    fn derive_unique_form(&self) -> String {
        match self {
            RawType::TypeId(type_id) => type_id.clone().into_owned(),
            RawType::Type(formula_type) => formula_type.derive_unique_form(),
            _ => "Undefined".to_string()
        }
    }
}

impl Display for RawType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.derive_unique_form())
    }
}

impl RawType {
    /// Convert to `RawType::TypeId()`.
    pub fn alias_type(&self) -> Self {
        RawType::TypeId(self.type_id())
    }

    pub fn is_subtype_of(&self, other: &RawType) -> bool {
        match other {
            RawType::Type(raw_type) => {
                match raw_type {
                    FormulaTypeEnum::CompositeType(c) => {
                        c.arguments.iter().any(|(_, subtype)| {
                            self == subtype || self.is_subtype_of(subtype)
                        })
                    },
                    // TODO: Consider basic type and union type.
                    _ => false
                }
            },
            RawType::TypeId(_tid) => { false },
            _ => false
        }
    }

    /// Return the alias of the type as `Cow<str>`.
    pub fn type_id<'a>(&self) -> Cow<'a, str> {
        match self {
            RawType::Type(type_enum) => Cow::from(format!("{}", type_enum)),
            RawType::TypeId(tid) => tid.clone(),
            RawType::Undefined => Cow::from("~Undefined")
        }
    }

    /// Wrap the base type to create a new type with an additional prefix.
    pub fn rename_type(&self, scope: String) -> RawType {
        match self {
            RawType::Type(raw_type_enum) => {
                match raw_type_enum {
                    FormulaTypeEnum::BaseType(_) => {
                        self.clone()
                    },
                    _ => {
                        let base = raw_type_enum.clone();
                        let type_enum = FormulaTypeEnum::RenamedType(
                            RenamedType { scope, base: Box::new(RawType::Type(base)) }
                        );
                        RawType::Type(type_enum)
                    }
                }
            },
            RawType::TypeId(type_id) => {
                let renamed_typeid = format!("{}.{}", scope, type_id); 
                RawType::TypeId(Cow::from(renamed_typeid))
            },
            _ => RawType::Undefined
        }
    }

    /// Unroll RenamedType to return the base type for only one level.
    pub fn unrename_type(&self) -> Option<RawType> {
        match self {
            RawType::Type(raw_type_enum) => {
                match raw_type_enum {
                    FormulaTypeEnum::RenamedType(rtype) => {
                        Some(rtype.base.as_ref().clone())
                    },
                    _ => { None }
                }
            },
            _ => None
        }
    }
}

/// `FormulaTypeEnum` is the type created directly from type definition in the domain without optimization. 
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Serialize, Deserialize)]
pub enum FormulaTypeEnum {
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
}

impl Display for FormulaTypeEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.derive_unique_form())
    }
}

/// Each type has an unique form to be derived as string.
impl HasUniqueForm<String> for FormulaTypeEnum {
    fn derive_unique_form(&self) -> String {
        match self {
            FormulaTypeEnum::BaseType(t) => t.derive_unique_form(),
            FormulaTypeEnum::CompositeType(t) => t.derive_unique_form(),
            FormulaTypeEnum::EnumType(t) => t.derive_unique_form(),
            FormulaTypeEnum::RangeType(t) => t.derive_unique_form(),
            FormulaTypeEnum::RenamedType(t) => t.derive_unique_form(),
            FormulaTypeEnum::UnionType(t) => t.derive_unique_form(),
        }
    }
}

#[pyclass]
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

#[pyclass]
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

#[pyclass]
#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct CompositeType {
    #[pyo3(get, set)]
    pub name: String,
    // #[pyo3(get, set)]
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

#[pyclass]
#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct UnionType {
    #[pyo3(get, set)]
    pub name: String,
    // #[pyo3(get, set)]
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


mod tests {
    use super::*;
    use crate::parser::combinator::parse_program;
    use crate::module::*;
    use std::collections::HashSet;
    use std::path::Path;
    use std::fs;

    fn load_graph_domain() -> Domain {
        let path = Path::new("./tests/testcase/p0.4ml");
        let content = fs::read_to_string(path).unwrap() + "EOF";
        let (_, program_ast) = parse_program(&content);
        let env: Env = program_ast.build_env();
        let graph = env.get_domain_by_name("Graph").unwrap();
        graph.clone()
    }

    #[test]
    fn test_type_equality() {
        let t1 = RawType::TypeId(Cow::from("hello"));
        let t2 = RawType::TypeId(Cow::from("hello".to_string()));
        let t3 = RawType::Type(FormulaTypeEnum::CompositeType(
            CompositeType {
                name: "hello".to_string(),
                arguments: vec![]
            }
        ));
        assert_eq!(t1, t2);
        assert_eq!(t1, t3);

        // Hash value is computed from type alias too.
        let mut set = HashSet::new();
        set.insert(t1);
        assert!(set.contains(&t2));
        assert!(set.contains(&t3));
    }

    #[test]
    fn test_sorted_composite_types() {
        let graph = load_graph_domain();
        let sorted_types = graph.meta_info().sorted_composite_types();
        assert_eq!(sorted_types.get(0).unwrap().type_id(), "Node");
        println!("{:#?}", sorted_types);
    }
}