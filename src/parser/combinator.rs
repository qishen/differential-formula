use crate::parser::ast::*;
use crate::constraint::*;
use crate::module::*;
use crate::term::*;
use crate::expression::*;
use crate::type_system::*;

use std::fs;
use std::path::Path;
use std::convert::TryInto;
use std::str::FromStr;
use std::sync::Arc;
use std::collections::*;

use nom::character::*;
use nom::character::complete::*;
use nom::number::complete::*;

use num::*;


// Export this function to parse FORMULA file in string format.
pub fn load_program(content: String) -> Env {
    let result = program(&content[..]).unwrap();
    // Make sure the whole file is parsed rather than part of the program.
    assert_eq!(result.0, "EOF");
    // println!("{:?}", result.0);
    let program_ast = result.1;
    program_ast.build_env()
}

// Start with '//' and end with '\n'
named!(comment<&str, &str>, 
    recognize!(
        delimited!(tag!("//"), many0!(none_of("\n")), tag!("\n"))
    )
);

// Skip is multiples of comment, tab, new line or white space.
named!(skip<&str, &str>,
    recognize!(
        many0!(alt!(comment | multispace1))
    )
);

// The first letter has to be alpha and the rest be alphanumeric.
named!(id<&str, String>,
    map!(recognize!(tuple!(alpha1, alphanumeric0)), |x| { x.to_string() })
);

// Just treat it like a variable term despite the % sign.
// In model transformation it will be replaced with another term.
named!(param_id<&str, Term>,
    do_parse!(
        tag!("%") >>
        pid: id >>
        (Variable::new(pid, vec![]).into())
    )
);

// Same pattern as `id` but return ast instead of string.
// Two types of typename: 
// 1. Native type e.g. Node or Edge.
// 2. Chained renamed type alias e.g. left.Node or right.x.y.z.Node
named!(typename<&str, TypeDefAst>,
    map!(separated_nonempty_list!(tag!("."), id), |mut names| {
        let name = names.remove(names.len()-1);
        AliasTypeDefAst {
            chained_scopes: names,
            name,
        }.into()
    })
);

// Two types of typename: 1. Node ::= new (id: String). 2. Node ::= new (String).
named!(tagged_typename<&str, (Option<String>, TypeDefAst)>,
    alt!(
        map!(
            tuple!(id, delimited!(space0, tag!(":"), space0), typename),
            |(id_str, sep, t)| {
                (Some(id_str.to_string()), t)
            } 
        ) |
        map!(typename, |t| { 
            (None, t) 
        })
    )
);

named!(composite_typedef<&str, TypeDefAst>,
    do_parse!(
        t: id >>
        delimited!(space0, tag!("::="), space0) >>
        opt!(delimited!(space0, tag!("new"), space0)) >>
        args: delimited!(
            delimited!(space0, tag!("("), space0), 
            separated_list!(delimited!(space0, tag!(","), space0), tagged_typename),    
            delimited!(space0, tag!(")"), space0)
        ) >>
        dot: tag!(".") >>
        (parse_composite_typedef(t, args))
    )
);

fn parse_composite_typedef(t: String, args: Vec<(Option<String>, TypeDefAst)>) -> TypeDefAst {
    let mut boxed_args = vec![];
    for (id, typedef) in args {
        boxed_args.push((id, Box::new(typedef)));
    }

    CompositeTypeDefAst {
        name: t,
        args: boxed_args,
    }.into()
}

