use std::borrow::Cow;
use std::collections::HashSet;
use std::convert::TryInto;

use differential_datalog::record::IntoRecord;
// Trait that must be implemented by an instance of a DDlog program. 
// Type that represents a set of changes to DDlog relations.
// Returned by `DDlog::transaction_commit_dump_changes()`.
use differential_datalog::record::FromRecord;
use differential_datalog::{DDlog, DDlogDump, DDlogDynamic}; 
use differential_datalog::DeltaMap; // A trait representing the changes resulting from a given update.
use differential_datalog::ddval::DDValue; // A generic DLog value type
use differential_datalog::ddval::DDValConvert; //Another helper trair
use differential_datalog::program::RelId; // Numeric relations id
use differential_datalog::program::Update; // A type representing updates to the database
// use differential_datalog::record::{FromRecord, IntoRecord}; // A type representing individual facts
// The `differential_datalog::program::config` module declares datatypes
// used to configure DDlog program on startup.
use differential_datalog::program::config::{Config, ProfilingConfig};
use differential_datalog::api::HDDlog;

use formula2ddlog_ddlog::typedefs::langs::formula::Term;
use formula2ddlog_ddlog::typedefs::langs::lib::list::{from_nonnull_vec, from_vec};
use formula2ddlog_ddlog::Relations;
use formula2ddlog_ddlog::relid2name;

// import all types defined by the datalog program itself
use formula2ddlog_ddlog::typedefs::ddlog_std::{
    Option as DDOption, Ref, Vec, 
    option_unwrap_or_default, 
    ref_new, 
    vec_empty};
use formula2ddlog_ddlog::typedefs::langs::formula::*;
use formula2ddlog_ddlog::typedefs::langs::ddlog::*;
use formula2ddlog_ddlog::typedefs::langs::lib::list::*;
use formula2ddlog_ddlog::typedefs::langs::lib::operators::*;


use differential_formula::term::*;
use differential_formula::module::*;
use differential_formula::constraint::{Constraint as FConstraint};
use differential_formula::expression::{
    Expr as FExpr, 
    BaseExpr as FBaseExpr, 
    BasicExprOps, 
    SetComprehension
};
use differential_formula::rule::Rule as FRule;
use differential_formula::parser::combinator::parse_program;

fn convert_raw_type(raw_type: RawType) -> Option<TypeSpec> {
    match raw_type {
        RawType::Type(t) => {
            match t {
                FormulaTypeEnum::BaseType(basic_type) => {
                    let type_spec = match basic_type {
                        BaseType::Boolean => TypeSpec::Boolean,
                        BaseType::Integer => TypeSpec::Integer,
                        BaseType::NegInteger => TypeSpec::Integer,
                        BaseType::PosInteger => TypeSpec::Integer,
                        BaseType::Rational => TypeSpec::FloatNum,
                        BaseType::String => TypeSpec::String
                    };
                    Some(type_spec)
                },
                FormulaTypeEnum::CompositeType(composite_type) => {
                    let name = composite_type.name;
                    let arguments = composite_type.arguments.into_iter().map(|arg| {
                        let (name_opt, arg_raw_type) = arg;
                        let arg_type_spec = convert_raw_type(arg_raw_type).unwrap();
                        Field { field_name: name_opt.unwrap(), type_spec: arg_type_spec }
                    }).collect::<Vec<Field>>();
                    let type_spec = TypeSpec::CompositeType { name, arguments };
                    Some(type_spec)
                },
                FormulaTypeEnum::UnionType(union_type) => {
                    let name = union_type.name;
                    let subtypes = union_type.subtypes.into_iter().map(|subtype| {
                        let subtype_spec = convert_raw_type(subtype).unwrap();
                        subtype_spec
                    }).collect::<Vec<TypeSpec>>();
                    let type_spec = TypeSpec::UnionType { name, subtypes };
                    Some(type_spec)
                },
                // TODO: Handle enumeration type and range type
                _ => { Some(TypeSpec::Boolean) }
            }
        },
        _ => { Some(TypeSpec::Boolean) }
    }
}

