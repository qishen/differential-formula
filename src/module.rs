use std::collections::{HashSet, HashMap};
use std::vec::Vec;
use std::fmt::*;

use bimap::BiMap;
use num::BigInt;
use enum_dispatch::enum_dispatch;
use petgraph::graph::*;
use petgraph::algo::*;

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
    type_map: HashMap<String, RawType>,
    // Rules defined in transformation.
    rules: Vec<Rule<T>>,
}

impl<T> MetaInfo<T> where T: TermStructure {
    /// Use type map and rules to create a new MetaInfo instance basically represents Formula Domain.
    pub fn new(type_map: HashMap<String, RawType>, rules: Vec<Rule<T>>) -> Self {
        MetaInfo { type_map, rules }
    }

    /// Merge two domains into one and ignore the same type definitions and rules from the second domain.
    pub fn merge(&self, other: &Self) -> Self {
        let mut type_map = self.type_map.clone();
        type_map.extend(other.type_map.clone());
        let mut rules = self.rules.clone();
        rules.extend(other.rules.clone());

        MetaInfo {
            type_map,
            rules
        }
    }

    /// Create a new `MetaInfo` with a prefix added to type definition and the sort of every term 
    /// occuring in the rule.
    pub fn rename(&self, scope: String) -> Self {
        let mut renamed_type_map = HashMap::new();
        for (_, formula_type) in self.type_map.iter() {
            let renamed_type: RawType = formula_type.rename_type(scope.clone());
            renamed_type_map.insert(format!("{}", renamed_type), renamed_type);
        }

        // TODO: rename all rules.
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

    /// Return a reference of type map in which the types are not sorted.
    pub fn type_map(&self) -> &HashMap<String, RawType> {
        &self.type_map
    }

    /// Look up a type in the hash map by its type name.
    pub fn get_type_by_name(&self, name: &str) -> Option<&RawType> {
        self.type_map.get(name)
    }

    pub fn composite_types(&self) -> Vec<(&String, &RawType)> {
        let composite_types: Vec<_> = self.type_map().iter().filter(|(_, raw_type)| {
            match raw_type {
                RawType::Type(type_enum) => {
                    match type_enum {
                        FormulaTypeEnum::CompositeType(_) => true,
                        _ => false
                    }
                },
                _ => false
            }
        }).collect();
        composite_types
    }

    /// Return all composite types according to their dependency relationship.
    /// e.g. Node(id: Integer) should appear in the list before Edge(src: Node, dst: Node)
    pub fn sorted_composite_types(&self) -> Vec<&RawType> {
        let mut sorted_types = vec![];
        let mut graph = Graph::new();
        let mut nodes = vec![];
        let mut unsorted_types: Vec<&RawType> = self.type_map.iter().map(|(_, raw_type)| raw_type).collect();
        unsorted_types.sort();
        for raw_type in unsorted_types {
            if let RawType::Type(raw_type_enum) = raw_type {
                match raw_type_enum {
                    FormulaTypeEnum::CompositeType(_) => {
                        let node = graph.add_node(raw_type);
                        nodes.push(node);
                    },
                    _ => {}
                }
            }
        } 

        for n1 in nodes.iter() {
            for n2 in nodes.iter() {
                let ct1 = graph.node_weight(n1.clone()).unwrap().clone();
                let ct2 = graph.node_weight(n2.clone()).unwrap().clone();
                if ct1.is_subtype_of(ct2) {
                    graph.add_edge(n1.clone(), n2.clone(), 1);
                }
            }
        }

        let indexes = toposort(&graph, None).unwrap();
        for index in indexes {
            let &raw_type = graph.node_weight(index).unwrap();
            sorted_types.push(raw_type);
        }

        sorted_types
    }
}

pub trait AccessModel {

    type Output: TermStructure;

    /// Return alias map.
    fn alias_map(&self) -> BiMap<String, &Self::Output>;

    /// Return all terms as references in a vector.
    fn terms(&self) -> Vec<&Self::Output>;

    /// Check if a term exists with term reference.
    fn contains_term(&self, term: &Self::Output) -> bool;

    /// Add a new term and return its integer id.
    fn add_term(&mut self, term: Self::Output) -> &Self::Output;

