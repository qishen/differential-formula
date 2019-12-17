extern crate num;
use crate::term::*;
use crate::expression::*;
use crate::type_system::*;

use std::str::FromStr;
use std::sync::Arc;
use std::collections::*;
use nom::IResult;
use nom::character::streaming::*;
use nom::number::complete::*;
use num::*;
use enum_dispatch::enum_dispatch;


#[enum_dispatch(TermAst)]
trait TermAstBehavior {}

impl TermAstBehavior for CompositeTermAst {}
impl TermAstBehavior for Term {}

#[enum_dispatch]
#[derive(Debug, Clone)]
enum TermAst {
    CompositeTermAst,
    Term, // It only represents Atom or Variable.
}

impl TermAst {
    fn to_term(&self, domain: &Domain) -> Term {
        match self {
            TermAst::CompositeTermAst(cterm_ast) => {
                let mut term_arguments = vec![];
                for argument in cterm_ast.arguments.clone() {
                    let term = argument.to_term(domain);
                    term_arguments.push(Arc::new(term));
                }

                let sort = domain.get_type(&cterm_ast.name);

                Composite {
                    sort,
                    arguments: term_arguments,
                    alias: cterm_ast.alias.clone(),
                }.into()
            },
            TermAst::Term(term) => {
                term.clone()
            }
        }
    }
}

#[derive(Debug, Clone)]
struct CompositeTermAst {
    name: String,
    arguments: Vec<Box<TermAst>>,
    alias: Option<String>
}


#[enum_dispatch(TypeDefAst)]
trait TypeDefAstBehavior {
    fn name(&self) -> Option<String>;
}

#[enum_dispatch]
#[derive(Debug, Clone)]
enum TypeDefAst {
    AliasTypeDefAst,
    CompositeTypeDefAst,
    UnionTypeDefAst,
    RangeTypeDefAst,
    EnumTypeDefAst,
}


#[derive(Debug, Clone)]
struct AliasTypeDefAst {
    name: String,
}

impl TypeDefAstBehavior for AliasTypeDefAst {
    fn name(&self) -> Option<String> {
        Some(self.name.clone())
    }
}


// e.g. Edge ::= new(src: Node, dst: Node).
#[derive(Debug, Clone)]
struct CompositeTypeDefAst {
    name: String,
    args: Vec<(String, Box<TypeDefAst>)>,
}

impl TypeDefAstBehavior for CompositeTypeDefAst {
    fn name(&self) -> Option<String> {
        Some(self.name.clone())
    }
}


// X ::= A + B + C.
#[derive(Debug, Clone)]
struct UnionTypeDefAst {
    name: Option<String>,
    subtypes: Vec<Box<TypeDefAst>>,
}

impl TypeDefAstBehavior for UnionTypeDefAst {
    fn name(&self) -> Option<String> {
        self.name.clone()
    }
}


// Range ::= {0..100}.
#[derive(Debug, Clone)]
struct RangeTypeDefAst {
    name: Option<String>,
    low: String,
    high: String,
}

impl TypeDefAstBehavior for RangeTypeDefAst {
    fn name(&self) -> Option<String> {
        self.name.clone()
    }
}


// Enum ::= {"HELLO", "WORLD", 2, 1.23}
#[derive(Debug, Clone)]
struct EnumTypeDefAst {
    name: Option<String>,
    enums: Vec<String>,
}

impl TypeDefAstBehavior for EnumTypeDefAst {
    fn name(&self) -> Option<String> {
        self.name.clone()
    }
}




named!(id<&str, String>,
    map!(tuple!(alpha1, alphanumeric0), |(x, y)| { x.to_string() + y })
);

// The first letter has to be alpha and the rest be alphanumeric.
named!(typename<&str, TypeDefAst>,
    map!(tuple!(alpha1, alphanumeric0), |(x, y)| { 
        let name = x.to_string() + y;
        AliasTypeDefAst { name: name }.into() 
    })
);