fn convert_term(atomic_term: AtomicTerm) -> Term {
    match atomic_term {
        AtomicTerm::Atom(atom) => {
            match atom.val {
                AtomEnum::Bool(bool) => Term::AtomBool { i5: bool },
                AtomEnum::Int(bigint) => Term::AtomInt { i1: bigint.try_into().unwrap() },
                AtomEnum::Float(float) => Term::AtomFloat { i4: float },
                AtomEnum::Str(str) => Term::AtomStr { i0: str }
            }
        },
        AtomicTerm::Variable(variable) => {
            let fragments = Vec::from(variable.fragments);
            Term::Variable { root: variable.root, fragments }
        },
        AtomicTerm::Composite(composite) => {
            let name = composite.sort.type_id().to_string();
            let mut argument_vec: Vec<Ref<Term>> = composite.arguments.into_iter().map(|x| {
                let arg = convert_term(x); 
                arg.into()
            }).collect();
            let arguments = from_vec(&mut argument_vec); 
            Term::Composite { name, arguments }
        }
    }
}

fn convert_setcompre(rid: String, sc: SetComprehension) -> Setcompre {
    let sc_rule: FRule = sc.clone().into();
    let sop = match &sc.op {
        differential_formula::expression::SetCompreOp::Count => SetOp::Count,
        differential_formula::expression::SetCompreOp::Sum => SetOp::Sum,
        differential_formula::expression::SetCompreOp::MaxAll => SetOp::Max,
        differential_formula::expression::SetCompreOp::MinAll => SetOp::Min,
        _ => SetOp::Count
    };
    let atom: AtomEnum = AtomEnum::Int(sc.default.clone());
    let default_term = AtomicTerm::Atom(AtomicAtom{
        sort: RawType::TypeId(Cow::from("Integer")),
        val: atom
    });
    let default = convert_term(default_term);
    Setcompre { rule: ref_new(&convert_rule(rid, sc_rule)), sop, default}
}

fn convert_expr(expr: FExpr) -> Expr {
    match expr {
        FExpr::BaseExpr(base_expr) => {
            match base_expr {
                FBaseExpr::Term(term) => { 
                    Expr::BaseExpr { term: convert_term(term)} 
                },
                FBaseExpr::SetComprehension(setcompre) => {
                    // TODO: Derive a new name for the rule of set comprehension 
                    Expr::SetcompreExpr { sc: ref_new(&convert_setcompre("".to_string(), setcompre)) }
                }
            }
        },
        FExpr::ArithExpr(arith_expr) => {
            let aop = match arith_expr.op {
                differential_formula::expression::ArithmeticOp::Add => ArithOp::Plus,
                differential_formula::expression::ArithmeticOp::Mul => ArithOp::Mul,
                differential_formula::expression::ArithmeticOp::Min => ArithOp::Minus,
                differential_formula::expression::ArithmeticOp::Div => ArithOp::Div,
            };
            let left_expr = convert_expr(arith_expr.left.as_ref().clone());
            let right_expr = convert_expr(arith_expr.right.as_ref().clone());
            Expr::ArithExpr { left: ref_new(&left_expr), right: ref_new(&right_expr), aop }
        }, 
    }
}

