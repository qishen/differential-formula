extern crate rand;
extern crate timely;
extern crate differential_dataflow;
extern crate abomonation_derive;
extern crate abomonation;

use std::iter::*;
use std::vec::Vec;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::string::String;

use petgraph::graph::*;
use petgraph::algo::*;

use crate::term::*;
use crate::expression::*;
use crate::constraint::*;


pub struct DontCareVarGen {
    counter: i64
}

impl DontCareVarGen {
    pub fn new() -> Self {
        DontCareVarGen { counter: 0 }
    }

    pub fn generate(&mut self) -> Term {
        let var: Term = Variable::new(format!("~dc{}", self.counter), vec![]).into();
        self.counter += 1;
        var
    }
}

#[derive(Clone, Debug)]
pub struct Rule {
    head: Vec<Term>,
    body: Vec<Constraint>,
}

impl FormulaExpr for Rule {
    // All variables are found recursively including the ones in set comprehension.
    fn variables(&self) -> HashSet<Term> {
        // Head is ignored because the variables it has already exist in the body.
        self.body.variables()
    }

    fn replace(&mut self, pattern: &Term, replacement: &Term) {
        self.head.replace(pattern, replacement);
        self.body.replace(pattern, replacement);
    }

    fn replace_set_comprehension(&mut self, generator: &mut DontCareVarGen) -> HashMap<Term, SetComprehension> {
        self.body.replace_set_comprehension(generator)
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let term_strs: Vec<String> = self.head.iter().map(|x| {
            let term_str = format!("{}", x);
            term_str
        }).collect();

        let constraint_strs: Vec<String> = self.body.iter().map(|x| {
            let con_str = format!("{}", x);
            con_str
        }).collect();

        let head_str = match self.is_conformance_rule() {
            true => "conforms".to_string(),
            false => term_strs.join(", ")
        };
        let body_str = constraint_strs.join(", ");
        write!(f, "{}", head_str + " :- " + &body_str)
    }
}


impl Rule {
    pub fn new(head: Vec<Term>, body: Vec<Constraint>) -> Self {
        let mut rule = Rule { head, body };

        // Don't-care variable generator to generate some variables like ~dc that user
        // can't use when writing rules.
        let mut dc_generator = DontCareVarGen::new();

        // Convert negation to set comprehension.
        rule.replace_negative_predicate(&mut dc_generator);

        // Replace undeclared set comprehension with variable term and declare set 
        // comprehensions as new constraints. 
        // e.g. 1 + count({..}) > 3 will be converted to 1 + ~dc > 3, ~dc = count({..}).
        let map = rule.replace_set_comprehension(&mut dc_generator);

        // Add set comprehension declaration into current rule.
        for (var, setcompre) in map.into_iter() {
            let left: Expr = BaseExpr::Term(var).into();
            let right: Expr = BaseExpr::SetComprehension(setcompre).into();
            let binary: Constraint = Binary { op: BinOp::Eq, left, right }.into();
            rule.body.push(binary);
        }

        if !rule.validate() {
            // TODO: Raise exception
        }

        rule
    }
    
    /// Check if it is a headless conformance rule.
    pub fn is_conformance_rule(&self) -> bool {
        self.head.len() == 0
    }

    pub fn get_head(&self) -> Vec<Term> {
        self.head.clone()
    }

    pub fn get_body(&self) -> Vec<Constraint> {
        self.body.clone()
    }

    /// Return all constraints in the body of current rule that are predicates to match terms,
    /// we assume there are no negative predicates since they are all converted to set comprehensions.
    pub fn predicate_constraints(&self) -> Vec<&Constraint> {
        self.body.iter().filter(|x| {
            match x {
                Constraint::Predicate(_p) => true,
                _ => false,
            }
        }).collect()
    }
   
    /// Return all binary constraints in the body of current rule excluding the ones in set comprehensions.
    pub fn binary_constraints(&self) -> Vec<&Constraint> {
        self.body.iter().filter(|x| {
            match x {
                Constraint::Binary(_b) => true,
                _ => false,
            }
        }).collect()
    }

