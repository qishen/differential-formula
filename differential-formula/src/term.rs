use std::sync::Arc;
use std::vec::Vec;
use std::collections::*;
use std::convert::TryInto;
use std::fmt;
use std::fmt::{Debug, Display};
use std::string::String;

use enum_dispatch::enum_dispatch;
use im::OrdMap;
use num::*;
use serde::{Serialize, Deserialize};

use crate::type_system::*;
use crate::util::GenericMap;


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

    pub fn fragments_diff(a: &Term, b: &Term) -> Option<Vec<String>> {
        let av: Variable = a.clone().try_into().unwrap();
        let bv: Variable = b.clone().try_into().unwrap();
        // Root vars have to be equal and b has longer fragments than a or same length.
        if av.var != bv.var || av.fragments.len() > bv.fragments.len() {
            None
        }
        else {
            let mut remains = bv.fragments;
            for aval in av.fragments {
                let bval = remains.remove(0);
                if aval != bval {
                    return None;
                }
            }
            Some(remains)
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

    // Check if two binding map has conflits in variable mappings.
    pub fn has_conflit<T>(outer: &T, inner: &T) -> bool 
    where T: GenericMap<Term, Term>
    {
        // Filter out conflict binding tuple of outer and inner scope.
        for inner_key in inner.keys() {
            let var: Variable = inner_key.clone().try_into().unwrap();
            let key_root = inner_key.root_var();
            let inner_val = inner.get(inner_key).unwrap();
            if outer.contains_key(inner_key) {
                let outer_val = outer.get(inner_key).unwrap();
                if inner_val != outer_val {
                    return true;
                }
            }
            // outer variable: x (won't be x.y...), inner variable: x.y.z...
            else if outer.contains_key(&key_root) {
                let labels = Variable::fragments_diff(&key_root, inner_key).unwrap();
                let outer_val = outer.get(&key_root).unwrap();
                let outer_sub_val = outer_val.get_subterm_by_labels(&labels).unwrap();
                if inner_val != &outer_sub_val {
                    return true;
                }
            }
        }

        false
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
