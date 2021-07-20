use std::borrow::*;
use std::iter::*;
use std::vec::Vec;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Display};
use std::string::String;

use num::*;

use crate::expression::*;
use crate::term::*;
use crate::type_system::*;
use crate::rule::*;
use crate::constraint::Constraint;
use crate::util::*;


#[readonly::make]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SetComprehension {
    pub vars: Vec<AtomicTerm>,
    pub condition: Vec<Constraint>,
    pub op: SetCompreOp,
    pub default: BigInt,
}

impl Display for SetComprehension {
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
        let setcompre_str = format!("{}({{ {} | {} }})", self.op, head_str, body_str); 
        write!(f, "{}", setcompre_str)
    }
}

impl SetComprehension {
    pub fn new(vars: Vec<AtomicTerm>, condition: Vec<Constraint>, op: SetCompreOp, default: BigInt) -> Self {
        SetComprehension {
            vars,
            condition,
            op,
            default,
        }
    }

    pub fn matched_variables(&self) -> HashSet<AtomicTerm> {
        // Convert it into a headless rule to use some rule methods.
        let rule: Rule = self.clone().into();
        rule.predicate_matched_variables()
    }
}

impl BasicExprOps for SetComprehension {
    fn variables(&self) -> HashSet<AtomicTerm> {
        // let mut vars = self.vars.variables();
        // vars.extend(self.condition.variables());
        // vars
        unimplemented!()
    }

    fn replace_pattern(&mut self, pattern: &AtomicTerm, replacement: &AtomicTerm) {
        // self.vars.replace_pattern(pattern, replacement);
        // self.condition.replace_pattern(pattern, replacement);
        unimplemented!()
    }
}

impl SetCompreOps for SetComprehension {
    fn has_set_comprehension(&self) -> bool {
        true
    }

    fn set_comprehensions(&self) -> Vec<&SetComprehension> {
        vec![self]
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) 
    -> HashMap<AtomicTerm, SetComprehension> {
        // let dc_name = generator.generate_name();
        // let var = generator.generate_dc_term();
        // Recursively replace set comprehension inside the condition of current set comprehension.
        self.condition.replace_set_comprehension(generator)
    }

}

// Turn SetComprehension into a headless rule.
impl From<SetComprehension> for Rule {
    fn from(setcompre: SetComprehension) -> Self {
        Rule::new(setcompre.vars, setcompre.condition.clone())
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
    pub fn aggregate<'a, I, T>(&self, terms: I) -> BigInt 
    where 
        // `T` represents an iterator of a tuple that the first thing is a reference and the second thing is the count.
        I: Iterator<Item=&'a(&'a T, isize)>,
        T: 'a + TermStructure, 
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
                // let mut sum = BigInt::from_i64(0).unwrap();
                // for (term, count) in terms {
                //     let atom_enum = term.into_atom_enum().unwrap();
                //     match atom_enum {
                //         AtomEnum::Int(i) => { sum += i.clone() * count },
                //         _ => {}
                //     }
                // }
                // sum
                todo!()
            },
            SetCompreOp::MaxAll => {
                // let mut max = BigInt::from_i64(std::isize::MIN as i64).unwrap();
                // for (term, count) in terms {
                //     let atom_enum = term.into_atom_enum().unwrap();
                //     match atom_enum {
                //         AtomEnum::Int(i) => { if i > max { max = i.clone(); } },
                //         _ => {}
                //     }
                // }
                // max
                todo!()
            },
            //SetCompreOp::MinAll => {
            _ => {
                // let mut min = BigInt::from_i64(std::isize::MAX as i64).unwrap();
                // for (term, count) in terms {
                //     let atom_enum = term.into_atom_enum().unwrap();
                //     match atom_enum {
                //         AtomEnum::Int(i) => { 
                //             if i < min { min = i.clone(); } 
                //         },
                //         _ => {}
                //     }
                // }
                // min
                todo!()
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