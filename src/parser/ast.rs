use std::collections::*;
use std::sync::Arc;
use std::convert::TryInto;
use std::borrow::Borrow;
use enum_dispatch::enum_dispatch;
use num::*;

use crate::term::*;
use crate::type_system::*;
use crate::expression::*;
use crate::rule::*;
use crate::constraint::*;
use crate::module::*;
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
    pub fn to_term<FM: FormulaModule>(&self, module: &FM) -> Term {
        match self {
            TermAst::CompositeTermAst(cterm_ast) => {
                let mut term_arguments = vec![];
                for argument in cterm_ast.arguments.clone() {
                    let term = argument.to_term(module);
                    term_arguments.push(Arc::new(term));
                }
                let sort_name = cterm_ast.sort.name().unwrap();
                let sort = module.type_map().get(&sort_name).unwrap().clone();
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
pub enum ModuleSentenceAst {
    Type(TypeDefAst),
    Rule(RuleAst),
    Term(TermAst),
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
    pub output: Vec<TaggedDomainAst>, 
    pub typedefs: Vec<TypeDefAst>,
    pub rules: Vec<RuleAst>,
    pub terms: Vec<TermAst>,
}

#[derive(Clone, Debug)]
pub struct TransformationAst {
    pub result_name: String,
    pub transform_name: String,
    pub params: Vec<TermAst>
}

#[derive(Clone, Debug)]
pub struct DomainAst {
    pub name: String,
    pub types: Vec<TypeDefAst>,
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
            self.create_transform(
                transform_name.clone(), 
                &mut transform_map, 
                &mut domain_map
            );
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

    pub fn create_transform(
        &self, transform_name: String, 
        transform_map: &mut HashMap<String, Transform>,
        domain_map: &mut HashMap<String, Domain>
    ) -> Transform 
    {
        if transform_map.contains_key(&transform_name) {
            return transform_map.get(&transform_name).unwrap().clone();
        }

        // Those are the params and returns for transform(x1, x2 ... x3) -> (y1, y2, y3)
        let mut input_type_map = HashMap::new();
        let mut input_domain_map = HashMap::new();
        let mut output_domain_map = HashMap::new();

        let transform_ast = self.transform_ast_map.get(&transform_name).unwrap();

        let mut params = vec![];
        let mut input_type_ast_map = HashMap::new();
        let mut tagged_domain_asts = vec![];

        // Need to store the position of each param in transformation.
        for param in transform_ast.inputs.iter() {
            match param {
                TransformParamAst::TaggedDomain(d) => {
                    params.push(d.tag.clone());
                },
                TransformParamAst::TaggedType(t) => {
                    params.push(t.tag.clone());
                }
            }
        }

        for output_tagged_domain_ast in transform_ast.output.clone() {
            // Find the domain and add it as one of the params for transform's output.
            let tag = output_tagged_domain_ast.tag.clone();
            let domain_name = output_tagged_domain_ast.domain.clone();
            let domain = self.create_domain(domain_name.clone(), domain_map);
            output_domain_map.insert(domain_name, domain);
            tagged_domain_asts.push(output_tagged_domain_ast);
        }

        for input in transform_ast.inputs.clone() {
            match input {
                TransformParamAst::TaggedDomain(input_tagged_domain_ast) => {
                    let tag = input_tagged_domain_ast.tag.clone();
                    let domain_name = input_tagged_domain_ast.domain.clone();
                    let domain = self.create_domain(domain_name.clone(), domain_map);
                    input_domain_map.insert(domain_name, domain);
                    tagged_domain_asts.push(input_tagged_domain_ast);
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

        // Get all type maps from each domain and merge them together with renamed types.
        for tagged_domain_ast in tagged_domain_asts.iter() {
            let tag = tagged_domain_ast.tag.clone();
            let domain_name = tagged_domain_ast.domain.clone();
            let domain = self.create_domain(domain_name, domain_map);

            for (type_name, formula_type_arc) in domain.type_map.iter() {
                let formula_type: Type = formula_type_arc.as_ref().clone();
                match formula_type {
                    Type::BaseType(_) => {},
                    _ => {
                        let renamed_type = formula_type.rename_type(tag.clone());
                        type_map.insert(renamed_type.name(), Arc::new(renamed_type));
                    }
                }
            }
        }

        // Add types that are defined in transform.
        let mut type_ast_map = HashMap::new();
        for type_ast in transform_ast.typedefs.iter() {
            let name = type_ast.name().unwrap();
            type_ast_map.insert(name, type_ast.clone());
        } 

        for type_name in type_ast_map.keys() {
            self.create_type(type_name.clone(), &type_ast_map, &mut type_map);
        }

        // A temporary domain for transform to create rules and term.
        let temp_transform_domain = Domain {
            name: transform_name.clone(),
            type_map: type_map.clone(),
            rules: vec![],
        };

        // Add rules into domain and converting rule ASTs need type information in domain.
        let mut rules = vec![];
        for rule_ast in transform_ast.rules.iter() {
            rules.push(rule_ast.to_rule(&temp_transform_domain));
        }

        // Add terms that defined in the transform.
        let mut terms = HashSet::new();
        for term_ast in transform_ast.terms.iter() {
            let term = Arc::new(term_ast.to_term(&temp_transform_domain));
            terms.insert(term);
        }

        // Some parameters that are known types in `type_map`
        for (_, type_ast) in input_type_ast_map {
            let input_type = type_map.get(&type_ast.name().unwrap()).unwrap();
        }

        let mut transform = Transform {
            name: transform_name.clone(),
            type_map,
            rules,
            params,
            input_type_map,
            input_domain_map,
            output_domain_map,
            terms
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
        for type_ast in domain_ast.types.iter() {
            let name = type_ast.name().unwrap();
            type_ast_map.insert(name, type_ast.clone());
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

        // Store terms that don't have alias.
        let mut raw_terms = vec![];
        // Map alias to term while the term could have variable that points to another term.
        let mut raw_alias_map = HashMap::new();

        // alias map and terms store after variable propagation.
        let mut alias_map: HashMap<Arc<Term>, Arc<Term>> = HashMap::new();
        let mut model_store = HashSet::new();
        
        // Deep clone is not really necessary for keeping all ASTs but just keep them.
        for term_ast in model_ast.models.clone() {
            // Something tricky here: A renamed alias could be treated as a variable.
            // e.g. Iso(Left.v1, Right.v2) after parsing the arguments are variables with fragments.
            // Terms in the model should not contain variables with fragments not even in partial model.
            let mut term = term_ast.to_term(domain);

            // Reursively traverse the term to find all variables with fragments and fix them.
            term.traverse_mut(
                &|t| { 
                    match t {
                        Term::Variable(v) => { v.fragments.len() > 0 },
                        _ => false
                    }
                }, 
                &mut |mut t| {
                    // Use displayed name as the root name for the new variable term.
                    let name = format!("{}", t); 
                    *t = Variable::new(name, vec![]).into();
                }
            );

            match &term {
                Term::Composite(c) => {
                    match &c.alias {
                        None => {
                            // if term does not have alias, add it to model store.
                            raw_terms.push(term);
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

        // TODO: Need to check if they are duplicates from sub-models.
        // Import sub-models into `model_store` with a copy of Arc<Term>.
        for submodel_name in model_ast.submodels.iter() {
            self.create_model(submodel_name.clone(), model_map, domain_map);
            let submodel = model_map.get(submodel_name).unwrap();

            // Copy all terms.
            for term_arc in submodel.terms.iter() {
                if !model_store.contains(term_arc) {
                    model_store.insert(term_arc.clone());
                }
            }
            // Copy alias map to raw alias map.
            raw_alias_map.extend(submodel.alias_map.clone());
        }

        // Import renamed sub-models with the type changed.
        for scope in model_ast.renamed_submodels.keys() {
            let submodel_name = model_ast.renamed_submodels.get(scope).unwrap();
            self.create_model(submodel_name.clone(), model_map, domain_map);

            // Just make a deep copy since they are all Arc<Term>.
            let submodel = model_map.get(submodel_name).unwrap();
            
            // Copy all renamed terms.
            for term_arc in submodel.terms.iter() {
                let renamed_term = term_arc.rename(scope.clone());
                // Because the renamed term has unique scope so don't need to check duplicates.
                model_store.insert(Arc::new(renamed_term));
            }

            // Copy renamed alias map to raw alias map.
            for (name, term) in submodel.alias_map.iter() {
                raw_alias_map.insert(
                    Arc::new(name.rename(scope.clone())), 
                    Arc::new(term.rename(scope.clone()))
                );
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

        // Propagate binding to composite terms that don't have alias associated with them.
        for term in raw_terms {
            let new_term = term.propagate_bindings(&alias_map).unwrap();
            model_store.insert(new_term);
        }

        for (key, term) in alias_map.iter() {
            model_store.insert(term.clone());
        }

        let model = Model::new(
            model_name.clone(), 
            domain.clone(), 
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
            if raw_alias_map.contains_gkey(&raw_term_var) {
                self.propagate_alias_map(&raw_term_var, raw_alias_map, alias_map);
            }
        }
    
        let new_term = raw_term.propagate_bindings(alias_map).unwrap();
        alias_map.ginsert(Arc::new(var.clone()), new_term);
    }
}

#[enum_dispatch(ExprAst)]
trait ExprAstBehavior {
    fn to_expr<FM: FormulaModule>(&self, domain: &FM) -> Expr;
}

#[enum_dispatch]
#[derive(Clone, Debug)]
pub enum ExprAst {
    BaseExprAst,
    ArithExprAst,
}

#[enum_dispatch(BaseExprAst)]
trait BaseExprAstBehavior {
    fn to_base_expr<FM: FormulaModule>(&self, module: &FM) -> BaseExpr;
}

#[enum_dispatch]
#[derive(Clone, Debug)]
pub enum BaseExprAst {
    SetComprehensionAst,
    TermAst,
}

impl ExprAstBehavior for BaseExprAst {
    fn to_expr<FM: FormulaModule>(&self, module: &FM) -> Expr {
        let base_expr = self.to_base_expr(module);
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
    fn to_base_expr<FM: FormulaModule>(&self, module: &FM) -> BaseExpr {
        let mut vars = vec![];
        let mut condition = vec![];
        for term_ast in self.vars.clone() {
            vars.push(term_ast.to_term(module));
        }

        for constraint_ast in self.condition.clone() {
            condition.push(constraint_ast.to_constraint(module));
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
    fn to_base_expr<FM: FormulaModule>(&self, module: &FM) -> BaseExpr {
        let term = self.to_term(module);
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
    fn to_expr<FM: FormulaModule>(&self, module: &FM) -> Expr {
        let left = self.left.to_expr(module);
        let right = self.right.to_expr(module);

        ArithExpr {
            op: self.op.clone(),
            left: Arc::new(left),
            right: Arc::new(right),
        }.into()
    }
}

#[enum_dispatch(ConstraintAst)]
trait ConstraintAstBehavior {
    fn to_constraint<FM: FormulaModule>(&self, module: &FM) -> Constraint;
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
    fn to_constraint<FM: FormulaModule>(&self, module: &FM) -> Constraint {
        let typename = self.sort.name().unwrap();
        let sort = module.type_map().get(&typename).unwrap().clone();

        TypeConstraint { var: self.var.to_term(module), sort }.into()
    }
}


#[derive(Clone, Debug)]
pub struct PredicateAst {
    pub negated: bool,
    pub term: TermAst,
    pub alias: Option<String>,
}

impl ConstraintAstBehavior for PredicateAst {
    fn to_constraint<FM: FormulaModule>(&self, module: &FM) -> Constraint {
        let alias = match self.alias.clone() {
            None => None,
            Some(a) => {
                let term: Term = Variable::new(a, vec![]).into();
                Some(term)
            }
        };
        
        // TermAst is either Term or CompositeTermAst.
        let real_term = match &self.term {
            TermAst::Term(var_term) => {
                // If it's a variable then don't treat it as a variable term,
                // instead convert it into a constant (A composite term with zero argument)
                let var: Variable = var_term.clone().try_into().unwrap();
                let constant = var.root.clone();
                let nullary_term = Term::create_constant(constant);
                nullary_term
            },
            _ => { self.term.to_term(module) }
        };

        Predicate {
            negated: self.negated,
            term: real_term,
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
    fn to_constraint<FM: FormulaModule>(&self, module: &FM) -> Constraint {
        Binary {
            op: self.op.clone(),
            left: self.left.to_expr(module),
            right: self.right.to_expr(module),
        }.into()
    }
}

#[derive(Clone, Debug)]
pub struct RuleAst {
    pub head: Vec<TermAst>,
    pub body: Vec<ConstraintAst>,
}

impl RuleAst {
    pub fn to_rule<FM: FormulaModule>(&self, module: &FM) -> Rule {
        let mut head = vec![];
        for term_ast in self.head.clone() {
            head.push(term_ast.to_term(module));
        }

        let mut body = vec![];
        for constraint_ast in self.body.clone() {
            body.push(constraint_ast.to_constraint(module));
        }

        Rule::new(head, body)
    }
}