    pub fn type_constraints(&self) -> Vec<&Constraint> {
        self.body.iter().filter(|x| {
            match x {
                Constraint::TypeConstraint(_t) => true,
                _ => false,
            }
        }).collect()
    }

    /// Convert negated constraint `no X(..)` into two constraints with set comprehension,
    /// ~dc1 = count({ ~dc | ~dc is X(..) }), ~dc1 = 0.
    fn replace_negative_predicate(&mut self, generator: &mut DontCareVarGen) {
        let mut constraints = vec![];
        let clist: Vec<Constraint> = self.body.drain(..).collect(); 
        for constraint in clist.into_iter() {
            match constraint {
                Constraint::Predicate(predicate) => {
                    if predicate.negated {
                        let introduced_var = generator.generate();
                        let (b1, b2) = predicate.convert_negation(introduced_var).unwrap();
                        constraints.push(b1);
                        constraints.push(b2);
                    }
                    else { constraints.push(predicate.into()); }
                },
                _ => { constraints.push(constraint); }
            }
        }

        self.body = constraints;
    }

    /// Return all existing variables in the body of a rule including the ones in set comprehension.
    pub fn variables_current_level(&self) -> HashSet<Term> {
        let mut var_set = HashSet::new();
        for constraint in self.body.iter() {
            match constraint {
                Constraint::Predicate(predicate) => {
                    var_set.extend(predicate.variables());
                },
                Constraint::Binary(binary) => {
                    // Skip variables in set comprehension.
                    var_set.extend(binary.variables_current_level());
                },
                Constraint::TypeConstraint(type_constraint) => {
                    var_set.insert(type_constraint.var.clone());
                }
            }
        }

        var_set
    }

    /// Return derived variables by simply deducting matched variables from all variables.
    /// A derived variable is used to declare an arithmetic expression or set comprehension that
    /// doesn't appear in the predicates and should not have fragments.
    /// e.g. x = c * c + 1, x = count({..})
    pub fn declaration_variables(&self) -> HashSet<Term> {
        // If a variable with fragments like x.y.z = a + b is mistaken as derived variable, 
        // then remove it from derived variable list.
        let mut derived_vars = HashSet::new();
        let pred_matched_vars = self.predicate_matched_variables();
        let all_vars = self.variables_current_level();

        // println!("rule: {}, pred: {:?}, all vars {:?}", self, pred_matched_vars, all_vars);

        for var in all_vars.into_iter() {
            // var cannot have fragments as a derived variable.
            if !pred_matched_vars.contains(&var) && &var == var.root() {
                derived_vars.insert(var);
            }
        }

        derived_vars
    }

    /// Find all variables that are matched in the predicates of the current rule and the rules 
    /// in set comprehension are not included in this method.
    pub fn predicate_matched_variables(&self) -> HashSet<Term> {
        let mut var_set = HashSet::new();
        for constraint in self.body.iter() {
            match constraint {
                Constraint::Predicate(predicate) => {
                    let vars = predicate.variables();
                    var_set.extend(vars);
                },
                _ => {}
            }
        }

        var_set
    }

    pub fn validate(&self) -> bool {
        let pred_matched_vars = self.predicate_matched_variables();
        let declaration_vars = self.declaration_variables();
        let all_vars = self.variables_current_level();
        // Make sure there is no unused variables.
        for var in all_vars.iter() {
            if !pred_matched_vars.contains(var) && !declaration_vars.contains(var) {
                return false;
            }
        }

        // All variables in the head should appear in the body too.
        for var in self.head.variables().iter() {
            if !all_vars.contains(var) { return false; }
        }

        true
    }

