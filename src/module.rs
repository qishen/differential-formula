use std::borrow::*;
use std::vec::Vec;
use std::collections::*;
use std::fmt::*;
use std::string::String;

use bimap::BiMap;
use enum_dispatch::enum_dispatch;

use crate::expression::*;
use crate::term::*;
use crate::rule::*;
use crate::type_system::*;

#[enum_dispatch]
#[derive(Debug, Clone)]
pub enum Program<T> where T: TermStructure 
{
    Domain(Domain<T>),
    Model(Model<T>),
    Transform(Transform<T>),
}

/// Meta information in a Formula Module like types and rules with a generic type for Formula
/// type to enable easy access of wrapped Formula type when building terms.
#[derive(Clone, Debug)]
pub struct MetaInfo<T> where T: TermStructure {
    // Map string to a wrapped Formula type. 
    type_map: HashMap<String, T::SortOutput>,
    // Rules defined in transformation.
    rules: Vec<Rule<T>>,
}

impl<T> MetaInfo<T> where T: TermStructure 
{
    /// Use type map and rules to create a new MetaInfo instance basically represents Formula Domain.
    pub fn new(type_map: HashMap<String, T::SortOutput>, rules: Vec<Rule<T>>) -> Self {
        MetaInfo { type_map, rules }
    }

    /// Rename all types and rules all with additional prefix to distinguish between different
    /// instances of the same domain.
    pub fn rename(&self, scope: String) -> Self {
        let mut renamed_type_map = HashMap::new();
        for (type_name, formula_type) in self.type_map.iter() {
            let formula_type = formula_type.clone();
            match formula_type.borrow() {
                Type::BaseType(_) => {},
                _ => {
                    let renamed_type: T::SortOutput = formula_type.borrow().rename_type(scope.clone()).into();
                    renamed_type_map.insert(format!("{}", renamed_type), renamed_type);
                }
            }
        }

        // Rules from input or output domains are all skipped.
        let renamed_rules = vec![];

        MetaInfo {
            type_map: renamed_type_map,
            rules: renamed_rules
        }
    }

    /// Add a new rule into meta info.
    pub fn add_rule(&mut self, rule: Rule<T>) {
        self.rules.push(rule);
    }

    /// Return all rules in current module without order.
    pub fn rules(&self) -> Vec<Rule<T>> {
        self.rules.clone()
    }

    /// Return all headless conformance rules without order.
    pub fn conformance_rules(&self) -> Vec<Rule<T>> {
        let mut conformance_rules = vec![];
        for rule in self.rules.clone() {
            if rule.is_conformance_rule() {
                conformance_rules.push(rule);
            }
        }
        conformance_rules
    }

    /// Return stratified rules excluding comformance rules.
    pub fn stratified_rules(&self) -> Vec<Vec<Rule<T>>> {
        // TODO: Rules may contains variable like `%id` that needs to be instantiated in transformation.
        let mut stratified_rules = vec![];
        for rule in self.rules.clone() {
            if !rule.is_conformance_rule() {
                stratified_rules.push(rule);
            }
        }
        vec![stratified_rules]
    }

    /// Return a reference of type map.
    pub fn type_map(&self) -> &HashMap<String, T::SortOutput> {
        &self.type_map
    }

    /// Return a type map that turn generic sort type into `AtomicStrType` by wrapping up basic type.
    pub fn atomic_type_map(&self) -> HashMap<String, AtomicType> {
        let mut atomic_type_map = HashMap::new();
        for (k, v) in self.type_map() {
            let t: &Type = v.borrow();
            atomic_type_map.insert(k.clone(), t.clone().into());
        }
        atomic_type_map
    }

    /// Look up a type in the hash map by its type name.
    pub fn get_type_by_name(&self, name: &String) -> Option<&T::SortOutput> {
        self.type_map.get(name)
    }
}


/// ModelStore efficiently stores all terms indexed in two hash maps, the first one map an unique
/// id to a term and the other one maps alias string to a term, while there is no specific requirement
/// for the type of term that it can be for example `Arc<Term>` in multi-threads scenario or `Rc<Term>` 
/// in single thread scenario or just `Term` if you don't care about too many duplicates as long as it 
/// implements the required traits that make it look like a term.
#[derive(Clone, Debug)]
pub struct ModelStore<T> where T: TermStructure {
    // Map each term to an unique id bi-directionally.
    term_map: BiMap<usize, T>,
    // Map alias string as a variable term to term.
    alias_map: BiMap<T, T>,
    // Use counter to assign id for terms and increment.
    counter: usize,
}

