extern crate num;

use crate::constraint::*;
use crate::term::*;
use crate::expression::*;
use crate::type_system::*;
use crate::rule::*;

use std::convert::TryInto;
use std::str::FromStr;
use std::sync::Arc;
use std::collections::*;
use nom::character::*;
use nom::character::complete::*;
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
    map!(recognize!(tuple!(alpha1, alphanumeric0)), |x| { x.to_string() })
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


named!(domain_rules<&str, Vec<RuleAst>>,
    many0!(
        delimited!(multispace0, terminated!(rule, tag!(".")), multispace0)
    )
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
    rules: Vec<RuleAst>,
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
        tag!("{") >>
        typedefs: delimited!(multispace0, domain_types, multispace0) >> 
        rules: delimited!(multispace0, domain_rules, multispace0) >>
        tag!("}") >>
        (ProgramAst::Domain(
            DomainAst {
                name: domain_name,
                type_map: typedefs,
                rules,
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
                delimited!(
                    multispace0, 
                    terminated!(composite, delimited!(multispace0, tag!("."), multispace0)), 
                    multispace0
                )
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


// Export this function to parse FORMULA file in string format.
pub fn parse_str(content: &str) -> Env {
    program(content).unwrap().1
}

// Return a domain map and a model map at the end of parsing.
named!(program<&str, Env>,
    map!(
        many0!(
            preceded!(multispace0, alt!(domain | model))
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

            let mut domain = Domain {
                name: domain_name.clone(),
                type_map: arc_type_map,
                rules: vec![],
            };

            // Add rules into domain.
            for rule_ast in domain_ast.rules {
                domain.add_rule(rule_ast.to_rule(&domain));
            }

            domain_map.insert(domain_name, domain);
        }

        let mut model_map = HashMap::new();
        for model_ast in model_asts {
            // Get the domain that current model requires.
            let domain = domain_map.get(&model_ast.domain_name).unwrap();
            let model_name = model_ast.model_name;
            let mut alias_map = HashMap::new();
            let mut raw_model_store = vec![];
            let mut model_store = vec![];

            for term_ast in model_ast.models {
                // Convert AST into term according to its type.
                let term = term_ast.to_term(domain);
                raw_model_store.push(term.clone());
                match term.clone() {
                    Term::Composite(c) => {
                        match c.alias {
                            None => {},
                            Some(alias) => {
                                let vterm: Term = Variable::new(alias, vec![]).into();
                                alias_map.insert(vterm, term);
                            }
                        }
                    },
                    _ => {},
                }

            }

            // Some alias in the term are treated as variables and need to replace them with the right term.
            for raw_term in raw_model_store {
                let term = raw_term.propagate_bindings(&alias_map);
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


#[enum_dispatch(ExprAst)]
trait ExprAstBehavior {
    fn to_expr(&self, domain: &Domain) -> Expr;
}

#[enum_dispatch]
#[derive(Clone, Debug)]
enum ExprAst {
    BaseExprAst,
    ArithExprAst,
}


#[enum_dispatch(BaseExprAst)]
trait BaseExprAstBehavior {
    fn to_base_expr(&self, domain: &Domain) -> BaseExpr;
}

#[enum_dispatch]
#[derive(Clone, Debug)]
enum BaseExprAst {
    SetComprehensionAst,
    TermAst,
}

impl ExprAstBehavior for BaseExprAst {
    fn to_expr(&self, domain: &Domain) -> Expr {
        let base_expr = self.to_base_expr(domain);
        base_expr.into()
    }
}

#[derive(Clone, Debug)]
struct SetComprehensionAst {
    vars: Vec<TermAst>,
    condition: Vec<ConstraintAst>,
    op: SetCompreOp,
    default: Option<BigInt>,
}

impl BaseExprAstBehavior for SetComprehensionAst {
    fn to_base_expr(&self, domain: &Domain) -> BaseExpr {
        let mut vars = vec![];
        let mut condition = vec![];
        for term_ast in self.vars.clone() {
            vars.push(term_ast.to_term(domain));
        }

        for constraint_ast in self.condition.clone() {
            condition.push(constraint_ast.to_constraint(domain));
        }
        
        SetComprehension {
            vars,
            condition,
            op: self.op.clone(),
            default: self.default.clone(),
        }.into()
    }
}
impl BaseExprAstBehavior for TermAst {
    fn to_base_expr(&self, domain: &Domain) -> BaseExpr {
        let term = self.to_term(domain);
        term.into()
    }
}

#[derive(Clone, Debug)]
struct ArithExprAst {
    op: ArithmeticOp,
    left: Box<ExprAst>,
    right: Box<ExprAst>,
}

impl ExprAstBehavior for ArithExprAst {
    fn to_expr(&self, domain: &Domain) -> Expr {
        let left = self.left.to_expr(domain);
        let right = self.right.to_expr(domain);
        ArithExpr {
            op: self.op.clone(),
            left: Arc::new(left),
            right: Arc::new(right),
        }.into()
    }
}


#[enum_dispatch(ConstraintAst)]
trait ConstraintAstBehavior {
    fn to_constraint(&self, domain: &Domain) -> Constraint;
}

#[enum_dispatch]
#[derive(Clone, Debug)]
enum ConstraintAst {
    PredicateAst,
    BinaryAst,
}


#[derive(Clone, Debug)]
struct PredicateAst {
    negated: bool,
    term: TermAst,
    alias: Option<String>,
}

impl ConstraintAstBehavior for PredicateAst {
    fn to_constraint(&self, domain: &Domain) -> Constraint {
        let alias = match self.alias.clone() {
            None => None,
            Some(a) => {
                let term: Term = Variable::new(a, vec![]).into();
                Some(term)
            }
        };

        Predicate {
            negated: self.negated,
            term: self.term.to_term(domain),
            alias,
        }.into()
    }
}

#[derive(Clone, Debug)]
struct BinaryAst {
    op: BinOp,
    left: ExprAst,
    right: ExprAst,
}

impl ConstraintAstBehavior for BinaryAst {
    fn to_constraint(&self, domain: &Domain) -> Constraint {
        Binary {
            op: self.op.clone(),
            left: self.left.to_expr(domain),
            right: self.right.to_expr(domain),
        }.into()
    }
}



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
    map!(alt!(tag!("count") | tag!("sum") | tag!("minAll") | tag!("maxAll")), |x| {
        match x {
            "count" => SetCompreOp::Count,
            "sum" => SetCompreOp::Sum,
            "minAll" => SetCompreOp::MinAll,
            _ => SetCompreOp::MaxAll,
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
        map!(alt!(variable_ast | atom_ast), |x| { 
            let base_expr: BaseExprAst = x.into();
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

fn parse_setcompre(vars: Vec<TermAst>, condition: Vec<ConstraintAst>, op: SetCompreOp, default: Option<Term>) -> SetComprehensionAst {
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
        map!(binary, |x| { x.into() }) 
    )
);

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

#[derive(Clone, Debug)]
struct RuleAst {
    head: Vec<TermAst>,
    body: Vec<ConstraintAst>,
}

impl RuleAst {
    fn to_rule(&self, domain: &Domain) -> Rule {
        let mut head = vec![];
        for term_ast in self.head.clone() {
            head.push(term_ast.to_term(domain));
        }

        let mut body = vec![];
        for constraint_ast in self.body.clone() {
            body.push(constraint_ast.to_constraint(domain));
        }

        Rule {
            head,
            body,
        }
    }
}


named!(rule<&str, RuleAst>,
    do_parse!(
        head: separated_list!(tag!(","), alt!(composite | variable_ast)) >>
        delimited!(space0, tag!(":-"), space0) >>
        body: separated_list!(tag!(","), 
            delimited!(space0, constraint, space0)
        ) >>
        (parse_rule(head, body))
    )
);

fn parse_rule(head: Vec<TermAst>, body: Vec<ConstraintAst>) -> RuleAst {
    RuleAst {
        head,
        body,
    }
}


named!(composite<&str, TermAst>, 
    do_parse!(
        alias: opt!(terminated!(id, delimited!(multispace0, tag!("is"), multispace0))) >>
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
        (parse_term(alias, t, args))
    )
);

fn parse_term(alias: Option<String>, sort: String, args: Vec<TermAst>) -> TermAst {
    CompositeTermAst {
        name: sort.to_string(),
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
        fragments: separated_list!(tag!("."), id) >>
        (parse_variable(var, fragments))
    )
);

fn parse_variable(var: &str, fragments: Vec<String>) -> Term {
    Variable::new(var.to_string(), fragments).into()
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
        delimited!(char!('"'), alphanumeric0, char!('"')), 
        |atom_str| { Atom::Str(atom_str.to_string()).into() }
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
    fn test_components() {
        assert_eq!(id("xxx").unwrap().0, "");
        assert_eq!(varname("xx ").unwrap().0, " ");
        assert_eq!(varname("xx22' ").unwrap().0, " ");
        assert_eq!(varname("yy22'' ").unwrap().0, " ");
        assert_eq!(varname("_").unwrap().0, "");
        assert_eq!(typename("yyy").unwrap().0, "");
        assert_eq!(typename("b3aab2c").unwrap().0, "");
        assert_eq!(tagged_typename("id : Hello").unwrap().0, "");
    }

    #[test]
    fn test_term() {
        assert_eq!(atom("1.23").unwrap().0, "");
        assert_eq!(atom("true").unwrap().0, "");
        assert_eq!(atom("-122").unwrap().0, "");
        assert_eq!(atom("\"helloworld\"").unwrap().0, "");
        assert_eq!(atom("123E-02").unwrap().0, "");
        assert_eq!(atom("-11223344").unwrap().0, "");
        assert_eq!(variable("hello_world ").unwrap().0, " ");
        assert_eq!(variable_ast("hi ").unwrap().0, " ");
        assert_eq!(composite("Edge(node1 , Node(\"hello\"))").unwrap().0, "");
        assert_eq!(composite("Node(\"hi\")").unwrap().0, "");
    }

    #[test]
    fn test_typedef() {
        assert_eq!(composite_typedef("Edge ::= new(src: Node, dst : Node ).").unwrap().0, "");
        union_typedef("X  ::= A + B + C+  D  .");
    }

    #[test]
    fn test_expr() {
        assert_eq!(base_expr("x ").unwrap().0, " ");
        assert_eq!(base_expr("count({a , udge(a, b) | X(m, 11.11),odge(a, a), Edge(b, Node(\"hello\")) })").unwrap().0, "");
        assert_eq!(base_expr("minAll( 1 , {c, d | c is Node(a), d is Node(b)})").unwrap().0, "");
        assert_eq!(expr("a+b*c ,").unwrap().0, " ,");
        assert_eq!(expr("(a+b)*c ,").unwrap().0, " ,");

        let setcompre_str = "maxAll(10, {x | x is X(a, b, c)})";
        let expr1_str = &format!(" ( a + {}  ) /  b .", setcompre_str)[..];
        assert_eq!(expr(expr1_str).unwrap().0, " .");

        let binary1_str = &format!("(b + {}) / x = d + e .", setcompre_str)[..];
        assert_eq!(binary(binary1_str).unwrap().0, " .");
        
        let rule_str = "Edge(a, b) :- Edge(b, c), Edge(c, a).";
        assert_eq!(rule(rule_str).unwrap().0, ".");

        //println!("{:?}", output);
    }

    #[test]
    fn test_program() {
        let graph_domain =  
            "domain Graph { 
                Node ::= new(id: String). 
                Edge ::= new(src:Node, dst: Node). 
                Item ::= Node + Edge.

                Edge(a, c) :- Edge(a, b), Edge(b, c).
            }";

        let graph_model1 = 
            "model m of Graph {
                Node(\"helloworld\") .
                Edge(Node(\"hello\"), Node(\"world\")).
            }";

        let graph_model2 = 
            "model m of Graph {
                n1 is Node(\"helloworld\") .
                e1 is Edge(n1, n1).
            }";

        assert_eq!(domain(graph_domain).unwrap().0, "");
        assert_eq!(model(graph_model1).unwrap().0, "");
        assert_eq!(model(graph_model2).unwrap().0, "");
        
        let program1_str = &format!("{}EOF", graph_domain)[..];
        assert_eq!(program(program1_str).unwrap().0, "EOF");
        
        let program2_str = &format!("{} {}EOF", graph_domain, graph_model1)[..];
        assert_eq!(program(program2_str).unwrap().0, "EOF");

        let program3_str = &format!("{} {}EOF", graph_domain, graph_model2)[..];
        assert_eq!(program(program3_str).unwrap().0, "EOF");

        //let output = program(program4_str);
        //println!("{:?}", output);
    }

}