    /// Return the first constraint with set comprehension as the picked candidate or just select
    /// the first candidate if all candidates don't have set comprehension.
    pub fn elect_declaration_constraint<'a>(&self, candidates: Vec<&'a Constraint>) -> &'a Constraint {
        for constraint in candidates.clone().into_iter() {
            let binary: Binary = constraint.clone().try_into().unwrap();
            match binary.right {
                Expr::BaseExpr(base_expr) => {
                    if let BaseExpr::SetComprehension(setcompre) = base_expr {
                        return constraint;
                    }
                },
                _ => {}
            }
        }

        return candidates[0];
    }

    /// A list of constraints in which every variable inside it can be directly evaluated by binding map.
    /// Simply return all binary constraints that are not definition constraints.
    pub fn pure_constraints(&self) -> Vec<&Constraint> {
        let mut pure_constraints = vec![];
        let declaration_constraint_set: HashSet<&Constraint> = HashSet::from_iter(
            self.ordered_declaration_constraints()
        );

        for constraint in self.binary_constraints() {
            if !declaration_constraint_set.contains(constraint) {
                pure_constraints.push(constraint);
            }
        }

        pure_constraints
    }

    pub fn ordered_declaration_constraints(&self) -> Vec<&Constraint> {
        self.sort_declaration_constraints().0
    }

    /// Definition constraints are declarations of new derived variable and they were never matched
    /// in other predicates in the same rule.
    /// e.g. c = count({..}), c = val +100, val = c * c, val = c + a, val = a + 1, X(a).
    /// Re-arrange the order of definition constraints to make sure c = count({..}) is executed
    /// before val = c * c.
    pub fn sort_declaration_constraints(&self) -> (Vec<&Constraint>, Vec<&Constraint>) {
        let mut declaration_constraints = vec![];
        // Some declaration constraints are downgraded to pure constraints.
        let mut downgraded_constraints = vec![];
        for (variable, candidates) in self.declaration_constraint_map().into_iter() {
            let picked = self.elect_declaration_constraint(candidates.clone());
            for candidate in candidates.into_iter() {
                if candidate == picked {
                    declaration_constraints.push(candidate);
                } else {
                    downgraded_constraints.push(candidate);
                }
            }
        }

        let mut graph = Graph::new();
        let mut nodes = vec![];
        for constraint in declaration_constraints {
            // Each node is indexed and associated with a weight in which you can store some data.
            let node = graph.add_node(constraint);
            nodes.push(node);
        }

        for n1 in nodes.iter() {
            for n2 in nodes.iter() {
                let b1: Binary = graph.node_weight(n1.clone()).unwrap().clone().clone().try_into().unwrap();
                let b2: Binary = graph.node_weight(n2.clone()).unwrap().clone().clone().try_into().unwrap();
                let base_expr: BaseExpr = b1.left.try_into().unwrap();
                let left_var: Term = base_expr.try_into().unwrap();

                if b2.right.variables().contains(&left_var) {
                    graph.add_edge(n1.clone(), n2.clone(), 1);
                }
            }
        }

        let mut sorted_declaration_constraints = vec![];
        let indexes = toposort(&graph, None).unwrap();

        for index in indexes {
            let &constraint = graph.node_weight(index).unwrap();
            sorted_declaration_constraints.push(constraint);
        }

        (sorted_declaration_constraints, downgraded_constraints)
    }

    /// Return a map that maps declaration variable to a list of expressions because the declaration
    /// variable may occur in more than one binary constraint, which is a single variable term on the
    /// left side of the binary constraint. e.g. val -> [count({..}), c * c]
    fn declaration_constraint_map(&self) -> HashMap<Term, Vec<&Constraint>> {
        let declaration_vars = self.declaration_variables();
        let mut map = HashMap::new();
        for constraint in self.body.iter() {
            if let Constraint::Binary(b) = constraint {
                if let Expr::BaseExpr(left_be) = &b.left {
                    if let BaseExpr::Term(t) = left_be {
                        // It has to be a declaration variable.
                        if declaration_vars.contains(t) {
                            if !map.contains_key(t) {
                                map.insert(t.clone(), vec![]);
                            }
                            map.get_mut(t).unwrap().push(constraint);
                        }
                    }
                }
            }
        }

        // println!("{} @@@@@ {:?} ##### {:?}", self, declaration_vars, map);

        map
    }

}