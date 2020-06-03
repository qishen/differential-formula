use std::borrow::*;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::sync::Arc;
use std::vec::Vec;
use std::collections::*;
use std::convert::TryInto;
use std::fmt;
use std::fmt::{Debug, Display};
use std::string::String;
use std::hash::{Hash, Hasher};

use num::*;
use im::OrdSet;
use enum_dispatch::enum_dispatch;
use serde::{Serialize, Deserialize};
use differential_dataflow::hashable::*;

use crate::type_system::*;
use crate::expression::*;
use crate::util::*;


// A wrapped Term that cache the hash and use hash to compare ordering first.
pub type HashedTerm = OrdHashableWrapper<Term>;

#[enum_dispatch]
pub trait UniqueForm {
    /// Each Formula term has an unique form as a string but can be changed to regular
    /// types in the future. This method provides easy access to the unique form.
    fn unique_form(&self) -> &String;

    /// Generate an unique form of the term as string.
    fn create_unique_form(&self) -> String;

    /// The term is not immutable and when unique form needs to be updated when it is
    /// internally mutated.
    fn update(&mut self);
}

impl UniqueForm for Atom {
    fn unique_form(&self) -> &String {
        &self.unique_form
    }

    fn create_unique_form(&self) -> String {
        let unique_form = match &self.val {
            AtomEnum::Int(i) => format!("{}", i),
            AtomEnum::Bool(b) => format!("{:?}", b),
            AtomEnum::Str(s) => format!("\"{:?}\"", s),
            AtomEnum::Float(f) => format!("{}", f),
        };
        unique_form
    }

    fn update(&mut self) {
        let new_form = self.create_unique_form();
        self.unique_form = new_form;
    }
}

impl UniqueForm for Variable {
    fn unique_form(&self) -> &String {
        &self.unique_form
    }
    
    fn create_unique_form(&self) -> String {
        format!("{}.{}", self.root, self.fragments.join("."))
    }

    fn update(&mut self) {
        let new_form = self.create_unique_form();
        self.unique_form = new_form;
    }
}

impl UniqueForm for Composite {
    fn unique_form(&self) -> &String {
        &self.unique_form
    }

    fn create_unique_form(&self) -> String {
        let mut args = vec![];
        for arg in self.arguments.iter() {
            args.push(format!("{}", arg));
        }

        let args_str = args.join(", ");
        let unique_form = format!("{}({})", self.sort.name(), args_str);
        return unique_form;
    }

    fn update(&mut self) {
        let new_form = self.create_unique_form();
        self.unique_form = new_form;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Composite {
    pub unique_form: String,

    pub sort: Arc<Type>,

    pub arguments: Vec<Arc<Term>>,

    pub alias: Option<String>
}

impl Eq for Composite {} 

impl PartialEq for Composite {
    fn eq(&self, other: &Self) -> bool {
        self.unique_form == other.unique_form
    }
}

impl Ord for Composite {
    fn cmp(&self, other: &Self) -> Ordering {
        self.unique_form.cmp(&other.unique_form)
    }
}

impl PartialOrd for Composite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Composite {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sort.name().hash(state);
        for arg in self.arguments.iter() {
            arg.hash(state);
        }
    }
}

impl Display for Composite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let alias_str = match &self.alias {
            None => "".to_string(),
            Some(name) => format!("{} is ", name)
        };
        write!(f, "{}{}", alias_str, self.unique_form)
    }
}

impl Composite {
    pub fn new(sort: Arc<Type>, arguments: Vec<Arc<Term>>, alias: Option<String>) -> Self 
    {
        let mut composite = Composite {
            unique_form: "".to_string(),
            sort,
            arguments,
            alias,
        };
        composite.update();
        return composite;
    }

    pub fn validate(&self) -> bool {
        true
    }

}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Variable {
    pub unique_form: String,
    pub root: String,
    pub fragments: Vec<String>,
    pub base_term: Option<Arc<Term>>,
}

impl Eq for Variable {} 

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.unique_form == other.unique_form
    }
}

impl Ord for Variable {
    fn cmp(&self, other: &Self) -> Ordering {
        self.unique_form.cmp(&other.unique_form)
    }
}