    /// Remove a term by looking up with term reference and return the removed term if 
    /// the term is removed.
    fn remove_term(&mut self, term: &Self::Output) -> Option<Self::Output>;
}

/// ModelStore efficiently stores all terms indexed in two hash maps, the first one map an unique
/// id to a term and the other one maps alias string to a term, while there is no specific requirement
/// for the type of term that it can be for example `Arc<Term>` in multi-threads scenario or `Rc<Term>` 
/// in single thread scenario or just `Term` if you don't care about too many duplicates as long as it 
/// implements the required traits that make it look like a term.
#[derive(Clone)]
pub struct UUIdTermStore<T> where T: TermStructure {
    // Map each term to an unique integer id bi-directionally and the reference has to live
    // at least as long as the term store even though the reference should only point to the
    // term in the term set above.
    id_bimap: BiMap<BigInt, T>,
    // Map alias string as a variable term reference to term reference and the term reference
    // has to live at least as long as the term store.
    alias_bimap: BiMap<String, T>,
    // Use counter to assign id for new terms and increment each time
    // The number of terms will not hit a limit because BigInt is used here.
    counter: BigInt,
}

impl<T> Debug for UUIdTermStore<T> where T: TermStructure {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let term_str_list: Vec<String> = self.id_bimap.iter().map(|(k, v)| {
            format!("{}: {}", k, v)
        }).collect();

        let alias_str_list: Vec<String> = self.alias_bimap.iter().map(|(k, v)| {
            format!("{} -> {}", k, v)
        }).collect();

        write!(f, "terms: [\n{} \n] \n alias map: [\n{} \n]", 
            term_str_list.join(", \n"),
            alias_str_list.join(", \n")
        )
    }
}

impl<T> AccessModel for UUIdTermStore<T> where T: TermStructure {

    type Output = T;

    fn alias_map(&self) -> BiMap<String, &Self::Output> {
        let map: BiMap<_,_> = self.alias_bimap.iter().map(|(k, v)| {
            (k.clone(), v)
        }).collect();
        map
    }

    fn terms(&self) -> Vec<&Self::Output> {
        let terms: Vec<&T> = self.id_bimap.right_values().collect();
        terms
    }

    fn contains_term(&self, term: &Self::Output) -> bool {
        self.id_bimap.contains_right(term)
    }

    fn add_term(&mut self, term: T) -> &Self::Output {
        if !self.id_bimap.contains_right(&term) {
            let id = self.counter.clone();
            self.id_bimap.insert(id.clone(), term);
            self.counter += 1;
            return self.id_bimap.get_by_left(&id).unwrap();
        } else {
            let tid = self.id_bimap.get_by_right(&term).unwrap();
            return self.id_bimap.get_by_left(&tid).unwrap();
        }
    }

    fn remove_term(&mut self, term: &T) -> Option<Self::Output>{
        self.alias_bimap.remove_by_right(term);
        self.id_bimap.remove_by_right(term).map(|(id, term)| term) 

    }
}

impl<T> UUIdTermStore<T> where T: TermStructure {
    pub fn new(terms: HashSet<T>, alias_map: HashMap<String, T>) -> Self {
        let mut counter: BigInt = 0.into();
        let mut id_bimap = BiMap::new();
        let mut alias_bimap = BiMap::new();

        for term in terms.into_iter() {
            id_bimap.insert(counter.clone(), term);
            counter += 1;
        }

        for (k, v) in alias_map.into_iter() {
            alias_bimap.insert(k, v);
        }

        let mut store = UUIdTermStore {
            id_bimap,
            alias_bimap,
            counter,
        };

        store.replace_alias();
        store
    }

    fn replace_alias_in_term(&self, term: &mut T) {
        term.traverse_mut(
            &|t| { t.is_var() && self.alias_bimap.contains_left(&format!("{}", t)) }, 
            &mut |t| {
                let var_name = format!("{}", t);
                let mut replacement = self.alias_bimap.get_by_left(&var_name).unwrap().clone();
                // Recursively replace all variables in both `term` and the replacement because
                // The replacement in raw alias map may still have variables inside.
                self.replace_alias_in_term(&mut replacement);
                *t = replacement;
            }
        );
    }

    pub fn replace_alias(&mut self) {
        let mut new_id_bimap = BiMap::new();
        let mut new_alias_bimap = BiMap::new();

        for (id, tref) in self.id_bimap.iter() {
            let mut term = tref.clone(); 
            self.replace_alias_in_term(&mut term);
            new_id_bimap.insert(id.clone(), term);
        }

        for (alias, tref) in self.alias_bimap.iter() {
            let mut term = tref.clone();
            self.replace_alias_in_term(&mut term);
            new_alias_bimap.insert(alias.clone(), term);
        }

        self.id_bimap = new_id_bimap;
        self.alias_bimap = new_alias_bimap;
    }

