use std::borrow::Borrow;
use std::sync::Arc;
use std::hash::Hash;
use std::fmt::*;
use std::marker::PhantomData;

use serde::{Serialize, Deserialize};

use super::generic::*;
use super::VisitTerm;
use crate::type_system::*;
use crate::util::wrapper::*;

/// Wrap an atomic type with unique string form wrapper.
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct AtomicStrType {
    inner: Arc<UniqueFormWrapper<String, Type>>
}

impl BorrowedType for AtomicStrType {}

impl Debug for AtomicStrType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.inner)
    }
}

impl Display for AtomicStrType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.inner)
    }
}

impl Borrow<String> for AtomicStrType {
    fn borrow(&self) -> &String {
        self.inner.as_ref().unique_form()
    }
}

impl Borrow<Type> for AtomicStrType {
    fn borrow(&self) -> &Type {
        self.inner.as_ref().item()
    }
}

impl From<Type> for AtomicStrType {
    fn from(item: Type) -> Self {
        let wrapper: UniqueFormWrapper<String, Type> = item.into();
        AtomicStrType { inner: Arc::new(wrapper) }
    }
}

impl HasUniqueForm<String> for AtomicStrType {
    fn derive_unique_form(&self) -> String {
        self.inner.derive_unique_form()
    }
}

impl UniqueForm<String> for AtomicStrType {
    fn unique_form(&self) -> &String {
        self.inner.unique_form()
    }

    fn update_unique_form(&mut self) {
        self.inner.update_unique_form();
    }
}

/// Wrap an atomic term with unique string form wrapper.
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct AtomicStrTerm {
    term: Arc<UniqueFormWrapper<String, AtomicTerm>>
}

impl BorrowedTerm for AtomicStrTerm {
    type SortOutput = AtomicStrType;
    type TermOutput = AtomicTerm;
}

impl Debug for AtomicStrTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.term)
    }
}

impl Display for AtomicStrTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.term)
    }
}

impl Borrow<String> for AtomicStrTerm {
    fn borrow(&self) -> &String {
        self.term.as_ref().unique_form()
    }
}

impl From<AtomicTerm> for AtomicStrTerm {
    fn from(item: AtomicTerm) -> AtomicStrTerm {
        let wrapper: UniqueFormWrapper<String, AtomicTerm> = item.into();
        AtomicStrTerm { term: Arc::new(wrapper) }
    }
}

impl HasUniqueForm<String> for AtomicStrTerm {
    fn derive_unique_form(&self) -> String {
        self.term.derive_unique_form()
    }
}

impl UniqueForm<String> for AtomicStrTerm {
    fn unique_form(&self) -> &String {
        self.term.unique_form()
    }

    fn update_unique_form(&mut self) {
        self.term.update_unique_form();
    }
}

/// AtomicTerm is safe to transfer between threads with both type and sub-terms thread safe.
/// AtomicTerm can be converted into Term<S, T> without copies of 
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum AtomicTerm {
    Atom(AtomicAtom),
    Variable(AtomicVariable),
    Composite(AtomicComposite),
}

impl BorrowedTerm for AtomicTerm {
    type SortOutput = AtomicStrType;
    type TermOutput = AtomicTerm;
}

impl HasUniqueForm<String> for AtomicTerm {
    fn derive_unique_form(&self) -> String {
        match self {
            AtomicTerm::Atom(a) => a.derive_unique_form(),
            AtomicTerm::Variable(v) => v.derive_unique_form(),
            AtomicTerm::Composite(c) => c.derive_unique_form(),
        }
    }
}

/// Convert AtomicTerm into Term<S, T> smoothly without any deep copies of the fields.
impl From<AtomicTerm> for Term<AtomicStrType, AtomicStrTerm> {    
    fn from(item: AtomicTerm) -> Term<AtomicStrType, AtomicStrTerm> {
        match item {
            AtomicTerm::Atom(a) => {
                let atom: Atom<AtomicStrType, AtomicStrTerm> = a.into();
                Term::Atom(atom)
            },
            AtomicTerm::Variable(v) => {
                let variable: Variable<AtomicStrType, AtomicStrTerm> = v.into();
                Term::Variable(variable)
            },
            AtomicTerm::Composite(c) => {
                let composite: Composite<AtomicStrType, AtomicStrTerm> = c.into();
                Term::Composite(composite)
            }
        }
    }
}