impl PartialOrd for Variable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Variable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.root.hash(state);
        for elt in self.fragments.iter() {
            elt.hash(state);
        }
    }
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
        let mut var = match fragments.len() == 0 {
            true => {
                Variable {
                    unique_form: "".to_string(),
                    root,
                    fragments,
                    base_term: None,
                }
            },
            false => {
                // Create a base term with same root but no fragments so base term can be easily 
                // accessed later without clones.
                let base_term: Term = Variable::new(root.clone(), vec![]).into();
                Variable {
                    unique_form: "".to_string(),
                    root,
                    fragments,
                    base_term: Some(Arc::new(base_term))
                }
            }
        };
        var.update();
        return var;
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AtomEnum {
    Int(BigInt),
    Str(String),
    Bool(bool),
    Float(BigRational),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Atom {
    pub unique_form: String,
    pub val: AtomEnum,
}

impl From<AtomEnum> for Atom {
    fn from(val: AtomEnum) -> Self {
        Atom::new(val)
    }
}

impl From<AtomEnum> for Term {
    fn from(val: AtomEnum) -> Self {
        Atom::new(val).into()
    }
}

impl Atom {
    pub fn new(val: AtomEnum) -> Self {
        let mut atom = Atom { 
            unique_form: "".to_string(), 
            val 
        };
        atom.update();
        atom
    }
}

impl Eq for Atom {} 

impl PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        self.unique_form == other.unique_form
    }
}

impl Ord for Atom {
    fn cmp(&self, other: &Self) -> Ordering {
        self.unique_form.cmp(&other.unique_form)
    }
}

impl PartialOrd for Atom {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Atom {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.unique_form.hash(state);
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let atom_str = match &self.val {
            AtomEnum::Int(i) => format!("{}", i),
            AtomEnum::Bool(b) => format!("{:?}", b),
            AtomEnum::Str(s) => format!("\"{:?}\"", s),
            AtomEnum::Float(f) => format!("{}", f),
        };
        write!(f, "{}", atom_str)
    }
}

#[enum_dispatch(UniqueForm)]
#[derive(Clone, Serialize, Deserialize)]
pub enum Term {
    Composite,
    Variable,
    Atom
}

impl Eq for Term {} 

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        self.unique_form() == other.unique_form()
    }
}

impl Ord for Term {
    fn cmp(&self, other: &Self) -> Ordering {
        self.unique_form().cmp(&other.unique_form())
    }
}

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Term {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.unique_form().hash(state);
    }
}

pub trait FormulaTerm {
    /// Compare two Formula terms and return a binding if every variable in the first term `a` including 
    /// itself can match to the terms in the second term `b` at exact the same position. Fail if a conflict
    /// is detected or two terms have different types. This methods can be called by any borrowed term like
    /// Box<Term>, Rc<Term> or Arc<Term> that implementes Borrow<Term> and the binding map accepts those
    /// types too. `K` and `V` must implement trait `From<Term>` because a clone is made and then automatically
    /// converted to the instance of `K` or `V`.
    fn get_bindings_in_place<M, K, V, T>(&self, binding: &mut M, term: T) -> bool 
    where 
        M: GenericMap<K, V>,
        K: Borrow<Term> + From<Term> + Clone,
        V: Borrow<Term> + From<Term> + Clone,
        T: Borrow<Term> + Clone;
    
    /// Simply return a hash map that maps variables to terms.
    fn get_bindings<T>(&self, term: T) -> Option<HashMap<Term, Term>>
    where
        T: Borrow<Term> + Clone;

    /// Use `BTreeMap` when there is additional requirement that the map needs to implement `Ord` trait.
    fn get_ordered_bindings<T>(&self, term: T) -> Option<BTreeMap<Term, Term>>
    where
        T: Borrow<Term> + Clone;

    /// The same `BTreeMap` wrapped in `OrdHashableWrappr` that stashs the hash of the map and use the
    /// cached hash to decide the ordering of two maps. Only decide the ordering of two maps by recursively
    /// digging into two maps when the two hashes are equal in case of hash collision.
    fn get_cached_bindings<T>(&self, term: T) -> Option<QuickHashOrdMap<Term, Term>>
    where
        T: Borrow<Term> + Clone;
    
