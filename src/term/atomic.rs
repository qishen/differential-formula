use std::borrow::*;
use std::collections::*;
use std::hash::*;
use std::sync::Arc;
use std::hash::Hash;
use std::fmt::*;

use serde::{Serialize, Deserialize};

use crate::module::Env;
use super::generic::*;
use crate::type_system::*;
use crate::util::wrapper::*;
use crate::parser::ast::*;
use crate::parser::combinator::parse_program;

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
        let mut args = vec![];
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
        let mut args = vec![];
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
                let root = var.root_term.as_ref().map_or(self, |root| &root);
                root
            },
            _ => { self } // Return itself for atom and composite term.
        }
    }

    fn create_variable_term(sort: Option<Self::SortOutput>, root: String, fragments: Vec<String>) -> Self {
        // Use undefined for the sort because it is unknown without context if no sort in provided in params.
        let var_sort = match sort {
            Some(sort) => sort,
            None => Type::undefined().into()
        };
        let var = AtomicVariable::new(var_sort, root, fragments);
        AtomicTerm::Variable(var)
    }

    fn create_atom_term(sort: Option<Self::SortOutput>, atom_enum: AtomEnum) -> Self {
        let atom = AtomicAtom::new(sort, atom_enum);
        AtomicTerm::Atom(atom)
    }

    fn create_constant(constant: String) -> (Self::SortOutput, Self) {
        let nullary_type = Type::CompositeType(CompositeType { name: constant, arguments: vec![] });
        let composite = AtomicComposite::new(nullary_type.clone().into(), vec![], None);
        let term = AtomicTerm::Composite(composite);
        return (nullary_type.into(), term);
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

    fn find_subterm_by_labels(&self, labels: &Vec<&String>) -> Option<&Self> {
        let mut current_term = self; 
        for label in labels {
            let subterm_opt = current_term.find_argument_by_label(label);
            if let Some(subterm) = subterm_opt {
                current_term = subterm;
            } else {
                return None;
            }
        }
        Some(current_term)
    }

    fn is_direct_subterm_of(&self, term: &Self) -> bool {
        match term {
            AtomicTerm::Composite(composite) => {
                for arg in composite.arguments.iter() {
                    // Check if self is equal to one of its arguments.
                    if self == arg { return true; }
                }
                false
            },
            AtomicTerm::Variable(var) => {
                match self {
                    AtomicTerm::Variable(sub_var) => { // e.g. `x.y.z` is a subterm of `x.y`.
                        if var.root == sub_var.root && sub_var.fragments.starts_with(&var.fragments){ true }
                        else { false }
                    },
                    _ => { false }
                }
            },
            AtomicTerm::Atom(_) => { false }
        }
    }


    fn fragments_diff<'a>(&'a self, term: &'a Self) -> Option<Vec<&'a String>> {
        match self {
            AtomicTerm::Variable(v1) => {
                let len1 = v1.fragments.len();
                match term {
                    AtomicTerm::Variable(v2) => {
                        let len2 = v2.fragments.len();
                        if v1.fragments.starts_with(&v2.fragments) {
                            let mut labels = vec![];
                            for i in len2 .. len1 {
                                labels.push(v1.fragments.get(i).unwrap());
                            } 
                            Some(labels)
                        }
                        else if v2.fragments.starts_with(&v1.fragments) {
                            let mut labels = vec![];
                            for i in len1 .. len2 {
                                labels.push(v2.fragments.get(i).unwrap());
                            }
                            Some(labels)
                        }
                        else { None }
                    },
                    _ => { None }
                }  
            },
            _ => { None }
        }
    }

    fn into_atom_enum(&self) -> Option<AtomEnum> {
        match self {
            AtomicTerm::Atom(atom) => {
                Some(atom.val.clone())
            },
            _ => None,
        }
    }

    fn from_term_ast(ast: &TermAst, type_map: &HashMap<String, Self::SortOutput>) -> Self {
        // Get undefined sort from the type map or generate a new one and insert into type map.
        let undefined_sort: AtomicType = match type_map.contains_key("~Undefined") {
            true => type_map.get("~Undefined").unwrap().clone().into(),
            false => { Type::undefined().into() }
        };

        let atomic_term = match ast {
            TermAst::CompositeTermAst(cterm_ast) => {
                let mut term_arguments = vec![];
                for argument in cterm_ast.arguments.clone() {
                    let term = AtomicTerm::from_term_ast(argument.as_ref(), type_map);
                    term_arguments.push(term);
                }
                let sort_name = cterm_ast.sort.name().unwrap();
                let sort: AtomicType = type_map.get(&sort_name).unwrap().clone().into();
                let composite = AtomicComposite::new(sort, term_arguments, cterm_ast.alias.clone());
                AtomicTerm::Composite(composite)
            },
            TermAst::VariableTermAst(vterm_ast) => {
                // The sort of variable term is undefined at this point.
                let var = AtomicVariable::new(
                    undefined_sort.clone(),
                    vterm_ast.root.clone(), 
                    vterm_ast.fragments.clone()
                );
                AtomicTerm::Variable(var)
            },
            TermAst::AtomTermAst(atom_enum) => {
                let atom = AtomicAtom {
                    sort: undefined_sort.clone(),
                    val: atom_enum.clone()
                };
                AtomicTerm::Atom(atom)
            }
        };

        atomic_term
    }

    fn remove_alias(&mut self) -> Option<String> {
        match self {
            AtomicTerm::Composite(composite) => {
                let alias = composite.alias.clone();
                composite.alias = None;
                alias
            },
            _ => { None }
        }
    }

    fn load_program(text: String) -> Env<Self> {
        let text = text + " EOF";
        let result = parse_program(&text[..]);
        // Make sure the whole file is parsed rather than part of the program.
        assert_eq!(result.0, "EOF");
        // println!("{:?}", result.0);
        let program_ast = result.1;
        program_ast.build_env()
    }
}

