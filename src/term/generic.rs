use std::cell::*;
use std::vec::Vec;
use std::collections::*;
use std::convert::*;
use std::fmt::{Debug, Display};
use std::string::String;
use std::hash::Hash;

use im::OrdSet;
use num::*;
use serde::{Serialize, Deserialize};

use crate::module::Env;
use crate::term::VisitTerm;
use crate::type_system::*;
use crate::expression::*;
use crate::util::*;
use crate::util::map::*;
use crate::parser::ast::TermAst;


// Enum of three types of terms: atom, composite and variable.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TermType {
    Atom, 
    Composite, 
    Variable
}

/// A trait that requires a term implementation to have a tree-like data structure and 
/// provides access to sort and its arguments. This trait must be implemented for any 
/// term implementation like `AtomicTerm` or `IndexedTerm`.
pub trait TermStructure: Sized + Clone + Hash + Ord + Display + Debug + differential_dataflow::ExchangeData {

    type SortOutput: BorrowedType;

    /// Return a reference of the type of the term.
    fn sort(&self) -> &Self::SortOutput;

    /// Return a reference of a vector of terms as arguments and the arguments have the same type as Self.
    fn arguments(&self) -> Vec<&Self>;

    /// Ruturn a mutable reference of a vector of terms with type of Self.
    fn arguments_mut(&mut self) -> Vec<&mut Self>;
    
    /// Get the type of Formula term as one of the three different kinds.
    fn term_type(&self) -> TermType;

    /// Use this method to return the root of variable term or just return itself for composite or atom.
    fn root(&self) -> &Self;

    /// Create a variable term given root and fragments.
    fn create_variable_term(sort: Option<Self::SortOutput>, root: String, fragments: Vec<String>) -> Self;

    /// Create an atom term given AtomEnum.
    fn create_atom_term(sort: Option<Self::SortOutput>, atom_enum: AtomEnum) -> Self;

    /// Create a nullary composite type with no arguments inside and return the singleton term or constant.
    fn create_constant(constant: String) -> (Self::SortOutput, Self);

    /// Check if the variable is a do-not-care variable term.
    fn is_dc_variable(&self) -> bool;

    /// Rename the term with scope and only applied to composite and variable terms. A new type with scope
    /// added will be created and then assigned to each composite term as the sort. A mutable hashset of types
    /// is added as param to avoid creating same types repeatedly.
    /// e.g. variable `a.b.c` into `(scope.a).b.c`. 
    /// e.g. composite `Edge(a, Node(b))` into `scope.Edge(scope.a, scope.Node(scope.b))`
    fn rename(&self, scope: String, types: &mut HashSet<Self::SortOutput>) -> Self;

    /// Find subterm in the current term given the labels or variable fragments.
    fn find_subterm_by_labels(&self, labels: &Vec<&String>) -> Option<&Self>;

    /// Check if a term is the subterm of another term. e.g. Variable term `x.y.z` is a subterm of `x.y` or `x`.
    /// Node(1) is the subterm of Edge(Node(1), n2).
    fn is_direct_subterm_of(&self, term: &Self) -> bool;

    /// Only apply to two variable terms and get the difference of their fragments as a vector of string.
    // fn fragments_diff(&self, term: &Self) -> Option<Vec<&String>>;
    fn fragments_diff<'a>(&'a self, term: &'a Self) -> Option<Vec<&'a String>>;

    /// Try to convert term into AtomEnum.
    fn into_atom_enum(&self) -> Option<AtomEnum>;

    /// Convert `TermAst` into a term.
    fn from_term_ast(ast: &TermAst, type_map: &HashMap<String, Self::SortOutput>) -> Self;

    /// Remove alias from composite term and return the alias
    fn remove_alias(&mut self) -> Option<String>;

    /// Parse text and load the program into environment.
    fn load_program(text: String) -> Env<Self>;
}

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum AtomEnum {
    Int(BigInt),
    Str(String),
    Bool(bool),
    Float(BigRational),
}

// impl<S, T> Display for Atom<S, T> where S: BorrowedType, T: TermStructure {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let atom_str = match &self.val {
//             AtomEnum::Int(i) => format!("{}", i),
//             AtomEnum::Bool(b) => format!("{:?}", b),
//             AtomEnum::Str(s) => format!("\"{:?}\"", s),
//             AtomEnum::Float(f) => format!("{}", f),
//         };
//         write!(f, "{}", atom_str)
//     }
// }