    /// Clone itself and mutate the cloned term by replacing variables with terms in the map.
    fn propagate_bindings<M, K, V>(&self, map: &M) -> Term
    where 
        M: GenericMap<K, V>,
        K: Borrow<Term>,
        V: Borrow<Term>;
        
    /// Find the subterm of a composite term when given a variable term with fragments.
    fn find_subterm<T>(&self, var_term: T) -> Option<Term> 
    where 
        T: Borrow<Term>;

    /// A similar method as find_subterm() method but given a list of labels as the argument to derive subterm.
    fn find_subterm_by_labels(&self, labels: &Vec<String>) -> Option<Term>;

    /// Update the binidng if the term (in borrowed form) is the subterm of one of the variable in the binding,
    /// e.g. `x.y.z` wants to update the binding with variable `x.y` as key in the binding. A subterm in the term
    /// that `x.y` points to will be derived and `x.y.z` -> `subterm` will be added into the current binding.
    fn update_binding<M, K, V>(&self, binding: &mut M) -> bool
    where 
        M: GenericMap<K, V>,
        K: Borrow<Term> + From<Term>,
        V: Borrow<Term> + From<Term>;
}

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
            &mut |mut term| { 
                *term = replacement.clone(); 
            }
        );
        println!("After replacement: {}", self);
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Term, SetComprehension> {
        // No set comprehension exists in terms.
        HashMap::new()
    }
}

impl<B> FormulaTerm for B where B: Borrow<Term> {
    fn get_bindings_in_place<M, K, V, T>(&self, binding: &mut M, term: T) -> bool 
    where 
        M: GenericMap<K, V>,
        K: Borrow<Term> + From<Term> + Clone,
        V: Borrow<Term> + From<Term> + Clone,
        T: Borrow<Term> + Clone,
    {
        match self.borrow() {
            Term::Atom(sa) => { false }, // Atom cannot be a pattern.
            Term::Variable(sv) => { 
                // Detect a conflict in variable binding and return false.
                if binding.contains_gkey(self.borrow()) && 
                   binding.gget(self.borrow()).unwrap().borrow() != term.borrow() {
                    return false;
                } 

                // Skip the do-not-care variable represented by underscore.
                if !self.borrow().is_dc_variable() {
                    // Some deep clones happen here and could affect performance when converting
                    // borrowed data to owned data.
                    
                    // TODO: if K and V are Arc<Term> then don't need to gain ownership and simply make
                    // a reference copy. While in reality we don't know what are the exact types of 
                    // B, K, V, T and that's why I have to turn them into owned data first.
                    let k: K = self.borrow().to_owned().into();
                    let v: V = term.borrow().to_owned().into();
                    binding.ginsert(k, v);
                }

                return true;
            },
            Term::Composite(sc) => {
                match term.borrow() {
                    Term::Composite(tc) => {
                        if sc.sort != tc.sort || sc.arguments.len() != tc.arguments.len() {
                            return false;
                        }

                        for i in 0..sc.arguments.len() {
                            let x = sc.arguments.get(i).unwrap().clone();
                            let y = tc.arguments.get(i).unwrap().clone();
    
                            match x.borrow() {
                                Term::Atom(xa) => {
                                    // Atom arguments need to be equal otherwise fail.
                                    if x != y { return false; }
                                },
                                _ => {
                                    let has_binding = x.get_bindings_in_place(binding, y.borrow());
                                    if !has_binding { return false; }
                                }
                            }
                        }
                    },
                    _ => { return false; } // Composite pattern won't match atom or variable.
                };
        
                return true;
            },
        }
    }

    fn get_bindings<T>(&self, term: T) -> Option<HashMap<Term, Term>>
    where
        T: Borrow<Term> + Clone
    {
        let mut bindings = HashMap::new();
        let has_binding = self.get_bindings_in_place(&mut bindings, term);
        if has_binding { Some(bindings) } else { None }
    }