/// Put some term-related static methods here.
impl AtomicTerm {
    /// Given a string create a nullary composite type with no arguments inside and return the singleton term.
    pub fn create_constant(constant: String) -> AtomicTerm {
        let nullary_type = Type::CompositeType(
            CompositeType { name: constant, arguments: vec![] }
        );
        // let wrapped_nullary_type: UniqueFormWrapper<String, Type> = nullary_type.into();
        let term = AtomicTerm::Composite(
            AtomicComposite::new(
                nullary_type.into(), 
                vec![], 
                None
            )
        );
        return term;
    }

    /// Convert from AtomicTerm by default.
    pub fn create_atom(atom_enum: AtomEnum) -> AtomicTerm {
        let atomic_term = AtomicTerm::Atom(AtomicAtom::new(atom_enum));
        return atomic_term;
    }

    /// Create an atomic variable.
    pub fn create_variable(root: String, fragments: Vec<String>) -> AtomicTerm {
        let undefined = Type::undefined();
        let var = AtomicVariable::new(undefined.into(), root, fragments);
        AtomicTerm::Variable(var)
    }

    /// Compare two lists of variable terms and return true if some terms in one list
    /// are subterms of the terms in another list. 
    pub fn has_deep_intersection<I, T>(a: I, b: I) -> bool where I: Iterator<Item=T>, T: BorrowedTerm,
    {
        for v1 in a {
            for v2 in b {
                if v1.has_subterm(&v2).unwrap() || 
                   v2.has_subterm(&v1).unwrap() {
                    return true;
                }
            }
        }
        return false;
    }
}

impl Display for AtomicTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let term_str = match self {
            AtomicTerm::Composite(c) => format!("{}", c),
            AtomicTerm::Variable(v) => format!("{}", v),
            AtomicTerm::Atom(a) => format!("{}", a),
        };
        write!(f, "{}", term_str)
    }
}

impl Debug for AtomicTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Rewrite Debug trait in the same way as Display.
        write!(f, "{}", self)
    }
}

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct AtomicAtom {
    // The type of the atom as Arc<UniqueFormWrapper<String, Type>>
    pub sort: AtomicStrType,
    // The field holding the value as AtomEnum defined in generic term.
    pub val: AtomEnum,
}

impl HasUniqueForm<String> for AtomicAtom {
    fn derive_unique_form(&self) -> String {
        // The same as display name
        format!("{}", self)
    }
}

impl Display for AtomicAtom {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let atom_str = match &self.val {
            AtomEnum::Int(i) => format!("{}", i),
            AtomEnum::Bool(b) => format!("{:?}", b),
            AtomEnum::Str(s) => format!("\"{:?}\"", s),
            AtomEnum::Float(f) => format!("{}", f),
        };
        write!(f, "{}", atom_str)
    }
}

impl AtomicAtom {
    pub fn new(atom_enum: AtomEnum) -> AtomicAtom {
        // Decide the sort based on the enum value.
        let base_type = match atom_enum {
            AtomEnum::Bool(b)  => { BaseType::Boolean },
            AtomEnum::Float(f) => { BaseType::Rational },
            AtomEnum::Int(i)   => { BaseType::Integer },
            AtomEnum::Str(s)   => { BaseType::String }
        };

        // TODO: Use static constants for types without having to create a new type each time.
        // Create a new base type for atom and should use a copy of base type reference.
        let base_type = Type::BaseType(base_type);

        AtomicAtom {
            sort: base_type.into(),
            val: atom_enum
        }
    }
}

