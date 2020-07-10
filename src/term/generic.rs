use std::borrow::*;
use std::cell::*;
use std::vec::Vec;
use std::collections::*;
use std::convert::*;
use std::fmt;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::string::String;
use std::hash::{Hash, Hasher};

use im::OrdSet;
use num::*;
use serde::{Serialize, Deserialize};

use super::atomic::*;
use crate::term::VisitTerm;
use crate::type_system::*;
use crate::expression::*;
use crate::util::*;
use crate::util::map::*;
use crate::util::wrapper::*;


/// This is a supertrait that implements both mutable and immutable borrow traits with some fundamental
/// derivable traits. I change it from generic typed trait BorrowedTerm<S, T> to trait with associated
/// types. Let's say a struct Foo implements the trait BorrowedTerm with two associated types X which
/// implements BorrowedType and Y which implements BorrowedTerm. The supertrait has some sub-traits 
/// like Borrow<B> a trait with generic type B and here the generic type B is replaced by Term<S, T>
/// that also has generic parameters. The Borrow<B> trait expects the type who implements it can be
/// converted to a reference &B or &Term<S, T> in our case but what's exactly S and T here if the trait
/// does not provide generic parameters? For each type who wants to implement BorrowedTerm with 
/// associated types, it only has one choice for its associated types because it's not trait with generic
/// parameters, then one trait can have many implementations for just one type since you can have many
/// different combinations of those generic parameters in the trait. Here S and T are decided by the 
/// associated types of the type that tries to implement BorrowedTerm as <Self as BorrowedTerm>::Output.
pub trait BorrowedTerm: 
    // For example, access a reference of Term<Arc<Type>, Arc<AtomicTerm>> from AtomicTerm, where
    // Arc<Type> and Arc<AtomicTerm> are the associated types in AtomicTerm.
    Borrow<Term<<Self as BorrowedTerm>::SortOutput, <Self as BorrowedTerm>::TermOutput>> + 
    // acess a mutable reference of Term<Arc<Type>, Arc<AtomicTerm>> from mutable AtomicTerm.
    BorrowMut<Term<<Self as BorrowedTerm>::SortOutput, <Self as BorrowedTerm>::TermOutput>> + 
    // A trait that enable conversion from Term<S, T> into AtomicTerm
    // From<Term<<Self as BorrowedTerm>::SortOutput, <Self as BorrowedTerm>::TermOutput>> + 
    // A trait that enable conversion from AtomicTerm::TermOutput which is Arc<AtomicTerm> back into AtomicTerm.
    From<<Self as BorrowedTerm>::TermOutput> +
    // Being able to convert from AtomicTerm, which is the default term.
    From<AtomicTerm> + 
    // Has an unique form as string to be accessed as reference and the term can be borrowd as string.
    UniqueForm<String> + Borrow<String> +
    // Some basic derivable traits.
    Eq + Hash + Clone + Ord + Debug + Display + differential_dataflow::ExchangeData
{
    type SortOutput: BorrowedType;
    type TermOutput: From<Self>; // Require Self::TermOutput to be able to convert back to Self.
}

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum AtomEnum {
    Int(BigInt),
    Str(String),
    Bool(bool),
    Float(BigRational),
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
pub struct Atom<S, T> where S: BorrowedType, T: BorrowedTerm {
    // The type of Atom term.
    pub sort: S,
    // The field that holds the data.
    pub val: AtomEnum,
    // Need this for the generic type T even though atom term does not hold any term.
    pub term: PhantomData<T>
}

// The hash only depends on the atom value and skip the type.
impl<S, T> Hash for Atom<S, T> where S: BorrowedType, T: BorrowedTerm {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.val.hash(state);
    }
}

impl<S, T> Display for Atom<S, T> where S: BorrowedType, T: BorrowedTerm {
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

/// A generic variable term that does not require a specific type of reference.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
pub struct Variable<S, T> where S: BorrowedType, T: BorrowedTerm {
    // If variable is inside a predicate, the type could be derived otherwise use Undefined.
    // A variable is meaningless without context.
    pub sort: S,
    // Make a variable term with no fragments for easy access of root term.
    pub root: String,
    // The remaining fragments of the variable term.
    pub fragments: Vec<String>,
    // Need this for the generic type T even though variable term does not hold any term.
    pub term: PhantomData<T>
}

/// Only use display name with root and fragments to compute hash in order to allow term
/// hash map to accept String as key too where Term: Borrow<String> and the hash value of 
/// both Term and String can return the same value without having to provide an owned value
/// of term when I want to check if the root term is also in the hash map.
impl<S, T> Hash for Variable<S, T> where S: BorrowedType, T: BorrowedTerm {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let display_name = format!("{}", self);
        display_name.hash(state);
    }
}

impl<S, T> Display for Variable<S, T> where S: BorrowedType, T: BorrowedTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fragments = self.fragments.join(".");
        if self.fragments.len() > 0 {
            write!(f, "{}.{}", self.root, fragments)
        } else {
            write!(f, "{}", self.root)
        }
    }
}

