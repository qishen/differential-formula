use std::borrow::*;
use std::collections::*;
use std::fmt::*;
use std::hash::*;
use std::hash::Hash;

use serde::{Serialize, Deserialize};
use differential_datalog::record::*;
use ddlog_derive::*;

use crate::expression::*;
use crate::module::Env;
use super::generic::*;
use crate::type_system::*;
use crate::util::wrapper::*;
use crate::parser::ast::*;
use crate::parser::combinator::parse_program;

/// AtomicTerm is safe to transfer between threads with both type and sub-terms thread safe.
/// Note that the equality of composite terms are decided by pointer equality, so the copy of
/// the term is not equal to the original term.
/// A sub-term may be shared by multiple terms to reduce redundancy.
// #[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, 
        //  Mutator, 
         Serialize, Deserialize)]
pub enum AtomicTerm {
    Atom(AtomicAtom),
    Variable(AtomicVariable),
    Composite(AtomicComposite)
}

// Convert string into a variable term with unknown sort.
impl From<String> for AtomicTerm {
    fn from(item: String) -> Self {
        AtomicTerm::gen_raw_variable_term(item, vec![])
    }
}

// Convert &str into a variable term with unknown sort.
impl From<&str> for AtomicTerm {
    fn from(item: &str) -> Self {
        AtomicTerm::gen_raw_variable_term(item.to_string(), vec![])
    }
}

impl From<usize> for AtomicTerm {
    fn from(item: usize) -> Self {
        let atom_enum = AtomEnum::Int(num::BigInt::from(item));
        AtomicTerm::gen_atom_term(atom_enum)
    }
}

