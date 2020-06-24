use std::borrow::*;
use std::cell::*;
use std::sync::*;
use std::vec::Vec;
use std::collections::*;
use std::convert::*;
use std::fmt;
use std::fmt::{Debug, Display};
use std::string::String;
use std::hash::Hash;

use derivative::*;
use im::OrdSet;
use num::*;
use enum_dispatch::enum_dispatch;
use serde::{Serialize, Deserialize};

use crate::term::{FormulaTerm, FromWithIndex};
use crate::type_system::*;
use crate::expression::*;
use crate::module::{Model, MetaInfo};
use crate::util::*;
use crate::util::map::*;


/// This trait implements both mutable and immutable borrow with some basic derivable traits.
/// Some recursion occurs in trait definition. It's very interesting here that the trait
/// has one generic type param that needs to implement the trait itself.
pub trait BorrowedTerm<S, T>: Borrow<Term<S, T>> + BorrowMut<Term<S, T>> + From<Term<S, T>> + Eq + Hash + Clone + Ord + Debug + Display 
where 
    S: BorrowedType, T: BorrowedTerm<S, T> {}

impl<S, T> BorrowedTerm<S, T> for Term<S, T> where S: BorrowedType, T: BorrowedTerm<S, T> {}

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum AtomEnum {
    Int(BigInt),
    Str(String),
    Bool(bool),
    Float(BigRational),
}

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Atom {
    pub unique_form: String,
    pub val: AtomEnum,
}

impl From<AtomEnum> for Atom {
    fn from(val: AtomEnum) -> Self {
        Atom::new(val)
    }
}

impl<S, T> From<AtomEnum> for Term<S, T> where S: BorrowedType, T: BorrowedTerm<S, T> {
    fn from(val: AtomEnum) -> Self {
        Term::Atom(Atom::new(val))
    }
}

impl Atom {
    pub fn new(val: AtomEnum) -> Self {
        let mut atom = Atom { 
            unique_form: "".to_string(), 
            val 
        };
        atom
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

// A generic variable term that does not require a specific type of reference.
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Variable<S, T> where T: BorrowedTerm<S, T> {
    pub root: String,
    pub fragments: Vec<String>,
    pub base_term: Option<T>
}

impl<S, T> Display for Variable<S, T> where T: BorrowedTerm<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rest = self.fragments.join(".");
        if self.fragments.len() > 0 {
            rest = ".".to_string() + &rest[..]; 
        }
        write!(f, "{}{}", self.root, rest)
    }
}

impl<S, T> Variable<S, T> where T: BorrowedTerm<S, T>, S: BorrowedType {
    pub fn new(root: String, fragments: Vec<String>) -> Self {
        let mut var = match fragments.len() == 0 {
            true => {
                Variable {
                    root,
                    fragments,
                    base_term: None,
                }
            },
            false => {
                // Create a base term with same root but no fragments so base term can be easily 
                // accessed later without clones.
                let base_term: Term<S, T> = Term::Variable(Variable::new(root.clone(), vec![]));
                Variable {
                    root,
                    fragments,
                    base_term: Some(base_term.into())
                }
            }
        };

        return var;
    }
}

// TODO: Exclude alias for auto-derived trait like Eq, Hash and Ord.
/// A generic Composite struct that does not require a specific smart pointer like 
/// `Rc<T>` or `Arc<T>` for its sort and arguments and of course you can use the native
/// data types without wrappers.
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Composite<S, T> where S: BorrowedType, T: BorrowedTerm<S, T> {
    pub sort: S,

    pub arguments: Vec<T>,

    pub alias: Option<String>,
}

