use std::collections::*;

use crate::expression::{BasicExprOps, SetComprehension, SetCompreOp};
use crate::module::*;
use crate::parser::combinator::parse_program;
use crate::term::*;
use crate::type_system::*;
use crate::rule::Rule;


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
                        format!("typedef {} = {{ {} }}", self.type_id(), args_str)
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
                        let subtype_strs: Vec<String> = union_type.subtypes.iter().map(|subtype| {
                            let subtype_name = subtype.type_id();
                            format!("{}_{} {{ t: {} }}", union_name, subtype_name, subtype_name)
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

impl DDLogFormat for AtomicTerm {
    fn into_ddlog_format(&self) -> Option<String> {
        let str = match self {
            AtomicTerm::Composite(composite) => {
                let type_name = composite.sort.type_id();
                let subterm_strs: Vec<String> = composite.arguments.iter().map(|x| {
                    x.into_ddlog_format().unwrap()
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
        Some(str)
    }
}


fn convert_rule_body(rule: Rule) -> (String, Vec<String>, Vec<String>, Vec<String>) {
    let pos_strs: Vec<String> = rule.positive_terms().into_iter().map(|term| {
        format!("{}[{}]", term.type_id(), term.into_ddlog_format().unwrap())
    }).collect();
    let pos_preds_str = pos_strs.join(", ");

    // Negation to set comprehension
    let neg_setcompre_strs: Vec<String> = rule.negated_setcompre_terms().into_iter().enumerate()
    .map(|(i, term)| {
        let neg_vars = term.variables();
        let neg_var_strs: Vec<String> = neg_vars.iter().map(|x| format!("{}", x)).collect();
        let neg_vars_str = neg_var_strs.join(", ");
        format!("var g{} = ({}).group_by(()), var count{} = g.group_count(), count{} == 0",
            i, neg_vars_str, i, i
        )
    }).collect();

    // Negation to set difference
    let neg_setdiff_strs: Vec<String> = rule.negated_setdiff_terms().into_iter().map(|term| {
        format!("not {}[{}]", term.type_id(), term.into_ddlog_format().unwrap())
    }).collect();
       
    let mut pre_aggr_rules = vec![];
    let mut setcompre_head_union_types = vec![];
    let mut setcompre_head_union_relations = vec![];
    let mut post_aggr_rule_body_strs = vec![];
    // Assume each set comprehension is defined by a variable
    for (i, (var, setcompre)) in rule.setcompre_map().iter().enumerate() {
        let aggr_result_name = format!("{}", var);
        let union_name = format!("SetcompreHeadUnion{}", i);
        let (union_type_str, container_relation_str, pre_aggr_rule_str, post_aggr_rule_body_str) = 
        convert_setcompre(union_name, aggr_result_name, HashSet::new(), setcompre.clone());
        pre_aggr_rules.push(pre_aggr_rule_str);
        setcompre_head_union_types.push(union_type_str);
        setcompre_head_union_relations.push(container_relation_str);
        post_aggr_rule_body_strs.push(post_aggr_rule_body_str);
    }

    let body_str = format!("{}, {}, {}, {}.", 
        pos_preds_str, 
        neg_setdiff_strs.join(", "),
        neg_setcompre_strs.join(", "),
        post_aggr_rule_body_strs.join(", ")
    );

    (body_str, pre_aggr_rules, setcompre_head_union_types, setcompre_head_union_relations)
}

fn convert_setcompre_head(union_name: String, container_name: String, head: Vec<AtomicTerm>) 
-> (String, String) {
    // TODO: Add constants or do we have to? 
    let mut type_names = HashSet::new();
    for term in head.iter() {
        // If the term is a variable, it must have a valid type assigned to it.
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
        format!("{}[{} {{ {} }}]", container_name, container_name, term)
    }).collect(); 
    let container_head_str = container_strs.join(",");
    (union_type_str, container_head_str)
}

fn convert_setcompre(
    union_name: String,
    aggr_result_name: String, 
    outer_vars: HashSet<AtomicTerm>, 
    setcompre: SetComprehension) 
-> (String, String, String, String) {
    let container_name = format!("{}Container", union_name);
    let head = setcompre.vars.clone();
    let (union_type_str, container_head_str) = convert_setcompre_head(
        union_name, 
        container_name.clone(), 
        head
    );

    let container_relation_str = format!("output relation {} [{}]", container_name, container_name);
    let setcompre_rule: Rule = setcompre.clone().into();
    let (rule_body_str, _, _, _) = convert_rule_body(setcompre_rule);
    let pre_aggr_rule_str = format!("{} :- {}", container_head_str, rule_body_str);

    let op_str = match setcompre.op {
        SetCompreOp::Sum => "group_sum()",
        _ => "group_count()"
    };
    let post_aggr_rule_body_str = format!("{}[{}{{u}}], var g = u.group_by(()), var {} = g.{}", 
        container_name, 
        container_name,
        aggr_result_name,
        op_str
    );
    (union_type_str, container_relation_str, pre_aggr_rule_str, post_aggr_rule_body_str)
}

#[test]
fn test_convert_type() {
    let path = std::path::Path::new("./tests/testcase/p0.4ml");
    let content = std::fs::read_to_string(path).unwrap() + "EOF";
    let (_, program_ast) = parse_program(&content);
        
    let env: Env = program_ast.build_env();
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
        t1.into_ddlog_format().unwrap(), 
        "TwoEdge{ Edge{ Node{ 1 }, b }, Edge{ c, Node{ 2 } } }"
    );
    
    let (body_str, pre_aggr_rules, setcompre_head_union_types, setcompre_head_union_relations) =
        convert_rule_body(r1);
    println!("{}", body_str);
    println!("{:?}", pre_aggr_rules);
    println!("{:?}", setcompre_head_union_types);
    println!("{:?}", setcompre_head_union_relations);
}