named!(union_typedef<&str, TypeDefAst>,
    do_parse!(
        t: id >>
        delimited!(space0, tag!("::="), space0) >>
        subs: separated_list!(tag!("+"), delimited!(space0, typename, space0)) >>
        tag!(".") >>
        (parse_union_typedef(t, subs))
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

// The conformance rule is treated as a headless rule.
named!(conformance<&str, RuleAst>,
    do_parse!(
        tag!("conforms") >>
        skip >>
        body: separated_list!(
            delimited!(skip, tag!(","), skip),
            constraint
        ) >>
        terminated!(skip, tag!(".")) >>
        (parse_rule(vec![], body))
    )
);

// Comments are allowed between constraints in both head and body.
named!(rule<&str, RuleAst>,
    do_parse!(
        head: separated_list!(
            delimited!(skip, tag!(","), skip), 
            alt!(composite | variable_ast)
        ) >>
        delimited!(skip, tag!(":-"), skip) >>
        body: separated_list!(
            delimited!(skip, tag!(","), skip),
            constraint
        ) >>
        terminated!(skip, tag!(".")) >>
        (parse_rule(head, body))
    )
);

fn parse_rule(head: Vec<TermAst>, body: Vec<ConstraintAst>) -> RuleAst {
    RuleAst {
        head,
        body,
    }
}

// Two types of inheritances:
// 1. domain X extends Y, Z {}. 
// 2. domain Y extends left:: Y, right:: Y {}.
named!(subdomain<&str, (Option<String>, String)>,
    alt!(
        map!(
            tuple!(id, delimited!(space0, tag!("::"), space0), id), 
            |(scope, sep, domain)| { (Some(scope), domain) } 
        ) |
        map!(id, |t| { (None, t) })
    )
);

named!(module_sentence<&str, ModuleSentenceAst>,
    alt!(
        map!(alt!(rule | conformance), |x| { ModuleSentenceAst::Rule(x) }) |
        map!(alt!(composite_typedef | union_typedef), |x| { ModuleSentenceAst::Type(x) }) |
        map!(
            tuple!(term, terminated!(skip, tag!("."))), |(x, _)| { ModuleSentenceAst::Term(x) }
        )
    )
);

named!(
    domain<&str, ModuleAst>, 
    do_parse!(
        tag!("domain") >>
        domain_name: delimited!(multispace0, id, multispace0) >>
        subdomains_data: opt!(tuple!(
            map!(
                delimited!(
                    space0,
                    alt!(tag!("includes") | tag!("extends")),
                    space0
                ), 
                |x| { x.to_string() }
            ),
            separated_list!(delimited!(space0, tag!(","), space0), subdomain)
        )) >>
        skip >>
        tag!("{") >>
        skip >>
        sentences: separated_list!(skip, module_sentence) >>
        skip >>
        tag!("}") >>
        (parse_domain(domain_name, sentences, subdomains_data))
    )
);

fn parse_domain(
    domain_name: String, 
    sentences: Vec<ModuleSentenceAst>,
    subdomains_opt: Option<(String, Vec<(Option<String>, String)>)>
) -> ModuleAst {
    let mut inherit_type = "None".to_string();
    let mut subdomains = vec![];
    let mut renamed_subdomains = HashMap::new();

    if let Some(subs) = subdomains_opt {
        inherit_type = subs.0;
        for (scope_opt, name) in subs.1 {
            if let Some(scope) = scope_opt {
                renamed_subdomains.insert(scope, name);
            } else {
                subdomains.push(name);
            }
        }
    };

    let mut rules = vec![];
    let mut typedefs = vec![];

    for sentence in sentences {
        match sentence {
            ModuleSentenceAst::Rule(r) => { rules.push(r); },
            ModuleSentenceAst::Type(t) => { typedefs.push(t); },
            _ => {} // terms are not allowed in domain.
        }
    }

    let domain_ast = DomainAst {
        name: domain_name,
        types: typedefs,
        rules: rules,
        inherit_type,
        subdomains,
        renamed_subdomains,
    };

    ModuleAst::Domain(domain_ast)
}

named!(model<&str, ModuleAst>, 
    do_parse!(
        tag!("model") >>
        model_name: delimited!(multispace0, id, multispace0) >>
        tag!("of") >>
        domain_name: delimited!(multispace0, id, multispace0) >>
        skip >>
        submodels_data: opt!(tuple!(
            map!(
                delimited!(
                    space0,
                    alt!(tag!("includes") | tag!("extends")),
                    space0
                ), 
                |x| { x.to_string() }
            ),
            // Use `subdomain` to parse inheritance because the syntax is the same as domain inheritance.
            separated_list!(delimited!(space0, tag!(","), space0), subdomain)
        )) >>
        skip >>
        models: delimited!(
            tag!("{"),
            delimited!(
                skip,
                many0!(
                    delimited!(
                        skip,
                        terminated!(composite, delimited!(multispace0, tag!("."), multispace0)), 
                        skip
                    )
                ),
                skip
            ),
            tag!("}")
        ) >>
        (parse_model(model_name, domain_name, models, submodels_data))
    )
);

fn parse_model(
    model_name: String, domain_name: String,
    models: Vec<TermAst>,
    submodels_opt: Option<(String, Vec<(Option<String>, String)>)>
) -> ModuleAst 
{
    let mut inherit_type = "None".to_string();
    let mut submodels = vec![];
    let mut renamed_submodels = HashMap::new();

    if let Some(subs) = submodels_opt {
        inherit_type = subs.0;
        for (scope_opt, name) in subs.1 {
            if let Some(scope) = scope_opt {
                renamed_submodels.insert(scope, name);
            } else {
                submodels.push(name);
            }
        }
    };

    let model_ast = ModelAst {
        model_name,
        domain_name,
        models,
        inherit_type,
        submodels,
        renamed_submodels,
    };

    ModuleAst::Model(model_ast)
}

named!(tagged_domain<&str, TaggedDomainAst>,
    do_parse!(
        tag: id >>
        delimited!(space0, tag!("::"), space0) >>
        domain: id >>
        (TaggedDomainAst { tag, domain })
    )
);

// transform parameter is either id:: Domain or id: Term.
named!(transform_param<&str, TransformParamAst>,
    alt!(
        map!(tagged_domain, |x| { 
            TransformParamAst::TaggedDomain(x) 
        }) | 
        map!(tagged_typename, |x| {
            let tag = x.0.unwrap(); // tag cannot be none here.
            let formula_type = x.1;
            TransformParamAst::TaggedType(
                TaggedTypeAst { tag, formula_type }
            ) 
        })
    )
);

// Parse model transformation command in cmd e.g. r = SimpleCopy(V(1), m1) and model name
// will be treated as variable term here.
named!(pub transformation<&str, TransformationAst>,
    do_parse!(
        result_name: id >>
        delimited!(space0, tag!("="), space0) >>
        transform_name: id >>
        tag!("(") >>
        params: separated_list!(delimited!(space0, tag!(","), space0), term) >>
        tag!(")") >>
        (TransformationAst { result_name, transform_name, params })
    )
);

named!(transform<&str, ModuleAst>, 
    do_parse!(
        tag!("transform") >>
        transform_name: delimited!(multispace0, id, multispace0) >>
        tag!("(") >>
        inputs: separated_list!(delimited!(space0, tag!(","), space0), transform_param) >>
        tag!(")") >>
        skip >>
        tag!("returns") >>
        skip >>
        output: delimited!(
            tag!("("), 
            separated_list!(delimited!(space0, tag!(","), space0), tagged_domain),
            tag!(")")
        ) >>
        skip >>
        tag!("{") >>
        skip >>
        sentences: separated_list!(skip, module_sentence) >>
        skip >>
        tag!("}") >>
        (parse_transform(transform_name, inputs, output, sentences))
    )
);

fn parse_transform(
    transform_name: String, 
    inputs: Vec<TransformParamAst>,
    output: Vec<TaggedDomainAst>, 
    sentences: Vec<ModuleSentenceAst>,
) -> ModuleAst
{
    let mut rules = vec![];
    let mut typedefs = vec![];
    let mut terms = vec![];

    for sentence in sentences {
        match sentence {
            ModuleSentenceAst::Rule(r) => { rules.push(r); },
            ModuleSentenceAst::Type(t) => { typedefs.push(t); },
            ModuleSentenceAst::Term(term) => { terms.push(term); } 
        }
    }
    
    let transform_ast = TransformAst {
        transform_name,
        inputs,
        output,
        typedefs,
        rules,
        terms,
    };

    ModuleAst::Transform(transform_ast)
}

named!(program<&str, ProgramAst>,
    map!(many0!(
        delimited!(skip, alt!(domain | model | transform), skip)
    ), |modules| {
        let mut domain_ast_map = HashMap::new();
        let mut model_ast_map = HashMap::new();
        let mut transform_ast_map = HashMap::new();

        for module in modules {
            match module {
                ModuleAst::Domain(domain_ast) => {
                    domain_ast_map.insert(domain_ast.name.clone(), domain_ast);
                },
                ModuleAst::Model(model_ast) => {
                    model_ast_map.insert(model_ast.model_name.clone(), model_ast);
                },
                ModuleAst::Transform(transform_ast) => {
                    transform_ast_map.insert(transform_ast.transform_name.clone(), transform_ast);
                }
            }
        }

        ProgramAst {
            domain_ast_map,
            model_ast_map,
            transform_ast_map,
        }
    })
);

named!(bin_op<&str, BinOp>,
    map!(alt!(tag!("=") | tag!("!=") | tag!(">") | tag!(">=") | tag!("<") | tag!("<=")), |x| {
        match x {
            "="  => BinOp::Eq,
            "!=" => BinOp::Ne,
            ">"  => BinOp::Gt,
            ">=" => BinOp::Ge,
            "<"  => BinOp::Lt,
            _    => BinOp::Le,
        }
    })
);

named!(arith_op<&str, ArithmeticOp>,
    map!(alt!(tag!("+") | tag!("-") | tag!("*") | tag!("/")), |x| {
        match x {
            "+" => ArithmeticOp::Add,
            "-" => ArithmeticOp::Min,
            "*" => ArithmeticOp::Mul,
            _   => ArithmeticOp::Div,
        }
    })
);

named!(setcompre_op<&str, SetCompreOp>,
    map!(alt!(
        tag!("count") | tag!("sum") | tag!("minAll") | 
        tag!("maxAll") | tag!("topK") | tag!("bottomK")), |x| {
        match x {
            "count" => SetCompreOp::Count,
            "sum" => SetCompreOp::Sum,
            "minAll" => SetCompreOp::MinAll,
            "maxAll" => SetCompreOp::MaxAll,
            "topK" => SetCompreOp::TopK,
            _ => SetCompreOp::BottomK,
        }
    })
);

named!(base_expr<&str, ExprAst>,
    alt!(
        map!(setcompre, |x| { 
            let base_expr: BaseExprAst = x.into(); 
            base_expr.into()
        }) |
        // Can only be either atom of numeric value or variable that represent numeric value.
        map!(alt!(param_id | variable | atom), |term| { 
            let term_ast: TermAst = term.into();
            let base_expr: BaseExprAst = term_ast.into();
            base_expr.into()
        }) | 
        parens_arith_expr
    )
);

named!(parens_arith_expr<&str, ExprAst>,
    do_parse!(
        delimited!(space0, tag!("("), space0) >>
        expr: arith_expr_low >>
        preceded!(space0, tag!(")")) >>
        (expr.into())
    )
);

named!(mul_div_op<&str, ArithmeticOp>,
    map!(alt!(tag!("*") | tag!("/")), |x| {
        match x {
            "*" => ArithmeticOp::Mul,
            _   => ArithmeticOp::Div,
        }
    })
);

named!(plus_minus_op<&str, ArithmeticOp>,
    map!(alt!(tag!("+") | tag!("-")), |x| {
        match x {
            "+" => ArithmeticOp::Add,
            _   => ArithmeticOp::Min,
        }
    })
);

// Mul and Div have higher priority in arithmetic expression.
named!(arith_expr_high<&str, ExprAst>,
    do_parse!(
        base: preceded!(space0, base_expr) >>
        remain: many0!(tuple!(
            preceded!(space0, mul_div_op), 
            preceded!(space0, base_expr)
        )) >>
        (parse_arith_expr(base, remain))
    )
);

// Add and Minus have lower priority in arithmetic expression.
named!(arith_expr_low<&str, ExprAst>,
    do_parse!(
        base: preceded!(space0, arith_expr_high) >>
        remain: many0!(tuple!(
            preceded!(space0, plus_minus_op), 
            preceded!(space0, arith_expr_high)
        )) >>
        (parse_arith_expr(base, remain))
    )
);

fn parse_arith_expr(base: ExprAst, remain: Vec<(ArithmeticOp, ExprAst)>) -> ExprAst {
    let mut left = base;
    for (op, right) in remain {
        let new_left = ArithExprAst {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }.into();
        left = new_left;
    }
    left
}

named!(expr<&str, ExprAst>,
    // Must put base_expr matching after arith_expr matching, otherwise since arith_expr contains 
    // base_expr, parser will stop after finding the first match of base expression.
    alt!(arith_expr_low | base_expr)
);

named!(setcompre<&str, SetComprehensionAst>, 
    do_parse!(
        op: setcompre_op >>
        tag!("(") >>
        default: opt!(terminated!(
            delimited!(space0, atom, space0), 
            terminated!(tag!(","), space0))) >>
        tag!("{") >>
        vars: separated_list!(tag!(","), 
            delimited!(space0, alt!(composite | variable_ast), space0)
        ) >>
        delimited!(space0, tag!("|"), space0) >>
        condition: separated_list!(tag!(","), 
            delimited!(space0, constraint, space0)
        ) >>
        tag!("}") >>
        tag!(")") >>
        (parse_setcompre(vars, condition, op, default))
    )
);

fn parse_setcompre(
    vars: Vec<TermAst>, 
    condition: Vec<ConstraintAst>, 
    op: SetCompreOp, 
    default: Option<Term>
) -> SetComprehensionAst {
    let default_value = match default {
        None => None,
        Some(term) => {
            // The term has to be an integer as default value.
            let atom: Atom = term.try_into().unwrap();
            let num = match atom {
                Atom::Int(num) => Some(num),
                _ => None,
            };

            num
        }
    };

    SetComprehensionAst {
        vars,
        condition,
        op,
        default: default_value,
    }
}

/* 
Something is tricky here that we have to match predicate first because letter 'E'
can be matched as a float number, so any composite term with type name starting with
'E' could be matched in binary.
*/
named!(constraint<&str, ConstraintAst>,
    alt!(
        map!(predicate, |x| { x.into() }) |
        map!(binary, |x| { x.into() }) |
        map!(type_constraint, |x| { x.into() })
    )
);

named!(type_constraint<&str, TypeConstraintAst>,
    do_parse!(
        var: variable_ast >>
        op: delimited!(space0, alt!(tag!("is") | tag!(":")), space0) >>
        sort: typename >>
        (parse_type_constraint(var, sort))
    )
);

fn parse_type_constraint(var: TermAst, sort: TypeDefAst) -> TypeConstraintAst{
    TypeConstraintAst {
        var,
        sort
    }
}

named!(binary<&str, BinaryAst>,
    do_parse!(
        left: expr >>
        op: delimited!(space0, bin_op, space0) >>
        right: expr >>
        (parse_binary(op, left, right))
    )
);

fn parse_binary(op: BinOp, left: ExprAst, right: ExprAst) -> BinaryAst {
    BinaryAst {
        op,
        left,
        right,
    }
}

named!(predicate<&str, PredicateAst>,
    do_parse!(
        neg: opt!(delimited!(space0, tag!("no"), space0)) >>
        alias: opt!(terminated!(id, delimited!(space0, tag!("is"), space0))) >>
        term: composite >>
        (parse_predicate(neg, alias, term))
    )
);

fn parse_predicate(neg: Option<&str>, alias: Option<String>, term: TermAst) -> PredicateAst {
    let negated = match neg {
        None => false,
        _ => true,
    };

    PredicateAst {
        negated,
        term,
        alias,
    }
}

// Match composite, variable or atom term.
named!(pub term<&str, TermAst>,
    alt!(
        composite |
        map!(atom, |x| { x.into() }) |
        map!(variable, |x| { x.into() })
    )
);

named!(composite<&str, TermAst>, 
    do_parse!(
        alias: opt!(terminated!(id, delimited!(multispace0, tag!("is"), multispace0))) >>
        t: typename >>
        args: delimited!(
            char!('('), 
            separated_list!(
                tag!(","), 
                delimited!(
                    space0, 
                    alt!(
                        term |
                        // Handle some weird expression like %id only in model transformation.
                        map!(param_id, |x| { x.into() }) 
                    ), 
                    space0
                )
            ), 
            char!(')')
        ) >>
        (parse_composite(alias, t, args))
    )
);

fn parse_composite(alias: Option<String>, sort: TypeDefAst, args: Vec<TermAst>) -> TermAst {
    CompositeTermAst {
        sort,
        arguments: args.into_iter().map(|x| Box::new(x)).collect(),
        alias,
    }.into()
}


// Underscore and quote sign at the end is allowed in variable name.
named!(varname<&str, &str>, 
    complete!(alt!(
        tag!("_") |
        recognize!(tuple!(
            alpha1,
            //take_while!(is_alphanumeric_char),
            //many0!(alphanumeric1),
            many0!(alt!(alphanumeric1 | tag!("_"))),
            many0!(tag!("'"))
        ))
    ))
);

fn is_alphanumeric_char(c: char) -> bool {
    is_alphanumeric(c as u8)
}

named!(variable_ast<&str, TermAst>, 
    map!(variable, |x| {
        x.into()
    })
);

named!(variable<&str, Term>,
    do_parse!(
        var: varname >>
        fragments: opt!(preceded!(tag!("."), separated_list!(tag!("."), id))) >>
        (parse_variable(var, fragments))
    )
);

fn parse_variable(var: &str, fragments: Option<Vec<String>>) -> Term {
    let frags = match fragments {
        None => vec![],
        Some(list) => list,
    };

    Variable::new(var.to_string(), frags).into()
}

named!(atom_ast<&str, TermAst>,
    map!(atom, |x| {
        x.into()
    })
);

named!(atom<&str, Term>,
    alt!(atom_float | atom_integer | atom_bool | atom_string)
);

named!(atom_integer<&str, Term>,
    map!(tuple!(opt!(alt!(char!('+') | char!('-'))), digit1),
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
        delimited!(char!('"'), many0!(none_of("\"")), char!('"')), 
        |char_vec| { 
            let s: String = char_vec.into_iter().collect();
            Atom::Str(s).into() 
        }
    )
);

// Match float but need to exclude integer.
named!(atom_float<&str, Term>,
    map!(
        recognize!(float), 
        |float_str| { 
            if let Ok(i) = BigInt::from_str(float_str) {
                return Atom::Int(i).into();
            } else {
                let num = f32::from_str(float_str).unwrap();
                return Atom::Float(BigRational::from_f32(num).unwrap()).into(); 
            }
        }
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
    fn test_blank() {
        assert_eq!(comment("// comment \n\n").unwrap().0, "\n");
        assert_eq!(skip("// comment \t\n  \n .").unwrap().0, ".");
    }

    #[test]
    fn test_components() {
        assert_eq!(id("xxx").unwrap().0, "");
        assert_eq!(varname("xx ").unwrap().0, " ");
        assert_eq!(varname("xx22' ").unwrap().0, " ");
        assert_eq!(varname("yy22'' ").unwrap().0, " ");
        assert_eq!(varname("_").unwrap().0, "");
        // typename matching won't terminate until it hits char that is not alphanumerical or dot.
        assert_eq!(typename("yyy ").unwrap().0, " ");
        assert_eq!(typename("b3aab2c ").unwrap().0, " ");
        assert_eq!(typename("left.Node ").unwrap().0, " ");
        assert_eq!(tagged_typename("id : Hello ").unwrap().0, " ");
        assert_eq!(tagged_typename("id : right.Hello ").unwrap().0, " ");
    }

    #[test]
    fn test_term() {
        // Don't put number at the end of a rule otherwise '.' will be missing.
        assert_eq!(atom("20.").unwrap().0, "");
        assert_eq!(atom("1.23").unwrap().0, "");
        assert_eq!(atom("true").unwrap().0, "");
        assert_eq!(atom("-122").unwrap().0, "");
        assert_eq!(atom("\"helloworld\"").unwrap().0, "");
        assert_eq!(atom("123E-02").unwrap().0, "");
        assert_eq!(atom("-11223344").unwrap().0, "");
        assert_eq!(variable("hello_world ").unwrap().0, " ");
        assert_eq!(variable_ast("hi ").unwrap().0, " ");
        assert_eq!(variable("a.b.c ").unwrap().0, " ");
        assert_eq!(composite("Edge(node1 , Node(\"hello\"))").unwrap().0, "");
        assert_eq!(composite("Node(\"hi\")").unwrap().0, "");
    }

    #[test]
    fn test_typedef() {
        assert_eq!(composite_typedef("Edge ::= new(src: Node, dst : Node ).").unwrap().0, "");
        assert_eq!(composite_typedef("Edge ::= new(src: Left.Node, dst: Node).").unwrap().0, "");
        assert_eq!(union_typedef("X  ::= A + B + C+  D  .").unwrap().0, "");
    }

    #[test]
    fn test_expr() {
        assert_eq!(base_expr("x ").unwrap().0, " ");
        assert_eq!(
            base_expr("count({a , udge(a, b) | X(m, 11.11),odge(a, a), Edge(b, Node(\"hello\")) })").unwrap().0, 
            "");
        assert_eq!(base_expr("minAll( 1 , {c, d | c is Node(a), d is Node(b)})").unwrap().0, "");
        assert_eq!(expr("a+b*c ,").unwrap().0, " ,");
        assert_eq!(expr("(a+b)*c ,").unwrap().0, " ,");

        let setcompre_str = "maxAll(10, {x | x is X(a, b, c)})";
        let expr1_str = &format!(" ( a + {}  ) /  b .", setcompre_str)[..];
        assert_eq!(expr(expr1_str).unwrap().0, " .");

        // 'E' or 'e' is recognized as variable instead of float.
        let binary1_str = &format!("(b + {}) / x = d + e .", setcompre_str)[..];
        assert_eq!(binary(binary1_str).unwrap().0, " .");
        assert_eq!(binary("aggr * 2 = 20 .").unwrap().0, " .");
    }

    #[test]
    fn test_parse_rules() {
        // rules.txt contains both normal rules and conformance rules.
        let path = Path::new("./tests/testcase/rules.txt");
        let content = fs::read_to_string(path).unwrap();
        let rules = content.split("\n--------\n");
        for formula_rule in rules {
            println!("{:?}", formula_rule);
            assert_eq!(module_sentence(&formula_rule[..]).unwrap().0, "\n--EOF--");
        }
    }

    #[test]
    fn test_parse_domains() {
        let path = Path::new("./tests/testcase/domains.txt");
        let content = fs::read_to_string(path).unwrap();
        let domains = content.split("\n--------\n");
        for formula_domain in domains {
            println!("{:?}", formula_domain);
            assert_eq!(domain(&formula_domain[..]).unwrap().0, "");
        }
    }

    #[test]
    fn test_parse_models() {
        let path = Path::new("./tests/testcase/models.txt");
        let content = fs::read_to_string(path).unwrap();
        let models = content.split("\n--------\n");
        for formula_model in models {
            println!("{:?}", formula_model);
            assert_eq!(model(&formula_model[..]).unwrap().0, "");
        }
    }

    #[test]
    fn test_parse_transformation() {
        let path = Path::new("./tests/testcase/transformations.txt");
        let content = fs::read_to_string(path).unwrap();
        let trans = content.split("\n--------\n");
        for formula_tran in trans {
            println!("{:?}", formula_tran);
            assert_eq!(transform(&formula_tran[..]).unwrap().0, "");
        }
    }

    #[test]
    fn test_parse_programs() {
        let path = Path::new("./tests/testcase/programs.txt");
        let content = fs::read_to_string(path).unwrap();
        let programs = content.split("\n--------\n");
        for formula_program in programs {
            println!("{:?}", formula_program);
            let result = program(&formula_program[..]).unwrap();
            assert_eq!(result.0, "EOF");
            let program_ast = result.1;
            let env = program_ast.build_env();
            //println!("{:#?}", env);
            //println!("{:#?}", env.model_map);
            println!("{:#?}", env.transform_map);
        }
    }

}