impl<S, T> Display for Composite<S, T> where S: BorrowedType, T: BorrowedTerm<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl<S, T> Composite<S, T> where S: BorrowedType, T: BorrowedTerm<S, T> {
    pub fn new(sort: S, arguments: Vec<T>, alias: Option<String>) -> Self {
        let mut composite = Composite {
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

#[enum_dispatch]
pub trait TermTrait {}
impl TermTrait for Atom {}
impl<S, T> TermTrait for Variable<S, T> {}
impl<S, T> TermTrait for Composite<S, T> {}

#[enum_dispatch(TermTrait)]
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Term<S, T> where S: BorrowedType, T: BorrowedTerm<S, T> {
    Composite(Composite<S, T>),
    Variable(Variable<S, T>),
    Atom(Atom)
}

// Put some term-related static methods here.
impl<S, T> Term<S, T> 
where 
    S: BorrowedType, 
    T: BorrowedTerm<S, T> + FormulaExprTrait<SortOutput=S, TermOutput=T>
{
    /// Given a string create a nullary composite type with no arguments inside
    /// and return the singleton term or constant in other words.
    pub fn create_constant(constant: String) -> T {
        let nullary_type: Type = Type::CompositeType(
            CompositeType { name: constant, arguments: vec![] }
        );
        let term: Term<S, T> = Term::Composite(
            Composite::new(nullary_type.into(), vec![], None)
        );
        return term.into();
    }

    /// Compare two lists of variable terms and return true if some terms in one list
    /// are subterms of the terms in another list. 
    pub fn has_deep_intersection<I>(a: I, b: I) -> bool where I: Iterator<Item=T> {
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

impl<S, T> Display for Term<S, T> where S: BorrowedType, T: BorrowedTerm<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let term_str = match self.borrow() {
            Term::Composite(c) => format!("{}", c),
            Term::Variable(v) => format!("{}", v),
            Term::Atom(a) => format!("{}", a),
        };
        write!(f, "{}", term_str)
    }
}

impl<S, T> Debug for Term<S, T> where S: BorrowedType, T: BorrowedTerm<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let term_str = match self.borrow() {
            Term::Composite(c) => format!("{}", c),
            Term::Variable(v) => format!("{}", v),
            Term::Atom(a) => format!("{}", a),
        };
        write!(f, "{}", term_str)
    }
}

// Do nothing and just return the term.
impl<S, T> FromWithIndex<S, Term<S, T>> for Term<S, T> {
    fn from_with_index(item: Term<S, T>, index: Arc<RwLock<Model<S, T>>>) -> Self {
        item
    }
}

/// Implement `FormulaExprTrait` not directly for Term<S, T> instead the implementation is 
/// for any type that implements the `BorrowedTerm` trait that wraps Term<S, T>
impl<S, T> FormulaExprTrait for T where S: BorrowedType, T: BorrowedTerm<S, T>
{   
    type SortOutput = S;
    type TermOutput = T; 

