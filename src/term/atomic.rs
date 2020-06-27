use std::sync::*;
use std::hash::Hash;
use std::fmt::*;
use std::marker::PhantomData;

use serde::{Serialize, Deserialize};

use crate::type_system::*;
use super::generic::*;
use super::VisitTerm;


#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum AtomicTerm {
    Atom(AtomicAtom),
    Variable(AtomicVariable),
    Composite(AtomicComposite),
}

// Need to explicitly implement the trait for the atomic term and wrapper over atomic term.
impl BorrowedTerm for AtomicTerm {}
impl BorrowedTerm for Arc<AtomicTerm> {}

// Convert AtomicTerm into Term<S, T> smoothly without any deep copies of the fields.
impl<S, T> From<AtomicTerm> for Term<S, T> where S: BorrowedType, T: BorrowedTerm 
{    
    fn from(item: AtomicTerm) -> Term<Arc<Type>, Arc<AtomicTerm>> {
        match item {
            AtomicTerm::Atom(a) => {
                let atom: Atom<Arc<Type>, Arc<AtomicTerm>> = a.into();
                Term::Atom(atom)
            },
            AtomicTerm::Variable(v) => {
                let variable: Variable<Arc<Type>, Arc<AtomicTerm>> = v.into();
                Term::Variable(variable)
            },
            AtomicTerm::Composite(c) => {
                let composite: Composite<Arc<Type>, Arc<AtomicTerm>> = c.into();
                Term::Composite(composite)
            }
        }
    }
}

/// Put some term-related static methods here.
impl AtomicTerm {
    /// Given a string create a nullary composite type with no arguments inside
    /// and return the singleton term or constant in other words.
    pub fn create_constant(constant: String) -> AtomicTerm {
        let nullary_type = Type::CompositeType(
            CompositeType { name: constant, arguments: vec![] }
        );
        let term = AtomicTerm::Composite(
            AtomicComposite::new(
                nullary_type.into(), 
                vec![], 
                None
            )
        );
        return term;
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
    // The type of the atom.
    pub sort: Arc<Type>,
    // The field holding the value.
    pub val: AtomEnum,
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

// Convert AtomicAtom into generic Atom<S> with no extra cost.
impl<S, T> From<AtomicAtom> for Atom<S, T> where S: BorrowedType, T: BorrowedTerm
{
    // When convert AtomicAtom into Atom<S>, the concrete type of S is decided as Arc<Type>.
    fn from(item: AtomicAtom) -> Atom<Arc<Type>, Arc<AtomicTerm>> {
        Atom {
            sort: item.sort,
            val: item.val,
            term: PhantomData, // It pretends to hold an Arc<AtomicTerm> but actually empty to pass compiler check.
        }
    }
}

/// A generic variable term that does not require a specific type of reference.
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct AtomicVariable {
    // If variable is inside a predicate, the type could be derived otherwise use Undefined.
    // A variable is meaningless without context.
    pub sort: Arc<Type>,
    // A string to represent the root of the variable term.
    pub root: String,
    // The remaining fragments of the variable term.
    pub fragments: Vec<String>,
    // Create a reference to access root variable term like getting `x` term given `x.y.z` 
    // as a variable term. If the variable does not have fragments than the `root_term` is None.
    pub root_term: Option<AtomicTerm>,
}

impl Display for AtomicVariable {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut rest = self.fragments.join(".");
        if self.fragments.len() > 0 {
            rest = ".".to_string() + &rest[..]; 
        }
        write!(f, "{}{}", self.root, rest)
    }
}

// Convert AtomicVariable into generic a Variable<S, T> with no extra cost.
impl<S, T> From<AtomicVariable> for Variable<S, T> where S: BorrowedType, T: BorrowedTerm
{
    fn from(item: AtomicVariable) -> Variable<Arc<Type>, Arc<AtomicTerm>> {
        let root_term: Option<Term<Arc<Type>, Arc<AtomicTerm>>> = item.root_term.map(|x| x.into());
        Variable {
            sort: item.sort,
            root: item.root,
            fragments: item.fragments,
            root_term: root_term
        }
    }
}

impl AtomicVariable {
    /// Create a new variable term given sort, root and fragments.
    pub fn new(sort: Arc<Type>, root: String, fragments: Vec<String>) -> Self {
        if fragments.len() == 0 {
            let var = AtomicVariable {
                sort,
                root,
                fragments,
                root_term: None
            };

            return var;
        } else {
            // The sort of root term is unknown without context.
            // TODO: Create the same type on every initialization is bad.
            let undefined_sort = Type::Undefined(
                Undefined{ name: "Undefined".to_string() }
            );

            let root_var = AtomicVariable {
                sort: undefined_sort.into(),
                root,
                fragments: vec![],
                root_term: None
            };

            let var = AtomicVariable {
                sort,
                root,
                fragments,
                root_term: Some(AtomicTerm::Variable(root_var))
            };

            return var;
        }
    }
}

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct AtomicComposite {
    // The type of the composite term.
    pub sort: Arc<Type>,
    // The atomically wrapped arguments.
    pub arguments: Vec<Arc<AtomicTerm>>,
    // May or may not have an string alias.
    pub alias: Option<String>,
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
        let term_str = format!("{}({})", self.sort.name(), args_str);
        write!(f, "{}{}", alias_str, term_str)
    }
}

// Convert AtomicComposite into generic Composite<S, T> with no extra cost.
impl<S, T> From<AtomicComposite> for Composite<S, T> where S: BorrowedType, T: BorrowedTerm
{
    fn from(item: AtomicComposite) -> Composite<Arc<Type>, Arc<AtomicTerm>> {
        Composite {
            sort: item.sort,
            arguments: item.arguments,
            alias: item.alias
        }
    }
}

impl AtomicComposite { 
    pub fn new(sort: Arc<Type>, arguments: Vec<Arc<AtomicTerm>>, alias: Option<String>) -> Self {
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