use std::any::Any;
use std::collections::*;
use std::iter::FromIterator;

use crate::expression::{BasicExprOps, SetComprehension, SetCompreOp};
use crate::module::*;
use crate::parser::combinator::parse_program;
use crate::term::*;
use crate::type_system::*;
use crate::rule::Rule;
use crate::util::map::GenericMap;


pub trait DDLogFormat {
    fn into_ddlog_format(&self) -> Option<String>;
}

// TODO: Use Ref type to wrap subtypes in the fields
impl DDLogFormat for RawType {
    fn into_ddlog_format(&self) -> Option<String> {
        match self {
            // `TypeId` does not provide enough information for conversion
            RawType::TypeId(_) => None,
            RawType::Type(type_enum) => {
                let str = match type_enum {
                    FormulaTypeEnum::BaseType(base_type) => {
                        match base_type {
                            BaseType::Boolean => format!("bool"),
                            BaseType::String => format!("string"),
                            BaseType::Integer => format!("usize"),
                            BaseType::PosInteger => format!("usize"),
                            BaseType::NegInteger => format!("usize"),
                            BaseType::Rational => format!("float"),
                        }
                    },
                    FormulaTypeEnum::CompositeType(composite_type) => {
                        let arg_strs: Vec<String> = composite_type.arguments.iter().map(|(alias, subtype)| {
                            let subtype_name = match subtype {
                                RawType::TypeId(_) => todo!(),
                                RawType::Type(type_enum) => {
                                    match type_enum {
                                        FormulaTypeEnum::BaseType(_) => subtype.into_ddlog_format().unwrap(),
                                        _ => subtype.type_id().to_string()
                                    }
                                },
                            };
                            format!("{}: {}", alias.clone().unwrap_or("None".to_string()), subtype_name)
                        }).collect();
                        let args_str = arg_strs.join(", ");
                        format!("typedef {} = {}{{ {} }}", self.type_id(), self.type_id(), args_str)
                    },
                    FormulaTypeEnum::EnumType(enum_type) => {
                        // Each enum is represented by one unique string.
                        let enum_strs: Vec<String> = enum_type.items.iter().map(|atom| 
                            format!("{}", atom)).collect();
                        let enums_str = enum_strs.join(" | ");
                        format!("typedef {} = {}", self.type_id(), enums_str)
                    },
                    FormulaTypeEnum::RangeType(_) => todo!(),
                    FormulaTypeEnum::RenamedType(_) => todo!(),
                    FormulaTypeEnum::UnionType(union_type) => {
                        let union_name = &union_type.name;
                        let subtype_strs: Vec<String> = union_type.subtypes.iter().enumerate().map(|(i, subtype)| {
                            let subtype_name = match subtype.is_base_type() {
                                true => subtype.into_ddlog_format().unwrap(),
                                false => subtype.type_id().to_string()
                            };
                            format!("{}_{} {{ t{}: {} }}", union_name, subtype_name, i, subtype_name)
                        }).collect();
                        let subtypes_str = subtype_strs.join(" | ");
                        format!("typedef {} = {}", union_name, subtypes_str)
                    },
                };
                Some(str)
            },
        }
    }
}

fn convert_term(term: &AtomicTerm) -> String {
    let str = match term {
        AtomicTerm::Composite(composite) => {
            let type_name = composite.sort.type_id();
            let subterm_strs: Vec<String> = composite.arguments.iter().map(|x| {
                convert_term(x)
            }).collect();
            let subterms_str = subterm_strs.join(", ");
            let str = format!("{}{{ {} }}", type_name, subterms_str);
            str
        },
        AtomicTerm::Variable(variable) => {
            variable.root.clone()
        },
        AtomicTerm::Atom(atom) => {
            format!("{}", atom)
        }
    };
    str
}

/// Convert the term in the same way but return a string map for replaced variables. 
fn convert_term_without_identical_vars(term: &AtomicTerm) -> (String, HashMap<String, Vec<String>>) {
    let (normalized_term, tmap) = term.replace_identical_vars();
    let mut vmap = HashMap::new();
    for (k, vals) in tmap {
        // Add some binary constraint later as k == val1, val1 == val2, ...
        let val_strs: Vec<String> = vals.into_iter().map(|x| format!("{}", x)).collect();
        vmap.insert(format!("{}", k), val_strs);
    }
    let str = convert_term(&normalized_term);
    (str, vmap)
}