    fn variables(&self) -> HashSet<Self::TermOutput> {
        // Allow multiple mutable reference for closure.
        let vars = RefCell::new(HashSet::new());
        self.traverse(
            &|term| {
                match term.borrow() {
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

    fn replace_pattern(&mut self, pattern: &Self::TermOutput, replacement: &Self::TermOutput) {
        self.traverse_mut(
            &|term| { return term.borrow() == pattern.borrow(); }, 
            &mut |mut term| { 
                *term = replacement.clone();
            }
        );
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) 
    -> HashMap<Self::TermOutput, SetComprehension<Self::SortOutput, Self::TermOutput>> 
    {
        // set comprehension does not exist in terms.
        HashMap::new()
    }
}

/// Implement the `FormulaTerm` trait in a way that it works both for native `Term` and boxed `Term` like `Arc<Term>`.
/// `T` could be native Term or boxed Term like `Arc<Term>` or `Box<Term>` and `T` needs to satisfy some traits that
/// enables `T` to be converted back and forth between `Term` and `Arc<Term>` because the arguments of composite
/// term has `Arc<Term>` type and we don't know what exactly is the type of `T`.
impl<S, T> FormulaTerm for T where S: BorrowedType, T: BorrowedTerm<S, T>
{
    // Assuming the output has the same type as the type of input being invoked.
    type Output = T;

    fn traverse<F1, F2>(&self, pattern: &F1, logic: &F2)
    where F1: Fn(&Self::Output) -> bool, F2: Fn(&Self::Output)
    {
        if pattern(self) {
            logic(self);
        }

        // Recursively match all arguments in the composite term even the term is already matched.
        // For example: List ::= new (content: Integer, next: List + {NIL}). We can write a pattern 
        // like List(a, b) that not only match List(1, List(2, NIL)) but also match its child List(2, NIL).
        match self.borrow() {
            Term::Composite(c) => {
                for arg in c.arguments.iter() {
                    // Convert native term or boxed term like `Arc<Term>` into Self::Output in order to
                    // use the traverse method, where Self::Output could be native term or boxed term.
                    let converted_arg: Self::Output = arg.clone().into();
                    converted_arg.traverse(pattern, logic);
                }
            },
            _ => {}
        };
    }

    fn traverse_mut<F1, F2>(&mut self, pattern: &F1, logic: &mut F2) 
    where 
        F1: Fn(&Self::Output) -> bool, 
        F2: FnMut(&mut Self::Output)
    {
        if pattern(self) {
            logic(self);
        }

        // `self` is `Arc<Term>` which may or may not own the inner term. If `self` has weak pointers
        // then the inner value will be cloned in `self` while other pointers keep the old inner value.
        // let inner_or_cloned = Arc::make_mut(self);

        if let Term::Composite(c) = self.borrow_mut() {
            for arg in c.arguments.iter_mut() {
                // Clone Arc<Term> and convert into Self::Output, it could be Term or just Arc<Term>.
                // Conversion into Term from Arc<Term> may need a deep copy but Arc<Term> should be fine
                // with only a reference copy.
                let mut new_arg: Self::Output = arg.clone().into();
                new_arg.traverse_mut(pattern, logic);
                // Convert whatever Self::Output is back into Arc<Term>.
                *arg = new_arg.into();
            }
        }
    }

    /// Convert non-ground term into a normalized form with variables replaced with normalized
    /// variables starting with `~p`.
    fn normalize(&self) -> (Self::Output, HashMap<Self::Output, Self::Output>) {
        let mut generator = NameGenerator::new("~p");

        // Map normalized variables to original variables.
        let mut vmap: HashMap<Self::Output, Self::Output> = HashMap::new();

        let mut normalized_term = self.clone();
        normalized_term.traverse_mut(
            &|term| {
                match term.borrow() {
                    Term::Variable(v) => true,
                    _ => false
                }
            },
            &mut |var| {
                // Create an immutable copy of var from mutable reference.
                let var_ref = var.clone();
                if !var.is_dc_variable() {
                    let p = match vmap.contains_key(var_ref.borrow()) {
                        true => {
                            vmap.get(var_ref.borrow()).unwrap().clone()
                        },
                        false => {
                            let dc_name = generator.generate_name();
                            let dc_var: Term<S, T> = Term::Variable(Variable::new(dc_name, vec![]));
                            let p: T = dc_var.into();
                            p
                        }
                    };
                    vmap.insert(p.clone(), var_ref);
                    *var = p.into();
                }
            }
        );

        return (normalized_term, vmap);
    }


    fn get_bindings_in_place<M>(&self, binding: &mut M, term: &Self::Output) -> bool 
    where 
        M: GenericMap<Self::Output, Self::Output>
    {
        match self.borrow() {
            Term::Atom(sa) => { false }, // Atom cannot be a pattern.
            Term::Variable(sv) => { 
                // Detect a conflict in variable binding and return false.
                if binding.contains_gkey(self.borrow()) && 
                   binding.gget(self.borrow()).unwrap() != term {
                    return false;
                } 

                // Skip the do-not-care variable represented by underscore.
                if !self.is_dc_variable() {
                    binding.ginsert(self.clone(), term.clone());
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
                            let x: Self::Output = sc.arguments.get(i).unwrap().clone().into();
                            let y: Self::Output = tc.arguments.get(i).unwrap().clone().into();
    
                            match x.borrow() {
                                Term::Atom(xa) => {
                                    // Atom arguments need to be equal otherwise fail.
                                    if x != y { return false; }
                                },
                                _ => {
                                    let has_binding = x.get_bindings_in_place(binding, &y);
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

    fn get_bindings(&self, term: &Self::Output) -> Option<HashMap<Self::Output, Self::Output>> {
        let mut bindings = HashMap::new();
        let has_binding = self.get_bindings_in_place(&mut bindings, term);
        if has_binding { Some(bindings) } else { None }
    }

    fn get_ordered_bindings(&self, term: &Self::Output) -> Option<BTreeMap<Self::Output, Self::Output>> {
        let mut bindings= BTreeMap::new();
        let has_binding = self.get_bindings_in_place(&mut bindings, term);
        if has_binding { Some(bindings) } else { None }
    }

    fn get_cached_bindings(&self, term: &Self::Output) -> Option<QuickHashOrdMap<Self::Output, Self::Output>> {
        let mut bindings= BTreeMap::new();
        let has_binding = self.get_bindings_in_place(&mut bindings, term);
        if has_binding { Some(bindings.into()) } else { None }
    }

    fn propagate_bindings<M>(&self, map: &M) -> Self::Output
    where 
        M: GenericMap<Self::Output, Self::Output>
    {
        // Make a clone and mutate the term when patterns are matched.
        let mut self_copy = self.clone();

        self_copy.traverse_mut(
            &|term| {
                if map.contains_gkey(term.borrow()) || map.contains_gkey(&term.root().borrow()) { return true; } 
                else { return false; }
            },
            &mut |mut term| {
                // Make an immutable clone here.
                let term_ref = term.clone();
                if map.contains_gkey(term_ref.borrow()) {
                    let replacement = map.gget(term_ref.borrow()).unwrap();
                    *term = replacement.clone();
                } else {
                    // The term here must be a variable term and have fragments like inside A(x.id, y.name).
                    // Dig into the root term to find the subterm by labels. 
                    let root = term.root();
                    let root_term = map.gget(root.borrow()).unwrap();
                    // Relax, it's just a reference copy no big deal :)
                    let val = root_term.find_subterm(term).unwrap();
                    *term = val;
                }
            }
        );

        return self_copy;
    }

    fn find_subterm(&self, var_term: &Self::Output) -> Option<Self::Output> 
    {
        if let Term::Variable(v) = var_term.borrow() {
            return self.find_subterm_by_labels(&v.fragments);
        } else { return None; }
    }

    fn find_subterm_by_labels(&self, labels: &Vec<String>) -> Option<Self::Output> {
        // Only apply to composite term and param must be a variable term.
        if let Term::Composite(cterm) = self.borrow() {
            let initial_term = self.clone();
            let init_type = cterm.sort.borrow().base_type();
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
                                        let cterm_arg = cterm.arguments.get(i).unwrap().clone();
                                        // Need to convert Arc<Term> into generic type T.
                                        return Some((new_ctype, cterm_arg.into()));
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

    fn update_binding<M>(&self, binding: &mut M) -> bool
    where 
        M: GenericMap<Self::Output, Self::Output>,
    {
        let var_ref: &Term<S, T> = self.borrow();
        match var_ref {
            Term::Variable(_) => {
                // Let's say `var` is `x.y.z` and the binding does not have root term of `x` as key 
                // but has some subterms of root term like `x.y` as key, then we only need to find
                // the subterm from `x.y` by looking up label `z`. Traverse the keys and find the 
                // first one that `var` is its subterm.
                for key in binding.gkeys() {
                    if key.has_subterm(self).unwrap() {
                        let value = binding.gget(key.borrow()).unwrap();
                        // find the fragments difference between `var` and `key`.
                        let labels = key.fragments_difference(self).unwrap();
                        let sub_value = value.find_subterm_by_labels(&labels).unwrap();
                        binding.ginsert(self.clone(), sub_value);
                        return true;
                    }
                }
                return false;
            },
            _ => { return false; }
        }
    }

    fn rename<BS, BT>(&self, scope: String, metainfo: &MetaInfo<BS, BT>) -> Self
    where 
        BS: BorrowedType, BT: BorrowedTerm<BS, BT>
    {
        // Make a copy of itself and apply a mutable rename.
        let mut self_clone = self.clone();
        self_clone.rename_mut(scope, metainfo);
        return self_clone;
    }

    fn rename_mut<BS, BT>(&self, scope: String, metainfo: &MetaInfo<BS, BT>)
    where 
        BS: BorrowedType, BT: BorrowedTerm<BS, BT>
    {
        self.traverse_mut(
            &|term| {
                match term.borrow() {
                    Term::Atom(_) => false, // Don't need to rename atom term.
                    _ => true,
                }
            }, 
            &mut |term| {
                // let inner_or_cloned = Arc::make_mut(term);
                match term.borrow_mut() {
                    Term::Variable(v) => {
                        // It looks like the renamed variable has fragments with a dot 
                        // but it actually does not. e.g. [x].[y] => [GraphIn.x].[y]
                        // After rename it still only have one fragment.
                        v.root = format!("{}.{}", scope, v.root);
                    },
                    Term::Composite(c) => {
                        // Assuming that the renamed type is already in MetaInfo.
                        let new_type_name = format!("{}.{}", scope, c.sort.name());
                        let new_type = metainfo.get_type_by_name(&new_type_name);
                    },
                    _ => {}
                }
            });
    }

    fn is_groundterm(&self) -> bool {
        match self.borrow() {
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

    fn root(&self) -> Self::Output {
        match self.borrow() {
            Term::Variable(v) => {
                match &v.base_term {
                    Some(boxed_term) => { boxed_term.clone().into() },
                    None => { self.clone() }
                }
            },
            _ => { self.clone() }
        }
    }

    fn is_dc_variable(&self) -> bool {
        match self.borrow() {
            Term::Variable(v) => {
                if v.root == "_" { true }
                else { false }
            },
            _ => { false }
        }
    }

    fn intersect(&self, other: &Self::Output) -> (HashSet<Self::Output>, HashSet<Self::Output>, HashSet<Self::Output>) 
    {
        let vars: HashSet<T> = self.variables();
        let other_vars: HashSet<T> = other.variables();

        let (l, m, r) = ldiff_intersection_rdiff(&OrdSet::from(vars), &OrdSet::from(other_vars));
        let left: HashSet<Self::Output> = l.into_iter().map(|x| x.into()).collect();
        let middle: HashSet<Self::Output> = m.into_iter().map(|x| x.into()).collect();
        let right: HashSet<Self::Output> = r.into_iter().map(|x| x.into()).collect();

        (left, middle, right)
    }

    fn has_conflict<M>(outer: &M, inner: &M) -> bool 
    where 
        M: GenericMap<Self::Output, Self::Output> 
    {
        // Filter out conflict binding tuple of outer and inner scope.
        for inner_key in inner.gkeys() {
            let key_root = inner_key.root();
            let inner_val = inner.gget(inner_key.borrow()).unwrap();
            if outer.contains_gkey(inner_key.borrow()) {
                let outer_val = outer.gget(inner_key.borrow()).unwrap();
                if inner_val != outer_val {
                    return true;
                }
            }

            // outer variable: x (won't be x.y...), inner variable: x.y.z...
            else if outer.contains_gkey(key_root.borrow()) {
                let outer_val = outer.gget(key_root.borrow()).unwrap();
                let outer_sub_val = outer_val.find_subterm(inner_key).unwrap();
                if inner_val != &outer_sub_val {
                    return true;
                }
            }
        }

        false
    }

    fn has_subterm(&self, term: &Self::Output) -> Option<bool> {
        match self.borrow() {
            Term::Variable(v1) => {
                match term.borrow() {
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

    fn fragments_difference(&self, term: &Self::Output) -> Option<Vec<String>> {
        match self.borrow() {
            Term::Variable(v1) => {
                let len1 = v1.fragments.len();
                match term.borrow() {
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