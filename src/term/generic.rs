use std::borrow::*;
use std::cell::*;
use std::vec::Vec;
use std::collections::*;
use std::convert::*;
use std::fmt::{Debug, Display};
use std::string::String;
use std::hash::Hash;

use num::*;
use ordered_float::*;
use serde::{Serialize, Deserialize};

use crate::module::Env;
use crate::term::*;
use crate::type_system::*;
use crate::expression::*;
use crate::util::*;
use crate::parser::ast::TermAst;

use differential_datalog::record::*;

/// A trait that requires a term implementation to have a tree-like data structure and 
/// provides access to sort and its arguments. This trait must be implemented for any 
/// term implementation like `AtomicTerm` or `IndexedTerm`.
pub trait TermStructure: Sized + Clone + Hash + Ord + Display + Debug {
    /// Return a reference of a vector of terms as arguments and the arguments have the same type as Self.
    fn arguments(&self) -> Vec<&Self>;

    /// Ruturn a mutable reference of a vector of terms with type of Self.
    fn arguments_mut(&mut self) -> Vec<&mut Self>;
   
    // Check if term is atom, variable or composite.
    fn is_atom(&self) -> bool;
    fn is_var(&self) -> bool;
    fn is_composite(&self) -> bool;

    /// Check if the variable term has fragments.
    fn is_fragmented_var(&self) -> bool;
    /// Check if the variable is a do-not-care variable term.
    fn is_donot_care_variable(&self) -> bool;

    // Use this method to return the root of variable term or just return itself for composite or atom.
    fn root(&self) -> Self;

    /// Create a variable term given root and fragments.
    // fn create_variable_term(sort: Option<RawType>, root: String, fragments: Vec<String>) -> Self;
    fn gen_raw_variable_term(root: String, fragments: Vec<String>) -> Self;

    /// Create an atom term given AtomEnum.
    fn gen_atom_term(atom_enum: AtomEnum) -> Self;

    /// Create a nullary composite type with no arguments inside and return the singleton term or constant.
    fn gen_constant(constant: String) -> (RawType, Self);

    /// Rename the term with scope and only applied to composite and variable terms. A new type with scope
    /// added will be created and then assigned to each composite term as the sort. A mutable hashset of types
    /// is added as param to avoid creating same types repeatedly.
    /// e.g. variable `a.b.c` into `(scope.a).b.c`. 
    /// e.g. composite `Edge(a, Node(b))` into `scope.Edge(scope.a, scope.Node(scope.b))`
    // fn rename(&self, scope: String, types: &mut HashSet<RawType>) -> Self;
    fn rename(&self, scope: String) -> Self;

    // Given a label as string and check if one of the arguments in composite term is related to the label
    // according to the type definition. e.g. Edge ::= new (src: Node, dst: Node) and we have an instance
    // e1 = Edge(_,_). The subterm represented by `e1.src` can be derived. 
    // fn find_argument_by_label(&self, label: &str) -> Option<&Self>;

    // Find subterm in the current term given the labels or variable fragments.
    // fn find_subterm_by_labels(&self, labels: &Vec<&String>) -> Option<&Self>;

    // Check if a term is the subterm of another term. e.g. Variable term `x.y.z` is a subterm of `x.y` or `x`.
    // Node(1) is the subterm of Edge(Node(1), n2).
    // fn is_direct_subterm_of(&self, term: &Self) -> bool;

    // Only apply to two variable terms and get the difference of their fragments as a vector of string.
    // fn fragments_diff(&self, term: &Self) -> Option<Vec<&String>>;
    // fn fragments_diff<'a>(&'a self, term: &'a Self) -> Option<Vec<&'a String>>;

    /// Convert `TermAst` into a term.
    fn from_term_ast(ast: &TermAst, type_map: &HashMap<String, RawType>) -> Self;

    /// Parse text and load the program into environment.
    fn load_program<'a>(text: String) -> Env<Self>;
}

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum AtomEnum {
    Int(BigInt),
    Str(String),
    Bool(bool),
    Float(OrderedFloat<f32>),
}

impl IntoRecord for AtomEnum {
    fn into_record(self) -> Record {
        match self {
            AtomEnum::Bool(bool) => bool.into_record(),
            AtomEnum::Str(str) => str.into_record(), 
            AtomEnum::Int(bigint) => bigint.into_record(), 
            AtomEnum::Float(float) => float.into_record(),
        }
    }
}

// Any generic term is a Formula expression and we want to return all results as the same generic
// term itself rather than its generic parameter T.
impl<T> BasicExprOps for T where T: TermStructure {   

    type TermOutput = T;

    fn variables(&self) -> HashSet<Self::TermOutput> {
        // Allow multiple mutable reference for closure.
        let vars = RefCell::new(HashSet::new());
        self.traverse(
            &|term| { term.is_var() && !term.is_donot_care_variable() },
            &|term| { vars.borrow_mut().insert(term.clone()); }
        );
        vars.into_inner()
    }

    fn replace_pattern(&mut self, pattern: &Self::TermOutput, replacement: &Self::TermOutput) {
        self.traverse_mut(
            &|term| { return term == pattern; }, 
            &mut |term| { 
                *term = replacement.clone();
            }
        );
    }
}

impl<T> TermTraversal for T where T: TermStructure {
    fn traverse<F1, F2>(&self, pattern: &F1, logic: &F2)
    where 
        F1: Fn(&Self) -> bool, 
        F2: Fn(&Self)
    {
        if pattern(self) { logic(self); }
        for arg in self.arguments() {
            arg.traverse(pattern, logic);
        }
    }

