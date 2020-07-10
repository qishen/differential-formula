extern crate rand;
extern crate abomonation_derive;
extern crate abomonation;

use std::borrow::Borrow;
use std::collections::*;
use std::fmt;
use std::fmt::{Debug, Display};

use enum_dispatch::enum_dispatch;
use num::*;

use crate::term::*;
use crate::expression::*;
use crate::type_system::*;
use crate::util::*;
use crate::util::map::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Predicate<T> where T: BorrowedTerm {
    pub negated: bool,
    pub term: T,
    pub alias: Option<T>, // Must be a variable term and could have fragments
}

impl<T> Expression for Predicate<T> where T: BorrowedTerm {

    type TermOutput = T;

    fn variables(&self) -> HashSet<Self::TermOutput> {
        let mut var_set = HashSet::new();
        var_set.extend(self.term.variables());
        // Don't forget to add alias to variable set.
        if let Some(var) = self.alias.clone() {
            var_set.insert(var);
        }
        var_set
    }

    fn replace_pattern(&mut self, pattern: &Self::TermOutput, replacement: &Self::TermOutput) {
        self.term.replace_pattern(pattern, replacement);
        self.alias.replace_pattern(pattern, replacement);
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Self::TermOutput, SetComprehension<Self::TermOutput>> {
        HashMap::new()
    }
}

impl<T> Display for Predicate<T> where T: BorrowedTerm {
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

impl<T> Predicate<T> where T: BorrowedTerm {
    /// Negative predicate constraint is count({setcompre}) = 0 in disguise and a negated predicate
    /// will be converted into two binary constraints with a random variable to hold the set comprehension.
    pub fn convert_negation(&self, var: T) -> Option<(Constraint<T>, Constraint<T>)> {
        // Positive predicate is not allowed to be converted into Binary constraint.
        if !self.negated {
            return None;
        }

        // Give it an alias starting with "~" that cannot be accepted by parser to avoid 
        // collision with other user-defined variables.
        let alias: T = Term::Variable(Variable::new(None, "~dc".to_string(), vec![])).into();
        let vars: Vec<T> = vec![alias.clone()];
        let pos_predicate = Predicate {
            negated: false,
            term: self.term.clone(),
            alias: Some(alias),
        };

        let setcompre = SetComprehension::new(
            vars, 
            vec![Constraint::Predicate(pos_predicate)],
            SetCompreOp::Count,
            BigInt::from_i64(0 as i64).unwrap(),
        );

        // A little copy here from T to Term<S, T> but should be fine since it's on meta model.
        let var_base_expr: BaseExpr<T> = BaseExpr::Term(var);
        let setcompre_base_expr: BaseExpr<T> = BaseExpr::SetComprehension(setcompre);
        
        let binary = Binary {
            op: BinOp::Eq,
            left: Expr::BaseExpr(var_base_expr.clone()),
            right: Expr::BaseExpr(setcompre_base_expr),
        };

        let big_zero = BigInt::from_i64(0 as i64).unwrap();
        let zero_atom_enum = AtomEnum::Int(big_zero);
        let zero_term: AtomicTerm = AtomicTerm::create_atom(zero_atom_enum);
        let t: T = zero_term.into();
        let zero_base_expr: BaseExpr<T> = BaseExpr::Term(t);

        let binary2 = Binary {
            op: BinOp::Eq,
            left: Expr::BaseExpr(var_base_expr),
            right: Expr::BaseExpr(zero_base_expr),
        };

        let tuple = (Constraint::Binary(binary), Constraint::Binary(binary2));
        Some(tuple)
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
pub struct Binary<T> where T: BorrowedTerm {
    pub op: BinOp,
    pub left: Expr<T>,
    pub right: Expr<T>,
}

impl<T> Expression for Binary<T> where T: BorrowedTerm {

    type TermOutput = T;

    fn variables(&self) -> HashSet<Self::TermOutput> {
        let mut set = HashSet::new();
        set.extend(self.left.variables());
        set.extend(self.right.variables());
        set
    }

    fn replace_pattern(&mut self, pattern: &Self::TermOutput, replacement: &Self::TermOutput) {
        self.left.replace_pattern(pattern, replacement);
        self.right.replace_pattern(pattern, replacement);
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) 
    -> HashMap<Self::TermOutput, SetComprehension<Self::TermOutput>> 
    {
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

impl<T> Display for Binary<T> where T: BorrowedTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.op, self.right)
    }
}

impl<T> Binary<T> where T: BorrowedTerm {
    /// Assume all set comprehensions are separatedly declared like `a = count({..}) and 
    /// will not occur elsewhere in other parts of the expression.
    pub fn variables_current_level(&self) -> HashSet<T> {
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

    pub fn evaluate<M>(&self, binding: &M) -> Option<bool> where M: GenericMap<T, T> {
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
pub struct TypeConstraint<T> where T: BorrowedTerm {
    pub var: T,
    pub sort: T::SortOutput,
}

impl<T> Expression for TypeConstraint<T> where T: BorrowedTerm {

    type TermOutput = T;

    fn variables(&self) -> HashSet<Self::TermOutput> {
        let mut set = HashSet::new();
        set.insert(self.var.clone());
        set
    }

    fn replace_pattern(&mut self, pattern: &Self::TermOutput, replacement: &Self::TermOutput) {
        self.var.replace_pattern(pattern, replacement);
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) 
    -> HashMap<Self::TermOutput, SetComprehension<Self::TermOutput>> {
        HashMap::new()
    }
}

impl<T> Display for TypeConstraint<T> where T: BorrowedTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.var, self.sort)
    }
}

// #[enum_dispatch]
// pub trait ConstraintBehavior {}
// impl<T> ConstraintBehavior for Predicate<T> where T: BorrowedTerm {}
// impl<T> ConstraintBehavior for TypeConstraint<T> where T: BorrowedTerm {}
// impl<T> ConstraintBehavior for Binary<T> where T: BorrowedTerm {}

#[enum_dispatch]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Constraint<T> where T: BorrowedTerm {
    Predicate(Predicate<T>),
    Binary(Binary<T>),
    TypeConstraint(TypeConstraint<T>),
}

impl<T> Display for Constraint<T> where T: BorrowedTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let con_str = match self {
            Constraint::Predicate(p) => { format!("{}", p) },
            Constraint::Binary(b) => { format!("{}", b) },
            Constraint::TypeConstraint(t) => { format!("{}", t) },
        };

        write!(f, "{}", con_str) 
    }
}

impl<T> Expression for Constraint<T> where T: BorrowedTerm {

    type TermOutput = T;

    fn variables(&self) -> HashSet<Self::TermOutput> {
        match self {
            Constraint::Predicate(p) => p.variables(),
            Constraint::Binary(b) => b.variables(),
            Constraint::TypeConstraint(t) => t.variables()
        }
    }

    fn replace_pattern(&mut self, pattern: &Self::TermOutput, replacement: &Self::TermOutput) {
        match self {
            Constraint::Predicate(p) => p.replace_pattern(pattern, replacement),
            Constraint::Binary(b) => b.replace_pattern(pattern, replacement),
            Constraint::TypeConstraint(t) => t.replace_pattern(pattern, replacement),
        };
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<Self::TermOutput, SetComprehension<Self::TermOutput>> {
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