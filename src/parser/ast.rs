use std::collections::*;
use std::sync::Arc;
use std::borrow::Borrow;
use num::*;

use crate::term::*;
use crate::type_system::*;
use crate::expression::*;
use crate::rule::*;
use crate::constraint::*;
use crate::module::*;
use crate::util::*;
use crate::util::map::*;
use crate::util::wrapper::*;

// #[enum_dispatch]
// trait TermAstBehavior {}
// impl TermAstBehavior for CompositeTermAst {}
// impl TermAstBehavior for VariableTermAst {}
// impl TermAstBehavior for AtomEnum {}

// #[enum_dispatch(TermAstBehavior)]
#[derive(Debug, Clone)]
pub enum TermAst {
    CompositeTermAst(CompositeTermAst),
    VariableTermAst(VariableTermAst),
    AtomTermAst(AtomEnum),
}

#[derive(Debug, Clone)]
pub struct CompositeTermAst {
    pub sort: TypeDefAst,
    pub arguments: Vec<Box<TermAst>>,
    pub alias: Option<String>
}

#[derive(Debug, Clone)]
pub struct VariableTermAst {
    pub root: String,
    pub fragments: Vec<String>
}

pub trait TypeDefAstBehavior {
    fn name(&self) -> Option<String>;
}

#[derive(Debug, Clone)]
pub enum TypeDefAst {
    AliasTypeDefAst(AliasTypeDefAst),
    CompositeTypeDefAst(CompositeTypeDefAst),
    UnionTypeDefAst(UnionTypeDefAst),
    RangeTypeDefAst(RangeTypeDefAst),
    EnumTypeDefAst(EnumTypeDefAst),
}