// Convert AtomicAtom into generic Atom<S> with no extra cost.
impl<S, T> From<AtomicAtom> for Atom<S, T> where S: BorrowedType, T: BorrowedTerm {
    // When convert AtomicAtom into Atom<S>, the concrete type of S is decided as Arc<Type>.
    fn from(item: AtomicAtom) -> Atom<AtomicStrType, AtomicStrTerm> {
        Atom {
            sort: item.sort,
            val: item.val,
            term: PhantomData, // It pretends to hold a value but actually empty to pass the compiler check.
        }
    }
}

/// A generic variable term that does not require a specific type of reference.
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct AtomicVariable {
    // If variable is inside a predicate, the type could be derived otherwise use Undefined.
    // A variable is meaningless without context.
    pub sort: AtomicStrType,
    // A string to represent the root of the variable term.
    pub root: String,
    // The remaining fragments of the variable term.
    pub fragments: Vec<String>,
}

impl HasUniqueForm<String> for AtomicVariable {
    fn derive_unique_form(&self) -> String {
        // The same as display name
        format!("{}", self)
    }
}

impl Display for AtomicVariable {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.fragments.len() > 0 {
            let rest = self.fragments.join(".");
            write!(f, "{}.{}", self.root, rest)
        } else {
            write!(f, "{}", self.root)
        }
    }
}

// Convert AtomicVariable into generic a Variable<S, T> with no extra cost.
impl<S, T> From<AtomicVariable> for Variable<S, T> where S: BorrowedType, T: BorrowedTerm {
    fn from(item: AtomicVariable) -> Variable<AtomicStrType, AtomicStrTerm> {
        Variable {
            sort: item.sort,
            root: item.root,
            fragments: item.fragments,
            term: PhantomData
        }
    }
}

impl AtomicVariable {
    /// Create a new variable term given sort, root and fragments.
    pub fn new(sort: AtomicStrType, root: String, fragments: Vec<String>) -> Self {
        let var = AtomicVariable {
            sort,
            root,
            fragments,
        };

        return var;
    }
}

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct AtomicComposite {
    // The type of the composite term and has an unique form as string as Arc<UniqueFormWrapper<String, Type>>.
    pub sort: AtomicStrType,
    // The atomically wrapped arguments.
    pub arguments: Vec<AtomicStrTerm>,
    // May or may not have an string alias.
    pub alias: Option<String>,
}

// A unique form in string format can be derived from AtomicComposite.
impl HasUniqueForm<String> for AtomicComposite {
    fn derive_unique_form(&self) -> String {
        // The same as display name but not exactly we skip the alias here. e.g T(a, 1, E(1))
        let mut args = vec![];
        for arg in self.arguments.iter() {
            args.push(arg.unique_form().clone());
        }
        let args_str = args.join(", ");
        let term_str = format!("{}({})", self.sort.unique_form(), args_str);
        return term_str;
    }
}

impl Display for AtomicComposite {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let alias_str = match &self.alias {
            None => "".to_string(),
            Some(name) => format!("{} is ", name)
        };

        let mut args = vec![];
        for arg in self.arguments.iter() {
            args.push(format!("{}", arg));
        }

        let args_str = args.join(", ");
        let type_ref: &Type = self.sort.borrow();
        let term_str = format!("{}({})", type_ref, args_str);
        write!(f, "{}{}", alias_str, term_str)
    }
}

// Convert AtomicComposite into generic Composite<S, T> with no extra cost.
impl<S, T> From<AtomicComposite> for Composite<S, T> where S: BorrowedType, T: BorrowedTerm
{
    fn from(item: AtomicComposite) -> Composite<AtomicStrType, AtomicStrTerm> {
        Composite {
            sort: item.sort,
            arguments: item.arguments,
            alias: item.alias
        }
    }
}

impl AtomicComposite { 
    pub fn new(sort: AtomicStrType, arguments: Vec<AtomicStrTerm>, alias: Option<String>) -> Self {
        let mut composite = AtomicComposite {
            sort,
            arguments,
            alias,
        };
        return composite;
    }

    pub fn validate(&self) -> bool {
        true
    }
}