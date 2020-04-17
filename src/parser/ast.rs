use std::collections::*;
use std::sync::Arc;
use std::borrow::Borrow;
use enum_dispatch::enum_dispatch;
use num::*;

use crate::term::*;
use crate::type_system::*;
use crate::expression::*;
use crate::rule::*;
use crate::constraint::*;
use crate::util::*;

#[enum_dispatch(TermAst)]
trait TermAstBehavior {}

impl TermAstBehavior for CompositeTermAst {}
impl TermAstBehavior for Term {}

#[enum_dispatch]
#[derive(Debug, Clone)]
pub enum TermAst {
    CompositeTermAst,
    Term, // It only represents Atom or Variable.
}

impl TermAst {
    pub fn to_term(&self, domain: &Domain) -> Term {
        match self {
            TermAst::CompositeTermAst(cterm_ast) => {
                let mut term_arguments = vec![];
                for argument in cterm_ast.arguments.clone() {
                    let term = argument.to_term(domain);
                    term_arguments.push(Arc::new(term));
                }
                let sort_name = cterm_ast.sort.name().unwrap();
                let sort = domain.get_type(&sort_name);
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
pub struct CompositeTermAst {
    pub sort: TypeDefAst,
    pub arguments: Vec<Box<TermAst>>,
    pub alias: Option<String>
}

#[enum_dispatch(TypeDefAst)]
pub trait TypeDefAstBehavior {
    fn name(&self) -> Option<String>;
}

#[enum_dispatch]
#[derive(Debug, Clone)]
pub enum TypeDefAst {
    AliasTypeDefAst,
    CompositeTypeDefAst,
    UnionTypeDefAst,
    RangeTypeDefAst,
    EnumTypeDefAst,
}

#[derive(Debug, Clone)]
pub struct AliasTypeDefAst {
    pub chained_scopes: Vec<String>,
    pub name: String,
}

impl TypeDefAstBehavior for AliasTypeDefAst {
    fn name(&self) -> Option<String> {
        let mut names = self.chained_scopes.clone();
        names.push(self.name.clone());
        Some(names.join("."))
    }
}

// e.g. Edge ::= new(src: Node, dst: Node).
#[derive(Debug, Clone)]
pub struct CompositeTypeDefAst {
    pub name: String,
    pub args: Vec<(Option<String>, Box<TypeDefAst>)>,
}

impl TypeDefAstBehavior for CompositeTypeDefAst {
    fn name(&self) -> Option<String> {
        Some(self.name.clone())
    }
}

// X ::= A + B + C.
#[derive(Debug, Clone)]
pub struct UnionTypeDefAst {
    pub name: Option<String>,
    pub subtypes: Vec<Box<TypeDefAst>>,
}

impl TypeDefAstBehavior for UnionTypeDefAst {
    fn name(&self) -> Option<String> {
        self.name.clone()
    }
}

// Range ::= {0..100}.
#[derive(Debug, Clone)]
pub struct RangeTypeDefAst {
    pub name: Option<String>,
    pub low: String,
    pub high: String,
}

impl TypeDefAstBehavior for RangeTypeDefAst {
    fn name(&self) -> Option<String> {
        self.name.clone()
    }
}

// Enum ::= {"HELLO", "WORLD", 2, 1.23}
#[derive(Debug, Clone)]
pub struct EnumTypeDefAst {
    pub name: Option<String>,
    pub enums: Vec<String>,
}

impl TypeDefAstBehavior for EnumTypeDefAst {
    fn name(&self) -> Option<String> {
        self.name.clone()
    }
}


#[derive(Clone, Debug)]
pub enum ModuleAst {
    Domain(DomainAst),
    Model(ModelAst),
    Transform(TransformAst),
}

#[derive(Clone, Debug)]
pub struct TaggedDomainAst {
    pub tag: String,
    pub domain: String,
}

#[derive(Clone, Debug)]
pub struct TaggedTypeAst {
    pub tag: String,
    pub formula_type: TypeDefAst,
}

#[derive(Clone, Debug)]
pub enum TransformParamAst {
    TaggedDomain(TaggedDomainAst),
    TaggedType(TaggedTypeAst)
}

#[derive(Clone, Debug)]
pub struct TransformAst {
    pub transform_name: String, 
    pub inputs: Vec<TransformParamAst>,
    pub output: TaggedDomainAst, 
    pub typedefs: Vec<(String, TypeDefAst)>,
    pub rules: Vec<RuleAst>,
}

#[derive(Clone, Debug)]
pub struct DomainAst {
    pub name: String,
    pub types: Vec<(String, TypeDefAst)>,
    pub rules: Vec<RuleAst>,
    // includes or extends [scope :: subdomain]
    pub inherit_type: String,
    pub subdomains: Vec<String>,
    pub renamed_subdomains: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct ModelAst {
    pub model_name: String,
    pub domain_name: String,
    pub models: Vec<TermAst>,
    // includes or extends [scope :: subdomain]
    pub inherit_type: String,
    pub submodels: Vec<String>,
    pub renamed_submodels: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct ProgramAst {
    pub domain_ast_map: HashMap<String, DomainAst>,
    pub model_ast_map: HashMap<String, ModelAst>,
    pub transform_ast_map: HashMap<String, TransformAst>,
}

/*
const BUILTIN_TYPES_MAP: HashMap<String, Type> = HashMap::new();
BUILTIN_TYPES_MAP.insert("String".to_string(), Arc::new(BaseType::String.into()));
BUILTIN_TYPES_MAP.insert("Integer".to_string(), Arc::new(BaseType::Integer.into()));
BUILTIN_TYPES_MAP.insert("Boolean".to_string(), Arc::new(BaseType::Boolean.into()));
*/

impl ProgramAst {
    pub fn build_env(&self) -> Env {
        let mut domain_map = HashMap::new();
        let mut model_map = HashMap::new();
        let mut transform_map = HashMap::new();

        for domain_name in self.domain_ast_map.keys() {
            self.create_domain(domain_name.clone(), &mut domain_map);
        }

        for model_name in self.model_ast_map.keys() {
            self.create_model(model_name.clone(), &mut model_map, &domain_map);
        }

        for transform_name in self.transform_ast_map.keys() {
            self.create_transformation(transform_name.clone(), &mut transform_map, &mut domain_map);
        }

        Env {
            domain_map,
            model_map,
            transform_map,
        }
    }

    fn create_type(
        &self, t: String, 
        ast_map: &HashMap<String, TypeDefAst>,    // Only has type ASTs in current domain.
        type_map: &mut HashMap<String, Arc<Type>>,     // Recursively put new created type into type map.
    ) 
    -> Arc<Type>
    {
        if type_map.contains_key(&t) {
            let existing_type = type_map.get(&t).unwrap();
            return existing_type.clone();
        } 
        
        let type_ast = ast_map.get(&t).unwrap();
        let new_type = match type_ast {
            TypeDefAst::AliasTypeDefAst(aliastypedef) => {
                // At this point, type from subdomains should be available in `type_map`.
                let full_name = aliastypedef.name().unwrap();
                if type_map.contains_key(&full_name) {
                    type_map.get(&full_name).unwrap().clone()
                } else {
                    self.create_type(full_name, ast_map, type_map)
                }
            },
            TypeDefAst::CompositeTypeDefAst(ctypedef) => {
                let mut args = vec![];
                let typename = ctypedef.name().unwrap();
                for (id_opt, arg_ast) in ctypedef.args.iter() {
                    let name = arg_ast.name().unwrap();
                    let subtype_opt = type_map.get(&name);
                    let subtype = match subtype_opt {
                        Some(t) => { t.clone() },
                        None => { 
                            self.create_type(name, ast_map, type_map)
                        }
                    };
                    args.push((id_opt.clone(), subtype));
                }

                let composite_type: Type = CompositeType {
                    name: typename,
                    arguments: args,
                }.into();

                Arc::new(composite_type)
            },
            TypeDefAst::UnionTypeDefAst(utypedef) => {
                let mut subtypes = vec![];
                let typename = utypedef.name().unwrap();
                for subtype_ast in utypedef.subtypes.iter() {
                    let name = subtype_ast.name().unwrap();
                    let subtype_opt = type_map.get(&name);
                    let subtype = match subtype_opt {
                        Some(t) => { t.clone() },
                        None => { self.create_type(name, ast_map, type_map) }
                    };
                    subtypes.push(subtype);
                }

                let union_type = UnionType {
                    name: typename,
                    subtypes: subtypes,
                }.into();

                Arc::new(union_type)
            },
            _ => { 
                unimplemented!()
            }
        };

        // Add new type to the type map.
        type_map.insert(t, new_type.clone());

        return new_type;
    }

    pub fn import_builtin_types(&self, type_map: &mut HashMap<String, Arc<Type>>) {
        // TODO: There are more types to add here.
        type_map.insert("String".to_string(), Arc::new(BaseType::String.into()));
        type_map.insert("Integer".to_string(), Arc::new(BaseType::Integer.into()));
        type_map.insert("Boolean".to_string(), Arc::new(BaseType::Boolean.into()));
    }

    pub fn create_transformation(
        &self, transform_name: String, 
        transform_map: &mut HashMap<String, Transform>,
        domain_map: &mut HashMap<String, Domain>,
    ) -> Transform 
    {
        if transform_map.contains_key(&transform_name) {
            return transform_map.get(&transform_name).unwrap().clone();
        }

        let transform_ast = self.transform_ast_map.get(&transform_name).unwrap();

        let mut input_type_ast_map = HashMap::new();
        let mut tagged_domains = vec![];
        tagged_domains.push(transform_ast.output.clone());
        for input in transform_ast.inputs.clone() {
            match input {
                TransformParamAst::TaggedDomain(tagged_domain) => {
                    tagged_domains.push(tagged_domain);
                },
                TransformParamAst::TaggedType(tagged_type) => {
                    //let type_ast = tagged_type.formula_type;
                    input_type_ast_map.insert(tagged_type.tag.clone(), tagged_type.formula_type.clone());
                }
            }
        }

        // Include all types from inputs, output and ones defined in transformation.
        let mut type_map = HashMap::new();
        self.import_builtin_types(&mut type_map);

        for tagged_domain in tagged_domains.iter() {
            let scope = tagged_domain.tag.clone();
            let domain_name = tagged_domain.domain.clone();
            let domain = self.create_domain(domain_name, domain_map);

            for (type_name, formula_type_arc) in domain.type_map.iter() {
                let formula_type: Type = formula_type_arc.as_ref().clone();
                match formula_type {
                    Type::BaseType(_) => {},
                    _ => {
                        let renamed_type = formula_type.rename_type(scope.clone());
                        type_map.insert(renamed_type.name(), Arc::new(renamed_type));
                    }
                }
            }
        }

        let mut type_ast_map = HashMap::new();
        for (typename, type_ast) in transform_ast.typedefs.iter() {
            type_ast_map.insert(typename.clone(), type_ast.clone());
        } 

        for type_name in type_ast_map.keys() {
            self.create_type(type_name.clone(), &type_ast_map, &mut type_map);
        }

        let transform_domain = Domain {
            name: transform_name.clone(),
            type_map: type_map.clone(),
            rules: vec![],
        };

        // Add rules into domain and converting rule ASTs need type information in domain.
        // TODO: add conformance rules.
        let mut rules = vec![];
        for rule_ast in transform_ast.rules.iter() {
            rules.push(rule_ast.to_rule(&transform_domain));
        }

        // Some parameters that are known types in `type_map`
        let mut input_type_map = HashMap::new();
        for (_, type_ast) in input_type_ast_map {
            let input_type = type_map.get(&type_ast.name().unwrap()).unwrap();
        }

        let mut transform = Transform {
            name: transform_name.clone(),
            type_map,
            rules,
            input_type_map
        };

        transform_map.insert(transform_name.clone(), transform.clone());
        transform
    }

    pub fn create_domain(&self, domain_name: String, domain_map: &mut HashMap<String, Domain>) -> Domain {
        if domain_map.contains_key(&domain_name) { 
            return domain_map.get(&domain_name).unwrap().clone(); 
        }

        let mut type_map = HashMap::new();
        self.import_builtin_types(&mut type_map);

        let domain_ast = self.domain_ast_map.get(&domain_name).unwrap();

        for subdomain_name in domain_ast.subdomains.iter() {
            let subdomain = self.create_domain(subdomain_name.clone(), domain_map);
            type_map.extend(subdomain.type_map.clone());

            if domain_ast.inherit_type == "extends" {
                // TODO: import comformance rules if inheritance type is `extends`.
            }
        }

        for (scope, subdomain_name) in domain_ast.renamed_subdomains.iter() {
            let subdomain = self.create_domain(subdomain_name.clone(), domain_map);
            for (type_name, formula_type_arc) in subdomain.type_map.iter() {
                let formula_type: Type = formula_type_arc.as_ref().clone();
                match formula_type {
                    Type::BaseType(_) => {},
                    _ => {
                        let renamed_type = formula_type.rename_type(scope.clone());
                        type_map.insert(renamed_type.name(), Arc::new(renamed_type));
                    }
                }
            }

            if domain_ast.inherit_type == "extends" {
                // TODO: import comformance rules if inheritance type is `extends`.
            }
        }

        // `type_ast_map` contains both native type and type alias that are from subdomains.
        let mut type_ast_map = HashMap::new();
        for (typename, type_ast) in domain_ast.types.iter() {
            type_ast_map.insert(typename.clone(), type_ast.clone());
        }

        for type_name in type_ast_map.keys() {
            self.create_type(type_name.clone(), &type_ast_map, &mut type_map);
        }

        let mut domain = Domain {
            name: domain_name.clone(),
            type_map: type_map,
            rules: vec![],
        };

        // Add rules into domain and converting rule ASTs need type information in domain.
        // TODO: add conformance rules.
        for rule_ast in domain_ast.rules.iter() {
            domain.add_rule(rule_ast.to_rule(&domain));
        }

        domain_map.insert(domain_name.clone(), domain.clone());
        domain
    }

    /// It won't return a model because deep copy of large amounts of data
    /// is too expensive and all models are stored in `model_map`.
    pub fn create_model(
        &self, 
        model_name: String,
        model_map: &mut HashMap<String, Model>, 
        domain_map: &HashMap<String, Domain>
    ) 
    {
        if model_map.contains_key(&model_name) { return; }
        
        let model_ast = self.model_ast_map.get(&model_name).unwrap();
        let domain = domain_map.get(&model_ast.domain_name).unwrap();

        let mut raw_alias_map = HashMap::new();
        let mut alias_map: HashMap<Arc<Term>, Arc<Term>> = HashMap::new();
        let mut model_store = vec![];
        let mut untouched_models = vec![];
        
        // TODO: deep clone is not really necessary for keeping all ASTs.
        for term_ast in model_ast.models.clone() {
            let term = term_ast.to_term(domain);
            match &term {
                Term::Composite(c) => {
                    match &c.alias {
                        None => {
                            // if term does not have alias, add it to model store.
                            untouched_models.push(term);
                        },
                        Some(alias) => {
                            // alias here shouldn't have fragments and add term to model store later.
                            let vterm: Term = Variable::new(alias.clone(), vec![]).into();
                            raw_alias_map.insert(Arc::new(vterm), Arc::new(term));
                        }
                    }
                },
                _ => {},
            }

        }

        /* 
        Alias in the raw term is treated as variables and needs to be replaced with the real term.
        Term propagations have to follow the order that for example n1 = Node(x) needs to be handled 
        prior to e1 is Edge(n1, n1), otherwise the raw term may be used in propagation.
        */
        for key in raw_alias_map.keys() {
            self.propagate_alias_map(key, &raw_alias_map, &mut alias_map);
        }

        for (key, term) in alias_map.iter() {
            model_store.push(term.clone());
        }

        // Hanle composite terms that don't have alias associated with them.
        for term in untouched_models {
            let new_term = term.propagate_bindings(&alias_map);
            model_store.push(new_term.unwrap());
        }

        // TODO: Need to check if they are duplicates from sub-models.
        // Import sub-models into `model_store` with a copy of Arc<Term>.
        for submodel_name in model_ast.submodels.iter() {
            self.create_model(submodel_name.clone(), model_map, domain_map);
            let submodel = model_map.get(submodel_name).unwrap();
            for term_arc in submodel.models.iter() {
                model_store.push(term_arc.clone());
            }
        }

        // Import renamed sub-models with the type changed.
        for scope in model_ast.renamed_submodels.keys() {
            let submodel_name = model_ast.renamed_submodels.get(scope).unwrap();
            self.create_model(submodel_name.clone(), model_map, domain_map);
            let submodel = model_map.get(submodel_name).unwrap();
            for term_arc in submodel.models.iter() {
                let term: Term = term_arc.as_ref().clone();
                let renamed_term = term.rename(scope.clone());
                model_store.push(Arc::new(renamed_term));
            }
        }

        let model = Model::new(
            model_name.clone(), 
            model_ast.domain_name.clone(), 
            model_store, 
            alias_map
        );

        model_map.insert(model_name.clone(), model);
    }

    fn propagate_alias_map<T>(
        &self, var: &Term, 
        raw_alias_map: &T, 
        alias_map: &mut T
    ) where T: GenericMap<Arc<Term>, Arc<Term>>
    {
        let raw_term = raw_alias_map.gget(var).unwrap();
        // if current term has variables inside then propagate binding to them first.
        let raw_term_vars = raw_term.variables();
        for raw_term_var in raw_term_vars {
            if raw_alias_map.contains_gkey(raw_term_var) {
                self.propagate_alias_map(raw_term_var, raw_alias_map, alias_map);
            }
        }
    
        let new_term = raw_term.propagate_bindings(alias_map).unwrap();
        alias_map.ginsert(Arc::new(var.clone()), new_term);
    }
}

#[enum_dispatch(ExprAst)]
trait ExprAstBehavior {
    fn to_expr(&self, domain: &Domain) -> Expr;
}

#[enum_dispatch]
#[derive(Clone, Debug)]
pub enum ExprAst {
    BaseExprAst,
    ArithExprAst,
}

#[enum_dispatch(BaseExprAst)]
trait BaseExprAstBehavior {
    fn to_base_expr(&self, domain: &Domain) -> BaseExpr;
}

#[enum_dispatch]
#[derive(Clone, Debug)]
pub enum BaseExprAst {
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
pub struct SetComprehensionAst {
    pub vars: Vec<TermAst>,
    pub condition: Vec<ConstraintAst>,
    pub op: SetCompreOp,
    pub default: Option<BigInt>,
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
        
        // Count and Sum operator does not have explicit default value but let's set it to 0.
        let default = match self.default.clone() {
            None => { BigInt::from_i64(0 as i64).unwrap() },
            Some(val) => { val },
        };

        SetComprehension::new( 
            vars,
            condition,
            self.op.clone(),
            default,
        ).into()
    }
}

impl BaseExprAstBehavior for TermAst {
    fn to_base_expr(&self, domain: &Domain) -> BaseExpr {
        let term = self.to_term(domain);
        term.into()
    }
}

#[derive(Clone, Debug)]
pub struct ArithExprAst {
    pub op: ArithmeticOp,
    pub left: Box<ExprAst>,
    pub right: Box<ExprAst>,
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
pub enum ConstraintAst {
    PredicateAst,
    BinaryAst,
    TypeConstraintAst,
}

#[derive(Clone, Debug)]
pub struct TypeConstraintAst {
    pub var: TermAst,
    pub sort: TypeDefAst,
}

impl ConstraintAstBehavior for TypeConstraintAst {
    fn to_constraint(&self, domain: &Domain) -> Constraint {
        let typename = self.sort.name().unwrap();
        let sort = domain.type_map.get(&typename).unwrap().clone();

        TypeConstraint {
            var: self.var.to_term(domain),
            sort,
        }.into()
    }
}


#[derive(Clone, Debug)]
pub struct PredicateAst {
    pub negated: bool,
    pub term: TermAst,
    pub alias: Option<String>,
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
pub struct BinaryAst {
    pub op: BinOp,
    pub left: ExprAst,
    pub right: ExprAst,
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

#[derive(Clone, Debug)]
pub struct RuleAst {
    pub head: Vec<TermAst>,
    pub body: Vec<ConstraintAst>,
}

impl RuleAst {
    pub fn to_rule(&self, domain: &Domain) -> Rule {
        let mut head = vec![];
        for term_ast in self.head.clone() {
            head.push(term_ast.to_term(domain));
        }

        let mut body = vec![];
        for constraint_ast in self.body.clone() {
            body.push(constraint_ast.to_constraint(domain));
        }

        Rule::new(head, body)
    }
}