fn convert_constraint(constraint: FConstraint) -> Constraint {
    match constraint {
        FConstraint::Predicate(pred) => {
            let alias = match pred.alias {
                Some(x) => {
                    let alias_str = format!("{}", x);
                    DDOption::Some{ x: alias_str }
                }, 
                None => DDOption::None
            };
            Constraint::PredCons { 
                negated: pred.negated, 
                term: convert_term(pred.term), 
                alias
            }
        },
        FConstraint::Binary(bin) => {
            let bop = match bin.op {
                differential_formula::constraint::BinOp::Eq => BinOp::Eq,
                differential_formula::constraint::BinOp::Ne => BinOp::Neq,
                differential_formula::constraint::BinOp::Ge => BinOp::Geq,
                differential_formula::constraint::BinOp::Gt => BinOp::Gt,
                differential_formula::constraint::BinOp::Le => BinOp::Leq,
                differential_formula::constraint::BinOp::Lt => BinOp::Lt,
            };
            // TODO: Add expression assignment such as `var num = x * x`
            if bin.is_setcompre_assignment() {
                Constraint::AssignCons { 
                    variable: convert_term(bin.left_term().unwrap()), 
                    expr: convert_expr(bin.right)
                }
            } else {
                Constraint::BinaryCons {
                    left: convert_expr(bin.left),
                    right: convert_expr(bin.right),
                    bop 
                }
            }
        },
        FConstraint::TypeConstraint(_) => { todo!() }
    }
}

fn convert_rule(rid: String, rule: FRule) -> Rule {
    let mut terms: Vec<Term> = rule.head().into_iter().map(|x| convert_term(x.clone())).collect();
    let mut cons: Vec<Constraint> = rule.body().into_iter().map(|x| convert_constraint(x)).collect();
    let term_list_opt: Option<NonNullList<Term>> = from_nonnull_vec(&mut terms).into();
    let term_list = term_list_opt.unwrap();
    let cons_list_opt: Option<NonNullList<Constraint>> = from_nonnull_vec(&mut cons).into();
    let cons_list = cons_list_opt.unwrap();
    let rule = Rule { id: rid, head: term_list, body: cons_list };
    rule
}

pub struct DDLogTransformation{
    hddlog: HDDlog,
}

impl DDLogTransformation {
    pub fn new()  -> Result<DDLogTransformation, String> {
        let config = Config::new()
            .with_timely_workers(1)
            .with_profiling_config(ProfilingConfig::SelfProfiling);
        // let (hddlog, init_state) = formula2ddlog_ddlog::run_with_config(config, false); 
        let (hddlog, init_state) = formula2ddlog_ddlog::run_with_config(config, false)?;
        Self::dump_delta(&init_state);
        return Ok(Self{hddlog});
    }

    pub fn flush_updates(&mut self, updates: Vec<Update<DDValue>>) -> Result<DeltaMap<DDValue>, String> {
        self.hddlog.transaction_start()?;
        self.hddlog.apply_updates(&mut updates.into_iter())?;
        let delta = self.hddlog.transaction_commit_dump_changes()?;
        return Ok(delta); 
    }

    pub fn create_types(&mut self, types: Vec<RawType>) -> Vec<Update<DDValue>> {
        let updates = types.into_iter().map(|raw_type| {
            let type_spec = convert_raw_type(raw_type).unwrap();
            Update::Insert {
                relid: Relations::langs_formula_TypeSpec as RelId,
                v: type_spec.into_ddvalue()
            }
        }).collect::<Vec<_>>();
        updates
    } 

    pub fn create_terms(&mut self, terms: Vec<AtomicTerm>) -> Vec<Update<DDValue>> {
        let updates = terms.into_iter().map(|atomic_term| {
            let term = convert_term(atomic_term);
            Update::Insert {
                relid: Relations::langs_formula_InputTerm as RelId,
                v: term.into_ddvalue(),
            }
        }).collect::<Vec<_>>();
        updates
    } 

    pub fn create_rules(&mut self, rules: Vec<FRule>) -> Vec<Update<DDValue>> {
        let updates = rules.into_iter().enumerate().map(|(i, rule)| {
            let rule = convert_rule(i.to_string(), rule);
            Update::Insert {
                relid: Relations::langs_formula_InputRule as RelId,
                v: rule.into_ddvalue(),
            }
        }).collect::<Vec<_>>();
        updates
    } 

