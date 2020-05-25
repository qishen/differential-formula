extern crate rand;
extern crate abomonation_derive;
extern crate abomonation;

use std::borrow::Borrow;
use std::sync::Arc;
use std::collections::*;
use std::fmt;
use std::fmt::{Debug, Display};

use enum_dispatch::enum_dispatch;
use num::*;

use crate::term::*;
use crate::expression::*;
use crate::type_system::*;
use crate::util::*;


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Predicate {
    pub negated: bool,
    pub term: Term,
    pub alias: Option<Term>, // Must be a variable term.
}

impl FormulaExpr for Predicate {
    fn variables(&self) -> HashSet<Term> {
        let mut var_set = HashSet::new();
        var_set.extend(self.term.variables());
        // Don't forget to add alias to variable set.
        if let Some(var) = self.alias.clone() {
            var_set.insert(var);
        }
        var_set
    }

    fn replace(&mut self, pattern: &Term, replacement: &Term) {
        self.term.replace(pattern, replacement);
        FormulaExpr::replace(&mut self.alias, pattern, replacement);
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Term, SetComprehension> {
        HashMap::new()
    }
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


impl ConstraintBehavior for Predicate {}


impl Predicate {
    // Negative predicate constraint is count({setcompre}) = 0 in disguise.
    pub fn convert_negation(&self, var: Term) -> Option<(Constraint, Constraint)> {
        // Positive predicate is not allowed to be converted into Binary constraint.
        if !self.negated {
            return None;
        }

        // Give it an alias starting with "~" that cannot be accepted by parser to avoid 
        // collision with other user-defined variables.
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

impl FormulaExpr for Binary {
    fn variables(&self) -> HashSet<Term> {
        let mut set = HashSet::new();
        set.extend(self.left.variables());
        set.extend(self.right.variables());
        set
    }

    fn replace(&mut self, pattern: &Term, replacement: &Term) {
        self.left.replace(pattern, replacement);
        self.right.replace(pattern, replacement);
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Term, SetComprehension> {
        let mut map = HashMap::new();
        // If left side is a variable term and right side is set comprehension then do nothing.
        if let Expr::BaseExpr(be1) = &self.left {
            if let BaseExpr::Term(_) = be1 {
                if let Expr::BaseExpr(be2) = &self.right {
                    if let BaseExpr::SetComprehension(_) = be2 {
                        return map;
                    }
                }
            }
        }

        // Gather all set comprehensions from both left and right hand sides.
        let left_map = self.left.replace_set_comprehension(generator);
        let right_map = self.right.replace_set_comprehension(generator);
        map.extend(left_map);
        map.extend(right_map);
        map
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.op, self.right)
    }
}

impl ConstraintBehavior for Binary {}

impl Binary {
    /// Assume all set comprehensions are separatedly declared like `a = count({..}) and 
    /// will not occur elsewhere in other parts of the expression.
    pub fn variables_current_level(&self) -> HashSet<Term> {
        if let Expr::BaseExpr(base_expr) = &self.right {
            if let BaseExpr::SetComprehension(_setcompre) = base_expr {
                // if the right side is a set comprehension then return variable on the left side,
                // which is just one variable term to declare the set comprehension.
                return self.left.variables();
            }
        }
        // Return all variables if no set comprehension exists.
        self.variables()
    }

    /// Check if the binary constraint has set comprehension inside it.
    pub fn has_set_comprehension(&self) -> bool {
        return self.left.has_set_comprehension() || self.right.has_set_comprehension(); 
    }

    pub fn evaluate<M, T>(&self, binding: &M) -> Option<bool>
    where 
        M: GenericMap<T, T>,
        T: Borrow<Term>
    {
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

impl FormulaExpr for TypeConstraint {
    fn variables(&self) -> HashSet<Term> {
        let mut set = HashSet::new();
        set.insert(self.var.clone());
        set
    }

    fn replace(&mut self, pattern: &Term, replacement: &Term) {
        self.var.replace(pattern, replacement);

    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Term, SetComprehension> {
        HashMap::new()
    }
}

impl Display for TypeConstraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.var, self.sort.name())
    }
}


impl ConstraintBehavior for TypeConstraint {}


#[enum_dispatch(Constraint)]
pub trait ConstraintBehavior {}


#[enum_dispatch]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Constraint {
    Predicate,
    Binary,
    TypeConstraint,
}

impl FormulaExpr for Constraint {
    fn variables(&self) -> HashSet<Term> {
        match self {
            Constraint::Predicate(p) => p.variables(),
            Constraint::Binary(b) => b.variables(),
            Constraint::TypeConstraint(t) => t.variables()
        }
    }

    fn replace(&mut self, pattern: &Term, replacement: &Term) {
        match self {
            Constraint::Predicate(p) => p.replace(pattern, replacement),
            Constraint::Binary(b) => b.replace(pattern, replacement),
            Constraint::TypeConstraint(t) => t.replace(pattern, replacement),
        };
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Term, SetComprehension> {
        match self {
            Constraint::Predicate(p) => {
                return p.replace_set_comprehension(generator);
            },
            Constraint::Binary(b) => {
                return b.replace_set_comprehension(generator);
            },
            Constraint::TypeConstraint(t) => {
                return t.replace_set_comprehension(generator);
            },
        };
    }
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


