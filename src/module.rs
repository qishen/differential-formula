use std::sync::Arc;
use std::vec::Vec;
use std::collections::*;
use std::convert::TryInto;
use std::fmt::*;
use std::string::String;

use enum_dispatch::enum_dispatch;
//use petgraph::


use crate::expression::*;
use crate::term::*;
use crate::rule::*;
use crate::type_system::*;
use crate::util::*;


#[enum_dispatch]
#[derive(Debug, Clone)]
pub enum Program {
    Domain,
    Model,
    Transform,
}

pub trait FormulaModule {
    fn terms(&self) -> HashSet<Arc<Term>>;
    fn rules(&self) -> Vec<Rule>;
    fn conformance_rules(&self) -> Vec<Rule>;
    fn stratified_rules(&self) -> Vec<Vec<Rule>>;
    fn type_map(&self) -> &HashMap<String, Arc<Type>>;
}

#[derive(Debug, Clone)]
pub struct Transform {
    // The name of model transform.
    pub name: String,
    // Includes all types from inputs, output and ones defined in transformation.
    pub type_map: HashMap<String, Arc<Type>>,
    // Rules defined in transformation.
    pub rules: Vec<Rule>,
    // A list of strings representing the Ids of params.
    pub params: Vec<String>,
    // Some parameters in transform are terms.
    pub input_type_map: HashMap<String, Arc<Type>>,
    // The other parameters in transform are domains. 
    pub input_domain_map: HashMap<String, Domain>,
    // The domains in the output of transform.
    pub output_domain_map: HashMap<String, Domain>,
    // Yes, you can defined new terms in transform.
    pub terms: HashSet<Arc<Term>>,
}

impl FormulaModule for Transform {
    fn terms(&self) -> HashSet<Arc<Term>> {
        self.terms.clone()
    }

    fn rules(&self) -> Vec<Rule> {
        self.rules.clone()
    }

    fn conformance_rules(&self) -> Vec<Rule> {
        let mut conformance_rules = vec![];
        for rule in self.rules.clone() {
            if rule.is_conformance_rule() {
                conformance_rules.push(rule);
            }
        }
        conformance_rules
    }

    fn stratified_rules(&self) -> Vec<Vec<Rule>> {
        // TODO: Rules may contains variable like `%id` that needs to be instantiated in transformation.
        let mut stratified_rules = vec![];
        for rule in self.rules.clone() {
            if !rule.is_conformance_rule() {
                stratified_rules.push(rule);
            }
        }
        vec![stratified_rules]
    }

    fn type_map(&self) -> &HashMap<String, Arc<Type>> {
        &self.type_map
    }
}

impl Transform {
    pub fn get_id(&self, position: usize) -> Option<&String> {
        self.params.get(position)
    }
}

/// `Transformation` is the instantiation of `Transform`.
pub struct Transformation {
    pub transform: Transform,
    pub input_term_map: HashMap<String, Term>,
    pub input_model_map: HashMap<String, Model>,
}

impl FormulaModule for Transformation {
    fn terms(&self) -> HashSet<Arc<Term>> {
        // Rename terms in all model params and merge them together.
        let mut merged_terms = HashSet::new();

        for (id, model) in self.input_model_map.iter() {
            let rename_model = model.rename(id.clone());
            merged_terms.extend(rename_model.terms);
        }

        // Some additional terms may be defined in transform even with some %id that need to be replaced.
        for (key, replacement) in self.input_term_map.iter() {
            let pattern: Term = Variable::new(key.clone(), vec![]).into();
            for mut raw_term in self.transform.terms.clone() {
                let mut term = Arc::make_mut(&mut raw_term);
                term.replace(&pattern, replacement);
                merged_terms.insert(raw_term);
            }
        }

        merged_terms
    }

    fn rules(&self) -> Vec<Rule> {
        self.rules().clone()
    }

    fn conformance_rules(&self) -> Vec<Rule> {
        let mut raw_rules = self.transform.conformance_rules();
        for (key, replacement) in self.input_term_map.iter() {
            let pattern: Term = Variable::new(key.clone(), vec![]).into();
            for rule in raw_rules.iter_mut() {
                rule.replace(&pattern, replacement);
            }
        }
        raw_rules
    }

    fn stratified_rules(&self) -> Vec<Vec<Rule>> {
        // TODO: Need to replace things like %id.
        let mut raw_rules = self.transform.stratified_rules();
        for (key, replacement) in self.input_term_map.iter() {
            let pattern: Term = Variable::new(key.clone(), vec![]).into();
            for rule in raw_rules.iter_mut() {
                rule.replace(&pattern, replacement);
            }
        }
        raw_rules
    }

    fn type_map(&self) -> &HashMap<String, Arc<Type>> {
        self.transform.type_map()
    }
}

impl Transformation {
    pub fn new(transform: Transform, input_term_map: HashMap<String, Term>, input_model_map: HashMap<String, Model>) 
    -> Transformation 
    {   
        let mut transformation = Transformation {
            transform,
            input_term_map,
            input_model_map,
        };

        // Replace parameter like %id in transform rules with term from `input_term_map`.
        for (key, replacement) in transformation.input_term_map.iter() {
            let pattern: Term = Variable::new(key.clone(), vec![]).into();
            transformation.transform.rules.replace(&pattern, replacement);
        }

        transformation
    }
}

#[derive(Debug, Clone)]
pub struct Domain {
    pub name: String,
    pub type_map: HashMap<String, Arc<Type>>,
    pub rules: Vec<Rule>,
}

impl FormulaModule for Domain {
    fn terms(&self) -> HashSet<Arc<Term>> {
        HashSet::new()
    }