// Convert string into a variable term with unknown sort.
impl From<String> for AtomicTerm {
    fn from(item: String) -> Self {
        AtomicTerm::create_variable_term(None, item, vec![])
    }
}

impl From<&str> for AtomicTerm {
    fn from(item: &str) -> Self {
        AtomicTerm::create_variable_term(None, item.to_string(), vec![])
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

#[derive(Debug, PartialOrd, Ord, Eq, Clone, Serialize, Deserialize)]
pub struct AtomicAtom {
    // The type of the atom as Arc<UniqueFormWrapper<String, Type>>
    pub sort: AtomicType,
    // The field holding the value as AtomEnum defined in generic term.
    pub val: AtomEnum,
}

impl Hash for AtomicAtom {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.val.hash(state);
    }
}

impl PartialEq for AtomicAtom {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
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
    pub fn new(sort: Option<AtomicType>, atom_enum: AtomEnum) -> AtomicAtom {
        // create a sort if not provided in the params.
        let atom_sort = match sort {
            Some(sort) => sort,
            None => {
                // Decide the sort based on the enum value.
                let base_type = match &atom_enum {
                    AtomEnum::Bool(_)  => { BaseType::Boolean },
                    AtomEnum::Float(_) => { BaseType::Rational },
                    AtomEnum::Int(_)   => { BaseType::Integer },
                    AtomEnum::Str(_)   => { BaseType::String }
                };

                let base_type = Type::BaseType(base_type);
                base_type.into()
            }
        };

        AtomicAtom {
            sort: atom_sort,
            val: atom_enum
        }
    }
}

/// A generic variable term that does not require a specific type of reference.
#[derive(Debug, PartialOrd, Ord, Eq, Clone, Serialize, Deserialize)]
pub struct AtomicVariable {
    // If variable is inside a predicate, the type could be derived otherwise use Undefined.
    // A variable is meaningless without context.
    pub sort: AtomicType,
    // A string to represent the root of the variable term.
    pub root: String,
    // The remaining fragments of the variable term.
    pub fragments: Vec<String>,
    // A reference to the root term but is optional only if the variable has fragments.
    pub root_term: Option<Box<AtomicTerm>>
}

impl Hash for AtomicVariable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // TODO: May need to consider the sort with type inference to decide if two variables
        // with different names mean the same thing, but now we only check root and fragments
        // of a variable term.
        self.root.hash(state);
        self.fragments.hash(state);
    }
}

impl PartialEq for AtomicVariable {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root && self.fragments == other.fragments
    }
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
            sort: sort.clone(),
            root: root.clone(),
            fragments: vec![],
            root_term: None
        };
        
        if fragments.len() == 0 {
            return root_var;
        } else {
            let root_term = Some(Box::new(AtomicTerm::Variable(root_var)));
            let var = AtomicVariable {
                sort,
                root,
                fragments,
                root_term
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

#[derive(Debug, PartialOrd, Ord, Eq, Clone, Serialize, Deserialize)]
pub struct AtomicComposite {
    // The type of the composite term and has an unique form as string as Arc<UniqueFormWrapper<String, Type>>.
    pub sort: AtomicType,
    // The atomically wrapped arguments.
    pub arguments: Vec<AtomicTerm>,
    // May or may not have an string alias.
    pub alias: Option<String>,
}

impl Hash for AtomicComposite {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sort.hash(state);
        self.arguments.hash(state);
    }
}

impl PartialEq for AtomicComposite {
    fn eq(&self, other: &Self) -> bool {
        self.sort == other.sort && self.arguments == other.arguments
    }
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
        let mut arguments = vec![];
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
