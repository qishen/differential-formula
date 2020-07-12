use std::borrow::*;
use std::collections::HashSet;
use std::sync::Arc;
use std::hash::Hash;
use std::fmt::*;

use serde::{Serialize, Deserialize};

use super::generic::*;
use super::VisitTerm;
use crate::type_system::*;
use crate::util::wrapper::*;

/******************** AtomicType *******************/

/// Wrap an atomic type with unique string form wrapper.
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct AtomicType {
    inner: Arc<UniqueFormWrapper<String, Type>>
}

impl BorrowedType for AtomicType {}

impl Debug for AtomicType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.inner)
    }
}

impl Display for AtomicType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.inner)
    }
}

/// AtomicType can be borrowed as string which is its unique string form, 
impl Borrow<String> for AtomicType {
    fn borrow(&self) -> &String {
        self.inner.as_ref().unique_form()
    }
}

/// AtomicType can peel off the wrappers and be borrowed as native Type.
impl Borrow<Type> for AtomicType {
    fn borrow(&self) -> &Type {
        self.inner.as_ref().item()
    }
}

impl From<Type> for AtomicType {
    fn from(item: Type) -> Self {
        let wrapper: UniqueFormWrapper<String, Type> = item.into();
        AtomicType { inner: Arc::new(wrapper) }
    }
}

/// Type is able to derive an unique form in string.
impl HasUniqueForm<String> for AtomicType {
    fn derive_unique_form(&self) -> String {
        self.inner.derive_unique_form()
    }
}

// Type is able to provide a reference to its unique string form and update unique form when fields
// in Type are mutated.
impl UniqueForm<String> for AtomicType {
    fn unique_form(&self) -> &String {
        self.inner.unique_form()
    }

    fn update_unique_form(&mut self) {
        self.inner.update_unique_form();
    }
}

/******************** AtomicTerm *******************/

/// AtomicTerm is safe to transfer between threads with both type and sub-terms thread safe.
/// AtomicTerm can be converted into Term<S, T> without copies of 
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum AtomicTerm {
    Atom(AtomicAtom),
    Variable(AtomicVariable),
    Composite(AtomicComposite)
}

impl TermStructure for AtomicTerm {

    type SortOutput = AtomicType;

    fn sort(&self) -> &Self::SortOutput {
        match self {
            AtomicTerm::Atom(atom) => &atom.sort,
            AtomicTerm::Composite(composite) => &composite.sort,
            AtomicTerm::Variable(variable) => &variable.sort
        }
    }

    fn arguments(&self) -> Vec<&Self> {
        let args = vec![];
        match self {
            AtomicTerm::Composite(composite) => {
                for arg in composite.arguments.iter() {
                    args.push(arg);
                }
            },
            _ => {}
        }
        args
    }

    fn arguments_mut(&mut self) -> Vec<&mut Self> {
        let args = vec![];
        match self {
            AtomicTerm::Composite(composite) => {
                for arg in composite.arguments.iter_mut() {
                    args.push(arg);
                }
            },
            _ => {}
        }
        args
    }
    
    fn term_type(&self) -> TermType {
        match self {
            AtomicTerm::Atom(_) => TermType::Atom,
            AtomicTerm::Composite(_) => TermType::Composite,
            AtomicTerm::Variable(_) => TermType::Variable
        }
    }

    fn root(&self) -> &Self {
        match self {
            AtomicTerm::Variable(var) => {
                let root = var.root_term.map_or(self, |root| &root);
                root
            },
            _ => { self } // Return itself for atom and composite term.
        }
    }

    fn create_variable_term(root: String, fragments: Vec<String>) -> Self {
        // Use undefined for the sort because it is unknown without context.
        let undefined_sort: AtomicType = Type::undefined().into();
        let var = AtomicVariable::new(undefined_sort, root, fragments);
        AtomicTerm::Variable(var)
    }

    fn create_atom_term(atom_enum: AtomEnum) -> Self {
        let atom = AtomicAtom::new(atom_enum);
        AtomicTerm::Atom(atom)
    }

    fn is_dc_variable(&self) -> bool {
        match self {
            // Recognize as Do-not-care variable when it is an underscore.
            AtomicTerm::Variable(var) => {
                if var.root == "_" { true }
                else { false }
            },
            _ => { false }
        }
    }

    fn rename(&self, scope: String, types: &mut HashSet<Self::SortOutput>) -> Self {
        match self {
            AtomicTerm::Variable(var) => {
                AtomicTerm::Variable(var.rename(scope))
            },
            AtomicTerm::Composite(composite) => {
                AtomicTerm::Composite(composite.rename(scope, types))
            },
            AtomicTerm::Atom(_) => { self.clone() }
        }
    }