/// Return head terms and a list of type definition and relation pairs
fn convert_rule_head(rule: &Rule) -> (String, Vec<(String, String)>){
    let mut term_strs = vec![];
    let mut boolean_vars = vec![];
    for term in rule.head().into_iter() {
        match term {
            &AtomicTerm::Composite(_) => {
                let str = format!("{}[{}]", term.type_id(), convert_term(term));
                term_strs.push(str);
            },
            &AtomicTerm::Variable(_) => {
                // Create new bool type and relation for the boolean variable in the head
                let bool_constant_type = format!("typedef {}BoolConst = {}BoolConst{{}}", term, term);
                let bool_constant_relation = format!("output relation {}BoolConst[{}BoolConst]", 
                    term, term);
                boolean_vars.push((bool_constant_type, bool_constant_relation));
                term_strs.push(format!("{}BoolConst[{}BoolConst{{}}]", term, term));
            },
            _ => {}
        }
    }
    let head_terms_str = term_strs.join(", ");
    (head_terms_str, boolean_vars)
}

fn convert_binary() -> String {
    todo!()
}

fn convert_rule_body(rule_id: usize, rule: &Rule) 
-> (String, Vec<String>, Vec<String>, Vec<String>, Vec<String>, Vec<(String, String)>) {
    // Positive predicate terms
    let mut vmap: HashMap<String, HashSet<String>> = HashMap::new();
    let pos_strs: Vec<String> = rule.positive_terms().into_iter().map(|term| {
        // Derive some additional equality constraints if one variable occurs more than one in the term
        let (normalized_term, tmap) = term.replace_identical_vars(); 
        for (k, vals) in tmap {
            let k_str = format!("{}", k);
            let val_strs: Vec<String> = vals.into_iter().map(|x| format!("{}", x)).collect();
            if vmap.contains_key(&k_str) {
                let set = vmap.get_mut(&k_str).unwrap();
                set.extend(val_strs);
            } else {
                vmap.insert(k_str, HashSet::from_iter(val_strs.into_iter()));
            }
        }
        // Predicate in ddlog is expressed in the complete form of `Relation [Type]`
        format!("{}[{}]", term.type_id(), convert_term(&normalized_term))
    }).collect();

    let neg_setdiff_strs: Vec<String> = rule.negated_setdiff_terms().into_iter()
        .enumerate().map(|x| {
            todo!()
        }).collect();

    // Negation as set comprehension and create new rules for each comprehension in the form of
    // PredExists :- u is Pred(..), var g = (u).group_by(()), var count = g.group_count(), count == 0.
    // Bool constant `PredExists` could then be used in the body of other rules.
    let mut boolean_vars = vec![];
    let mut neg_bool_constraint_strs = vec![]; 
    let neg_setcompre_bool_const_rule_strs: Vec<String> = rule.negated_setcompre_terms().into_iter()
        .enumerate().map(|(i, term)| {
            let auto_aggr_var = format!("R{}N{}", rule_id, i);
            let bool_constant_type = format!("typedef {}BoolConst = {}BoolConst{{}}", 
                auto_aggr_var, auto_aggr_var);
            let bool_constant_relation = format!("output relation {}BoolConst[{}BoolConst]", 
                auto_aggr_var, auto_aggr_var);
            boolean_vars.push((bool_constant_type, bool_constant_relation));
            neg_bool_constraint_strs.push(format!("{}BoolConst[{}BoolConst{{}}]", auto_aggr_var, auto_aggr_var));

            let neg_vars = term.variables();
            let neg_var_strs: Vec<String> = neg_vars.iter().map(|x| format!("{}", x)).collect();
            let neg_vars_str = neg_var_strs.join(", ");
            let head = format!("{}BoolConst[{}BoolConst{{}}]", auto_aggr_var, auto_aggr_var);
            let aggr = format!("var g = ({}).group_by(()), var count = g.group_count(), count == 0", 
                neg_vars_str);

            // Derive some additional equality constraints if one variable occurs more than one in the term
            let mut setcompre_vmap: HashMap<String, HashSet<String>> = HashMap::new();
            let (normalized_term, tmap) = term.replace_identical_vars(); 
            for (k, vals) in tmap {
                let k_str = format!("{}", k);
                let val_strs: Vec<String> = vals.into_iter().map(|x| format!("{}", x)).collect();
                if setcompre_vmap.contains_key(&k_str) {
                    let set = setcompre_vmap.get_mut(&k_str).unwrap();
                    set.extend(val_strs);
                } else {
                    setcompre_vmap.insert(k_str, HashSet::from_iter(val_strs.into_iter()));
                }
            }

            // TODO: Handle derived binary constraints from setcompre_vmap.
            format!("{} :- {}[{}], {}.", 
                head, term.type_id(), 
                convert_term(&normalized_term), aggr
            )
        }).collect();

    // Negation as set difference
    // TODO: Figure out how to set difference with multiple negation in a rule
    let neg_setdiff_strs: Vec<String> = rule.negated_setdiff_terms().into_iter().map(|term| {
        let (normalized_term, tmap) = term.replace_identical_vars(); 
        for (k, vals) in tmap {
            let k_str = format!("{}", k);
            let val_strs: Vec<String> = vals.into_iter().map(|x| format!("{}", x)).collect();
            if vmap.contains_key(&k_str) {
                let set = vmap.get_mut(&k_str).unwrap();
                set.extend(val_strs);
            } else {
                vmap.insert(k_str, HashSet::from_iter(val_strs.into_iter()));
            }
        }
        format!("not {}[{}]", term.type_id(), convert_term(&normalized_term))
    }).collect();
       
    let mut pre_aggr_rules = vec![];
    let mut setcompre_head_union_types = vec![];
    let mut setcompre_head_union_relations = vec![];
    let mut post_aggr_rule_body_strs = vec![];

    // Assume each set comprehension is defined by a variable
    for (i, (var, setcompre)) in rule.setcompre_map().iter().enumerate() {
        // Rename the variable name to hold the result of set comprehension.
        let aggr_result_name = format!("aggr{}_{}", i, var);
        let union_name = format!("R{}SC{}Union", rule_id, i);
        let (union_type_str, 
            container_relation_str, 
            pre_aggr_rule_str, 
            post_aggr_rule_body_str
        ) = convert_setcompre(union_name, aggr_result_name, HashSet::new(), setcompre.clone());
        pre_aggr_rules.push(pre_aggr_rule_str);
        setcompre_head_union_types.push(union_type_str);
        setcompre_head_union_relations.push(container_relation_str);
        post_aggr_rule_body_strs.push(post_aggr_rule_body_str);
    }

    let str_vectors= vec![
        pos_strs, 
        neg_setdiff_strs, 
        neg_bool_constraint_strs,
        post_aggr_rule_body_strs
    ];
    let body_part_strs: Vec<String> = str_vectors.iter()
        .filter(|vector| vector.len() > 0)
        .map(|vector| vector.join(", "))
        .collect();
    let body_str = body_part_strs.join(", ");

    (
        body_str, 
        pre_aggr_rules, 
        neg_setcompre_bool_const_rule_strs, 
        setcompre_head_union_types, 
        setcompre_head_union_relations, 
        boolean_vars
    )
}