// // Put some term-related static methods here.
// impl<S, T> Term<S, T> where S: BorrowedType, T: TermStructure {
//     /// Compare two lists of variable terms and return true if some terms in one list
//     /// are subterms of the terms in another list. 
//     pub fn has_deep_intersection<I>(a: I, b: I) -> bool where I: Iterator<Item=T> {
//         for v1 in a {
//             for v2 in b {
//                 if v1.has_subterm(&v2).unwrap() || 
//                    v2.has_subterm(&v1).unwrap() {
//                     return true;
//                 }
//             }
//         }
//         return false;
//     }
// }



/// Any generic term is a Formula expression and we want to return all results as the same generic
/// term itself rather than its generic parameter T.
impl<T> Expression for T where T: TermStructure {   

    type TermOutput = T;

    fn variables(&self) -> HashSet<Self::TermOutput> {
        // Allow multiple mutable reference for closure.
        let vars = RefCell::new(HashSet::new());
        self.traverse(
            &|term| {
                match term.term_type() {
                    TermType::Variable => true,
                    _ => false
                }
            },
            &|term| {
                if !term.is_dc_variable() {
                    vars.borrow_mut().insert(term.clone());
                }
            }
        );
        vars.into_inner()
    }

    fn replace_pattern(&mut self, pattern: &Self::TermOutput, replacement: &Self::TermOutput) {
        self.traverse_mut(
            &|term| { return term == pattern; }, 
            &mut |mut term| { 
                *term = replacement.clone();
            }
        );
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Self::TermOutput, SetComprehension<Self::TermOutput>> {
        HashMap::new() // No set comprehension in terms.
    }
}

impl<T> VisitTerm for T where T: TermStructure {
    fn traverse<F1, F2>(&self, pattern: &F1, logic: &F2)
    where 
        F1: Fn(&Self) -> bool, 
        F2: Fn(&Self)
    {
        if pattern(self) {
            logic(self);
        }

        for arg in self.arguments() {
            arg.traverse(pattern, logic);
        }
    }

    fn traverse_mut<F1, F2>(&mut self, pattern: &F1, logic: &mut F2) 
    where 
        F1: Fn(&Self) -> bool, 
        F2: FnMut(&mut Self)
    {
        if pattern(self) {
            logic(self);
        }

        for arg in self.arguments_mut() {
            arg.traverse_mut(pattern, logic);
        }
    }

    fn normalize(&self) -> (Self, HashMap<Self, Self>) {
        let mut generator = NameGenerator::new("~p");
        // Map normalized variables to original variables.
        let mut vmap: HashMap<Self, Self> = HashMap::new();
        let mut normalized_term = self.clone();

        normalized_term.traverse_mut(
            &|term| { term.term_type() == TermType::Variable },
            &mut |var| {
                // Create an immutable copy of var from mutable reference.
                let var_clone = var.clone();
                if !var.is_dc_variable() {
                    let p = match vmap.contains_key(var) {
                        true => {
                            vmap.get(var).unwrap().clone()
                        },
                        false => {
                            let dc_name = generator.generate_name();
                            // Create a new type for each new variable, it's ok here.
                            let dc_var = Self::create_variable_term(None, dc_name, vec![]);
                            dc_var
                        }
                    };
                    vmap.insert(var_clone, p.clone());
                    *var = p;
                }
            }
        );

        return (normalized_term, vmap);
    }

    fn get_bindings_in_place<M>(&self, binding: &mut M, term: &Self) -> bool where M: GenericMap<Self, Self> {
        match self.term_type() {
            TermType::Variable => {
                // Detect a conflict in variable binding and return false.
                if binding.contains_gkey(self) && binding.gget(self).unwrap() != term { return false; } 
                if !self.is_dc_variable() { // Skip do-not-care variables.
                    binding.ginsert(self.clone(), term.clone());
                }
                return true;
            },
            TermType::Composite => {
                if self.sort() != term.sort() { return false; }
                for i in 0 .. self.arguments().len() {
                    let xargs = self.arguments();
                    let yargs = term.arguments();
                    let x = xargs.get(i).unwrap();
                    let y = yargs.get(i).unwrap();
                    let has_binding = x.get_bindings_in_place(binding, y);
                    if !has_binding { return false; }
                }
                return true;
            },
            TermType::Atom => {
                if self != term { return false; }
                else { return true; }
            }
        }
    }

    fn get_bindings(&self, term: &Self) -> Option<HashMap<Self, Self>> {
        let mut bindings = HashMap::new();
        let has_binding = self.get_bindings_in_place(&mut bindings, term);
        if has_binding { Some(bindings) } else { None }
    }

    fn get_ordered_bindings(&self, term: &Self) -> Option<BTreeMap<Self, Self>> {
        let mut bindings= BTreeMap::new();
        let has_binding = self.get_bindings_in_place(&mut bindings, term);
        if has_binding { Some(bindings) } else { None }
    }

