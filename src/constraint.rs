extern crate rand;
extern crate timely;
extern crate differential_dataflow;
extern crate abomonation_derive;
extern crate abomonation;

use std::sync::Arc;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use enum_dispatch::enum_dispatch;
use num::*;

use crate::term::*;
use crate::expression::*;
use crate::type_system::*;
use crate::util::GenericMap;


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Predicate {
    pub negated: bool,
    pub term: Term,
    pub alias: Option<Term>, // Must be a variable term.
}

impl Display for Predicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut term_str = format!("{}", self.term);
        let alias_str = match &self.alias {
            Some(a) => {
                format!("{} is ", a)
            },
            None => "".to_string()
        };

        term_str = alias_str + &term_str;

        if self.negated {
            term_str = "no ".to_string() + &term_str;
        }
        write!(f, "{}", term_str)
    }
}


impl ConstraintBehavior for Predicate {
    fn variables(&self) -> HashSet<Term> {
        let mut var_set = HashSet::new();
        let vars = self.term.variables();
        for var in vars.into_iter() {
            var_set.insert(var);
        }
        
        if self.alias != None {
            var_set.insert(self.alias.clone().unwrap());
        }

        var_set
    }
}


impl Predicate {
    // Negative predicate constraint is count({setcompre}) = 0 in disguise.
    pub fn to_binary_constraints(&self, var: Term) -> Option<(Constraint, Constraint)> {
        // Positive predicate is not allowed to be converted into Binary constraint.
        if !self.negated {
            return None;
        }

        // Give it an alias starting with "~" that cannot be accepted by parser to make sure it
        // will never coincide with variables in current rule defined by user.
        let alias: Term = Variable::new("~dc".to_string(), vec![]).into();
        let vars = vec![alias.clone()];
        let pos_predicate = Predicate {
            negated: false,
            term: self.term.clone(),
            alias: Some(alias),
        };

        let setcompre = SetComprehension::new(
            vars, 
            vec![pos_predicate.into()],
            SetCompreOp::Count,
            BigInt::from_i64(0 as i64).unwrap(),
        );

        let var_base_expr: BaseExpr = var.into();
        let setcompre_base_expr: BaseExpr = setcompre.into();
        
        let binary = Binary {
            op: BinOp::Eq,
            left: var_base_expr.clone().into(),
            right: setcompre_base_expr.into(),
        };

        let big_zero = BigInt::from_i64(0 as i64).unwrap();
        let zero_term: Term = Atom::Int(big_zero).into();
        let zero_base_expr: BaseExpr = zero_term.into();
        let binary2 = Binary {
            op: BinOp::Eq,
            left: var_base_expr.into(),
            right: zero_base_expr.into(),
        };

        Some((binary.into(), binary2.into()))
    }
}


#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum BinOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            BinOp::Eq => "=",
            BinOp::Ne => "!=",
            BinOp::Lt => "<",
            BinOp::Le => "<=",
            BinOp::Gt => ">",
            BinOp::Ge => ">=",
        };
        write!(f, "{}", op_str)
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Binary {
    pub op: BinOp,
    pub left: Expr,
    pub right: Expr,
}

impl Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.op, self.right)
    }
}

impl ConstraintBehavior for Binary {
    fn variables(&self) -> HashSet<Term> {
        let mut set = HashSet::new();
        set.extend(self.left.variables());
        set.extend(self.right.variables());
        set
    }
}

impl Binary {
    pub fn has_set_comprehension(&self) -> bool {
        return self.left.has_set_comprehension() || self.right.has_set_comprehension(); 
    }

    pub fn evaluate<T>(&self, binding: &T) -> Option<bool> where T: GenericMap<Term, Term> {
        // Cannot not directly handle set comprehension in evaluation of binary constraint.
        if self.has_set_comprehension() { 
            return None; 
        }
        else {
            let lvalue = self.left.evaluate(binding).unwrap();
            let rvalue = self.right.evaluate(binding).unwrap();
            let satisfied = match self.op {
                BinOp::Eq => { if lvalue == rvalue { true } else { false } },
                BinOp::Ge => { if lvalue >= rvalue { true } else { false } },
                BinOp::Gt => { if lvalue >  rvalue { true } else { false } },
                BinOp::Le => { if lvalue <= rvalue { true } else { false } },
                BinOp::Lt => { if lvalue <  rvalue { true } else { false } },
                BinOp::Ne => { if lvalue != rvalue { true } else { false } },
            };

            Some(satisfied)
        }
    }
}



#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeConstraint {
    pub var: Term,
    pub sort: Arc<Type>,
}

impl Display for TypeConstraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.var, self.sort.name())
    }
}

impl ConstraintBehavior for TypeConstraint {
    fn variables(&self) -> HashSet<Term> {
        let mut set = HashSet::new();
        set.insert(self.var.clone());
        set
    }
}


#[enum_dispatch(Constraint)]
pub trait ConstraintBehavior {
    fn variables(&self) -> HashSet<Term>;
}


#[enum_dispatch]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Constraint {
    Predicate,
    Binary,
    TypeConstraint,
}

impl Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let con_str = match self {
            Constraint::Predicate(p) => { format!("{}", p) },
            Constraint::Binary(b) => { format!("{}", b) },
            Constraint::TypeConstraint(t) => { format!("{}", t) },
        };

        write!(f, "{}", con_str) 
    }
}


