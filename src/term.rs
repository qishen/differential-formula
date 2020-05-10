use std::borrow::*;
use std::cell::RefCell;
use std::sync::Arc;
use std::vec::Vec;
use std::collections::*;
use std::convert::TryInto;
use std::fmt;
use std::fmt::{Debug, Display};
use std::string::String;

use derivative::*;
use enum_dispatch::enum_dispatch;
use im::{OrdMap, OrdSet};
use num::*;
use serde::{Serialize, Deserialize};

use crate::type_system::*;
use crate::expression::*;
use crate::util::*;


#[enum_dispatch(Term)]
pub trait FormulaTerm {}


#[derive(Derivative)]
#[derivative(Hash)]
#[derivative(PartialEq)]
#[derivative(Eq)]
//#[derivative(PartialOrd)]
//#[derivative(Ord)]
#[derive(Debug, Clone, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Composite {
    pub sort: Arc<Type>,
    pub arguments: Vec<Arc<Term>>,
    // Ignore `alias` when check equality or compute hash as one term may have multiple alias.
    #[derivative(Hash="ignore")]
    #[derivative(PartialEq="ignore")]
    //#[derivative(Eq="ignore")]
    pub alias: Option<String>
}

impl Display for Composite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut args = vec![];
        for arg in self.arguments.iter() {
            args.push(format!("{}", arg));
        }

        let args_str = args.join(", ");
        let alias_str = match &self.alias {
            None => "".to_string(),
            Some(name) => format!("{} is ", name)
        };

        write!(f, "{}{}({})", alias_str, self.sort.name(), args_str)
    }
}

impl Composite {
    pub fn new(sort: Arc<Type>, arguments: Vec<Arc<Term>>, alias: Option<String>) -> Self {
        Composite {
            sort,
            arguments,
            alias,
        }
    }

    pub fn validate(&self) -> bool {
        true
    }

}


impl FormulaTerm for Composite {}


#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Variable {
    pub root: String,
    pub fragments: Vec<String>,
    pub base_term: Option<Arc<Term>>,
}

impl Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rest = self.fragments.join(".");
        if self.fragments.len() > 0 {
            rest = ".".to_string() + &rest[..]; 
        }
        write!(f, "{}{}", self.root, rest)
    }
}

impl Variable {
    pub fn new(root: String, fragments: Vec<String>) -> Self {
        if fragments.len() == 0 {
            Variable {
                root,
                fragments,
                base_term: None,
            }
        }
        else {
            // Create a base term with same root but no fragments so base term can be easily accessed later without clones.
            let base_term: Term = Variable::new(root.clone(), vec![]).into();
            Variable {
                root,
                fragments,
                base_term: Some(Arc::new(base_term))
            }
        }
    }
}


impl FormulaTerm for Variable {}


#[enum_dispatch]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Atom {
    Int(BigInt),
    Str(String),
    Bool(bool),
    Float(BigRational),
}

impl Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let atom_str = match self {
            Atom::Int(i) => format!("{}", i),
            Atom::Bool(b) => format!("{:?}", b),
            Atom::Str(s) => format!("{:?}", s),
            Atom::Float(f) => format!("{}", f),
        };
        write!(f, "{}", atom_str)
    }
}

#[enum_dispatch]
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Term {
    Composite,
    Variable,
    Atom
}

impl FormulaTerm for Atom {}

impl Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let term_str = match self {
            Term::Composite(c) => format!("{}", c),
            Term::Variable(v) => format!("{}", v),
            Term::Atom(a) => format!("{}", a),
        };
        write!(f, "{}", term_str)
    }
}

impl Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let term_str = match self {
            Term::Composite(c) => format!("{}", c),
            Term::Variable(v) => format!("{}", v),
            Term::Atom(a) => format!("{}", a),
        };
        write!(f, "{}", term_str)
    }
}