impl From<TermAst> for AtomicTerm {
    fn from(item: TermAst) -> Self {
        match item {
            TermAst::CompositeTermAst(cast) => {
                let type_name = cast.sort.name().unwrap();
                let arguments: Vec<AtomicTerm> = cast.arguments.into_iter().map(|arg| {
                    // TODO: Change to nightly feature `into_inner()` to avoid copy
                    let argument: AtomicTerm = arg.as_ref().clone().into();
                    argument
                }).collect();
                let composite = AtomicComposite {
                    sort: RawType::TypeId(Cow::from(type_name)),
                    arguments,
                };
                AtomicTerm::Composite(composite)
            },
            TermAst::VariableTermAst(vast) => {
                AtomicTerm::gen_raw_variable_term(vast.root, vast.fragments)
            },
            TermAst::AtomTermAst(atom) => {
                AtomicTerm::gen_atom_term(atom)
            }
        }
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

impl Default for AtomicTerm {
    fn default() -> Self {
        AtomicTerm::Atom(AtomicAtom { 
            sort: RawType::undefined(),
            val: AtomEnum::Int(0.into()),
        })
    }
}

impl Mutator<AtomicTerm> for Record {
    fn mutate(&self, v: &mut AtomicTerm) -> std::result::Result<(), String> {
        todo!()
    }
}

impl IntoRecord for AtomicTerm {
    fn into_record(self) -> Record {
        match self {
            AtomicTerm::Atom(atom) => {
                atom.val.into_record()
            },
            AtomicTerm::Variable(var) => {
                // Record::Variable(Cow::from(format!("{}", var)))
                // TODO: Avoid the copy but fragments are skipped.
                Record::Variable(Cow::from(var.root))
            },
            AtomicTerm::Composite(composite) => {
                // Don't make extra copy for type name unless necessary.
                let type_name = match composite.sort {
                    RawType::Type(t) => {
                        match t {
                            FormulaTypeEnum::CompositeType(ctype) => Cow::from(ctype.name),
                            _ => { Cow::from(format!("{}", t)) }
                        }
                    },
                    RawType::TypeId(cow) => cow,
                };  
                let mut arguments = vec![];
                for arg in composite.arguments {
                    let r = arg.into_record();
                    arguments.push(r);
                }
                Record::PosStruct(type_name, arguments)
            }
        }
    }
}

impl FromRecord for AtomicTerm {
    fn from_record(val: &Record) -> std::result::Result<Self, String> {
        match val {
            Record::Bool(bool) => {
                let val = AtomEnum::Bool(bool.clone());
                let atom = AtomicAtom::new(val);
                Ok(AtomicTerm::Atom(atom)) 
            },
            Record::Int(integer) => {
                let val = AtomEnum::Int(integer.clone());
                let atom = AtomicAtom::new(val);
                Ok(AtomicTerm::Atom(atom))
            },
            Record::String(string) => {
                let val = AtomEnum::Str(string.clone());
                let atom = AtomicAtom::new(val);
                Ok(AtomicTerm::Atom(atom))
            },
            Record::Float(float) => {
                let val = AtomEnum::Float(float.clone());
                let atom = AtomicAtom::new(val);
                Ok(AtomicTerm::Atom(atom))
            },
            Record::PosStruct(name, arguments) => {
                let mut subterms = Vec::new();
                for arg in arguments {
                    if let Ok(term) = AtomicTerm::from_record(arg) {
                        subterms.push(term);
                    } else {
                        return Err(format!("Cannot convert argument of PosStruct Record into AtomicTerm."));
                    }
                }
                let composite = AtomicComposite {
                    sort: RawType::TypeId(name.clone()),
                    arguments: subterms,
                };
                Ok(AtomicTerm::Composite(composite))
            },
            _ => { Err(format!("Cannot convert Record into AtomicTerm."))}
        }
    }
}

impl TermStructure for AtomicTerm {

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
    
    fn is_atom(&self) -> bool {
        if let AtomicTerm::Atom(_) = self { true } else { false }
    }

    fn is_var(&self) -> bool {
        if let AtomicTerm::Variable(_) = self { true } else { false }
    }

    fn is_composite(&self) -> bool {
        if let AtomicTerm::Composite(_) = self { true } else { false }
    }

    fn is_fragmented_var(&self) -> bool {
        match self {
            AtomicTerm::Variable(var) => { var.fragments.len() > 0 },
            _ => { false }
        }
    }

    fn is_donot_care_variable(&self) -> bool {
        // Don't care vars start with underscore.
        match self {
            AtomicTerm::Variable(var) => { var.root == "_" },
            _ => { false }
        }
    }

    fn root(&self) -> Self {
        match self {
            AtomicTerm::Variable(var) => { Self::gen_raw_variable_term(var.root.clone(), vec![]) },
            _ => { self.clone() } 
        }
    }

    fn gen_raw_variable_term(root: String, fragments: Vec<String>) -> Self {
        let var = AtomicVariable::new(RawType::undefined(), root, fragments);
        AtomicTerm::Variable(var)
    }

    fn gen_atom_term(atom_enum: AtomEnum) -> Self {
        let atom = AtomicAtom::new(atom_enum);
        AtomicTerm::Atom(atom)
    }

    fn gen_constant(constant: String) -> (RawType, Self) {
        let nullary_type_enum = FormulaTypeEnum::CompositeType(
            CompositeType { 
                name: constant, 
                arguments: vec![] 
            }
        );
        let nullary_type = RawType::Type(nullary_type_enum);
        let composite = AtomicComposite::new(nullary_type.clone().into(), vec![], None);
        let term = AtomicTerm::Composite(composite);
        return (nullary_type.into(), term);
    }

    fn rename(&self, scope: String) -> Self {
        match self {
            AtomicTerm::Variable(var) => {
                AtomicTerm::Variable(var.rename(scope))
            },
            AtomicTerm::Composite(composite) => {
                AtomicTerm::Composite(composite.rename(scope))
            },
            AtomicTerm::Atom(_) => { self.clone() }
        }
    }

    fn from_term_ast(ast: &TermAst, type_map: &HashMap<String, RawType>) -> Self {
        let atomic_term = match ast {
            TermAst::CompositeTermAst(cterm_ast) => {
                let mut term_arguments = vec![];
                for argument in cterm_ast.arguments.clone() {
                    // TODO: It would be better to include a set of existing terms to avoid duplicates.
                    let atomic_term = AtomicTerm::from_term_ast(argument.as_ref(), type_map);
                    // let term = AtomicPtrTerm { ptr: Arc::new(atomic_term) };
                    term_arguments.push(atomic_term);
                }
                let sort_name = cterm_ast.sort.name().unwrap();
                let sort: RawType = type_map.get(&sort_name).unwrap().clone().into();
                let composite = AtomicComposite::new(sort, term_arguments, cterm_ast.alias.clone());
                AtomicTerm::Composite(composite)
            },
            TermAst::VariableTermAst(vterm_ast) => {
                // The sort of variable term is undefined at this point.
                let var = AtomicVariable::new(
                    RawType::undefined(),
                    vterm_ast.root.clone(), 
                    vterm_ast.fragments.clone()
                );
                AtomicTerm::Variable(var)
            },
            TermAst::AtomTermAst(atom_enum) => {
                let atom = AtomicAtom::new(atom_enum.clone());
                AtomicTerm::Atom(atom)
            }
        };

        atomic_term
    }

    /// Parse text and load the program into environment.
    fn load_program<'a>(text: String) -> Env {
        let text = text + " EOF";
        let result = parse_program(&text[..]);
        // Make sure the whole file is parsed rather than part of the program.
        assert_eq!(result.0, "EOF");
        // println!("{:?}", result.0);
        let program_ast = result.1;
        program_ast.build_env()
    }
}

impl AtomicTerm {
    /// Return the type id of a term which is either an owned string or a string reference.
    pub fn type_id<'a>(&self) -> Cow<'a, str> {
        match self {
            AtomicTerm::Composite(composite) => composite.sort.type_id(),
            AtomicTerm::Variable(var) => var.sort.type_id(),
            AtomicTerm::Atom(atom) => atom.sort.type_id()
        }
    }

