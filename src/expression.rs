extern crate num;
extern crate rand;

use std::borrow::*;
use std::convert::TryInto;
use std::iter::*;
use std::sync::Arc;
use std::vec::Vec;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Display};
use std::string::String;

use enum_dispatch::enum_dispatch;
use num::*;

use crate::term::*;
use crate::rule::*;
use crate::constraint::*;
use crate::util::*;


pub trait FormulaExpr {
    // Return all variables in the expression.
    fn variables(&self) -> HashSet<Term>;
    /// Find a term with certain pattern in the expression and replace it with another term.
    fn replace(&mut self, pattern: &Term, replacement: &Term);
    /// Find set comprehension in the expression and replace it with a don't-care variable to 
    /// represent it. The method will return a hash map mapping don't-care variable term to set 
    /// comprehension and there is a counter that is used to generate variable name.
    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Term, SetComprehension>;
}

impl<T: FormulaExpr> FormulaExpr for Option<T> {
    fn variables(&self) -> HashSet<Term> {
        match self {
            Some(expr) => {
                expr.variables()
            },
            None => { HashSet::new() }
        }
    }

    fn replace(&mut self, pattern: &Term, replacement: &Term) {
        match self {
            Some(expr) => {
                expr.replace(pattern, replacement);
            },
            None => {},
        };
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Term, SetComprehension> {
        match self {
            Some(expr) => {
                return expr.replace_set_comprehension(generator);
            },
            None => { return HashMap::new(); },
        };
    }
}

impl<T: FormulaExpr> FormulaExpr for Vec<T> {
    fn variables(&self) -> HashSet<Term> {
        let mut vars = HashSet::new();
        for element in self.iter() {
            let sub_vars = element.variables();
            vars.extend(sub_vars);
        }
        vars
    }

    fn replace(&mut self, pattern: &Term, replacement: &Term) {
        for element in self.iter_mut() {
            element.replace(pattern, replacement);
        }
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Term, SetComprehension> {
        let mut map = HashMap::new();
        for element in self.iter_mut() {
            let sub_map = element.replace_set_comprehension(generator);
            map.extend(sub_map);
        }
        map
    }
}

#[readonly::make]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SetComprehension {
    pub vars: Vec<Term>,
    pub condition: Vec<Constraint>,
    pub op: SetCompreOp,
    pub default: BigInt,
}

impl FormulaExpr for SetComprehension {
    fn variables(&self) -> HashSet<Term> {
        let mut vars = self.vars.variables();
        vars.extend(self.condition.variables());
        vars
    }