impl FormulaExpr for Term {
    fn variables(&self) -> HashSet<Term> {
        // Allow multiple mutable reference for closure.
        let vars = RefCell::new(HashSet::new());
        self.traverse(
            &|term| {
                match term {
                    Term::Variable(v) => true,
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

    fn replace(&mut self, pattern: &Term, replacement: &Term) {
        self.traverse_mut(
            &|term| { return term == pattern; }, 
            &mut |mut term| { *term = replacement.clone(); }
        );
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Term, SetComprehension> {
        // No set comprehension exists in terms.
        HashMap::new()
    }
}

impl Term {
    /// Given a string create a nullary composite type with no arguments inside
    /// and return the singleton term or constant in other words.
    pub fn create_constant(constant: String) -> Term {
        let nullary_type: Type = CompositeType {
            name: constant,
            arguments: vec![],
        }.into();
        
        Composite::new(Arc::new(nullary_type), vec![], None).into()
    }

    // TODO: it works but has too many copies on its sort.
    pub fn rename(&self, scope: String) -> Term {
        let new_term: Term = match self {
            Term::Composite(c) => {
                let t = c.sort.as_ref().clone();
                let new_t = t.rename_type(scope.clone());
                let mut new_args = vec![];
                // Recursively rename each one in the arguments.
                for arg_arc in c.arguments.iter() {
                    let arg: Term = arg_arc.as_ref().clone();
                    new_args.push(Arc::new(arg.rename(scope.clone())));
                }
                let mut c2 = c.clone();
                c2.sort = Arc::new(new_t);
                c2.arguments = new_args;
                c2.into()
            },
            Term::Variable(v) => {
                // It looks like the renamed variable has fragments with a dot but it actually does not.
                let mut v2 = v.clone();
                v2.root = format!("{}.{}", scope, v.root);
                v2.into()
            },
            _ => { self.clone() }
        };

        new_term
    }

    pub fn is_groundterm(&self) -> bool {
        match self {
            Term::Composite(composite) => {
                for arg in composite.arguments.iter() {
                    if !arg.is_groundterm() {
                        return false;
                    }
                }
                true
            },
            Term::Variable(_variable) => { false },
            Term::Atom(_atom) => { true },
        }
    }

    pub fn root(&self) -> &Term {
        match self {
            Term::Variable(v) => {
                match &v.base_term {
                    Some(boxed_term) => { boxed_term.borrow() },
                    None => { self }
                }
            },
            _ => { self }
        }
    }

    // Check if the term is a don't-care variable with variable root name as '_'.
    pub fn is_dc_variable(&self) -> bool {
        match self {
            Term::Variable(v) => {
                if v.root == "_" { true }
                else { false }
            },
            _ => { false }
        }
    }

    /// Compare two lists of variable terms and return true if some terms in one list
    /// are subterms of the terms in another list. 
    pub fn has_deep_intersection<'a, I>(a: I, b: I) -> bool 
    where I: Iterator<Item=&'a Term>
    {
        let mut b_cloned = vec![];
        for v in b {
            b_cloned.push(v);
        }

        for v1 in a {
            for v2 in b_cloned.iter() {
                if v1.has_subterm(&v2).unwrap() || v2.has_subterm(v1).unwrap() {
                    return true;
                }
            }
        }

        return false;
    }

    /// A static function that computes the intersection of two ordered sets.
    pub fn two_sets_intersection(a: OrdSet<Term>, b: OrdSet<Term>) 
    -> (OrdSet<Term>, OrdSet<Term>, OrdSet<Term>)
    {
        let mut middle = OrdSet::new();
        let mut left = a.clone();
        let mut right = b.clone();
        // Save the intersection and remove intersection from both sets and keep the leftovers.
        for overlap_var in a.intersection(b) {
            middle.insert(overlap_var.clone());
            left.remove(&overlap_var);
            right.remove(&overlap_var);
        }

        (left, middle, right)
    }
    
    /// Compare the variables in two terms and find the intersection part.
    pub fn intersect(&self, other: &Term) -> (OrdSet<Term>, OrdSet<Term>, OrdSet<Term>) {
        let vars = self.variables();
        let other_vars = self.variables();

        Term::two_sets_intersection(
            OrdSet::from(vars).into_iter().map(|x| x.clone()).collect(), 
            OrdSet::from(other_vars).into_iter().map(|x| x.clone()).collect(),
        )
    }

    // Check if two binding map has conflits in variable mappings.
    pub fn has_conflit<T>(outer: &T, inner: &T) -> bool 
    where 
        T: GenericMap<Arc<Term>, Arc<Term>>,
    {
        // Filter out conflict binding tuple of outer and inner scope.
        for inner_key in inner.gkeys() {
            let key_root = inner_key.root();
            let inner_val = inner.gget(inner_key).unwrap();
            if outer.contains_gkey(inner_key) {
                let outer_val = outer.gget(inner_key).unwrap().borrow();
                if inner_val != outer_val {
                    return true;
                }
            }
            // outer variable: x (won't be x.y...), inner variable: x.y.z...
            else if outer.contains_gkey(key_root) {
                let outer_val = outer.gget(key_root).unwrap();
                let outer_sub_val = Term::find_subterm(outer_val.clone(), inner_key).unwrap();
                if inner_val != &outer_sub_val {
                    return true;
                }
            }
        }

        false
    }

    /// Use HashMap to store the binding derived from term matching but the keys are not ordered.
    pub fn get_bindings(&self, term: &Arc<Term>) -> Option<HashMap<Arc<Term>, Arc<Term>>> {
        let mut bindings = HashMap::new();
        let has_binding = self.get_bindings_in_place(&mut bindings, term);
        if has_binding {
            Some(bindings)
        } else {
            None
        }
    }

    /// Use OrdMap to store the binding derived from term matching.
    pub fn get_ordered_bindings(&self, term: &Arc<Term>) -> Option<OrdMap<Arc<Term>, Arc<Term>>> {
        let mut bindings= OrdMap::new();
        let has_binding = self.get_bindings_in_place(&mut bindings, term);
        if has_binding {
            Some(bindings)
        } else {
            None
        }
    }

    /// Assume this method only called by composite term.
    pub fn get_bindings_in_place<T>(&self, binding: &mut T, term: &Arc<Term>) -> bool 
    where 
        T: GenericMap<Arc<Term>, Arc<Term>>
    {
        match self {
            Term::Atom(sa) => { false }, 
            Term::Variable(sv) => { true },
            Term::Composite(sc) => {
                match term.borrow() {
                    Term::Composite(c) => {
                        if sc.sort == c.sort {
                            for i in 0..sc.arguments.len() {
                                let x = sc.arguments.get(i).unwrap();
                                let y = c.arguments.get(i).unwrap();
        
                                match x.borrow() {
                                    Term::Atom(xa) => {
                                        // Atom arguments need to be equal.
                                        if x != y {
                                            return false;
                                        }
                                    },
                                    Term::Variable(xv) => {
                                        if !x.is_dc_variable() {
                                            binding.ginsert(x.clone(), y.clone());
                                        }
                                    },
                                    Term::Composite(xc) => {
                                        let mut sub_binding = HashMap::new();
                                        let has_binding = x.get_bindings_in_place(&mut sub_binding, y);
                                        if has_binding {
                                            for (k, v) in sub_binding.drain() {
                                                // Detect a variable binding conflict and return false immediately.
                                                if binding.contains_gkey(&k) {
                                                    if binding.gget(&k).unwrap() != &v {
                                                        return false;
                                                    }
                                                } else {
                                                    binding.ginsert(k, v);
                                                }
                                            }    
                                        } else {
                                            // No binding found for current argument.
                                            return false;
                                        }
                                    }
                                }
                            }
                        } else {
                            return false;
                        }
                    },
                    _ => {
                        return false;
                    }
                };
        
                true
            },
        }
        
    
    }

    // Traverse the term recursively to find the pattern without mutating the found term.
    pub fn traverse<F1, F2>(&self, pattern: &F1, logic: &F2)
    where F1: Fn(&Term) -> bool, F2: Fn(&Term)
    {
        if pattern(self) {
            logic(self);
        }

        // Recursively match all arguments in the composite term even the term is already matched.
        // For example: List ::= new (content: Integer, next: List + {NIL}). We can write a pattern 
        // like List(a, b) that not only match List(1, List(2, NIL)) but also match its child List(2, NIL).
        match self {
            Term::Composite(c) => {
                for arg in c.arguments.iter() {
                    arg.traverse(pattern, logic);
                }
            },
            _ => {}
        };
    }

    /// Traverse the term recursively from root to find the term that satifies the pattern
    /// and then apply the logic to the mutable term.
    pub fn traverse_mut<F1, F2>(&mut self, pattern: &F1, logic: &mut F2) 
    where F1: Fn(&Term) -> bool, F2: FnMut(&mut Term)
    {
        if pattern(self) {
            logic(self);
        }

        match self {
            Term::Composite(c) => {
                for arg_arc in c.arguments.iter_mut() {
                    let arg_term = Arc::make_mut(arg_arc);
                    arg_term.traverse_mut(pattern, logic);
                }
            },
            _ => {}
        };
    }


    pub fn propagate_binding<T>(&self, map: &T) -> Option<Arc<Term>>
    where T: GenericMap<Arc<Term>, Arc<Term>>
    {
        // Make a clone and mutate the term when pattern matched.
        let mut term = self.clone();
        term.traverse_mut(
            &|term| {
                if map.contains_gkey(term) || map.contains_gkey(term.root()) { return true; } 
                else { return false; }
            },
            &mut |mut term| {
                if map.contains_gkey(term) {
                    let replacement: &Term = map.gget(term).unwrap().borrow();
                    *term = replacement.clone();
                } else {
                    // Dig into the root term to find the subterm by labels. 
                    let root = term.root();
                    let root_term = map.gget(root).unwrap();
                    // *term = Term::find_subterm(root_term.clone(), arg_borrowed).unwrap();
                }
            }
        );

        Some(Arc::new(term))
    }
    
    /// Propagate the binding to a term (only works for composite term) and return a new term. 
    /// The map must implement GenericMap with the type of both key and value restricted to Arc<Term>.
    pub fn propagate_bindings<T>(&self, map: &T) -> Option<Arc<Term>> 
    where 
        T: GenericMap<Arc<Term>, Arc<Term>>,
    {
        let new_term = match self {
            Term::Composite(composite) => {
                let mut arguments = vec![];
                for i in 0..composite.arguments.len() {
                    let arg = composite.arguments.get(i).unwrap();
                    let arg_borrowed: &Term = arg.borrow();
                    let new_arg = match arg_borrowed {
                        Term::Composite(_) => {
                            arg_borrowed.propagate_bindings(map).unwrap()
                        },
                        Term::Variable(_) => {
                            let root = arg_borrowed.root();
                            let result;
                            if map.contains_gkey(arg_borrowed) {
                                // Find an exact match in hash map and return its value cloned.
                                result = map.gget(arg).unwrap().clone();
                            } else if map.contains_gkey(root) {
                                // Dig into the root term to find the subterm by labels. 
                                let root_term = map.gget(root).unwrap();
                                result = Term::find_subterm(root_term.clone(), arg_borrowed).unwrap();
                            } else {
                                // No match and just return variable itself.
                                result = arg.clone();
                            }
                            result
                        },
                        Term::Atom(a) => { arg.clone() }
                    };

                    arguments.push(new_arg);

                }

                let new_term = Composite {
                    sort: composite.sort.clone(),
                    arguments,
                    alias: None, // composite.alias.clone(), skip the alias is fine.
                }.into();

                Some(Arc::new(new_term))
            },
            // The function only applies to composite term.
            Term::Variable(v) => { None },
            Term::Atom(a) => { None }
        };

        new_term
    }

    // Add alias to the term and its subterms recursively if a match is found in reversed map.
    pub fn propagate_reverse_bindings<T: GenericMap<Arc<Term>, String>>(&self, reverse_map: &T) -> Option<Arc<Term>> {
        match self {
            Term::Composite(composite) => {
                let mut new_arguments = vec![];
                for arg in composite.arguments.iter() {
                    let arg_ref: &Term = arg.borrow();
                    let new_term = match arg_ref {
                        Term::Composite(composite) => {
                            arg.propagate_reverse_bindings(reverse_map).unwrap()
                        },
                        _ => { arg.clone() }
                    };
                    new_arguments.push(new_term);
                }

                // The new term does not contain alias but will change it later if a match is found in reverse alias map.
                let mut new_composite_term: Term = Composite {
                    sort: composite.sort.clone(),
                    arguments: new_arguments,
                    alias: None,
                }.into();

                // if the raw term is matched in reverse map with a string alias, add the alias to this composite term.
                if reverse_map.contains_gkey(&new_composite_term) {
                    let alias = reverse_map.gget(&new_composite_term).unwrap();
                    let mut new_composite: Composite = new_composite_term.try_into().unwrap();
                    new_composite.alias = Some(alias.clone());
                    new_composite_term = new_composite.into();
                }

                Some(Arc::new(new_composite_term))

            },
            _ => {
                None                
            }

        }

    }

    /// Find the subterm of a composite term when given a variable term with fragments.
    pub fn find_subterm<T>(composite_term: Arc<Term>, var_term: &T) -> Option<Arc<Term>> 
    where 
        T: Borrow<Term>,
    {
        // Only apply to composite term and param must be a variable term.
        match composite_term.borrow() {
            Term::Composite(c) => {
                match var_term.borrow() {
                    Term::Variable(v) => {
                        c.sort.find_subterm(&composite_term, &v.fragments)
                    },
                    _ => { None }
                }
            },
            _ => { None }
        }
    }

    /// A shortcut for find_subterm() method with only labels as the argument.
    /// 
    pub fn find_subterm_by_labels(composite_term: Arc<Term>, labels: &Vec<String>) -> Option<Arc<Term>> {
        match composite_term.borrow() {
            Term::Composite(c) => {
                c.sort.find_subterm(&composite_term, labels)
            },
            _ => { None }
        }
    }

    /// Check if a variable term is the subterm of another variable term. 
    /// Variable with longer fragments is the subterm of variable with shorter fragments.
    pub fn has_subterm(&self, term: &Term) -> Option<bool> {
        match self {
            Term::Variable(v1) => {
                match term {
                    Term::Variable(v2) => {
                        if v1.root == v2.root && v2.fragments.starts_with(&v1.fragments){
                            Some(true)
                        }
                        else {
                            Some(false)
                        }
                    },
                    _ => { None }
                }
            },
            _ => { None }
        }
    }

    /// If one variable term starts with another variable term, then return their difference in the fragments.
    pub fn fragments_difference(&self, term: &Term) -> Option<Vec<String>> {
        match self {
            Term::Variable(v1) => {
                let len1 = v1.fragments.len();
                match term {
                    Term::Variable(v2) => {
                        let len2 = v2.fragments.len();
                        if v1.fragments.starts_with(&v2.fragments) {
                            let mut labels = vec![];
                            for i in len2 .. len1 {
                                labels.push(v1.fragments.get(i).unwrap().clone());
                            } 
                            Some(labels)
                        }
                        else if v2.fragments.starts_with(&v1.fragments) {
                            let mut labels = vec![];
                            for i in len1 .. len2 {
                                labels.push(v2.fragments.get(i).unwrap().clone());
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

    /// Update the binidng if a variable term is the subterm of one of the variable terms in the binding,
    /// e.g. `x.y.z` wants to update binding with variable `x.y` as key in the binding, then derive the value
    /// for the subterm and add `x.y.z` to the binding too. Retrun true if binding is successfully updated with
    /// new derived subterm as the key.
    pub fn update_binding<T>(var: &Arc<Term>, binding: &mut T) -> bool
    where T: GenericMap<Arc<Term>, Arc<Term>>
    {
        let var_ref: &Term = var.borrow();
        match var_ref {
            Term::Variable(_) => {
                /*
                Let's say `var` is `x.y.z` and the binding does not have root term of `x` as key 
                but has some subterms of root term like `x.y` as key, then we only need to find
                the subterm from `x.y` by looking up label `z`. Traverse the keys and find the 
                first one that `var` is its subterm.
                */ 
                for key_arc in binding.gkeys() {
                    let key: &Term = key_arc.borrow();
                    if key.has_subterm(var_ref).unwrap() {
                        let value = binding.gget(key).unwrap();
                        // find the fragments difference between `var` and `key`.
                        let labels = key.fragments_difference(var_ref).unwrap();
                        let sub_value = Term::find_subterm_by_labels(value.clone(), &labels).unwrap();
                        binding.ginsert(var.clone(), sub_value);
                        return true;
                    }
                }
                return false;
            },
            _ => { 
                return false; 
            }
        }
    }
    
}