    fn get_cached_bindings(&self, term: &Self) -> Option<QuickHashOrdMap<Self, Self>> {
        let mut bindings= BTreeMap::new();
        let has_binding = self.get_bindings_in_place(&mut bindings, term);
        if has_binding { Some(bindings.into()) } else { None }
    }

    fn propagate_bindings<M>(&self, map: &M) -> Self where M: GenericMap<Self, Self> {
        // Make a clone and mutate the term when patterns are matched.
        let mut self_copy = self.clone();

        self_copy.traverse_mut(
            &|term| {
                if map.contains_gkey(term) || map.contains_gkey(term.root()) { return true; } 
                else { return false; }
            },
            &mut |mut term| {
                // Make an immutable clone here.
                if map.contains_gkey(term) {
                    let replacement = map.gget(term).unwrap();
                    *term = replacement.clone();
                } else {
                    // The term here must be a variable term and have fragments like inside A(x.id, y.name).
                    // Dig into the root term to find the subterm by labels. 
                    let root = term.root();
                    let root_val = map.gget(root).unwrap();
                    let diff = term.fragments_diff(root).unwrap();
                    let val = root_val.find_subterm_by_labels(&diff).unwrap();
                    *term = val.clone();
                }
            }
        );

        return self_copy;
    }

    fn update_binding<M>(&self, binding: &mut M) -> bool where M: GenericMap<Self, Self> {
        match self.term_type() {
            TermType::Variable => {
                for key in binding.gkeys() {
                    if self.is_direct_subterm_of(key) {
                        let value = binding.gget(key).unwrap();
                        let fragments_diff = self.fragments_diff(key).unwrap();
                        let sub_value = value.find_subterm_by_labels(&fragments_diff).unwrap();
                        binding.ginsert(self.clone(), sub_value.clone());
                        return true
                    }
                }
                false
            },
            _ => { false }
        }
    }

    fn is_groundterm(&self) -> bool {
        match self.term_type() {
            TermType::Composite => {
                for arg in self.arguments() {
                    if !arg.is_groundterm() { return false; }
                }
                true
            },
            TermType::Variable => { false },
            TermType::Atom => { true }
        }
    }

    fn intersect(&self, other: &Self) -> (HashSet<Self>, HashSet<Self>, HashSet<Self>) {
        let vars: HashSet<Self> = self.variables();
        let other_vars: HashSet<Self> = other.variables();

        let (l, m, r) = ldiff_intersection_rdiff(&OrdSet::from(vars), &OrdSet::from(other_vars));
        let left: HashSet<Self> = l.into_iter().map(|x| x.into()).collect();
        let middle: HashSet<Self> = m.into_iter().map(|x| x.into()).collect();
        let right: HashSet<Self> = r.into_iter().map(|x| x.into()).collect();

        (left, middle, right)
    }

    fn has_deep_intersection<'a, I>(a: I, b: I) -> bool where I: Iterator<Item=&'a Self> {
        let mut bv = vec![];
        for v in b {
            bv.push(v);
        }

        for v1 in a {
            for &v2 in bv.iter() {
                if v1 == v2 || v1.is_direct_subterm_of(v2) || v2.is_direct_subterm_of(v1) {
                    return true;
                }
            }
        }
        return false;
    }

    fn find_subterm(&self, var: &Self) -> Option<&Self> {
        // Find fragments diff between var and its root.
        let diff = var.root().fragments_diff(var);
        match diff {
            Some(diff) => {
                self.find_subterm_by_labels(&diff)
            },
            None => None
        }
    }

    // fn has_conflict<M>(outer: &M, inner: &M) -> bool where M: GenericMap<Self, Self> {
    //     // Filter out conflict binding tuple of outer and inner scope.
    //     for inner_key in inner.gkeys() {
    //         let key_root = inner_key.root();
    //         let inner_val = inner.gget(inner_key).unwrap();

    //         if outer.contains_gkey(inner_key) {
    //             let outer_val = outer.gget(inner_key).unwrap();
    //             if inner_val != outer_val {
    //                 return true;
    //             }
    //         }
    //         // outer variable: x (won't be x.y...), inner variable: x.y.z...
    //         else if outer.contains_gkey(key_root) {
    //             let outer_val = outer.gget(key_root).unwrap();
    //             let outer_sub_val = outer_val.find_subterm(inner_key).unwrap();
    //             if inner_val != &outer_sub_val {
    //                 return true;
    //             }
    //         }
    //     }

    //     false
    // }
}