    fn rules(&self) -> Vec<Rule> {
        self.rules.clone()
    }

    fn conformance_rules(&self) -> Vec<Rule> {
        let mut conformance_rules = vec![];
        for rule in self.rules.clone() {
            if rule.is_conformance_rule() {
                conformance_rules.push(rule);
            }
        }
        conformance_rules
    }

    fn stratified_rules(&self) -> Vec<Vec<Rule>> {
        // TODO: Rules may contains variable like `%id` that needs to be instantiated in transformation.
        let mut stratified_rules = vec![];
        for rule in self.rules.clone() {
            if !rule.is_conformance_rule() {
                stratified_rules.push(rule);
            }
        }
        vec![stratified_rules]
    }

    fn type_map(&self) -> &HashMap<String, Arc<Type>> {
        &self.type_map
    }
}

impl Domain {
    pub fn rename(&self, scope: String) -> Domain {
        let mut renamed_type_map = HashMap::new();
        for (type_name, formula_type_arc) in self.type_map.iter() {
            let formula_type: Type = formula_type_arc.as_ref().clone();
            match formula_type {
                Type::BaseType(_) => {},
                _ => {
                    let renamed_type = formula_type.rename_type(scope.clone());
                    renamed_type_map.insert(renamed_type.name(), Arc::new(renamed_type));
                }
            }
        }

        Domain {
            name: format!("{}.{}", scope.clone(), self.name),
            type_map: renamed_type_map,
            rules: vec![],
        }
    }

    pub fn get_type(&self, name: &String) -> Arc<Type> {
        self.type_map.get(name).unwrap().clone()
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }
}


#[derive(Debug, Clone)]
pub struct Model {
    pub model_name: String,
    pub domain: Domain,
    pub terms: HashSet<Arc<Term>>,
    // variable term to composite term mapping.
    pub alias_map: HashMap<Arc<Term>, Arc<Term>>, 
    // composite term to string alias mapping and the composite can't have alias built in.
    pub reverse_alias_map: HashMap<Arc<Term>, String>, 
}

impl FormulaModule for Model {
    fn terms(&self) -> HashSet<Arc<Term>> {
        self.terms.clone()
    }

    fn rules(&self) -> Vec<Rule> {
        self.domain.rules()
    }

    fn conformance_rules(&self) -> Vec<Rule> {
        self.domain.conformance_rules()
    }

    fn stratified_rules(&self) -> Vec<Vec<Rule>> {
        self.domain.stratified_rules()
    }

    fn type_map(&self) -> &HashMap<String, Arc<Type>> {
        &self.domain.type_map
    }
}

impl Model {
    pub fn new(model_name: String, 
        domain: Domain, 
        terms: HashSet<Arc<Term>>, 
        alias_map: HashMap<Arc<Term>, Arc<Term>>) -> Self
    {
        // Map alias-free term into its alias in string format.
        let mut reverse_alias_map = HashMap::new();
        for key in alias_map.gkeys() {
            let var: Variable = key.root().clone().try_into().unwrap();
            let var_str = var.root; // Assume it shouldn't have fragments in variable term.
            /* 
            Clone each entry in the alias map to create reverse alias map mapping composite term to 
            alias in the format of string and the alias is removed for each key.
            */
            let val_term = alias_map.gget(key).unwrap();
            reverse_alias_map.insert(val_term.clone(), var_str);
        }

        let model = Model {
            model_name,
            domain,
            terms,
            alias_map,
            reverse_alias_map,
        };

        model
    }

    pub fn rename(&self, scope: String) -> Model {
        let renamed_domain = self.domain.rename(scope.clone());
        let mut renamed_terms = HashSet::new();
        let mut renamed_alias_map = HashMap::new();

        for term_arc in self.terms.iter() {
            let term: Term = term_arc.as_ref().clone();
            let renamed_term = term.rename(scope.clone());
            renamed_terms.insert(Arc::new(renamed_term));
        }

        //  Update alias map while keep the same variables.
        for (key, term_arc) in self.alias_map.iter() {
            let term: Term = term_arc.as_ref().clone();
            let renamed_term = term.rename(scope.clone());
            let renamed_key = key.rename(scope.clone());
            renamed_alias_map.insert(Arc::new(renamed_key), Arc::new(renamed_term));
        }

        // TODO: reverse alias map.

        Model::new(
            format!("{}.{}", scope.clone(), self.model_name),
            renamed_domain,
            renamed_terms,
            renamed_alias_map,
        )
    }

    pub fn get_term_by_name(&self, name: &str) -> &Term {
        let var: Term = Variable::new(name.to_string(), vec![]).into();
        self.alias_map.get(&var).unwrap()
    }
}


#[derive(Debug, Clone)]
pub struct Env {
    pub domain_map: HashMap<String, Domain>,
    pub model_map: HashMap<String, Model>,
    pub transform_map: HashMap<String, Transform>,
}


impl Env {
    pub fn new() -> Self {
        Env {
            domain_map: HashMap::new(),
            model_map: HashMap::new(),
            transform_map: HashMap::new(),
        }
    }

    pub fn get_model_by_name(&self, name: &str) -> Option<&Model> {
        // Make a clone of the model in Env and return it.
        match self.model_map.get(name) {
            None => None,
            Some(model) => Some(model),
        }
    }

    pub fn get_domain_by_name(&self, name: &str) -> Option<&Domain> {
        // Make a clone of the domain in Env and return it.
        match self.domain_map.get(name) {
            None => None,
            Some(model) => Some(model),
        }
    }

    pub fn get_transform_by_name(&self, name: &str) -> Option<&Transform> {
        match self.transform_map.get(name) {
            None => None,
            Some(transform) => Some(transform)
        }
    }
}