named!(tagged_typename<&str, (String, TypeDefAst)>,
    alt!(
        map!(
            tuple!(id, delimited!(space0, tag!(":"), space0), typename),
            |(id_str, sep, t)| {
                (id_str.to_string(), t)
            } 
        ) |
        map!(typename, |t| { 
            ("".to_string(), t) 
        })
    )
);



named!(composite_typedef<&str, (String, TypeDefAst)>,
    do_parse!(
        t: id >>
        delimited!(space0, tag!("::="), space0) >>
        opt!(tag!("new")) >>
        args: delimited!(
            tag!("("), 
            separated_list!(tag!(","), delimited!(space0, tagged_typename, space0)),    
            tag!(").")
        ) >>

        ((t.clone(), parse_composite_typedef(t, args)))
    )
);

fn parse_composite_typedef(t: String, args: Vec<(String, TypeDefAst)>) -> TypeDefAst {
    let mut boxed_args = vec![];
    for (id, typedef) in args {
        boxed_args.push((id, Box::new(typedef)));
    }

    CompositeTypeDefAst {
        name: t,
        args: boxed_args,
    }.into()
}



named!(union_typedef<&str, (String, TypeDefAst)>,
    do_parse!(
        t: id >>
        delimited!(space0, tag!("::="), space0) >>
        subs: separated_list!(tag!("+"), delimited!(space0, typename, space0)) >>
        tag!(".") >>
        ((t.clone(), parse_union_typedef(t, subs)))
    )
);

fn parse_union_typedef(t: String, subtypes: Vec<TypeDefAst>) -> TypeDefAst {
    let mut boxed_subtypes = vec![];
    for subtype in subtypes {
        boxed_subtypes.push(Box::new(subtype));
    }

    UnionTypeDefAst {
        name: Some(t),
        subtypes: boxed_subtypes,
    }.into()
}


named!(atom_typedef,
    alt!(tag!("String") | tag!("Integer") | tag!("Boolean"))
);


named!(domain_types<&str, HashMap<String, Type>>,
    map!(many0!(
            delimited!(multispace0, alt!(composite_typedef | union_typedef), multispace0)
        ), |typedefs| {
        let mut ast_map = HashMap::new();
        let mut type_map = HashMap::new();
        type_map.insert("String".to_string(), BaseType::String.into());
        type_map.insert("Integer".to_string(), BaseType::Integer.into());
        type_map.insert("Boolean".to_string(), BaseType::Boolean.into());
        
        // Put all typedef AST into a hash map.
        for (t, typedef) in typedefs {
            ast_map.insert(t, typedef);
        }
        
        // Recursively create all types found in AST.
        for k in ast_map.keys() {
            let t = create_type(k.clone(), &ast_map, &mut type_map);
        }

        type_map
    })
);

fn create_type(t: String, ast_map: &HashMap<String, TypeDefAst>, type_map: &mut HashMap<String, Type>) -> Option<Type> {
    if !type_map.contains_key(&t) {
        let v = ast_map.get(&t).unwrap();
        let new_type = match v {
            TypeDefAst::AliasTypeDefAst(atypedef) => {
                let alias = atypedef.name.clone();
                create_type(alias, ast_map, type_map)
            },

            TypeDefAst::CompositeTypeDefAst(ctypedef) => {
                let mut args = vec![];
                let typename = ctypedef.name().unwrap();
                for (id, arg_ast) in ctypedef.args.iter() {
                    let name = arg_ast.name().unwrap();
                    let subtype_opt = type_map.get(&name);
                    let subtype = match subtype_opt {
                        Some(t) => { t.clone() },
                        None => { 
                            create_type(name, ast_map, type_map).unwrap() 
                        }
                    };
                    let mut id_opt = None;
                    if id != "" {
                        id_opt = Some(id.clone());
                    }
                    args.push((id_opt, subtype));
                }

                let ctype: Type = CompositeType {
                    name: typename,
                    arguments: args,
                }.into();

                Some(ctype)
            },

            TypeDefAst::UnionTypeDefAst(utypedef) => {
                let mut subtypes = vec![];
                let typename = utypedef.name().unwrap();
                for subtype_ast in utypedef.subtypes.iter() {
                    let name = subtype_ast.name().unwrap();
                    let subtype_opt = type_map.get(&name);
                    let subtype = match subtype_opt {
                        Some(t) => { t.clone() },
                        None => { create_type(name, ast_map, type_map).unwrap() }
                    };
                    subtypes.push(subtype);
                }

                let utype: Type = UnionType {
                    name: typename,
                    subtypes: subtypes,
                }.into();

                Some(utype)
            },
            _ => { None }
        };

        // Add new type to the type map.
        type_map.insert(t, new_type.clone().unwrap());

        return new_type;

    } else {
        let type_in_map = type_map.get(&t).unwrap();
        return Some(type_in_map.clone());
    }
}