    pub fn rename(&self, scope: String, type_map: &HashMap<String, RawType>) -> Self {
        let mut renamed_alias_map = HashMap::new();
        let mut renamed_terms = HashSet::new();
        let mut type_set = HashSet::new();

        for (_, t) in type_map {
            type_set.insert(t.clone());
        }

        for term_ref in self.terms().into_iter() {
            let renamed_term = term_ref.clone();
            renamed_term.rename(scope.clone());
            renamed_terms.insert(renamed_term);
        }

        for (key, val) in self.alias_bimap.clone().into_iter() {
            let v = val.rename(scope.clone());
            renamed_alias_map.insert(format!("{}.{}", scope, key), v);
        }

        UUIdTermStore::new(renamed_terms, renamed_alias_map)
    }

    pub fn get_term_by_id(&self, id: BigInt) -> Option<&T> {
        self.id_bimap.get_by_left(&id)
    }

    pub fn get_term_by_alias(&self, name: &String) -> &T {
        self.alias_bimap.get_by_left(name).unwrap()
    }

    pub fn get_id_by_term(&self, term: &T) -> Option<BigInt> {
        self.id_bimap.get_by_right(&term).map(|x| x.clone())
    }

    pub fn remove_term_by_id(&mut self, id: BigInt) -> Option<(BigInt, T)> {
        self.id_bimap.remove_by_left(&id)
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
    pub store: UUIdTermStore<T>,
    // A list of strings representing the Ids of params.
    pub params: Vec<String>,
    // Some parameters in transform are terms.
    pub input_type_map: HashMap<String, RawType>,
    // The other parameters in transform are domains. 
    pub input_domain_map: HashMap<String, Domain<T>>,
    // The domains in the output of transform.
    pub output_domain_map: HashMap<String, Domain<T>>,
}

impl<T> Transform<T> where T: TermStructure {
    pub fn new(name: String, 
        type_map: HashMap<String, RawType>, 
        rules: Vec<Rule<T>>,
        params: Vec<String>, 
        input_type_map: HashMap<String, RawType>, 
        input_domain_map: HashMap<String, Domain<T>>,
        output_domain_map: HashMap<String, Domain<T>>, 
        terms: HashSet<T>
    ) -> Self {
        let store = UUIdTermStore::new(terms, HashMap::new());
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
    pub store: UUIdTermStore<T>,
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

impl<'a, T> FormulaModule<T> for Transformation<T> where T: TermStructure
{
    fn terms(&self) -> Vec<&T> {
        // Includes all terms in submodels and terms defined in Transform.
        let mut merged_terms = vec![];

        for (_id, model) in self.input_model_map.iter() {
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
            let pattern = T::gen_raw_variable_term(key.clone(), vec![]);
            for raw_term in transform.terms() {
                let mut term = raw_term.clone();
                // FIXME
                // term.replace_pattern(&pattern, replacement);
                inherited_terms.insert(term);
            }
        }

        // Rename all input models.
        //for (id, model) in input_model_map.into_iter() {
            // let renamed_model = model.rename(id.clone(), metainfo.type_map());
            // renamed_input_model_map.insert(id, renamed_model);
        //}

        // Replace parameter like %id in transform rules with term from `input_term_map`.
        for (key, replacement) in input_term_map.iter() {
            let pattern = T::gen_raw_variable_term(key.clone(), vec![]);
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
    pub fn new(name: String, type_map: HashMap<String, RawType>, rules: Vec<Rule<T>>) -> Self {
        let metainfo = MetaInfo::new(type_map, rules);
        Domain { name, metainfo }
    }

    pub fn rename(&self, scope: String) -> Self {
        let mut renamed_type_map = HashMap::new();
        for (_type_name, formula_type) in self.metainfo.type_map.iter() {
            let formula_type = formula_type.clone();
            let renamed_type = formula_type.rename_type(scope.clone());
            renamed_type_map.insert(format!("{}", renamed_type), renamed_type);
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
    pub metainfo: MetaInfo<T>,
    // A model store with generic term type.
    pub store: UUIdTermStore<T>,
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
    pub fn new(name: String, metainfo: MetaInfo<T>, store: UUIdTermStore<T>) -> Self {
        Model {
            name,
            metainfo,
            store
        }
    }

    /// Return a model store reference
    pub fn model_store(&self) -> &UUIdTermStore<T> {
        &self.store
    }

    /// Rename the model by adding scope to each term and type.
    pub fn rename(&self, scope: String, type_map: &HashMap<String, RawType>) -> Self {
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