    fn find_subterm(&self, labels: &Vec<String>) -> Option<&Self> {
        let result = match self {
            AtomicTerm::Composite(cterm) => {
                let initial_term = self;
                let mut init_type: &Type = cterm.sort.borrow();
                let init_type = init_type.base_type();
                // The final value is a tuple of type and term.
                let result = labels.iter().fold(Some((init_type, initial_term)), 
                    |state, label| {
                    if let Some((ctype_enum, subterm)) = state {
                        if let Type::CompositeType(ctype) = ctype_enum {
                            let new_state = ctype.arguments.iter().enumerate().find_map(
                                |(i, (arg_label_opt, t))| {
                                if let Some(arg_label) = arg_label_opt {
                                    if arg_label == label {
                                        if let AtomicTerm::Composite(cterm) = subterm.borrow() {
                                            // Update the composite type for the next round. Note that `t` could 
                                            // be a renamed type wrapping a composite type.
                                            let type_ref: &Type = t.borrow();
                                            let new_ctype = type_ref.base_type();
                                            let cterm_arg = cterm.arguments.get(i).unwrap();
                                            // Need to convert Arc<Term> into generic type T.
                                            return Some((new_ctype, cterm_arg));
                                        }
                                    }
                                } 
                                return None;
                            });
                            return new_state;
                        } 
                    } 
                    return None;
                });

                result.map(|(_, term)| {
                    term
                })
            },
            _ => { None }
        };

        None
    }
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

/// Put some term-related static methods here.
impl AtomicTerm {
    /// Given a string create a nullary composite type with no arguments inside and return the singleton term.
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
    pub fn has_deep_intersection<I, T>(a: I, b: I) -> bool 
    where 
        I: Iterator<Item=T>, 
        T: TermStructure,
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

    /// Given a label as string and check if one of the arguments in composite term is related to the label
    /// according to the type definition. e.g. Edge ::= new (src: Node, dst: Node) and we have an instance
    /// e1 = Edge(_,_). The subterm represented by `e1.src` can be derived. 
    pub fn find_argument_by_label(&self, label: &str) -> Option<&AtomicTerm> {
        let result = match self {
            AtomicTerm::Composite(composite) => {
                let native_type: &Type = composite.sort.borrow();
                let base_type = native_type.base_type();
                match base_type {
                    Type::CompositeType(ctype) => {
                        let result = ctype.arguments.iter().enumerate().find_map(|(i, (arg_label_opt, t))| {
                            match arg_label_opt {
                                Some(arg_label) => {
                                    if arg_label == label {
                                        let arg = composite.arguments.get(i).unwrap();
                                        return Some(arg);
                                    } else { return None; }
                                },
                                None => None
                            }
                        });

                        result
                    },
                    _ => { None }
                }
            },
            _ => { None }
        };

        result
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
    pub sort: AtomicType,
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

/// A generic variable term that does not require a specific type of reference.
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct AtomicVariable {
    // If variable is inside a predicate, the type could be derived otherwise use Undefined.
    // A variable is meaningless without context.
    pub sort: AtomicType,
    // A string to represent the root of the variable term.
    pub root: String,
    // The remaining fragments of the variable term.
    pub fragments: Vec<String>,
    // A reference to the root term but is optional only if the variable has fragments.
    pub root_term: Option<AtomicTerm>
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

impl AtomicVariable {
    /// Create a new variable term given sort, root and fragments. The sort is always the type of root variable
    /// and has no relations with fragments and the type of fragments can be derived from root type later.
    pub fn new(sort: AtomicType, root: String, fragments: Vec<String>) -> Self {
        let root_var = AtomicVariable {
            sort,
            root: root.clone(),
            fragments: vec![],
            root_term: None
        };
        
        if fragments.len() == 0 {
            return root_var;
        } else {
            let var = AtomicVariable {
                sort,
                root,
                fragments,
                root_term: Some(AtomicTerm::Variable(root_var))
            };
            return var;
        }
    }
    
    /// Check if the variable term has fragments or not.
    pub fn is_root(&self) -> bool {
        self.fragments.len() == 0 && self.root_term == None
    }

    /// Rename the variable with a scope added to the root.
    pub fn rename(&self, scope: String) -> Self {
        let renamed_root = scope + &self.root[..];
        AtomicVariable::new(self.sort.clone(), renamed_root, self.fragments.clone())
    }
}

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct AtomicComposite {
    // The type of the composite term and has an unique form as string as Arc<UniqueFormWrapper<String, Type>>.
    pub sort: AtomicType,
    // The atomically wrapped arguments.
    pub arguments: Vec<AtomicTerm>,
    // May or may not have an string alias.
    pub alias: Option<String>,
}

impl AtomicComposite { 
    pub fn new(sort: AtomicType, arguments: Vec<AtomicTerm>, alias: Option<String>) -> Self {
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

    pub fn rename(&self, scope: String, renamed_types: &mut HashSet<AtomicType>) -> Self {
        let arguments = vec![];
        for arg in self.arguments.iter() {
            let renamed_arg = arg.rename(scope.clone(), renamed_types);
            arguments.push(renamed_arg);
        }

        // Look up in the type set for the first renamed type whose base type is the same as the sort in current
        // term so this type in the type can be reused to create renamed term otherwise need to create a new type
        // and then be inserted into the mutable type set.
        let result = renamed_types.iter().find(|&atomic_type| {
            let renamed_type: &Type = atomic_type.borrow();
            renamed_type.unrename_type() == self.sort.borrow() 
        });

        let renamed_type = match result {
            Some(atomic_type) => {
                atomic_type.clone()
            },
            None => {
                let native_type: &Type = self.sort.borrow();
                let renamed_type = native_type.clone().rename_type(scope.clone());
                let atomic_renamed_type: AtomicType = renamed_type.into();
                renamed_types.insert(atomic_renamed_type.clone());
                atomic_renamed_type
            }
        };

        AtomicComposite {
            sort: renamed_type,
            arguments,
            alias: self.alias.clone() // TODO: Should I rename the alias too?
        }
    }
}

// A unique form in string format can be derived from AtomicComposite.
impl HasUniqueForm<String> for AtomicComposite {
    fn derive_unique_form(&self) -> String {
        // The same as display name but not exactly we skip the alias here. e.g T(a, 1, E(1))
        let mut args = vec![];
        for arg in self.arguments.iter() {
            args.push(arg.derive_unique_form());
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