impl<T> ModelStore<T> where T: TermStructure {
    pub fn new(terms: HashSet<T>, alias_map: HashMap<T, T>) -> Self {
        let mut counter = 0;
        let mut term_map = BiMap::new();
        for term in terms.into_iter() {
            term_map.insert(counter, term);
            counter += 1;
        }

        let mut alias_bimap = BiMap::new();
        for (k, v) in alias_map {
            alias_bimap.insert(k, v);
        }

        ModelStore {
            term_map,
            alias_map: alias_bimap,
            counter
        }
    }

    pub fn rename(&self, scope: String, type_map: &HashMap<String, T::SortOutput>) -> Self {
        let mut renamed_alias_map = HashMap::new();
        let mut renamed_terms = HashSet::new();
        let mut type_set = HashSet::new();

        for (_, t) in type_map {
            type_set.insert(t.clone());
        }

        for term_ref in self.terms().into_iter() {
            let mut renamed_term = term_ref.clone();
            renamed_term.rename(scope.clone(), &mut type_set);
            renamed_terms.insert(renamed_term);
        }

        for (key, val) in self.alias_map.clone().into_iter() {
            let new_key = key.rename(scope.clone(), &mut type_set);
            let new_val = val.rename(scope.clone(), &mut type_set);
            renamed_alias_map.insert(new_key, new_val);
        }

        ModelStore::new(renamed_terms, renamed_alias_map)
    }

    // Return alias map.
    pub fn alias_map(&self) -> &BiMap<T, T> {
        &self.alias_map
    }

    // Return all terms as references in a vector.
    pub fn terms(&self) -> Vec<&T> {
        self.term_map.right_values().collect()
    }

    // Check if a term exists with term reference.
    pub fn contains_term(&self, term: &T) -> bool {
        self.term_map.contains_right(term)
    }

    /// Add a new term and return its integer id.
    pub fn add_term(&mut self, term: T) -> usize {
        // Don't insert if the term is found in the map.
        if self.term_map.contains_right(&term) {
            return *self.term_map.get_by_right(&term).unwrap();
        }
        // Increment the counter after insertion.
        let id = self.counter;
        self.term_map.insert(id, term);
        self.counter += 1;
        return id;
    }

    /// Remove a term by looking up with term reference and return a tuple of id and term.
    pub fn remove_term(&mut self, term: &T) -> Option<(usize, T)>{
        if self.term_map.contains_right(term) {
            return self.term_map.remove_by_right(term);
        } else {
            return None;
        }
    }

    /// Remove a term by looking up with id and return a tuple of id and term.
    pub fn remove_term_by_id(&mut self, id: usize) -> Option<(usize, T)> {
        if self.term_map.contains_left(&id) {
            return self.term_map.remove_by_left(&id);
        } else {
            return None;
        }
    }

    /// Return a term reference by looking up its id.
    pub fn get_term_by_id(&self, id: usize) -> Option<&T> {
        self.term_map.get_by_left(&id)
    }

    // Return a term id by looking up its term reference.
    pub fn get_id_by_term(&self, term: &T) -> Option<usize> {
        self.term_map.get_by_right(term).map(|x| *x)
    }

    // Return a term reference by looking up its alias, which is also a term.
    pub fn get_term_by_alias(&self, name: &T) -> &T {
        self.alias_map.get_by_left(name).unwrap()
    }
}

pub trait FormulaModule<T> where T: TermStructure {
    // Return all terms as reference in a vector.
    fn terms(&self) -> Vec<&T>;
    // Return meta inforamtion with type map and rules.
    fn meta_info(&self) -> &MetaInfo<T>;
    // Add a rule.
    fn add_rule(&mut self, rule: Rule<T>);
}

#[derive(Debug, Clone)]
pub struct Transform<T> where T: TermStructure {
    // The name of transform.
    pub name: String,
    // Meta information including the ones from input and output domains.
    pub metainfo: MetaInfo<T>,
    // Store terms defined in the transform module.
    pub store: ModelStore<T>,
    // A list of strings representing the Ids of params.
    pub params: Vec<String>,
    // Some parameters in transform are terms.
    pub input_type_map: HashMap<String, T::SortOutput>,
    // The other parameters in transform are domains. 
    pub input_domain_map: HashMap<String, Domain<T>>,
    // The domains in the output of transform.
    pub output_domain_map: HashMap<String, Domain<T>>,
}