    fn get_ordered_bindings<T>(&self, term: T) -> Option<BTreeMap<Term, Term>>
    where
        T: Borrow<Term> + Clone
    {
        let mut bindings= BTreeMap::new();
        let has_binding = self.get_bindings_in_place(&mut bindings, term);
        if has_binding { Some(bindings) } else { None }
    }

    fn get_cached_bindings<T>(&self, term: T) -> Option<QuickHashOrdMap<Term, Term>>
    where
        T: Borrow<Term> + Clone
    {
        let mut bindings= BTreeMap::new();
        let has_binding = self.get_bindings_in_place(&mut bindings, term);
        if has_binding { Some(bindings.into()) } else { None }
    }

    fn propagate_bindings<M, K, V>(&self, map: &M) -> Term
    where 
        M: GenericMap<K, V>,
        K: Borrow<Term>,
        V: Borrow<Term>,
    {
        // Make a clone and mutate the term when patterns are matched.
        let mut term = self.borrow().clone();
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
                    // The term here must be a variable term and have fragments like inside A(x.id, y.name).
                    // Dig into the root term to find the subterm by labels. 
                    let root = term.root();
                    let root_term = map.gget(root).unwrap();
                    let val = root_term.borrow().find_subterm(term.borrow()).unwrap();
                    *term = val;
                }
            }
        );

        term
    }

    fn find_subterm<T>(&self, var_term: T) -> Option<Term> 
    where 
        T: Borrow<Term>,
    {
        if let Term::Variable(v) = var_term.borrow() {
            return self.find_subterm_by_labels(&v.fragments);
        } else { return None; }
    }

    fn find_subterm_by_labels(&self, labels: &Vec<String>) -> Option<Term> {
        // Only apply to composite term and param must be a variable term.
        if let Term::Composite(cterm) = self.borrow() {
            let initial_term = self.borrow();
            let init_type = cterm.sort.base_type();
            let result = labels.iter().fold(Some((init_type, initial_term)), |state, label| {
                if let Some((ctype_enum, subterm)) = state {
                    if let Type::CompositeType(ctype) = ctype_enum {
                        let new_state = ctype.arguments.iter().enumerate().find_map(|(i, (arg_label_opt, t))| {
                            if let Some(arg_label) = arg_label_opt {
                                if arg_label == label {
                                    if let Term::Composite(cterm) = subterm.borrow() {
                                        // Update the composite type for the next round. Note that `t` could 
                                        // be a renamed type wrapping a composite type.
                                        let new_ctype = t.base_type();
                                        let cterm_arg: &Term = cterm.arguments.get(i).unwrap().borrow();
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

            if let Some((_, found_term)) = result {
                return Some(found_term.clone());
            }
        }
        return None;
    }

    fn update_binding<M, K, V>(&self, binding: &mut M) -> bool
    where 
        M: GenericMap<K, V>,
        K: Borrow<Term> + From<Term>,
        V: Borrow<Term> + From<Term>
    {
        let var_ref: &Term = self.borrow();
        match var_ref {
            Term::Variable(_) => {
                // Let's say `var` is `x.y.z` and the binding does not have root term of `x` as key 
                // but has some subterms of root term like `x.y` as key, then we only need to find
                // the subterm from `x.y` by looking up label `z`. Traverse the keys and find the 
                // first one that `var` is its subterm.
                for key_borrowed in binding.gkeys() {
                    let key: &Term = key_borrowed.borrow();
                    if key.has_subterm(var_ref).unwrap() {
                        let value = binding.gget(key).unwrap();
                        // find the fragments difference between `var` and `key`.
                        let labels = key.fragments_difference(var_ref).unwrap();
                        let sub_value = value.borrow().find_subterm_by_labels(&labels).unwrap();
                        binding.ginsert(self.borrow().clone().into(), sub_value.into());
                        return true;
                    }
                }
                return false;
            },
            _ => { return false; }
        }
    }
}

impl Term {
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

        if let Term::Composite(c) = self {
            for arg in c.arguments.iter_mut() {
                let arg_term = Arc::make_mut(arg);
                arg_term.traverse_mut(pattern, logic);
            }
        }

        // Need to change the unique form since the term is modified.
        self.update();
    }

    /// Convert non-ground term into a normalized form.
    pub fn normalize(&self) -> (Term, HashMap<Term, Term>) {
        let mut generator = NameGenerator::new("~p");

        // Map variables inside term to alias for normalization.
        let mut vmap: HashMap<Term, Term> = HashMap::new();

        // Allow multiple mutable reference for closure.
        // let vars = RefCell::new(HashSet::new());

        let mut normalized_term = self.clone();
        normalized_term.traverse_mut(
            &|term| {
                match term {
                    Term::Variable(v) => true,
                    _ => false
                }
            },
            &mut |var| {
                if !var.is_dc_variable() {
                    let pvar = match vmap.contains_key(var) {
                        true => {
                            vmap.get(var).unwrap().clone()
                        },
                        false => {
                            generator.generate_dc_term()
                        }
                    };
                    vmap.insert(var.clone(), pvar.clone());
                    *var = pvar;
                }
            }
        );

        // Need to change the unique form since the term is modified.
        normalized_term.update();

        return (normalized_term, vmap);
    }

    /// Given a string create a nullary composite type with no arguments inside
    /// and return the singleton term or constant in other words.
    pub fn create_constant(constant: String) -> Term {
        let nullary_type: Type = CompositeType {
            name: constant,
            arguments: vec![],
        }.into();
        
        // let wrapped_nullary_type = OrdHashableWrapper::new(nullary_type);
        Composite::new(Arc::new(nullary_type), vec![], None).into()
    }

    pub fn rename(&mut self, scope: String) {
        self.traverse_mut(
            &|term| {
                match term {
                    Term::Atom(_) => false, // Don't need to rename atom term.
                    _ => true,
                }
            }, 
            &mut |term| {
                match term {
                    Term::Variable(v) => {
                        // It looks like the renamed variable has fragments with a dot 
                        // but it actually does not. e.g. [x].[y] => [GraphIn.x].[y]
                        // Still only have one fragment.
                        v.root = format!("{}.{}", scope, v.root);
                    },
                    Term::Composite(c) => {
                        // TODO: A deep copy of type in every term looks bad.
                        let new_type = c.sort.rename_type(scope.clone());
                        // let wrapped_new_type = OrdHashableWrapper::new(new_type);
                        c.sort = Arc::new(new_type);
                    },
                    _ => {}
                }
            });
    }

    /// Check if the term has variable(s) inside it.
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

    /// Only apply to variable term to return the root term.
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
    pub fn has_conflict<M, K, V>(outer: &M, inner: &M) -> bool 
    where 
        M: GenericMap<K, V>,
        K: Borrow<Term>,
        V: Borrow<Term>
    {
        // Filter out conflict binding tuple of outer and inner scope.
        for inner_key in inner.gkeys() {
            let key_root = inner_key.borrow().root();
            let inner_val = inner.gget(inner_key.borrow()).unwrap();
            if outer.contains_gkey(inner_key.borrow()) {
                let outer_val = outer.gget(inner_key.borrow()).unwrap().borrow();
                if inner_val.borrow() != outer_val {
                    return true;
                }
            }

            // outer variable: x (won't be x.y...), inner variable: x.y.z...
            else if outer.contains_gkey(key_root) {
                let outer_val = outer.gget(key_root).unwrap();
                let outer_sub_val = outer_val.find_subterm(inner_key.borrow()).unwrap();
                if inner_val.borrow() != &outer_sub_val {
                    return true;
                }
            }
        }

        false
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
                let mut new_composite_term: Term = Composite::new(composite.sort.clone(), new_arguments, None).into();

                // if the raw term is matched in reverse map with a string alias, add the alias to this composite term.
                if reverse_map.contains_gkey(&new_composite_term) {
                    let alias = reverse_map.gget(&new_composite_term).unwrap();
                    let mut new_composite: Composite = new_composite_term.try_into().unwrap();
                    new_composite.alias = Some(alias.clone());
                    new_composite_term = new_composite.into();
                }

                Some(Arc::new(new_composite_term))
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
}