    fn replace(&mut self, pattern: &Term, replacement: &Term) {
        self.vars.replace(pattern, replacement);
        self.condition.replace(pattern, replacement);
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Term, SetComprehension> {
        let var = generator.generate_dc_term();
        // Set comprehension may have set comprehension expression inside itself.
        // TODO: convert it to a rule and do some changes.
        self.condition.replace_set_comprehension(generator)
    }
}

// Turn SetComprehension into a headless rule.
impl From<SetComprehension> for Rule {
    fn from(setcompre: SetComprehension) -> Self {
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
    pub fn aggregate<'a, I, T>(&self, terms: T) -> BigInt 
    where 
        // `T` represents an iterator of a tuple that the first thing is a reference 
        // and the second thing is the count.
        T: Iterator<Item=&'a(&'a I, isize)>,
        // `I` represents a reference of Formula Term.
        I: Borrow<Term> + Sized + 'static, 
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
                    let term_ref: &Term = (**term).borrow();
                    match term_ref {
                        Term::Atom(atom) => {
                            match atom {
                                Atom::Int(i) => { 
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
                    let term_ref: &Term = (**term).borrow();
                    match term_ref {
                        Term::Atom(atom) => {
                            match atom {
                                Atom::Int(i) => { if i > &max { max = i.clone(); } },
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
                    let term_ref: &Term = (**term).borrow();
                    match term_ref {
                        Term::Atom(atom) => {
                            match atom {
                                Atom::Int(i) => { 
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
        let mut setcompre_str = format!("{}({{ {} | {} }})", self.op, head_str, body_str); 
        write!(f, "{}", setcompre_str)
    }
}

impl SetComprehension {
    pub fn new(vars: Vec<Term>, condition: Vec<Constraint>, op: SetCompreOp, default: BigInt) -> Self {
        SetComprehension {
            vars,
            condition,
            op,
            default,
        }
    }

    pub fn matched_variables(&self) -> HashSet<Term> {
        let rule: Rule = self.clone().into();
        rule.predicate_matched_variables()
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ArithmeticOp {
    Add,
    Min,
    Mul,
    Div,
}

impl Display for ArithmeticOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            ArithmeticOp::Add => "+",
            ArithmeticOp::Min => "-",
            ArithmeticOp::Mul => "*",
            ArithmeticOp::Div => "/",
        };
        write!(f, "{}", op_str)
    }
}

#[enum_dispatch(BaseExpr)]
pub trait BaseExprBehavior {}
impl BaseExprBehavior for SetComprehension {}
impl BaseExprBehavior for Term {}

#[enum_dispatch]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BaseExpr {
    SetComprehension,
    Term,
}

// TODO: put them separately into methods in the BaseExprBehavior trait.
impl ExprBehavior for BaseExpr {
    fn has_set_comprehension(&self) -> bool {
        let has_setcompre = match self {
            BaseExpr::SetComprehension(s) => true,
            _ => false,
        };

        has_setcompre
    }

    // A Expr could have multiple set comprehensions.
    fn set_comprehensions(&self) -> Vec<SetComprehension> {
        let mut setcompres = vec![];
        match self {
            BaseExpr::SetComprehension(s) => {
                setcompres.push(s.clone());
            },
            _ => {},
        };

        setcompres
    }

    fn evaluate<T>(&self, binding: &T) -> Option<BigInt> 
    where 
        T: GenericMap<Arc<Term>, Arc<Term>>,
    {
        match self {
            BaseExpr::Term(term) => {
                match term {
                    Term::Atom(atom) => {
                        // The expression is a term of integer type.
                        match atom {
                            Atom::Int(num) => {
                                return Some(num.clone());
                            },
                            _ => { return None; },
                        }
                    },
                    Term::Variable(variable) => {
                        // The expression is a variable and find the value in hash map by that variable
                        let root_var = term.root();
                        let val_term = match root_var == term {
                            true => { 
                                binding.gget(term).unwrap().clone() 
                            },
                            false => {
                                // x.y.z does not exist in the binding but x exists.
                                let val_term = binding.gget(root_var).unwrap();
                                let val_subterm = Term::find_subterm(val_term.clone(), term).unwrap().clone();
                                val_subterm
                            }
                        };

                        // val_term must be an atom term for arithmetic evaluation.
                        let val_term_ref: &Term = val_term.borrow();
                        match val_term_ref {
                            Term::Atom(atom) => {
                                match atom {
                                    Atom::Int(num) => {
                                        return Some(num.clone())
                                    },
                                    _ => { None }
                                }
                            },
                            _ => { None }
                        }
                    },
                    _ => { return None; }
                }

            },
            _ => { return None; } // Can't directly evaluate set comprehension.
        }
    }
}

impl Display for BaseExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BaseExpr::SetComprehension(s) => write!(f, "{}", s),
            BaseExpr::Term(t) => write!(f, "{}", t),
        }
    }
}

impl FormulaExpr for BaseExpr {
    fn variables(&self) -> HashSet<Term> {
        match self {
            BaseExpr::Term(t) => t.variables(),
            BaseExpr::SetComprehension(s) => s.variables(),
        }
    }

    fn replace(&mut self, pattern: &Term, replacement: &Term) {
        match self {
            BaseExpr::SetComprehension(s) => s.replace(pattern, replacement),
            BaseExpr::Term(t) => t.replace(pattern, replacement),
        };
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Term, SetComprehension> {
        let mut map = HashMap::new();
        match self {
            BaseExpr::SetComprehension(setcompre) => {
                // It won't return anything but do some conversion if setcompre has setcompre inside itself.
                setcompre.replace_set_comprehension(generator);
            },
            BaseExpr::Term(t) => {
                return map;
            },
        };
        let introduced_var = generator.generate_dc_term();
        let mut base_expr: BaseExpr = BaseExpr::Term(introduced_var.clone());
        std::mem::swap(self, &mut base_expr);
        map.insert(introduced_var, base_expr.try_into().unwrap()); 
        return map;
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArithExpr {
    pub op: ArithmeticOp,
    pub left: Arc<Expr>,
    pub right: Arc<Expr>,
}

impl FormulaExpr for ArithExpr {
    fn variables(&self) -> HashSet<Term> {
        let mut vars = HashSet::new();
        let left_vars = self.left.variables();
        let right_vars = self.right.variables();
        vars.extend(left_vars);
        vars.extend(right_vars);
        vars
    }

    fn replace(&mut self, pattern: &Term, replacement: &Term) {
        Arc::make_mut(&mut self.left).replace(pattern, replacement);
        Arc::make_mut(&mut self.right).replace(pattern, replacement);
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Term, SetComprehension> {
        let mut map = HashMap::new();
        let left_map = Arc::make_mut(&mut self.left).replace_set_comprehension(generator);
        let right_map = Arc::make_mut(&mut self.right).replace_set_comprehension(generator);
        map.extend(left_map);
        map.extend(right_map);
        map
    }
}

impl Display for ArithExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.left, self.op, self.right)
    }
}

impl ExprBehavior for ArithExpr {
    fn has_set_comprehension(&self) -> bool {
        return self.left.has_set_comprehension() || self.right.has_set_comprehension();
    }

    fn set_comprehensions(&self) -> Vec<SetComprehension> {
        let mut list = vec![];
        let mut left_vec = self.left.set_comprehensions();
        let mut right_vec = self.right.set_comprehensions();
        list.append(&mut left_vec);
        list.append(&mut right_vec);
        list
    }

    fn evaluate<T>(&self, binding: &T) -> Option<BigInt> 
    where 
        T: GenericMap<Arc<Term>, Arc<Term>>, 
    {
        let lvalue = self.left.evaluate(binding).unwrap();
        let rvalue = self.right.evaluate(binding).unwrap();
        let result = match self.op {
            ArithmeticOp::Add => { lvalue + rvalue },
            ArithmeticOp::Div => { lvalue / rvalue },
            ArithmeticOp::Min => { lvalue - rvalue },
            ArithmeticOp::Mul => { lvalue * rvalue },
        };

        Some(result)
    } 
}


#[enum_dispatch(Expr)]
pub trait ExprBehavior {
    fn has_set_comprehension(&self) -> bool;
    fn set_comprehensions(&self) -> Vec<SetComprehension>;
    fn evaluate<T>(&self, binding: &T) -> Option<BigInt> 
    where 
        T: GenericMap<Arc<Term>, Arc<Term>>,
    ;
}

#[enum_dispatch]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expr {
    BaseExpr,
    ArithExpr,
}

impl FormulaExpr for Expr {
    fn variables(&self) -> HashSet<Term> {
        match self {
            Expr::BaseExpr(b) => b.variables(),
            Expr::ArithExpr(a) => a.variables(),
        }
    }

    fn replace(&mut self, pattern: &Term, replacement: &Term) {
        match self {
            Expr::BaseExpr(b) => b.replace(pattern, replacement),
            Expr::ArithExpr(a) => a.replace(pattern, replacement),
        }
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Term, SetComprehension> {
        match self {
            Expr::BaseExpr(b) => { return b.replace_set_comprehension(generator); },
            Expr::ArithExpr(a) => { return a.replace_set_comprehension(generator); }
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::BaseExpr(b) => write!(f, "{}", b),
            Expr::ArithExpr(a) => write!(f, "{}", a),
        }
    }
}
