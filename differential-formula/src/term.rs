use crate::type_system::*;

use std::sync::Arc;
use std::vec::Vec;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fmt;
use std::fmt::{Debug, Display};
use std::string::String;

use enum_dispatch::enum_dispatch;
use abomonation::Abomonation;

use num::*;


#[enum_dispatch(Term)]
pub trait TermBehavior {
    fn variables(&self) -> HashSet<Term>;
    fn get_bindings(&self, term: &Term) -> Option<HashMap<Term, Term>>;
    fn is_groundterm(&self) -> bool;
    fn propagate_bindings(&self, map: &HashMap<Term, Term>) -> Term;
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Composite {
    pub sort: Arc<Type>,
    pub arguments: Vec<Arc<Term>>,
    pub alias: Option<String>
}

impl Abomonation for Composite {}

impl Display for Composite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut args = vec![];
        for arg in self.arguments.iter() {
            args.push(format!("{}", arg));
        }
        let args_str = args.join(",");
        write!(f, "{}({})", self.sort.name(), args_str)
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

    fn get_bindings(&self, term: &Term) -> Option<HashMap<Term, Term>> {
        let mut bindings = HashMap::new();
        let result: Result<Composite, _> = term.clone().try_into();
        if let Ok(c) = result {
            if self.sort == c.sort {
                for i in 0..self.arguments.len() {
                    let x = self.arguments.get(i).unwrap().as_ref();
                    let y = c.arguments.get(i).unwrap().as_ref();

                    match x {
                        Term::Atom(a) => {
                            // Immediately return None if both arguments of Atom type are not equal.
                            if x != y {
                                return None;
                            }
                        },
                        _ => {
                            let sub_bindings_option = x.get_bindings(&y);
                            match sub_bindings_option {
                                Some(mut sub_bindings) => {
                                    for (k, v) in sub_bindings.drain() {
                                        // Detect a variable binding conflict and return None immediately.
                                        if bindings.contains_key(&k) {
                                            if bindings.get(&k).unwrap() != &v {
                                                return None;
                                            }
                                        } else {
                                            bindings.insert(k, v);
                                        }
                                    }    
                                },
                                None => { 
                                    // Return None when no binding found for current argument.
                                    return None; 
                                }
                            }
                        }
                    }
                }
            } else {
                return None;
            }
        } 

        Some(bindings)
    }

    fn is_groundterm(&self) -> bool {
        for arg in self.arguments.iter() {
            if !arg.is_groundterm() {
                return false;
            }
        }
        true
    }

    fn propagate_bindings(&self, map: &HashMap<Term, Term>) -> Term {
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
}


#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Variable {
    pub var: String,
    pub fragments: Vec<String>
}

impl Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.var)
    }
}

impl Abomonation for Variable {}

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
       set.insert(self.clone().into());
       set
    }

    fn get_bindings(&self, term: &Term) -> Option<HashMap<Term, Term>> {
        let mut map = HashMap::new();
        map.insert(self.clone().into(), term.clone());
        Some(map)
    }

    fn is_groundterm(&self) -> bool {
        false
    }

    fn propagate_bindings(&self, map: &HashMap<Term, Term>) -> Term {
        let vterm: Term = self.clone().into();
        if map.contains_key(&vterm) {
            return map.get(&vterm).unwrap().clone();
        } else {
            return vterm;
        }
    }
}


#[enum_dispatch]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Atom {
    Int(BigInt),
    Str(String),
    Bool(bool),
    Float(BigRational),
}

impl Abomonation for Atom {}

impl Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let atomStr = match self {
            Atom::Int(i) => format!("{:?}", i),
            Atom::Bool(b) => format!("{:?}", b),
            Atom::Str(s) => format!("{:?}", s),
            Atom::Float(f) => format!("{:?}", f),
        };
        write!(f, "{}", atomStr)
    }
}


impl TermBehavior for Atom {
    fn variables(&self) -> HashSet<Term> {
        HashSet::new()
    }

    fn get_bindings(&self, term: &Term) -> Option<HashMap<Term, Term>> {
        None
    }

    fn is_groundterm(&self) -> bool {
        true
    }

    fn propagate_bindings(&self, map: &HashMap<Term, Term>) -> Term {
        self.clone().into()
    }
}

#[enum_dispatch]
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Term {
    Composite,
    Variable,
    Atom
}

impl Abomonation for Term {}

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