impl TypeDefAstBehavior for TypeDefAst {
    fn name(&self) -> Option<String> {
        match self {
            TypeDefAst::AliasTypeDefAst(a) => a.name(),
            TypeDefAst::CompositeTypeDefAst(a) => a.name(),
            TypeDefAst::UnionTypeDefAst(a) => a.name(),
            TypeDefAst::RangeTypeDefAst(a) => a.name(),
            TypeDefAst::EnumTypeDefAst(a) => a.name(),
        }
    }
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
    pub items: Vec<TermAst>,
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
    pub fn build_env<T>(&self) -> Env<T> 
    where 
        T: TermStructure, 
        T::SortOutput: Into<AtomicType>
    {
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

    /// Recursively create atomic type `AtomicStrType`, add the new created type into the type map
    /// and return the new created type or just return what already exists in the type map.
    fn create_atomic_type<S>(
        &self, t: String, 
        ast_map: &mut HashMap<String, TypeDefAst>,      // Only has type ASTs in current domain.
        type_map: &mut HashMap<String, S>,  // Recursively put new created type into type map.
        generator: &mut NameGenerator,
    ) -> S
    where S: BorrowedType + Into<AtomicType>
    {
        if type_map.contains_key(&t) {
            let existing_type = type_map.get(&t).unwrap();
            return existing_type.clone();
        } 

        let type_ast = ast_map.get(&t).unwrap();
        let new_type = match type_ast.clone() {
            TypeDefAst::AliasTypeDefAst(aliastypedef) => {
                // At this point, type from subdomains should be available in `type_map`.
                let full_name = aliastypedef.name().unwrap();
                if type_map.contains_key(&full_name) {
                    type_map.get(&full_name).unwrap().clone()
                } else {
                    self.create_atomic_type(full_name, ast_map, type_map, generator)
                }
            },
            TypeDefAst::CompositeTypeDefAst(ctypedef) => {
                let mut args = vec![];
                let typename = ctypedef.name().unwrap();
                for (id_opt, arg_ast) in ctypedef.args.iter() {
                    // The name could be None if that's an inline type definition like `A ::= (a: B + C)`.
                    let name = match arg_ast.name() {
                        Some(name) => name,
                        None => {
                            // Need to add it to `ast_map` because only ASTs of named type are included. 
                            let auto_name = generator.generate_name();
                            ast_map.insert(auto_name.clone(), arg_ast.as_ref().clone());
                            auto_name
                        }
                    };

                    let subtype_opt = type_map.get(&name);
                    let subtype = match subtype_opt {
                        Some(t) => { t.clone() },
                        None => { 
                            self.create_atomic_type(name, ast_map, type_map, generator)
                        }
                    };

                    // Convert from associated type into AtomicType.
                    let arg = (id_opt.clone(), subtype.into());
                    args.push(arg);
                }

                let composite_type = Type::CompositeType(
                    CompositeType {
                        name: typename,
                        arguments: args,
                    }
                );

                composite_type.into()
            },
            TypeDefAst::UnionTypeDefAst(utypedef) => {
                let mut subtypes = vec![];
                let typename = match utypedef.name() {
                    Some(name) => name,
                    None => {
                        // Need to add it to `ast_map` because only ASTs of named type are included. 
                        let auto_name = generator.generate_name();
                        ast_map.insert(auto_name.clone(), type_ast.clone());
                        auto_name
                    }
                };

                for subtype_ast in utypedef.subtypes.iter() {
                    // It could be a enum type without name.
                    let name = match subtype_ast.name() {
                        Some(name) => name,
                        None => {
                            // Need to add it to `ast_map` because only ASTs of named type are included. 
                            let auto_name = generator.generate_name();
                            ast_map.insert(auto_name.clone(), subtype_ast.as_ref().clone());
                            auto_name
                        }
                    };

                    let subtype_opt = type_map.get(&name);
                    let subtype = match subtype_opt {
                        Some(t) => { t.clone() },
                        None => { self.create_atomic_type(name, ast_map, type_map, generator) }
                    };

                    // Convert from associated type into AtomicType.
                    subtypes.push(subtype.into());
                }

                let union_type = Type::UnionType(
                    UnionType {
                        name: typename,
                        subtypes: subtypes,
                    }
                );
                
                union_type.into()
            },
            TypeDefAst::EnumTypeDefAst(etypedef) => {
                let mut items = vec![];
                // Convert generic type map into atomic type map only for EnumType.
                let mut atomic_type_map: HashMap<String, AtomicType> = HashMap::new();
                for (key, val) in type_map.iter() {
                    atomic_type_map.insert(key.clone(), val.clone().into());
                }

                for term_ast in etypedef.items.clone() {
                    // type map must use AtomicType here and generic type is not accepted here.
                    let term = AtomicTerm::from_term_ast(&term_ast, &atomic_type_map);
                    if let AtomicTerm::Variable(v) = term {
                        // Create a constant from variable term with a nullary type.
                        let constant = AtomicTerm::create_constant(v.root).into();
                        items.push(constant);
                    } else if let AtomicTerm::Atom(_) = term {
                        items.push(term);
                    }
                }

                let name = match etypedef.name() {
                    Some(name) => name,
                    None => {
                        // Need to add it to `ast_map` because only ASTs of named type are included. 
                        let auto_name = generator.generate_name();
                        ast_map.insert(auto_name.clone(), type_ast.clone());
                        auto_name
                    }
                };

                let enum_type = Type::EnumType(EnumType { name, items });
                enum_type.into()
            },
            _ => { 
                unimplemented!()
            }
        };

        // Add new type to the type map.
        type_map.insert(t, new_type.clone());
        return new_type;
    }

    pub fn import_builtin_types<S: BorrowedType>(&self, type_map: &mut HashMap<String, S>) {
        // TODO: There are more types to add here.
        let string_type = Type::BaseType(BaseType::String);
        let integer_type = Type::BaseType(BaseType::Integer);
        let bool_type = Type::BaseType(BaseType::Boolean);
        let undefined_type = Type::undefined();
        type_map.insert("String".to_string(), string_type.into());
        type_map.insert("Integer".to_string(), integer_type.into());
        type_map.insert("Boolean".to_string(), bool_type.into());
        type_map.insert("~Undefined".to_string(), undefined_type.into());
    }

    pub fn create_transform<T>(
        &self, transform_name: String, 
        transform_map: &mut HashMap<String, Transform<T>>,
        domain_map: &mut HashMap<String, Domain<T>>
    ) -> Transform<T> 
    where 
        T: TermStructure, 
        T::SortOutput: Into<AtomicType>
    {
        if transform_map.contains_key(&transform_name) {
            return transform_map.get(&transform_name).unwrap().clone();
        }

        let mut generator = NameGenerator::new(&format!("{}_AUTOTYPE", transform_name)[..]);

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

            for (type_name, formula_type) in domain.meta_info().type_map().iter() {
                let formula_type: &Type = formula_type.borrow();
                match formula_type {
                    Type::BaseType(_) => {},
                    _ => {
                        let renamed_type = formula_type.rename_type(tag.clone());
                        let atomic_renamed_type: T::SortOutput = renamed_type.into();
                        let new_name = atomic_renamed_type.unique_form();
                        type_map.insert(new_name.clone(), atomic_renamed_type);
                    }
                }
            }
        }

        // Add types that are defined in transform.
        let mut type_ast_map = HashMap::new();
        for type_ast in transform_ast.typedefs.iter() {
            let name = match type_ast.name() {
                Some(name) => name,
                None => {
                    generator.generate_name()
                }
            };
            type_ast_map.insert(name, type_ast.clone());
        } 

        let type_names: Vec<String> = type_ast_map.keys().map(|x| x.clone()).collect();
        for type_name in type_names {
            self.create_atomic_type(type_name.clone(), &mut type_ast_map, &mut type_map, &mut generator);
        }

        let temp_metainfo = MetaInfo::new(type_map, vec![]);

        // Add rules into domain and converting rule ASTs need type information in domain.
        let mut rules = vec![];
        for rule_ast in transform_ast.rules.iter() {
            rules.push(rule_ast.to_rule(&temp_metainfo));
        }

        // Add terms that defined in the transform.
        let mut terms = HashSet::new();
        for term_ast in transform_ast.terms.iter() {
            let term = T::from_term_ast(term_ast, temp_metainfo.type_map());
            terms.insert(term.into());
        }

        // Some parameters that are known types in `type_map`
        for (_, type_ast) in input_type_ast_map {
            let input_type = temp_metainfo.type_map().get(&type_ast.name().unwrap()).unwrap();
        }

        let transform = Transform::new(
            transform_name.clone(),
            temp_metainfo.type_map().clone(),
            rules,
            params,
            input_type_map,
            input_domain_map,
            output_domain_map,
            terms
        );

        transform_map.insert(transform_name.clone(), transform.clone());
        transform
    }