/// Return union type defs and containers in the head of intermediate rule.
fn convert_setcompre_head(union_name: String, container_name: String, head: Vec<AtomicTerm>) 
-> (String, String) {
    // TODO: Add constants or do we really have to? 
    let mut type_names = HashSet::new();
    for term in head.iter() {
        // If the term is a variable, it must have a valid type not type id assigned to it.
        // TODO: Check if the right type is assigned to the variable term otherwise
        // the `term.type_id()` will return Undefined.
        type_names.insert(term.type_id().to_string());
    }
    let union_strs: Vec<String> = type_names.iter().enumerate().map(|(i, type_name)| {
        // For example, Union_Edge { edge: Edge }
        format!("{}_{} {{ {}: {} }}", 
            union_name, 
            type_name, 
            type_name.to_lowercase(), 
            type_name
        )
    }).collect();
    let union_type_str = format!("typedef {} = {}", union_name, union_strs.join(" | "));
    let container_strs: Vec<String> = head.iter().map(|term| {
        // For example, Item[Item_Edge { Edge{a, Node{1}} }]
        let type_name = term.type_id();
        format!("{}[{}_{} {{ {} }}]", 
            container_name, 
            union_name,
            type_name, 
            convert_term(term)
        )
    }).collect(); 
    let container_head_str = container_strs.join(", ");
    (union_type_str, container_head_str)
}

fn convert_setcompre(
    union_type_name: String,
    aggr_result_name: String, 
    outer_vars: HashSet<AtomicTerm>, 
    setcompre: SetComprehension) 
-> (String, String, String, String) {
    // Create a new relation to contain the head term in set comprehension.
    let container_name = format!("{}ContainerRel", union_type_name);
    let head = setcompre.vars.clone();
    let (union_type_str, container_head_str) = convert_setcompre_head(
        union_type_name.clone(), 
        container_name.clone(), 
        head
    );

    let container_relation_str = format!("output relation {} [{}]", container_name, union_type_name);
    let setcompre_rule: Rule = setcompre.clone().into();
    // Take the condition of set comprehension as the body of a rule
    // TODO: Handle nested set comprehension.
    let (rule_body_str, _, _, _, _, _) = convert_rule_body(666, &setcompre_rule);
    let pre_aggr_rule_str = format!("{} :- {}.", container_head_str, rule_body_str);

    // TODO: Add more setcompre operators
    let op_str = match setcompre.op {
        SetCompreOp::Sum => "group_sum()",
        _ => "group_count()"
    };

    // Aggregate on the whole group because no shared variabels with outer scope
    let post_aggr_rule_body_str = format!("{}[u], var g = u.group_by(()), var {} = g.{}", 
        container_name, 
        aggr_result_name,
        op_str
    );

    (union_type_str, container_relation_str, pre_aggr_rule_str, post_aggr_rule_body_str)
}