impl<S, T> Variable<S, T> where S: BorrowedType, T: BorrowedTerm {
    /// Create a new variable term given sort, root and fragments. The sort is optional because
    /// it's unknown without context unless you know it in advance and pass a sort as parameter.
    pub fn new(sort: Option<S>, root: String, fragments: Vec<String>) -> Self {
        let undefined_sort: S = Type::Undefined( Undefined{} ).into();
        let var_sort = match sort {
            Some(sort) => sort,
            None => undefined_sort,
        };

        let var = Variable {
            sort: var_sort,
            root,
            fragments,
            term: PhantomData
        };

        var
    }
}

// TODO: Exclude alias for auto-derived trait like Eq, Hash and Ord.
//
/// A generic Composite struct that does not require a specific smart pointer like 
/// `Rc<T>` or `Arc<T>` for its sort and arguments and of course you can use the native
/// data types without wrappers. Both sort and subterms has an unique form as string.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
pub struct Composite<S, T> where S: BorrowedType, T: BorrowedTerm {
    // The type of the composite term.
    pub sort: S,
    // A vector of terms as arguments.
    pub arguments: Vec<T>,
    // An optional alias for term.
    pub alias: Option<String>,
}

/// Use an unique form as string and use the string to compute hash, so string can be used
/// as the key in term hashmap in the same way as native term.
impl<S, T> Hash for Composite<S, T> where S: BorrowedType, T: BorrowedTerm {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sort.unique_form().hash(state);
        for arg in self.arguments.iter() {
            arg.unique_form().hash(state);
        }
    }
}

impl<S, T> Display for Composite<S, T> where S: BorrowedType, T: BorrowedTerm {
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
        let term_str = format!("{}({})", self.sort.borrow(), args_str);
        write!(f, "{}{}", alias_str, term_str)
    }
}

impl<S, T> Composite<S, T> where S: BorrowedType, T: BorrowedTerm {
    /// Create a new term given type, arguments and an optional alias.
    pub fn new(sort: S, arguments: Vec<T>, alias: Option<String>) -> Self {
        let composite = Composite {
            sort,
            arguments,
            alias,
        };
        return composite;
    }

    /// Validate if the term comforms to its type definition.
    pub fn validate(&self) -> bool {
        true
    }
}

// #[enum_dispatch]
// pub trait TermTrait {}
// impl<S, T> TermTrait for Atom<S, T> {}
// impl<S, T> TermTrait for Variable<S, T> {}
// impl<S, T> TermTrait for Composite<S, T> {}

/// A generic term struct Term<S, T> that has two generic parameters but you can't use the same 
/// Term<S, T> to replace parameter T in Term<S, T> because of endless recursion on generic params.
/// Term<S, T> is served as the standard form of terms that any term implementation should be able
/// to convert into the form of Term<S, T> with generic params replaced by some concrete type like
/// AtomicTerm or IndexedTerm.
// #[enum_dispatch(TermTrait)]
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Clone)]
pub enum Term<S, T> where S: BorrowedType, T: BorrowedTerm {
    Atom(Atom<S, T>),
    Variable(Variable<S, T>),
    Composite(Composite<S, T>)
}

// Put some term-related static methods here.
impl<S, T> Term<S, T> where S: BorrowedType, T: BorrowedTerm {
    /// Create a nullary composite type with no arguments inside
    /// and return the singleton term or constant in other words.
    pub fn create_constant(constant: String) -> Self {
        let nullary_type: Type = Type::CompositeType(
            CompositeType { name: constant, arguments: vec![] }
        );

        let composite = Composite::new(nullary_type.into(), vec![], None);
        let term: Term<S, T> = Term::Composite(composite);

        return term;
    }

    // Create a variable with undefined sort.
    pub fn create_variable(root: String) -> Self {
        let var = Variable::new(None, root, vec![]);
        Term::Variable(var)
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

impl<S, T> Display for Term<S, T> where S: BorrowedType, T: BorrowedTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let term_str = match self {
            Term::Composite(c) => format!("{}", c),
            Term::Variable(v) => format!("{}", v),
            Term::Atom(a) => format!("{}", a),
        };
        write!(f, "{}", term_str)
    }
}

impl<S, T> Debug for Term<S, T> where S: BorrowedType, T: BorrowedTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let term_str = match self {
            Term::Composite(c) => format!("{}", c),
            Term::Variable(v) => format!("{}", v),
            Term::Atom(a) => format!("{}", a),
        };
        write!(f, "{}", term_str)
    }
}

/// Any generic term is a Formula expression and we want to return all results as the same generic
/// term itself rather than its generic parameter T.
impl<T> Expression for T where T: BorrowedTerm
{   
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

/// Implement trait `VisitTerm` for any type that looks like a term that implements the `BorrowedTerm`
/// trait and the associated type `Output` is set to be the same type that implements the VisitTerm
/// trait. This trait is implemented for types like Term<S, T> and AtomicTerm.
impl<T> VisitTerm for T where T: BorrowedTerm
{
    // The associated type Output is the same as T itself.
    type Output = T;