impl<T> Transform<T> where T: TermStructure {
    pub fn new(name: String, 
        type_map: HashMap<String, T::SortOutput>, 
        rules: Vec<Rule<T>>,
        params: Vec<String>, 
        input_type_map: HashMap<String, T::SortOutput>, 
        input_domain_map: HashMap<String, Domain<T>>,
        output_domain_map: HashMap<String, Domain<T>>, 
        terms: HashSet<T>
    ) -> Self {
        let store = ModelStore::new(terms, HashMap::new());
        let metainfo = MetaInfo::new(type_map, rules);
        
        Transform {
            name,
            metainfo,
            store,
            params,
            input_type_map,
            input_domain_map,
            output_domain_map,
        }
    }

    pub fn get_id(&self, position: usize) -> Option<&String> {
        self.params.get(position)
    }
}

impl<T> FormulaModule<T> for Transform<T> where T: TermStructure
{
    fn terms(&self) -> Vec<&T> {
        self.store.terms()
    }

    fn meta_info(&self) -> &MetaInfo<T> {
        &self.metainfo
    }

    fn add_rule(&mut self, rule: Rule<T>) {
        self.metainfo.add_rule(rule);
    }
}

/// `Transformation` is the instantiation of `Transform` with input terms and models.
pub struct Transformation<T> where T: TermStructure {
    // Meta model information.
    pub metainfo: MetaInfo<T>,
    // Term store.
    pub store: ModelStore<T>,
    // The domain of Transformation.
    pub transform: Transform<T>,
    // Map string to term which is an input param.
    pub input_term_map: HashMap<String, T>,
    // Map string to model which is an input param.
    pub input_model_map: HashMap<String, Model<T>>,
    // The terms defined in Transform with alias like `%id` replaced.
    pub inherited_terms: HashSet<T>,
    // The rules defined in Transform with alias like `%id` replaced.
    pub inherited_rules: Vec<Rule<T>>,
}

impl<T> FormulaModule<T> for Transformation<T> where T: TermStructure
{
    fn terms(&self) -> Vec<&T> {
        // Includes all terms in submodels and terms defined in Transform.
        let mut merged_terms = vec![];

        for (id, model) in self.input_model_map.iter() {
            for term_ref in model.terms() {
                merged_terms.push(term_ref);
            }
        }

        for term_ref in self.inherited_terms.iter() {
            merged_terms.push(term_ref);
        }

        merged_terms
    }

    fn meta_info(&self) -> &MetaInfo<T> {
        &self.metainfo
    }
    
    fn add_rule(&mut self, rule: Rule<T>) {
        self.metainfo.add_rule(rule);
    }

    // fn rules(&self) -> Vec<Rule> {
    //     self.inherited_rules.clone()
    // }

    // fn conformance_rules(&self) -> Vec<Rule> {
    //     let mut raw_rules = self.transform.conformance_rules();
    //     for (key, replacement) in self.input_term_map.iter() {
    //         let pattern: Term = Variable::new(key.clone(), vec![]).into();
    //         for rule in raw_rules.iter_mut() {
    //             rule.replace_pattern(&pattern, replacement);
    //         }
    //     }
    //     raw_rules
    // }

    // fn stratified_rules(&self) -> Vec<Vec<Rule>> {
    //     // TODO: Need to replace things like %id.
    //     let mut raw_rules = self.transform.stratified_rules();
    //     for (key, replacement) in self.input_term_map.iter() {
    //         let pattern: Term = Variable::new(key.clone(), vec![]).into();
    //         for rule in raw_rules.iter_mut() {
    //             rule.replace_pattern(&pattern, replacement);
    //         }
    //     }
    //     raw_rules
    // }

    // fn type_map(&self) -> &HashMap<String, Arc<Type>> {
    //     self.transform.type_map()
    // }
}

