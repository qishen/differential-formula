use crate::type_system::*;

use core::hash::Hash;
use core::borrow::Borrow;
use std::sync::Arc;
use std::vec::Vec;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fmt;
use std::fmt::{Debug, Display};
use std::string::String;

use enum_dispatch::enum_dispatch;
use abomonation::Abomonation;

use im::OrdMap;
use num::*;
use serde::{Serialize, Deserialize};

/*
Rust standard lib does not provide a generic map interface like C# does after v1.0 because 
it's user's responsibility to define what is a generic map and what functions should be included
since different users have different needs for generic map interface. Zhong Kou Nan Tiao.
*/
pub trait GenericMap<K, V> {
    fn contains_key<Q>(&self, k: &Q) -> bool 
    where
        K: Borrow<Q>,
        Q: Hash + Eq;

    fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq;

    fn insert(&self, k: K, v: V) -> Option<V>;
}

impl<K, V> GenericMap<K, V> for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.contains_key(k)
    }

    fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.get(k)
    }

    fn insert(&self, k: K, v: V) -> Option<V> {
        self.insert(k, v)
    }
}

impl<K, V> GenericMap<K, V> for OrdMap<K, V>
where
    K: Eq + Hash,
{
    fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.contains_key(k)
    }

    fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.get(k)
    }

    fn insert(&self, k: K, v: V) -> Option<V> {
        self.insert(k, v)
    }
}


#[enum_dispatch(Term)]
pub trait TermBehavior {
    fn variables(&self) -> HashSet<Term>;
    fn get_bindings_in_place<T>(&self, binding: &mut T, term: &Term) -> bool where T: GenericMap<Term, Term>; 
    fn get_bindings(&self, term: &Term) -> Option<HashMap<Term, Term>>;
    fn get_ordered_bindings(&self, term: &Term) -> Option<OrdMap<Term, Term>>;
    fn is_groundterm(&self) -> bool;
    // Use GenericMap trait to make function accept different map implementations.
    fn propagate_bindings<T: GenericMap<Term, Term>>(&self, map: &T) -> Term;
    // Check if the term is a don't-care variable with variable root name as '_'.
    fn is_dc_variable(&self) -> bool; 
    fn root_var(&self) -> Term;
    fn get_subterm_by_label(&self, label: &String) -> Option<Term>;
    fn get_subterm_by_labels(&self, labels: &Vec<String>) -> Option<Term>;
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Composite {
    pub sort: Arc<Type>,
    pub arguments: Vec<Arc<Term>>,
    pub alias: Option<String>
}

//impl Abomonation for Composite {}

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

    pub fn variables_intersection(&self, another: Composite) -> (Vec<Term>, Vec<Term>, Vec<Term>) {
        let mut a = self.variables();
        let mut b = another.variables();
        Term::terms_intersection(a, b)
    }

    pub fn validate(&self) -> bool {
        true
    }

}

impl TermBehavior for Composite {
    fn variables(&self) -> HashSet<Term> {
        let mut set  = HashSet::new();
        for arg in self.arguments.iter() {
            let arg_vars = arg.variables();
            for var in arg_vars.into_iter() {
                set.insert(var);
            }
        }
        set
    }