#[derive(Clone, Debug)]
enum ProgramAst {
    Domain(DomainAst),
    Model(ModelAst),
}

#[derive(Clone, Debug)]
struct DomainAst {
    name: String,
    type_map: HashMap<String, Type>,
}

#[derive(Clone, Debug)]
struct ModelAst {
    model_name: String,
    domain_name: String,
    models: Vec<TermAst>,
}


named!(domain<&str, ProgramAst>, 
    do_parse!(
        tag!("domain") >>
        domain_name: delimited!(space0, id, space0) >>
        typedefs: delimited!(
            tag!("{"),
            delimited!(multispace0, domain_types, multispace0), 
            tag!("}")
        ) >>
        (ProgramAst::Domain(
            DomainAst {
                name: domain_name,
                type_map: typedefs,
            }
        ))
    )
);


named!(model<&str, ProgramAst>, 
    do_parse!(
        tag!("model") >>
        model_name: delimited!(multispace0, id, multispace0) >>
        tag!("of") >>
        domain_name: delimited!(multispace0, id, multispace0) >>
        models: delimited!(
            tag!("{"),
            many0!(
                delimited!(multispace0, composite, multispace0)
            ),
            tag!("}")
        ) >>
        (ProgramAst::Model(
            ModelAst {
                model_name,
                domain_name,
                models
            }
        ))
    )
);


// Return a domain map and a model map at the end of parsing.
named!(program<&str, Env>,
    map!(
        many0!(
            delimited!(multispace0, alt!(domain | model), multispace0)
        ),
        |list| {
        // filter them into domain, model and transformation categories.
        let mut domain_asts = vec![];
        let mut model_asts = vec![];
        for x in list {
            match x {
                ProgramAst::Domain(domain_ast) => {
                    domain_asts.push(domain_ast);
                },
                
                ProgramAst::Model(model_ast) => {
                    model_asts.push(model_ast);
                }
            }
        }

        let mut domain_map = HashMap::new();
        for domain_ast in domain_asts {
            let mut arc_type_map = HashMap::new(); 
            let domain_name = domain_ast.name;

            for (id, t) in domain_ast.type_map {
                arc_type_map.insert(id, Arc::new(t));
            }

            let domain = Domain {
                name: domain_name.clone(),
                type_map: arc_type_map,
            };

            domain_map.insert(domain_name, domain);
        }

        let mut model_map = HashMap::new();
        for model_ast in model_asts {
            // Get the domain that current model requires.
            let domain = domain_map.get(&model_ast.domain_name).unwrap();
            let model_name = model_ast.model_name;
            let mut model_store = vec![];
            for term_ast in model_ast.models {
                // Convert AST into term according to its type.
                let term = term_ast.to_term(domain);
                model_store.push(term);
            }

            let model = Model {
                model_name: model_name.clone(),
                domain_name: model_ast.domain_name,
                models: model_store,
            };

            model_map.insert(model_name, model);
        }

        Env {
            domain_map,
            model_map,
        }
    })
);


