extern crate rand;
extern crate abomonation_derive;
extern crate abomonation;

use std::collections::*;
use std::convert::*;
use std::fmt;
use std::fmt::{Debug, Display};

use num::*;

use crate::term::*;
use crate::expression::*;
use crate::util::*;
use crate::util::map::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Predicate {
    pub negated: bool,
    pub term: AtomicTerm,
    pub alias: Option<AtomicTerm>, // Must be a variable term and could have fragments
}

impl BasicExprOps for Predicate {
    fn variables(&self) -> HashSet<AtomicTerm> {
        let mut var_set = HashSet::new();
        var_set.extend(self.term.variables());
        // Don't forget to add alias to variable set.
        if let Some(var) = self.alias.clone() {
            var_set.insert(var);
        }
        var_set
    }

    fn replace_pattern(&mut self, pattern: &AtomicTerm, replacement: &AtomicTerm) {
        self.term.replace_pattern(pattern, replacement);
        self.alias.replace_pattern(pattern, replacement);
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

impl Predicate {
    /// Negative predicate constraint is count({setcompre}) = 0 in disguise and a negated predicate
    /// will be converted into two binary constraints with a random variable to hold the set comprehension.
    pub fn convert_negation(&self, var: AtomicTerm) -> Option<(Constraint, Constraint)> {
        // Positive predicate is not allowed to be converted into Binary constraint.
        if !self.negated {
            return None;
        }
        // Give it an alias starting with "~" that cannot be accepted by parser to avoid 
        // collision with other user-defined variables.
        let alias = AtomicTerm::gen_raw_variable_term("~dc".to_string(), vec![]);
        let vars: Vec<AtomicTerm> = vec![alias.clone()];
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
        let var_base_expr: BaseExpr = BaseExpr::Term(var);
        let setcompre_base_expr: BaseExpr = BaseExpr::SetComprehension(setcompre);
        
        let binary = Binary {
            op: BinOp::Eq,
            left: Expr::BaseExpr(var_base_expr.clone()),
            right: Expr::BaseExpr(setcompre_base_expr),
        };

        let big_zero = BigInt::from_i64(0 as i64).unwrap();
        let zero_atom_enum = AtomEnum::Int(big_zero);
        // Create a new type in atom but it's ok here.
        let t = AtomicTerm::gen_atom_term(zero_atom_enum);
        let zero_base_expr: BaseExpr = BaseExpr::Term(t);

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

impl BasicExprOps for Binary {
    fn variables(&self) -> HashSet<AtomicTerm> {
        let mut set = HashSet::new();
        set.extend(self.left.variables());
        set.extend(self.right.variables());
        set
    }

    fn replace_pattern(&mut self, pattern: &AtomicTerm, replacement: &AtomicTerm) {
        self.left.replace_pattern(pattern, replacement);
        self.right.replace_pattern(pattern, replacement);
    }
}

impl SetCompreOps for Binary {
    fn has_set_comprehension(&self) -> bool {
        self.left.has_set_comprehension() || self.right.has_set_comprehension()
    }

    fn set_comprehensions(&self) -> Vec<&SetComprehension> {
        let mut list = vec![];
        let mut left_vec = self.left.set_comprehensions();
        let mut right_vec = self.right.set_comprehensions();
        list.append(&mut left_vec);
        list.append(&mut right_vec);
        list
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) 
    -> HashMap<AtomicTerm, SetComprehension> {
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

impl Binary {
    /// Check if a binary constraint is actually a set comprehension assignment
    pub fn is_setcompre_assignment(&self) -> bool {
        if let Expr::BaseExpr(base_expr) = &self.left {
            if let BaseExpr::Term(_) = base_expr {
                if let Expr::BaseExpr(base_expr2) = &self.right {
                    if let BaseExpr::SetComprehension(_) = base_expr2 {
                        return true
                    }
                }
            }
        }
        false
    }

    /// Return the definition term from the left side of binary constraint
    pub fn left_term(&self) -> Option<AtomicTerm> {
        if let Expr::BaseExpr(base_expr) = &self.left {
            if let BaseExpr::Term(term) = base_expr {
                return Some(term.clone())
            }
        }
        None
    }

    /// Assume all set comprehensions are separatedly declared like `a = count({..}) and 
    /// will not occur elsewhere in other parts of the expression.
    pub fn variables_current_level(&self) -> HashSet<AtomicTerm> {
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

    pub fn evaluate<M>(&self, binding: &M) -> Option<bool> where M: GenericMap<AtomicTerm, AtomicTerm> {
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
    pub var: AtomicTerm,
    pub sort: RawType,
}

impl Display for TypeConstraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.var, self.sort)
    }
}

impl BasicExprOps for TypeConstraint {
    fn variables(&self) -> HashSet<AtomicTerm> {
        self.var.variables()
    }

    fn replace_pattern(&mut self, pattern: &AtomicTerm, replacement: &AtomicTerm) {
        self.replace_pattern(pattern, replacement);
    }
}

// #[enum_dispatch]
// pub trait ConstraintBehavior {}
// impl ConstraintBehavior for Predicate where T: TermStructure {}
// impl ConstraintBehavior for TypeConstraint where T: TermStructure {}
// impl ConstraintBehavior for Binary where T: TermStructure {}
// #[enum_dispatch(ConstraintBehavior)]

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Constraint {
    Predicate(Predicate),
    Binary(Binary),
    TypeConstraint(TypeConstraint),
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

impl TryFrom<Constraint> for Predicate {
    type Error = &'static str;

    fn try_from(value: Constraint) -> Result<Self, Self::Error> {
        match value {
            Constraint::Predicate(pred) => {
                Ok(pred)
            },
            _ => { Err("It's not a predicate constraint.") }
        }
    }
}

impl TryFrom<Constraint> for Binary {
    type Error = &'static str;

    fn try_from(value: Constraint) -> Result<Self, Self::Error> {
        match value {
            Constraint::Binary(binary) => {
                Ok(binary)
            },
            _ => { Err("It's not a binary constraint.") }
        }
    }
}

impl TryFrom<Constraint> for TypeConstraint {
    type Error = &'static str;

    fn try_from(value: Constraint) -> Result<Self, Self::Error> {
        match value {
            Constraint::TypeConstraint(tc) => {
                Ok(tc)
            },
            _ => { Err("It's not a type constraint.") }
        }
    }
}

impl BasicExprOps for Constraint {
    fn variables(&self) -> HashSet<AtomicTerm> {
        match self {
            Constraint::Predicate(p) => p.variables(),
            Constraint::Binary(b) => b.variables(),
            Constraint::TypeConstraint(t) => t.variables()
        }
    }

    fn replace_pattern(&mut self, pattern: &AtomicTerm, replacement: &AtomicTerm) {
        match self {
            Constraint::Predicate(p) => p.replace_pattern(pattern, replacement),
            Constraint::Binary(b) => b.replace_pattern(pattern, replacement),
            Constraint::TypeConstraint(t) => t.replace_pattern(pattern, replacement),
        };
    }
}

impl SetCompreOps for Constraint {
    fn has_set_comprehension(&self) -> bool {
        match self {
            Constraint::Predicate(p) => false,
            Constraint::Binary(b) => b.has_set_comprehension(),
            Constraint::TypeConstraint(t) => false,
        }
    }

    fn set_comprehensions(&self) -> Vec<&SetComprehension> {
        match self {
            Constraint::Predicate(p) => Vec::new(),
            Constraint::Binary(b) => b.set_comprehensions(),
            Constraint::TypeConstraint(t) => Vec::new()
        }
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) 
    -> HashMap<AtomicTerm, SetComprehension> {
        match self {
            Constraint::Predicate(p) => HashMap::new(),
            Constraint::Binary(b) => b.replace_set_comprehension(generator),
            Constraint::TypeConstraint(t) => HashMap::new(),
        }
    }
}

// impl Expression for Constraint where T: TermStructure {

//     type TermOutput = T;

//     fn variables(&self) -> HashSet<AtomicTerm> {
//         match self {
//             Constraint::Predicate(p) => p.variables(),
//             Constraint::Binary(b) => b.variables(),
//             Constraint::TypeConstraint(t) => t.variables()
//         }
//     }

//     fn replace_pattern(&mut self, pattern: &AtomicTerm, replacement: &AtomicTerm) {
//         match self {
//             Constraint::Predicate(p) => p.replace_pattern(pattern, replacement),
//             Constraint::Binary(b) => b.replace_pattern(pattern, replacement),
//             Constraint::TypeConstraint(t) => t.replace_pattern(pattern, replacement),
//         };
//     }

//     fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) -> HashMap<AtomicTerm, SetComprehension<AtomicTerm>> {
//         match self {
//             Constraint::Predicate(p) => {
//                 return p.replace_set_comprehension(generator);
//             },
//             Constraint::Binary(b) => {
//                 return b.replace_set_comprehension(generator);
//             },
//             Constraint::TypeConstraint(t) => {
//                 return t.replace_set_comprehension(generator);
//             },
//         };
//     }
// }