fn convert_domain(domain: &Domain) -> String {
    let mut typedef_strs = vec![];
    let mut relation_strs = vec![];
    let mut rule_strs = vec![];

    let rules = domain.meta_info().rules();
    let type_map = domain.meta_info().type_map();

    for (type_name, raw_type) in type_map {
        // Ignore base type like string, integer, etc.
        if !raw_type.is_base_type() {
            if let Some(type_str) = raw_type.into_ddlog_format() {
                typedef_strs.push(type_str);
            }
            // It is not base type but a type Id
            if type_name != "~Undefined" {
                relation_strs.push(format!("output relation {}[{}]", type_name, type_name));
            }
        }
    }

    for (i, rule) in rules.iter().enumerate() {
        let (rule_head_str, extras) = convert_rule_head(rule);
        for (bool_type, bool_relation) in extras {
            typedef_strs.push(bool_type);
            relation_strs.push(bool_relation);
        }

        let (rule_body_str, 
            pre_aggr_rules,
            neg_bool_const_rules, 
            setcompre_head_union_types, 
            setcompre_head_union_relations,
            boolean_vars
        ) = convert_rule_body(i, rule);

        // Those boolean vars are from negation that are treated as set comprehension
        for (bool_type, bool_relation) in boolean_vars {
            typedef_strs.push(bool_type);
            relation_strs.push(bool_relation);
        }

        rule_strs.extend(pre_aggr_rules);
        rule_strs.extend(neg_bool_const_rules);
        typedef_strs.extend(setcompre_head_union_types);
        relation_strs.extend(setcompre_head_union_relations);

        // Combine head and body to create a complete rule string
        let rule_str = format!("{} :- {}.", rule_head_str, rule_body_str);
        rule_strs.push(rule_str);
    }

    let domain_str = vec![
        typedef_strs.join("\n"),
        relation_strs.join("\n"),
        rule_strs.join("\n")
    ].join("\n\n\n");

    domain_str
}


fn graph_env() -> Env {
    let path = std::path::Path::new("./tests/testcase/p0.4ml");
    let content = std::fs::read_to_string(path).unwrap() + "EOF";
    let (_, program_ast) = parse_program(&content);
    let env: Env = program_ast.build_env();
    env
}

#[test]
fn test_convert_domain() {
    let env = graph_env();
    let graph_domain = env.get_domain_by_name("Graph").unwrap();
    println!("{:#?}", graph_domain);

    let domain_str = convert_domain(graph_domain);
    println!("{}", domain_str);
}

#[test]
fn test_convert_type() {
    let env = graph_env();
    let graph = env.get_domain_by_name("Graph").unwrap();
    let m = env.get_model_by_name("m").unwrap();
    println!("{:#?}", graph);

    let node_type = graph.meta_info().get_type_by_name("Node").unwrap();
    let edge_type = graph.meta_info().get_type_by_name("Edge").unwrap();
    let item_type = graph.meta_info().get_type_by_name("Item").unwrap();

    // assert_eq!(node_type.into_ddlog_format().unwrap(), "typedef Node = { id: string }");

    assert_eq!(edge_type.into_ddlog_format().unwrap(), "typedef Edge = { src: Node, dst: Node }");
    assert_eq!(
        item_type.into_ddlog_format().unwrap(), 
        "typedef Item = Item_Node { t: Node } | Item_Edge { t: Edge }"
    );

    let r1 = graph.meta_info().rules().get(0).unwrap().clone();
    let t1 = r1.head().get(0).unwrap().clone();
    assert_eq!(
        convert_term(t1), 
        "TwoEdge{ Edge{ Node{ 1 }, b }, Edge{ c, Node{ 2 } } }"
    );
    
    let (body_str, 
        pre_aggr_rules, 
        neg_bool_const_rules, 
        setcompre_head_union_types, 
        setcompre_head_union_relations,
        _
    ) = convert_rule_body(0, &r1);
    println!("{}", body_str);
    println!("{:?}", pre_aggr_rules);
    println!("{:?}", setcompre_head_union_types);
    println!("{:?}", setcompre_head_union_relations);
}
