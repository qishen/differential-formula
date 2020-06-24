use std::borrow::*;
use std::iter::*;
use std::vec::Vec;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Display};
use std::string::String;

use num::*;

use crate::expression::FormulaExprTrait;
use crate::term::*;
use crate::type_system::*;
use crate::rule::*;
use crate::constraint::Constraint;
use crate::util::*;


#[readonly::make]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SetComprehension<S, T> where S: BorrowedType, T: BorrowedTerm<S, T> {
    pub vars: Vec<Term<S, T>>,
    pub condition: Vec<Constraint<S, T>>,
    pub op: SetCompreOp,
    pub default: BigInt,
}

impl<S, T> Display for SetComprehension<S, T> where S: BorrowedType, T: BorrowedTerm<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let headterm_strs: Vec<String> = self.vars.iter().map(|x| {
            let term_str = format!("{}", x);
            term_str
        }).collect();

        let constraint_strs: Vec<String> = self.condition.iter().map(|x| {
            let con_str = format!("{}", x);
            con_str
        }).collect();

        let head_str = headterm_strs.join(", ");
        let body_str = constraint_strs.join(", ");
        let mut setcompre_str = format!("{}({{ {} | {} }})", self.op, head_str, body_str); 
        write!(f, "{}", setcompre_str)
    }
}

impl<S, T> SetComprehension<S, T> where S: BorrowedType, T: BorrowedTerm<S, T> {
    pub fn new(vars: Vec<Term<S, T>>, condition: Vec<Constraint<S, T>>, op: SetCompreOp, default: BigInt) -> Self {
        SetComprehension {
            vars,
            condition,
            op,
            default,
        }
    }

    pub fn matched_variables(&self) -> HashSet<T> {
        // Convert it into a headless rule to use some rule methods.
        let rule: Rule<S, T> = self.clone().into();
        rule.predicate_matched_variables()
    }
}

impl<S, T> FormulaExprTrait for SetComprehension<S, T> where S: BorrowedType, T: BorrowedTerm<S, T> {

    type SortOutput = S;
    type TermOutput = T;

    fn variables(&self) -> HashSet<Self::TermOutput> {
        let mut vars = self.vars.variables();
        vars.extend(self.condition.variables());
        vars
    }

    fn replace_pattern(&mut self, pattern: &Self::TermOutput, replacement: &Self::TermOutput) {
        self.vars.replace_pattern(pattern, replacement);
        self.condition.replace_pattern(pattern, replacement);
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) 
    -> HashMap<Self::TermOutput, SetComprehension<Self::SortOutput, Self::TermOutput>> 
    {
        // let dc_name = generator.generate_name();
        // let var = generator.generate_dc_term();
        // Set comprehension may have set comprehension expression inside itself.
        // TODO: convert it to a rule and do some changes.
        self.condition.replace_set_comprehension(generator)
    }
}

// Turn SetComprehension into a headless rule.
impl<S, T> From<SetComprehension<S, T>> for Rule<S, T> where S: BorrowedType, T: BorrowedTerm<S, T> {
    fn from(setcompre: SetComprehension<S, T>) -> Self {
        Rule::new(vec![], setcompre.condition.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SetCompreOp {
    Sum,
    Count,
    MinAll,
    MaxAll,
    TopK,
    BottomK,
}

impl Display for SetCompreOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            SetCompreOp::Sum => { "sum" },
            SetCompreOp::Count => { "count" },
            SetCompreOp::MinAll => { "minAll" },
            SetCompreOp::MaxAll => { "maxAll" },
            SetCompreOp::TopK => { "topK" },
            SetCompreOp::BottomK => { "bottomK" },
        };

        write!(f, "{}", op_str)
    }
}

impl SetCompreOp {
    pub fn aggregate<'a, I, S, T>(&self, terms: I) -> BigInt 
    where 
        // `T` represents an iterator of a tuple that the first thing is a reference 
        // and the second thing is the count.
        I: Iterator<Item=&'a(&'a T, isize)>,
        S: BorrowedType,
        T: BorrowedTerm<S, T> + Sized + 'static, 
    {
        match self {
            SetCompreOp::Count => {
                let mut num = BigInt::from_i64(0 as i64).unwrap();
                for (term, count) in terms {
                    num += count.clone() as i64;
                }                        
                num
            },
            SetCompreOp::Sum => {
                let mut sum = BigInt::from_i64(0).unwrap();
                for (term, count) in terms {
                    let term_ref: &Term<S, T> = (**term).borrow();
                    match term_ref {
                        Term::Atom(atom) => {
                            match &atom.val {
                                AtomEnum::Int(i) => { 
                                    sum += i.clone() * count;
                                },
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                }
                sum
            },
            SetCompreOp::MaxAll => {
                let mut max = BigInt::from_i64(std::isize::MIN as i64).unwrap();
                for (term, count) in terms {
                    let term_ref: &Term<S, T> = (**term).borrow();
                    match term_ref {
                        Term::Atom(atom) => {
                            match &atom.val {
                                AtomEnum::Int(i) => { if i > &max { max = i.clone(); } },
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                }
                max
            },
            //SetCompreOp::MinAll => {
            _ => {
                let mut min = BigInt::from_i64(std::isize::MAX as i64).unwrap();
                for (term, count) in terms {
                    let term_ref: &Term<S, T> = (**term).borrow();
                    match term_ref {
                        Term::Atom(atom) => {
                            match &atom.val {
                                AtomEnum::Int(i) => { 
                                    if i < &min { min = i.clone(); } 
                                },
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                }
                min
            },
            /*
            SetCompreOp::TopK => {
                let k = setcompre_default.clone();
                let mut max_heap = BinaryHeap::new();
                for (term, count) in terms {
                    let term_ref: &Term = term.borrow();
                    match term_ref {
                        Term::Atom(atom) => {
                            match atom {
                                Atom::Int(i) => { 
                                    max_heap.push(i.clone());
                                },
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                }

                let mut topk = vec![];
                for i in num_iter::range(BigInt::zero(), k) {
                    if !max_heap.is_empty() {
                        topk.push(max_heap.pop().unwrap());
                    }
                }
                topk
            },
            _ => {
                let k = setcompre_default.clone();
                let mut min_heap = BinaryHeap::new();
                for (term, count) in terms {
                    let term_ref: &Term = term.borrow();
                    match term_ref {
                        Term::Atom(atom) => {
                            match atom {
                                Atom::Int(i) => { 
                                    min_heap.push(Reverse(i.clone()));
                                },
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                }

                let mut bottomk = vec![];
                for i in num_iter::range(BigInt::zero(), k) {
                    if !min_heap.is_empty() {
                        let r = min_heap.pop().unwrap().0;
                        bottomk.push(r);
                    }
                }
                bottomk
            }
            */
        }
    }
}