named!(composite<&str, TermAst>, 
    do_parse!(
        t: id >>
        args: delimited!(
            char!('('), 
            separated_list!(
                tag!(","), 
                delimited!(
                    space0, 
                    alt!(
                        composite | 
                        map!(atom, |x| { x.into() }) |
                        map!(variable, |x| { x.into() })
                    ), 
                    space0
                )
            ), 
            char!(')')
        ) >>
        (parse_term(t, args))
    )
);

fn parse_term(sort: String, args: Vec<TermAst>) -> TermAst {
    CompositeTermAst {
        name: sort.to_string(),
        arguments: args.into_iter().map(|x| Box::new(x)).collect(),
        alias: None,
    }.into()
}


// Underscore and quote sign at the end is allowed in variable name.
named!(varname<&str, String>, 
    alt!(
        map!(tag!("_"), |x| { x.to_string() }) |
        map!(
            tuple!(
                alpha1,
                many0!(alt!(alphanumeric1 | tag!("_"))),
                many0!(tag!("'"))
            ),
            |(a, b, c)| {
                let mut final_str = a.to_string();
                for s in b { final_str.push_str(s); }
                for s in c { final_str.push_str(s); }
                final_str
            }
        )
    )
);

named!(variable<&str, Term>,
    do_parse!(
        var: varname >>
        fragments: separated_list!(tag!("."), id) >>
        (parse_variable(var, fragments))
    )
);

fn parse_variable(var: String, fragments: Vec<String>) -> Term {
    Variable::new(var, fragments).into()
}

named!(atom<&str, Term>,
    alt!(atom_string | atom_bool | atom_integer | atom_float)
);

named!(atom_integer<&str, Term>,
    map!(
        tuple!(opt!(alt!(char!('+') | char!('-'))), digit1),
        |(sign, num_str)| {
            let num = match sign {
                Some(sign_char) => { sign_char.to_string() + &num_str.to_string() },
                None => { num_str.to_string() }
            };

            Atom::Int(BigInt::from_str(&num[..]).unwrap()).into() 
        }
    )
);

named!(atom_string<&str, Term>,
    map!(
        delimited!(char!('"'), alphanumeric0, char!('"')), 
        |atom_str| { Atom::Str(atom_str.to_string()).into() }
    )
);

named!(atom_float<&str, Term>,
    map!(
        float, 
        |float_num| { Atom::Float(BigRational::from_f32(float_num).unwrap()).into() }
    )
);

named!(atom_bool<&str, Term>,
    map!(
        alt!(tag!("true") | tag!("false")), 
        |x| {
            match x {
                "true" => Atom::Bool(true).into(),
                _ => Atom::Bool(false).into(),
            }
        }
    )
);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_term() {
        assert_eq!(composite("Node(\"hi\")ttt").unwrap().0, "ttt");
        assert_eq!(atom("\"helloworld\"").unwrap().0, "");
        //assert_eq!(atom("123E-02").unwrap().0, "");
        assert_eq!(atom("-11223344 ").unwrap().0, " ");
        //assert_eq!(typename("3aab2c,").unwrap().0, ",");
        assert_eq!(variable("hello_world''',").unwrap().0, ",");
        assert_eq!(composite_typedef("Edge ::= new(src: Node, dst : Node ).").unwrap().0, "");
        union_typedef("X  ::= A + B + C+  D  .");
        composite("Edge(node1 , Node(\"hello\"))");

        let output = domain(
            "domain graph { 
                Node ::= new(id: String). 
                Edge ::= new(src:Node, dst: Node). 
                Item ::= Node + Edge.
            }"
        );

        let output1 = model(
            "model m of Graph {
                Node(\"helloworld\")
                Edge(Node(\"hello\"), Node(\"world\"))
            }"
        );

        let output2 = program(
            "
            domain Graph { 
                Node ::= new(id: String). 
                Edge ::= new(src:Node, dst: Node). 
                Item ::= Node + Edge.
            }
            
            model m of Graph {
                Node(\"helloworld\")
                Edge(Node(\"hello\"), Node(\"world\"))
            }

            pp
            "
        );

        println!("{:?}", output2);
    }

}