    pub fn dump_delta(delta: &DeltaMap<DDValue>) {
        for (rel, changes) in delta.iter() {
            println!("Changes to relation {}", relid2name(*rel).unwrap());
            for (val, weight) in changes.iter() {
                println!("{} {:+}", val, weight);
            }
        }
    }

    pub fn dump_delta_by_relid(delta: &DeltaMap<DDValue>, relid: RelId) {
        let ddrel_changes = delta.try_get_rel(relid).unwrap();
        for (val, weight) in ddrel_changes.iter() {
            println!("{:#?} {:+}", val, weight);
        }
    }

    pub fn print_program(delta: &DeltaMap<DDValue>) {
        let ddrule_changes = delta.try_get_rel(Relations::langs_ddlog_DDRule as RelId).unwrap();
        let ddtype_changes = delta.try_get_rel(Relations::langs_ddlog_DDTypeSpec as RelId).unwrap();
        let ddrel_changes = delta.try_get_rel(Relations::langs_ddlog_DDRelation as RelId).unwrap();
        for (val, _weight) in ddtype_changes.iter() {
            // println!("{} {:+}", val, weight);
            let ddtype_record = val.clone().into_record();
            let ddtype: DDTypeSpec = DDTypeSpec::from_record(&ddtype_record).unwrap();
            println!("{}", to_string_langs_ddlog_DDTypeSpec___Stringval(&ddtype));
        }
        for (val, _weight) in ddrel_changes.iter() {
            // println!("{} {:+}", val, weight);
            let ddrelation_record = val.clone().into_record();
            let ddrelation: DDRelation = DDRelation::from_record(&ddrelation_record).unwrap();
            println!("{}", to_string_langs_ddlog_DDRelation___Stringval(&ddrelation));
        }
        for (val, _weight) in ddrule_changes.iter() {
            // println!("{} {:+}", val, weight);
            let ddrule_record = val.clone().into_record();
            let ddrule: DDRule = DDRule::from_record(&ddrule_record).unwrap();
            println!("{}", to_string_langs_ddlog_DDRule___Stringval(&ddrule));
        }
    }

    pub fn stop(&mut self){
        self.hddlog.stop().unwrap();
    }
}

fn main() {
    let mut xform_ddlog = DDLogTransformation::new().unwrap();

	// let path = std::path::Path::new("../../../tests/testcase/p3.4ml");
	let path = std::path::Path::new("src/p.4ml");
    let content = std::fs::read_to_string(path).unwrap() + "EOF";
    let (_, program_ast) = parse_program(&content);
    let env: Env = program_ast.build_env();
    let graph = env.get_domain_by_name("Graph").unwrap();
    let m = env.get_model_by_name("m").unwrap();
    let meta = graph.meta_info();
    let rules: Vec<FRule> = meta.rules().into();
    let tmap = meta.type_map();
    let raw_types = tmap.into_iter().map(|(_name, raw_type)| raw_type.clone()).collect::<Vec<RawType>>();
    let terms = m.terms().into_iter().map(|x| x.clone()).collect::<Vec<AtomicTerm>>();

    let mut updates: Vec<Update<DDValue>> = vec_empty();
    let type_updates = xform_ddlog.create_types(raw_types);
    let term_updates = xform_ddlog.create_terms(terms);
    let rule_updates = xform_ddlog.create_rules(rules);
    updates.extend(type_updates);
    updates.extend(term_updates);
    updates.extend(rule_updates);
    let delta = xform_ddlog.flush_updates(updates).unwrap();

    // DDLogTransformation::dump_delta(&delta);
    // DDLogTransformation::dump_delta_by_relid(&delta, Relations::langs_formula_SubtermTypeSpec as RelId);
    // DDLogTransformation::dump_delta_by_relid(&delta, Relations::DDTermInSetcompreHead as RelId);
    DDLogTransformation::print_program(&delta);
}