    fn traverse<F1, F2>(&self, pattern: &F1, logic: &F2)
    where 
        F1: Fn(&Self::Output) -> bool, 
        F2: Fn(&Self::Output)
    {
        // Need to convert T into its associated term T::TermOutput. 
        // For example, conversion from AtomicTerm into Arc<AtomicTerm> with a little clone here.
        if pattern(self) {
            logic(self);
        }

        // Recursively match all arguments in the composite term even the term is already matched.
        // For example: List ::= new (content: Integer, next: List + {NIL}). We can write a pattern 
        // like List(a, b) that not only match List(1, List(2, NIL)) but also match its child List(2, NIL).
        match self.borrow() {
            Term::Composite(c) => {
                for arg in c.arguments.iter() {
                    // Convert from T::Output = Arc<AtomicTerm> back to T = AtomicTerm and a clone on
                    // Arc<SomeType> is fine but the convertion from Arc<SomeType> may copy the inner value.
                    let argt: Self::Output = arg.clone().into();
                    argt.traverse(pattern, logic);
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
                // Convert T back into T::TermOutput.
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
                let var_clone = var.clone();
                let var_ref: &Term<_,_> = var_clone.borrow();
                if !var.is_dc_variable() {
                    let p = match vmap.contains_key(var_ref) {
                        true => {
                            vmap.get(var_ref).unwrap().clone()
                        },
                        false => {
                            let dc_name = generator.generate_name();
                            let dc_var = AtomicTerm::create_variable(dc_name, vec![]);
                            let p: T = dc_var.into();
                            p
                        }
                    };
                    vmap.insert(p.clone(), var_clone);
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
        let self_term: &Term<_,_> = self.borrow();
        match self_term {
            Term::Atom(sa) => { false }, // Atom cannot be a pattern.
            Term::Variable(sv) => { 
                // Detect a conflict in variable binding and return false.
                if binding.contains_gkey(self_term) && 
                   binding.gget(self_term).unwrap() != term {
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
                let term_ref: &Term<_,_> = term.borrow();
                if map.contains_gkey(term_ref) || 
                   map.contains_gkey(term.var_root().unwrap()) { return true; } 
                else { return false; }
            },
            &mut |mut term| {
                // Make an immutable clone here.
                let term_ref: &Term<_,_> = term.clone().borrow();
                if map.contains_gkey(term_ref) {
                    let replacement = map.gget(term_ref).unwrap();
                    *term = replacement.clone();
                } else {
                    // The term here must be a variable term and have fragments like inside A(x.id, y.name).
                    // Dig into the root term to find the subterm by labels. 
                    let root = term.var_root().unwrap();
                    let root_term = map.gget(root).unwrap();
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
                                        let type_ref: &Type = t.borrow();
                                        let new_ctype = type_ref.base_type();
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
        let var_ref = self.borrow();
        match var_ref {
            Term::Variable(_) => {
                // Let's say `var` is `x.y.z` and the binding does not have root term of `x` as key 
                // but has some subterms of root term like `x.y` as key, then we only need to find
                // the subterm from `x.y` by looking up label `z`. Traverse the keys and find the 
                // first one that `var` is its subterm.
                for key in binding.gkeys() {
                    if key.has_subterm(self).unwrap() {
                        let key_term: &Term<_,_> = key.borrow();
                        let value = binding.gget(key_term).unwrap();
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

    fn rename(&self, scope: String, type_map: &HashMap<String, <Self::Output as BorrowedTerm>::SortOutput>) -> Self {
        // Make a copy of itself and apply a mutable rename.
        let mut self_clone = self.clone();
        self_clone.rename_mut(scope, type_map);
        return self_clone;
    }

    fn rename_mut(&mut self, scope: String, type_map: &HashMap<String, <Self::Output as BorrowedTerm>::SortOutput>) {
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
                        // Assuming that the renamed type is already in type map otherwise panic.
                        let new_type_name = format!("{}.{}", scope, c.sort);
                        let new_type = type_map.get(&new_type_name).unwrap().clone();
                        c.sort = new_type.into();
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

    // fn root(&self) -> &String {
    //     match self.borrow() {
    //         Term::Variable(v) => {
    //             match &v.root_term {
    //                 Some(term_ref) => { 
    //                     let term: T = term_ref.; 
    //                     term
    //                 },
    //                 None => { self }
    //             }
    //         },
    //         _ => { self }
    //     }
    // }

    fn var_root(&self) -> Option<&String> {
        match self.borrow() {
            Term::Variable(v) => {
                Some(&v.root)
            },
            _ => None
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

    fn intersect(&self, other: &Self::Output) -> (HashSet<Self::Output>, HashSet<Self::Output>, HashSet<Self::Output>) {
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
            let inner_key_term: &Term<_,_> = inner_key.borrow();
            let key_root = inner_key.var_root().unwrap();
            let inner_val = inner.gget(inner_key_term).unwrap();

            if outer.contains_gkey(inner_key_term) {
                let outer_val = outer.gget(inner_key_term).unwrap();
                if inner_val != outer_val {
                    return true;
                }
            }
            // outer variable: x (won't be x.y...), inner variable: x.y.z...
            else if outer.contains_gkey(key_root) {
                let outer_val = outer.gget(key_root).unwrap();
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