impl<T> Transformation<T> where T: TermStructure {
    pub fn new(transform: Transform<T>, input_term_map: HashMap<String, T>, input_model_map: HashMap<String, Model<T>>) -> Self {   
        let mut inherited_terms = HashSet::new();
        let mut inherited_rules = Vec::new();
        // let mut renamed_input_model_map = HashMap::new();

        // Some additional terms may be defined in transform even with alias like %id that need to be replaced.
        for (key, replacement) in input_term_map.iter() {
            let pattern = T::create_variable_term(None, key.clone(), vec![]);
            for raw_term in transform.terms() {
                let mut term = raw_term.clone();
                term.replace_pattern(&pattern, replacement);
                inherited_terms.insert(term);
            }
        }

        // Rename all input models.
        for (id, model) in input_model_map.into_iter() {
            // let renamed_model = model.rename(id.clone(), metainfo.type_map());
            // renamed_input_model_map.insert(id, renamed_model);
        }

        // Replace parameter like %id in transform rules with term from `input_term_map`.
        for (key, replacement) in input_term_map.iter() {
            let pattern = T::create_variable_term(None, key.clone(), vec![]);
            for mut rule in transform.meta_info().rules() {
                rule.replace_pattern(&pattern, replacement);
                inherited_rules.push(rule);
            }
        }

        // let transformation = Transformation {
        //     transform,
        //     input_term_map,
        //     input_model_map: renamed_input_model_map,
        //     inherited_terms,
        //     inherited_rules,
        // };

        // transformation

        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct Domain<T> where T: TermStructure {
    pub name: String,
    pub metainfo: MetaInfo<T>,
}

impl<T> FormulaModule<T> for Domain<T> where T: TermStructure {
    fn terms(&self) -> Vec<&T> {
        Vec::new()
    }

    fn meta_info(&self) -> &MetaInfo<T> where T: TermStructure {
        &self.metainfo
    }
    
    fn add_rule(&mut self, rule: Rule<T>) {
        self.metainfo.add_rule(rule);
    }
}

impl<T> Domain<T> where T: TermStructure {
    pub fn new(name: String, type_map: HashMap<String, T::SortOutput>, rules: Vec<Rule<T>>) -> Self {
        let metainfo = MetaInfo::new(type_map, rules);
        Domain { name, metainfo }
    }

    pub fn rename(&self, scope: String) -> Self {
        let mut renamed_type_map = HashMap::new();
        for (type_name, formula_type) in self.metainfo.type_map.iter() {
            let formula_type = formula_type.clone();
            match formula_type.borrow() {
                Type::BaseType(_) => {},
                _ => {
                    let renamed_type: T::SortOutput = formula_type.borrow().rename_type(scope.clone()).into();
                    renamed_type_map.insert(format!("{}", renamed_type), renamed_type);
                }
            }
        }

        let metainfo = MetaInfo::new(renamed_type_map, vec![]);
        let new_name = format!("{}.{}", scope.clone(), self.name);

        Domain {
            name: new_name,
            metainfo,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Model<T> where T: TermStructure {
    // The name of the model.
    pub name: String,
    // Meta info and rules that terms in the model need to conform.
    metainfo: MetaInfo<T>,
    // A model store with generic term type.
    store: ModelStore<T>,
}

impl<T> FormulaModule<T> for Model<T> where T: TermStructure {
    fn terms(&self) -> Vec<&T> {
        self.store.terms()
    }

    fn meta_info(&self) -> &MetaInfo<T> {
        &self.metainfo
    }

    fn add_rule(&mut self, rule: Rule<T>) {
        self.metainfo.add_rule(rule);
    }
}

impl<T> Model<T> where T: TermStructure {
    /// Provide domain, terms and alias mapping to create a new model.
    pub fn new(name: String, domain: &Domain<T>, terms: HashSet<T>, alias_map: HashMap<T, T>) -> Self {
        let metainfo = domain.metainfo.clone();
        let store = ModelStore::new(terms, alias_map);

        Model { 
            name, 
            metainfo, 
            store 
        }
    }

    /// Return a model store reference
    pub fn model_store(&self) -> &ModelStore<T> {
        &self.store
    }

    /// Rename the model by adding scope to each term and type.
    pub fn rename(&self, scope: String, type_map: &HashMap<String, T::SortOutput>) -> Self {
        let renamed_metainfo = self.metainfo.rename(scope.clone());
        let renamed_store = self.store.rename(scope.clone(), type_map);

        Model {
            name: self.name.clone(), 
            metainfo: renamed_metainfo,
            store: renamed_store,
        }
    }

}

#[derive(Debug, Clone)]
pub struct Env<T> where T: TermStructure {
    pub domain_map: HashMap<String, Domain<T>>,
    pub model_map: HashMap<String, Model<T>>,
    pub transform_map: HashMap<String, Transform<T>>,
}

impl<T> Env<T> where T: TermStructure {
    pub fn new() -> Self {
        Env {
            domain_map: HashMap::new(),
            model_map: HashMap::new(),
            transform_map: HashMap::new(),
        }
    }

    pub fn get_model_by_name(&self, name: &str) -> Option<&Model<T>> {
        // Make a clone of the model in Env and return it.
        match self.model_map.get(name) {
            None => None,
            Some(model) => Some(model),
        }
    }

    pub fn get_domain_by_name(&self, name: &str) -> Option<&Domain<T>> {
        // Make a clone of the domain in Env and return it.
        match self.domain_map.get(name) {
            None => None,
            Some(model) => Some(model),
        }
    }

    pub fn get_transform_by_name(&self, name: &str) -> Option<&Transform<T>> {
        match self.transform_map.get(name) {
            None => None,
            Some(transform) => Some(transform)
        }
    }
}