    fn traverse_mut<F1, F2>(&mut self, pattern: &F1, logic: &mut F2) 
    where 
        F1: Fn(&Self) -> bool, 
        F2: FnMut(&mut Self)
    {
        if pattern(self) { logic(self); }
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
            &|term| { term.is_var() && !term.is_donot_care_variable() },
            &mut |var| {
                // Create an immutable copy of var from mutable reference.
                let var_clone = var.clone();
                let p = match vmap.contains_key(var) {
                    true => {
                        vmap.get(var).unwrap().clone()
                    },
                    false => {
                        let dc_name = generator.generate_name();
                        let dc_var = Self::gen_raw_variable_term(dc_name, vec![]);
                        dc_var
                    }
                };
                vmap.insert(var_clone, p.clone());
                *var = p;
            }
        );
        return (normalized_term, vmap);
    }

    fn match_in_place<'a>(&'a self, binding: &mut HashMap<&'a Self, &'a Self>, term: &'a Self) -> bool {
        if self.is_var() {
            // Detect a conflict in variable binding and return false.
            if binding.contains_key(self) && *binding.get(&self).unwrap() != term { return false; } 
            if !self.is_donot_care_variable() { binding.ginsert(self, term); }
            return true;
        } else if self.is_composite() {
            // TODO: Loose the restriction on type here but may need to revert it later.
            // if self.sort() != term.sort() { return false; }
            if self.arguments().len() != term.arguments().len() { return false; } 
            for i in 0 .. self.arguments().len() {
                let xargs = self.arguments();
                let yargs = term.arguments();
                let x = xargs.get(i).unwrap();
                let y = yargs.get(i).unwrap();
                let has_binding = x.match_in_place(binding, y);
                if !has_binding { return false; }
            }
            return true;
        } else if self.is_atom() {
            if self != term { return false; } else { return true; }
        } else { false }
    }

    fn match_to<'a>(&'a self, term: &'a Self) -> Option<HashMap<&'a Self, &'a Self>> {
        let mut bindings = HashMap::new();
        let has_binding = self.match_in_place(&mut bindings, term);
        if has_binding { Some(bindings) } else { None }
    }

    fn propagate(&self, map: &HashMap<&Self, &Self>) -> Self {
        // Make a clone and mutate the term when patterns are matched.
        let mut new_term = self.clone();
        new_term.traverse_mut(
            &|term| {
                if map.contains_key(term) 
                // || map.contains_key(&term.root()) 
                { return true; } 
                else { return false; }
            },
            &mut |mut term| {
                // Make an immutable clone here.
                if map.contains_key(term) {
                    let replacement = *map.get(term).unwrap();
                    *term = replacement.clone();
                } else {
                    todo!()
                    // The term here must be a variable term and have fragments like inside A(x.id, y.name).
                    // Dig into the root term to find the subterm by labels. 
                    // FIXME: Not working with variables with fragments. 
                    // let root = term.root();
                    // let root_val = map.gget(root).unwrap();
                    // let diff = term.fragments_diff(root).unwrap();
                    // let val = root_val.find_subterm_by_labels(&diff).unwrap();
                    // *term = val.clone();
                }
            }
        );
        return new_term;
    }

    // fn update_binding<M>(&self, binding: &mut M) -> bool where M: GenericMap<Self, Self> {
    //     match self.term_type() {
    //         TermType::Variable => {
    //             for key in binding.gkeys() {
    //                 if self.is_direct_subterm_of(key) {
    //                     let value = binding.gget(key).unwrap();
    //                     let fragments_diff = self.fragments_diff(key).unwrap();
    //                     let sub_value = value.find_subterm_by_labels(&fragments_diff).unwrap();
    //                     binding.ginsert(self.clone(), sub_value.clone());
    //                     return true
    //                 }
    //             }
    //             false
    //         },
    //         _ => { false }
    //     }
    // }

    // fn is_groundterm(&self) -> bool {
    //     match self.term_type() {
    //         TermType::Composite => {
    //             for arg in self.arguments() {
    //                 if !arg.is_groundterm() { return false; }
    //             }
    //             true
    //         },
    //         TermType::Variable => { false },
    //         TermType::Atom => { true }
    //     }
    // }

    // fn intersect(&self, other: &Self) -> (HashSet<Self>, HashSet<Self>, HashSet<Self>) {
    //     let vars: HashSet<Self> = self.variables();
    //     let other_vars: HashSet<Self> = other.variables();

    //     let (l, m, r) = ldiff_intersection_rdiff(&OrdSet::from(vars), &OrdSet::from(other_vars));
    //     let left: HashSet<Self> = l.into_iter().map(|x| x.into()).collect();
    //     let middle: HashSet<Self> = m.into_iter().map(|x| x.into()).collect();
    //     let right: HashSet<Self> = r.into_iter().map(|x| x.into()).collect();

    //     (left, middle, right)
    // }

    // fn has_deep_intersection<'a, I>(a: I, b: I) -> bool where I: Iterator<Item=&'a Self> {
    //     let mut bv = vec![];
    //     for v in b {
    //         bv.push(v);
    //     }

    //     for v1 in a {
    //         for &v2 in bv.iter() {
    //             if v1 == v2 || v1.is_direct_subterm_of(v2) || v2.is_direct_subterm_of(v1) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }

    // fn find_subterm(&self, var: &Self) -> Option<&Self> {
    //     // Find fragments diff between var and its root.
    //     let diff = var.root().fragments_diff(var);
    //     match diff {
    //         Some(diff) => {
    //             self.find_subterm_by_labels(&diff)
    //         },
    //         None => None
    //     }
    // }

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