use std::collections::HashSet;
use std::convert::TryInto;

// Trait that must be implemented by an instance of a DDlog program. 
// Type that represents a set of changes to DDlog relations.
// Returned by `DDlog::transaction_commit_dump_changes()`.
use differential_datalog::{DDlog, DDlogDump, DDlogDynamic}; 
use differential_datalog::DeltaMap; // A trait representing the changes resulting from a given update.
use differential_datalog::ddval::DDValue; // A generic DLog value type
use differential_datalog::ddval::DDValConvert; //Another helper trair
use differential_datalog::program::RelId; // Numeric relations id
use differential_datalog::program::Update; // A type representing updates to the database
// use differential_datalog::record::{FromRecord, IntoRecord}; // A type representing individual facts
// The `differential_datalog::program::config` module declares datatypes
// used to configure DDlog program on startup.
use differential_datalog::program::config::{Config, ProfilingKind};
// use differential_datalog::api::HDDlog;


use formula2ddlog_ddlog::typedefs::langs::formula::Term;
use formula2ddlog_ddlog::typedefs::langs::lib::list::from_vec;
use rand::{Rng, SeedableRng, StdRng};
use formula2ddlog_ddlog::api::HDDlog;
use formula2ddlog_ddlog::Relations;
use formula2ddlog_ddlog::relid2name;

// import all types defined by the datalog program itself
use formula2ddlog_ddlog::typedefs::ddlog_std::{Ref, Vec, vec_empty};
use formula2ddlog_ddlog::typedefs::langs::formula::*;
use formula2ddlog_ddlog::typedefs::langs::ddlog::*;


use differential_formula::term::*;
use differential_formula::module::*;
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

pub struct DDLogTransformation{
    hddlog: HDDlog,
}

impl DDLogTransformation {
    pub fn new()  -> Result<DDLogTransformation, String> {
        // let config = Config::new();
        // let (hddlog, init_state) = formula2ddlog_ddlog::run_with_config(config, false); 
        let (hddlog, init_state) = formula2ddlog_ddlog::api::HDDlog::run(1, true)?;
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

    pub fn dump_delta(delta: &DeltaMap<DDValue>) {
        for (rel, changes) in delta.iter() {
            println!("Changes to relation {}", relid2name(*rel).unwrap());
            for (val, weight) in changes.iter() {
                println!("{} {:+}", val, weight);
            }
        }
    }

    pub fn stop(&mut self){
        self.hddlog.stop().unwrap();
    }
}


fn main() {
    let mut xform_ddlog = DDLogTransformation::new().unwrap();

	let path = std::path::Path::new("../../../tests/testcase/p3.4ml");
    let content = std::fs::read_to_string(path).unwrap() + "EOF";
    let (_, program_ast) = parse_program(&content);
    let env: Env = program_ast.build_env();
    let graph = env.get_domain_by_name("Graph").unwrap();
    let m = env.get_model_by_name("m").unwrap();
    let meta = graph.meta_info();
    let tmap = meta.type_map();
    let raw_types = tmap.into_iter().map(|(name, raw_type)| raw_type.clone()).collect::<Vec<RawType>>();
    let terms = m.terms().into_iter().map(|x| x.clone()).collect();

    let mut updates = vec_empty();
    let type_updates = xform_ddlog.create_types(raw_types);
    let term_updates = xform_ddlog.create_terms(terms);
    updates.extend(type_updates);
    updates.extend(term_updates);
    let delta = xform_ddlog.flush_updates(updates).unwrap();

    DDLogTransformation::dump_delta(&delta);
}