    /// Compare the variables in two terms and return a tuple of intersection, left and right.
    pub fn variable_diff(&self, other: &AtomicTerm) -> (Vec<AtomicTerm>, Vec<AtomicTerm>, Vec<AtomicTerm>) {
        let vars1 = self.variables();
        let vars2 = other.variables();

        let mut shared_vars: Vec<AtomicTerm> = vars1.clone().into_iter().filter(|x| { 
            vars2.contains(x) 
        }).collect();
        shared_vars.sort();

        let mut left_vars: Vec<AtomicTerm> = vars1.clone().into_iter().filter(|x| 
            !vars2.contains(x)
        ).collect();
        left_vars.sort();

        let mut right_vars: Vec<AtomicTerm> = vars2.clone().into_iter().filter(|x| 
            !vars1.contains(x)
        ).collect();
        right_vars.sort();

        (shared_vars, left_vars, right_vars)
    }
}

#[derive(Debug, PartialOrd, Ord, Eq, Clone, Serialize, Deserialize)]
pub struct AtomicAtom {
    // The type of the atom as Arc<UniqueFormWrapper<String, Type>>
    pub sort: RawType,
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
    pub fn new(atom_enum: AtomEnum) -> AtomicAtom {
        let base_type = match &atom_enum {
            AtomEnum::Bool(_)  => { BaseType::Boolean },
            AtomEnum::Float(_) => { BaseType::Rational },
            AtomEnum::Int(_)   => { BaseType::Integer },
            AtomEnum::Str(_)   => { BaseType::String }
        };
        let base_type = FormulaTypeEnum::BaseType(base_type);
        let atom_sort = RawType::Type(base_type);
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
    pub sort: RawType,
    // A string to represent the root of the variable term.
    pub root: String,
    // The remaining fragments of the variable term.
    pub fragments: Vec<String>,
    // A reference to the root term but is optional only if the variable has fragments.
    // pub root_term: Option<AtomicPtrTerm>
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
    pub fn new(sort: RawType, root: String, fragments: Vec<String>) -> Self {
        let root_var = AtomicVariable {
            sort: sort.clone(),
            root: root.clone(),
            fragments: vec![],
            // root_term: None
        };
        
        if fragments.len() == 0 {
            return root_var;
        } else { AtomicVariable { sort, root, fragments } }
    }
    
    /// Check if the variable term has fragments or not.
    pub fn is_root(&self) -> bool {
        self.fragments.len() == 0
    }

    /// Rename the variable with a scope added to the root.
    pub fn rename(&self, scope: String) -> Self {
        let renamed_root = scope + &self.root[..];
        AtomicVariable::new(self.sort.clone(), renamed_root, self.fragments.clone())
    }
}

#[derive(Hash, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Serialize, Deserialize)]
pub struct AtomicComposite {
    pub sort: RawType,
    pub arguments: Vec<AtomicTerm>,
}

impl AtomicComposite { 
    pub fn new(sort: RawType, arguments: Vec<AtomicTerm>, alias: Option<String>) -> Self {
        let composite = AtomicComposite {
            sort,
            arguments,
        };
        return composite;
    }

    pub fn validate(&self) -> bool {
        true
    }

    // pub fn rename(&self, scope: String, renamed_types: &mut HashSet<RawType>) -> Self {
    pub fn rename(&self, scope: String) -> Self {
        let mut arguments = vec![];
        for arg in self.arguments.iter() {
            let renamed_arg = arg.rename(scope.clone());
            arguments.push(renamed_arg);
        }
        AtomicComposite {
            sort: self.sort.rename_type(scope),
            arguments,
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
        let term_str = format!("{}({})", self.sort, args_str);
        return term_str;
    }
}

impl Display for AtomicComposite {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut args = vec![];
        for arg in self.arguments.iter() {
            args.push(format!("{}", arg));
        }
        let args_str = args.join(", ");
        let type_ref: &RawType = self.sort.borrow();
        let term_str = format!("{}({})", type_ref, args_str);
        write!(f, "{}", term_str)
    }
}