    fn get_bindings_in_place<T>(&self, binding: &mut T, term: &Term) -> bool 
    where
        T: GenericMap<Term, Term>
    {
        match term {
            Term::Composite(c) => {
                if self.sort == c.sort {
                    for i in 0..self.arguments.len() {
                        let x = self.arguments.get(i).unwrap().as_ref();
                        let y = c.arguments.get(i).unwrap().as_ref();

                        match x {
                            Term::Atom(a) => {
                                // Immediately return false if both Atom arguments are not equal.
                                if x != y {
                                    return false;
                                }
                            },
                            _ => {
                                // HashMap is faster than OrdMap in general.
                                let mut sub_binding = HashMap::new();
                                let has_binding = x.get_bindings_in_place(&mut sub_binding, &y);
                                if has_binding {
                                    for (k, v) in sub_binding.drain() {
                                        // Detect a variable binding conflict and return false immediately.
                                        if binding.contains_key(&k) {
                                            if binding.get(&k).unwrap() != &v {
                                                return false;
                                            }
                                        } else {
                                            binding.insert(k, v);
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
    }


    fn get_bindings(&self, term: &Term) -> Option<HashMap<Term, Term>> {
        let mut bindings = HashMap::new();
        let has_binding = self.get_bindings_in_place(&mut bindings, term);
        if has_binding {
            Some(bindings)
        } else {
            None
        }
    }

    fn get_ordered_bindings(&self, term: &Term) -> Option<OrdMap<Term, Term>> {
        let mut bindings = OrdMap::new();
        let has_binding = self.get_bindings_in_place(&mut bindings, term);
        if has_binding {
            Some(bindings)
        } else {
            None
        }
    }

    fn is_groundterm(&self) -> bool {
        for arg in self.arguments.iter() {
            if !arg.is_groundterm() {
                return false;
            }
        }
        true
    }

    fn propagate_bindings<T: GenericMap<Term, Term>>(&self, map: &T) -> Term {
        let mut composite = self.clone();
        for i in 0..composite.arguments.len() {
            let arg = composite.arguments.get(i).unwrap();
            if map.contains_key(arg) {
                let replacement = map.get(arg).unwrap();
                composite.arguments[i] = Arc::new(replacement.clone());
            } else {
                let term = arg.propagate_bindings(map);
                composite.arguments[i] = Arc::new(term);
            }
        }
        composite.into()
    }

    fn is_dc_variable(&self) -> bool { false }

    fn root_var(&self) -> Term {
        self.clone().into()
    }

    fn get_subterm_by_label(&self, label: &String) -> Option<Term> {
        let sort: CompositeType = self.sort.as_ref().clone().try_into().unwrap();
        for (i, (label_opt, t)) in sort.arguments.iter().enumerate() {
            match label_opt {
                Some(l) => {
                    if label == l {
                        let term = self.arguments.get(i).unwrap().as_ref().clone();
                        return Some(term);
                    }
                },
                None => {},
            }
        }

        None
    }

    fn get_subterm_by_labels(&self, labels: &Vec<String>) -> Option<Term> {
        let mut term: Term = self.clone().into();
        for fragment in labels {
            let term_opt = term.get_subterm_by_label(fragment);
            match term_opt {
                None => { return None; },
                Some(t) => { 
                    term = t; 
                }
            }
        }

        Some(term)
    }
}


#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Variable {
    pub var: String,
    pub fragments: Vec<String>
}

impl Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rest = self.fragments.join(".");
        if self.fragments.len() > 0 {
            rest = ".".to_string() + &rest[..]; 
        }
        write!(f, "{}{}", self.var, rest)
    }
}

//impl Abomonation for Variable {}

impl Variable {
    pub fn new(var: String, fragments: Vec<String>) -> Self {
        Variable {
            var,
            fragments,
        }
    }
}


impl TermBehavior for Variable {
    fn variables(&self) -> HashSet<Term> {
        let mut set = HashSet::new();
        if !self.is_dc_variable() {
            set.insert(self.clone().into());
        }

        set
    }

    fn get_bindings_in_place<T>(&self, binding: &mut T, term: &Term) -> bool 
    where T: GenericMap<Term, Term>
    {
        // Skip the variable if it is an underscore but still return true for empty map.
        if !self.is_dc_variable() {
            binding.insert(self.clone().into(), term.clone());
        }
        true
    } 

    fn get_bindings(&self, term: &Term) -> Option<HashMap<Term, Term>> {
        let mut map = HashMap::new();
        // Don't need to check if binding exists because it's always true.
        self.get_bindings_in_place(&mut map, term);
        Some(map)
    }

    fn get_ordered_bindings(&self, term: &Term) -> Option<OrdMap<Term, Term>> {
        let mut map = OrdMap::new();
        // Don't need to check if binding exists because it's always true.
        self.get_bindings_in_place(&mut map, term);
        Some(map)
    }

    fn is_groundterm(&self) -> bool {
        false
    }

    fn propagate_bindings<T: GenericMap<Term, Term>>(&self, map: &T) -> Term {
        let root = self.root_var();
        let vterm: Term = self.clone().into();
        if map.contains_key(&vterm) {
            // Fina an exact match in hash map and return its value.
            map.get(&vterm).unwrap().clone()
        } else if map.contains_key(&root) {
            // Dig into the root term to find the subterm by labels. 
            map.get(&root).unwrap().get_subterm_by_labels(&self.fragments).unwrap()
        } else {
            // No match and just return variable itself.
            vterm
        }
    }

    fn is_dc_variable(&self) -> bool {
        if self.var == "_" { true }
        else { false }
    }

    fn root_var(&self) -> Term {
        Variable::new(self.var.clone(), vec![]).into()
    }


    fn get_subterm_by_label(&self, label: &String) -> Option<Term> {
        None
    }

    fn get_subterm_by_labels(&self, labels: &Vec<String>) -> Option<Term> {
        None
    }
}


#[enum_dispatch]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Atom {
    Int(BigInt),
    Str(String),
    Bool(bool),
    Float(BigRational),
}

//impl Abomonation for Atom {}

impl Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let atomStr = match self {
            Atom::Int(i) => format!("{}", i),
            Atom::Bool(b) => format!("{:?}", b),
            Atom::Str(s) => format!("{:?}", s),
            Atom::Float(f) => format!("{}", f),
        };
        write!(f, "{}", atomStr)
    }
}


impl TermBehavior for Atom {
    fn variables(&self) -> HashSet<Term> {
        HashSet::new()
    }

    fn get_bindings_in_place<T>(&self, binding: &mut T, term: &Term) -> bool
    where T: GenericMap<Term, Term>
    {
        false
    }

    fn get_bindings(&self, term: &Term) -> Option<HashMap<Term, Term>> {
        None
    }

    fn get_ordered_bindings(&self, term: &Term) -> Option<OrdMap<Term, Term>> {
        None
    }

    fn is_groundterm(&self) -> bool {
        true
    }

    fn propagate_bindings<T: GenericMap<Term, Term>>(&self, map: &T) -> Term {
        self.clone().into()
    }

    fn is_dc_variable(&self) -> bool { false }

    fn root_var(&self) -> Term {
        self.clone().into()
    }

    fn get_subterm_by_label(&self, label: &String) -> Option<Term> {
        None
    }

    fn get_subterm_by_labels(&self, labels: &Vec<String>) -> Option<Term> {
        None
    }
}

#[enum_dispatch]
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Term {
    Composite,
    Variable,
    Atom
}


//impl Abomonation for Term {}

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



impl Term {
    pub fn terms_intersection(mut a: HashSet<Term>, mut b: HashSet<Term>) -> (Vec<Term>, Vec<Term>, Vec<Term>) {
        let mut intersection = Vec::new();
        let mut av = Vec::new();
        let mut bv = Vec::new();

        for term in a.drain() {
            if b.contains(&term) {
                b.remove(&term);
                intersection.push(term);
            } else {
                av.push(term);
            }
        }

        for term in b.drain() {
            bv.push(term);
        }

        av.sort();
        intersection.sort();
        bv.sort();

        (av, intersection, bv)
    }
}

#[macro_export]
macro_rules! atom {
    ($value:ident) => {
        if let Some(i) = (&$value as &Any).downcast_ref::<isize>() {
            let aterm: Term = Atom::Int(i).into();
            aterm
        } else if let Some(b) = (&$value as &Any).downcast_ref::<bool>() {
            let aterm: Term = Atom::Bool(b).into();
            aterm
        } else if let Some(s) = (&$value as &Any).downcast_ref::<String>() {
            let aterm: Term = Atom::Str(s).into();
            aterm
        }

        None
    };
}


#[macro_export]
macro_rules! variable {
    ($var:ident.$($fragment:ident).*) => {
        {
            let mut vlist = Vec::new();
            $(
                vlist.push(stringify!($fragment).to_string());
            )*
            
            let v = Variable {
                var: stringify!($var).to_string(),
                fragments: vlist, 
            };

            let vterm: Term = v.into();
            vterm
        }
    };
}


#[macro_export]
macro_rules! composite {
    ($sort:expr, $args:expr, $alias:ident) => {
        Composite {
            sort: $sort,
            arguments: $args,
            alias: Some(stringify!($alias).to_string()),
        }.into()
    };

    ($alias:ident is $sort:expr => ($($arg:expr),+)) => {
        {
            // TODO: arg could be either Arc<Term> or just Term.
            let mut alias = None;
            let alias_str = stringify!($alias).to_string(); 

            if alias_str != "None".to_string() {
                alias = Some(alias_str);
            }

            let c = Composite {
                sort: $sort,
                arguments: vec![$(
                    Arc::new($arg)
                ),+],
                alias: alias,
            };

            let cterm: Term = c.into();
            cterm
        }
    };

    ($sort:expr => ($($arg:expr),+)) => {
        composite!{ None is $sort($($arg),+) }
    };
}