    pub fn create_domain<T>(&self, 
        domain_name: String, 
        domain_map: &mut HashMap<String, Domain<T>>
    ) -> Domain<T> 
    where T: TermStructure, T::SortOutput: Into<AtomicType>
    {
        if domain_map.contains_key(&domain_name) { 
            return domain_map.get(&domain_name).unwrap().clone(); 
        }

        let mut type_map = HashMap::new();
        self.import_builtin_types(&mut type_map);
        let mut generator = NameGenerator::new(&format!("{}_AUTOTYPE", domain_name)[..]);
        let domain_ast = self.domain_ast_map.get(&domain_name).unwrap();

        for subdomain_name in domain_ast.subdomains.iter() {
            let subdomain = self.create_domain(subdomain_name.clone(), domain_map);
            type_map.extend(subdomain.meta_info().type_map().clone());

            if domain_ast.inherit_type == "extends" {
                // TODO: import comformance rules if inheritance type is `extends`.
            }
        }

        for (scope, subdomain_name) in domain_ast.renamed_subdomains.iter() {
            let subdomain = self.create_domain(subdomain_name.clone(), domain_map);
            for (type_name, formula_type) in subdomain.meta_info().type_map().iter() {
                let formula_type: &Type = formula_type.borrow();
                match formula_type {
                    Type::BaseType(_) => {},
                    _ => {
                        let atomic_renamed_type: T::SortOutput = formula_type.rename_type(scope.clone()).into();
                        let name = atomic_renamed_type.unique_form();
                        type_map.insert(name.clone(), atomic_renamed_type);
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
            let name = match type_ast.name() {
                Some(name) => name,
                None => {
                    // When it's an inline type definition.
                    generator.generate_name()
                }
            };
            type_ast_map.insert(name, type_ast.clone());
        }

        let type_names: Vec<String> = type_ast_map.keys().map(|x| x.clone()).collect();
        for type_name in type_names {
            self.create_atomic_type(type_name.clone(), &mut type_ast_map, &mut type_map, &mut generator);
        }

        let empty_rules: Vec<Rule<T>> = vec![];
        let mut metainfo = MetaInfo::new(type_map, empty_rules);

        for rule_ast in domain_ast.rules.iter() {
            let rule = rule_ast.to_rule(&metainfo);
            metainfo.add_rule(rule);
        }
        
        let domain = Domain {
            name: domain_name.clone(),
            metainfo,
        };

        domain_map.insert(domain_name.clone(), domain.clone());
        return domain;
    }

    /// Don't return the model because deep copy of large amounts of data
    /// is too expensive and all models are stored in `model_map`. By default
    /// use atomic term and atomic type to replace the generic params.
    pub fn create_model<T>(
        &self, 
        model_name: String,
        model_map: &mut HashMap<String, Model<T>>, 
        domain_map: &HashMap<String, Domain<T>>) 
    where 
        T: TermStructure,
        // T::SortOutput: Into<AtomicType>
    {
        if model_map.contains_key(&model_name) { return; }
        
        let model_ast = self.model_ast_map.get(&model_name).unwrap();
        let domain = domain_map.get(&model_ast.domain_name).unwrap();
        let undefined_sort = domain.meta_info().type_map().get("~Undefined").unwrap();

        // Store terms that don't have alias.
        let mut raw_terms = vec![];
        // Map alias to term while the term could have variable that points to another term.
        let mut raw_alias_map = HashMap::new();

        // alias map and terms store after variable propagation.
        let mut alias_map = HashMap::new();
        let mut model_store = HashSet::new();
        
        for term_ast in model_ast.models.iter() {
            // Something tricky here: A renamed alias could be treated as a variable.
            // e.g. Iso(Left.v1, Right.v2) after parsing the arguments are variables with fragments.
            // Terms in the model should not contain variables with fragments not even in partial model.
            let mut term = T::from_term_ast(term_ast, domain.meta_info().type_map());

            // Reursively traverse the term to find all variables with fragments and fix them.
            term.traverse_mut(
                &|t| { 
                    t != t.root() // Check if that's a variable term with fragments.
                }, 
                &mut |mut t| {
                    // Use display name as the root name for the new variable term.
                    let name = format!("{}", t); 
                    *t = T::create_variable_term(Some(undefined_sort.clone()), name, vec![]);
                }
            );

            // Remove alias from term and return alias.
            let alias = term.remove_alias();

            match alias {
                None => {
                    // if term does not have alias, add it to model store.
                    raw_terms.push(term);
                },
                Some(alias) => {
                    // alias here shouldn't have fragments and add term to model store later.
                    let vterm = T::create_variable_term(Some(undefined_sort.clone()), alias, vec![]);
                    raw_alias_map.insert(vterm, term);
                }
            }
        }

        // TODO: Need to check if they are duplicates from sub-models.
        // Import sub-models into `model_store` with a copy of Arc<Term>.
        for submodel_name in model_ast.submodels.iter() {
            self.create_model(submodel_name.clone(), model_map, domain_map);
            let submodel = model_map.get(submodel_name).unwrap();

            // Copy all terms.
            for term_ref in submodel.terms() {
                if !model_store.contains(term_ref) {
                    model_store.insert(term_ref.clone());
                }
            }
            // Copy alias map to raw alias map.
            raw_alias_map.extend(submodel.model_store().alias_map().clone());
        }

        // Import renamed sub-models with the type changed.
        for scope in model_ast.renamed_submodels.keys() {
            let submodel_name = model_ast.renamed_submodels.get(scope).unwrap();
            self.create_model(submodel_name.clone(), model_map, domain_map);

            // TODO: Submodels should be imported as traces.
            // Just make a deep copy as they are all Arc<Term> and rename the whole model.
            let submodel = model_map.get(submodel_name).unwrap().clone();
            let renamed_submodel = submodel.rename(scope.clone(), submodel.meta_info().type_map());
            let submodel_terms: Vec<_> = renamed_submodel.terms().into_iter().map(|x| x.clone()).collect();
            model_store.extend(submodel_terms);
            alias_map.extend(renamed_submodel.model_store().alias_map().clone());
        }

        // Alias in the raw term is treated as variables and needs to be replaced with the real term.
        // Term propagations have to follow the order that for example n1 = Node(x) needs to be handled 
        // prior to e1 is Edge(n1, n1), otherwise the raw term may be used in propagation.
        for key in raw_alias_map.keys() {
            self.propagate_alias_map(key.clone(), &raw_alias_map, &mut alias_map);
        }

        // Propagate binding to composite terms that don't have alias associated with them.
        for term in raw_terms {
            let new_term = term.propagate_bindings(&alias_map);
            model_store.insert(new_term);
        }

        for (_, term) in alias_map.iter() {
            model_store.insert(term.clone());
        }

        let model = Model::new(
            model_name.clone(), 
            domain, 
            model_store, 
            alias_map
        );

        model_map.insert(model_name.clone(), model);
    }

    fn propagate_alias_map<M, T>(&self, var: T, raw_alias_map: &M, alias_map: &mut M) 
    where 
        M: GenericMap<T, T>, 
        T: TermStructure
    {
        let raw_term = raw_alias_map.gget(&var).unwrap();
        // if current term has variables inside then propagate binding to them first.
        let raw_term_vars = raw_term.clone().variables();
        for raw_term_var in raw_term_vars {
            if raw_alias_map.contains_gkey(&raw_term_var) {
                self.propagate_alias_map(raw_term_var, raw_alias_map, alias_map);
            }
        }
        
        // Make a little clone here but that's ok for a tradeoff between elegancy and unnecessary clone.
        let new_term = raw_term.propagate_bindings(alias_map);
        let k = var.clone();
        let v = new_term.clone();
        alias_map.ginsert(k, v);
    }
}

trait ExprAstBehavior {
    fn to_expr<T>(&self, metainfo: &MetaInfo<T>) -> Expr<T> where T: TermStructure;
}

#[derive(Clone, Debug)]
pub enum ExprAst {
    BaseExprAst(BaseExprAst),
    ArithExprAst(ArithExprAst),
}

impl ExprAstBehavior for ExprAst {
    fn to_expr<T>(&self, metainfo: &MetaInfo<T>) -> Expr<T> where T: TermStructure {
        match self {
            ExprAst::BaseExprAst(b) => b.to_expr(metainfo),
            ExprAst::ArithExprAst(a) => a.to_expr(metainfo)
        }
    }
}

trait BaseExprAstBehavior {
    fn to_base_expr<T>(&self, metainfo: &MetaInfo<T>) -> BaseExpr<T> where T: TermStructure;
}

#[derive(Clone, Debug)]
pub enum BaseExprAst {
    SetComprehensionAst(SetComprehensionAst),
    TermAst(TermAst),
}

impl BaseExprAstBehavior for BaseExprAst {
    fn to_base_expr<T>(&self, metainfo: &MetaInfo<T>) -> BaseExpr<T> where T: TermStructure {
        match self {
            BaseExprAst::SetComprehensionAst(s) => s.to_base_expr(metainfo),
            BaseExprAst::TermAst(t) => t.to_base_expr(metainfo)
        }
    }
}

impl ExprAstBehavior for BaseExprAst {
    fn to_expr<T>(&self, metainfo: &MetaInfo<T>) -> Expr<T> where T: TermStructure {
        let base_expr = self.to_base_expr(metainfo);
        Expr::BaseExpr(base_expr)
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
    fn to_base_expr<T>(&self, metainfo: &MetaInfo<T>) -> BaseExpr<T> where T: TermStructure {
        let mut vars = vec![];
        let mut condition = vec![];
        // Need HashMap<String, AtomicStrType> instead of HashMap<String, S>.
        let atomic_type_map = metainfo.atomic_type_map();
        for term_ast in self.vars.iter() {
            let term = T::from_term_ast(term_ast, metainfo.type_map());
            vars.push(term);
        }

        for constraint_ast in self.condition.clone() {
            condition.push(constraint_ast.to_constraint(metainfo));
        }
        
        // Count and Sum operator does not have explicit default value but let's set it to 0.
        let default = match self.default.clone() {
            None => { BigInt::from_i64(0 as i64).unwrap() },
            Some(val) => { val },
        };

        let setcompre = SetComprehension::new( 
            vars,
            condition,
            self.op.clone(),
            default,
        );

        BaseExpr::SetComprehension(setcompre)
    }
}

impl BaseExprAstBehavior for TermAst {
    fn to_base_expr<T>(&self, metainfo: &MetaInfo<T>) -> BaseExpr<T> where T: TermStructure {
        let term = T::from_term_ast(self, metainfo.type_map());
        BaseExpr::Term(term)
    }
}

#[derive(Clone, Debug)]
pub struct ArithExprAst {
    pub op: ArithmeticOp,
    pub left: Box<ExprAst>,
    pub right: Box<ExprAst>,
}

impl ExprAstBehavior for ArithExprAst {
    fn to_expr<T>(&self, metainfo: &MetaInfo<T>) -> Expr<T> where T: TermStructure {
        let left = self.left.to_expr(metainfo);
        let right = self.right.to_expr(metainfo);
        let arith = ArithExpr {
            op: self.op.clone(),
            left: Arc::new(left),
            right: Arc::new(right),
        };
        Expr::ArithExpr(arith)
    }
}

trait ConstraintAstBehavior {
    fn to_constraint<T>(&self, metainfo: &MetaInfo<T>) -> Constraint<T> where T: TermStructure;
}

#[derive(Clone, Debug)]
pub enum ConstraintAst {
    PredicateAst(PredicateAst),
    BinaryAst(BinaryAst),
    TypeConstraintAst(TypeConstraintAst),
}

impl ConstraintAstBehavior for ConstraintAst {
    fn to_constraint<T>(&self, metainfo: &MetaInfo<T>) -> Constraint<T> where T: TermStructure {
        match self {
            ConstraintAst::PredicateAst(p) => p.to_constraint(metainfo),
            ConstraintAst::BinaryAst(b) => b.to_constraint(metainfo),
            ConstraintAst::TypeConstraintAst(t) => t.to_constraint(metainfo)
        }
    }
}

#[derive(Clone, Debug)]
pub struct TypeConstraintAst {
    pub var: TermAst,
    pub sort: TypeDefAst,
}

impl ConstraintAstBehavior for TypeConstraintAst {
    fn to_constraint<T>(&self, metainfo: &MetaInfo<T>) -> Constraint<T> where T: TermStructure {
        let typename = self.sort.name().unwrap();
        let sort = metainfo.type_map().get(&typename).unwrap().clone();
        let term = T::from_term_ast(&self.var, metainfo.type_map());
        let tc = TypeConstraint { var: term, sort };
        Constraint::TypeConstraint(tc)
    }
}


#[derive(Clone, Debug)]
pub struct PredicateAst {
    pub negated: bool,
    pub term: TermAst,
    pub alias: Option<String>,
}

impl ConstraintAstBehavior for PredicateAst {
    fn to_constraint<T>(&self, metainfo: &MetaInfo<T>) -> Constraint<T> where T: TermStructure {
        // Use `AtomicStrType` and atomic terms by default for the terms in constraints.
        let undefined = metainfo.type_map().get("~Undefined").unwrap().clone();
        let alias = match self.alias.clone() {
            None => None,
            Some(a) => {
                let term = T::create_variable_term(None, a, vec![]);
                Some(term)
            }
        };
        
        let real_term = match &self.term {
            TermAst::VariableTermAst(var_ast) => {
                // Convert variable term into a constant (A composite term with zero argument.
                let (_, nullary_term) = T::create_constant(var_ast.root.clone());
                nullary_term
            },
            _ => { 
                let term = T::from_term_ast(&self.term, metainfo.type_map());
                term
            }
        };

        let pred = Predicate {
            negated: self.negated,
            term: real_term.into(),
            alias,
        };

        Constraint::Predicate(pred)
    }
}

#[derive(Clone, Debug)]
pub struct BinaryAst {
    pub op: BinOp,
    pub left: ExprAst,
    pub right: ExprAst,
}

impl ConstraintAstBehavior for BinaryAst {
    fn to_constraint<T>(&self, metainfo: &MetaInfo<T>) -> Constraint<T> where T: TermStructure {
        let bin = Binary {
            op: self.op.clone(),
            left: self.left.to_expr(metainfo),
            right: self.right.to_expr(metainfo),
        };
        Constraint::Binary(bin)
    }
}

#[derive(Clone, Debug)]
pub struct RuleAst {
    pub head: Vec<TermAst>,
    pub body: Vec<ConstraintAst>,
}

impl RuleAst {
    pub fn to_rule<T>(&self, metainfo: &MetaInfo<T>) -> Rule<T> where T: TermStructure {
        let mut head = vec![];
        let atomic_type_map = metainfo.atomic_type_map();
        for term_ast in self.head.iter() {
            let term = T::from_term_ast(term_ast, metainfo.type_map());
            head.push(term);
        }

        let mut body = vec![];
        for constraint_ast in self.body.clone() {
            body.push(constraint_ast.to_constraint(metainfo));
        }

        Rule::new(head, body)
    }
}