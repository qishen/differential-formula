use std::vec::Vec;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Display};

use num::*;

use crate::expression::{BasicExprOps, SetCompreOps, SetComprehension};
use crate::expression::expr::ExprTrait;
use crate::term::*;
use crate::util::*;
use crate::util::map::*;


// #[enum_dispatch]
pub trait BaseExprTrait {}
impl BaseExprTrait for SetComprehension {}
// impl BaseExprTrait for T {}

// #[enum_dispatch(BaseExprTrait)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BaseExpr {
    SetComprehension(SetComprehension),
    Term(AtomicTerm),
}

impl TryFrom<BaseExpr> for SetComprehension {
    type Error = &'static str;

    fn try_from(value: BaseExpr) -> Result<Self, Self::Error> {
        match value {
            BaseExpr::SetComprehension(setcompre) => {
                Ok(setcompre)
            },
            _ => { Err("It's not a set comprehension.") }
        }
    }
}

// impl<T1> TryFrom<BaseExpr<T1>> for T1 where T1: TermStructure {
//     type Error = &'static str;

//     fn try_from(value: BaseExpr<T1>) -> Result<Self, Self::Error> {
//         match value {
//             BaseExpr::Term(term) => {
//                 Ok(term)
//             },
//             _ => { Err("It's not a term.") }
//         }
//     }
// }

impl ExprTrait for BaseExpr {
    fn evaluate<M>(&self, binding: &M) -> Option<BigInt> where M: GenericMap<AtomicTerm, AtomicTerm> {
        // let atom_enum = match self {
        //     BaseExpr::Term(term) => {
        //         if term == term.root() {
        //             let bigint_term = match binding.contains_gkey(term) {
        //                 true => binding.gget(term).unwrap(),
        //                 false => term,
        //             };
        //             let bigint_enum = bigint_term.into_atom_enum().unwrap();
        //             Some(bigint_enum)
        //         } else {
        //             // The term is not in the binding but the root of term is in the binding.
        //             let var_term = binding.gget(term.root()).unwrap();
        //             let fragment_diff = term.fragments_diff(term.root()).unwrap();
        //             let subterm = var_term.find_subterm_by_labels(&fragment_diff).unwrap();
        //             subterm.into_atom_enum()
        //         }
        //     },
        //     _ => { None } // No evaluation on set comprehension.
        // }.unwrap();

        // match atom_enum {
        //     AtomEnum::Int(i) => Some(i),
        //     _ => None
        // }
        unimplemented!()
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

impl BasicExprOps for BaseExpr {
    fn variables(&self) -> HashSet<AtomicTerm> {
        match self {
            BaseExpr::Term(t) => {
                t.variables()
            },
            BaseExpr::SetComprehension(s) => s.variables(),
        }
    }

    fn replace_pattern(&mut self, pattern: &AtomicTerm, replacement: &AtomicTerm) {
        match self {
            BaseExpr::SetComprehension(s) => s.replace_pattern(pattern, replacement),
            BaseExpr::Term(t) => t.replace_pattern(pattern, replacement),
        };
    }
}

impl SetCompreOps for BaseExpr {
    fn has_set_comprehension(&self) -> bool {
        let has_setcompre = match self {
            BaseExpr::SetComprehension(s) => true,
            _ => false,
        };
        has_setcompre
    }

    fn set_comprehensions(&self) -> Vec<&SetComprehension> {
        let mut setcompres = vec![];
        match self {
            BaseExpr::SetComprehension(s) => {
                setcompres.push(s);
            },
            _ => {},
        };
        setcompres
    }

    fn replace_set_comprehension(&mut self, generator: &mut NameGenerator) 
    -> HashMap<AtomicTerm, SetComprehension> {
        let mut map = HashMap::new();
        match self {
            BaseExpr::SetComprehension(setcompre) => {
                // Recursively replace setcompres in the conditions of current setcompre.
                setcompre.replace_set_comprehension(generator);
                // Make a deep copy to avoid ugly try_into().
                let replaced_setcompre = setcompre.clone(); 
                let dc_name = generator.generate_name();
                let dc_var = AtomicTerm::gen_raw_variable_term(dc_name, vec![]);
                let mut base_expr: BaseExpr = BaseExpr::Term(dc_var.clone());
                std::mem::swap(self, &mut base_expr);
                map.insert(dc_var, replaced_setcompre); 
            },
            BaseExpr::Term(_) => {},
        };
        return map;
    }
}
