#![feature(in_band_lifetimes)]
extern crate timely;
extern crate differential_dataflow;

use timely::dataflow::operators::{ToStream, Accumulate, Capture};
use timely::dataflow::operators::capture::Extract;
use timely::Configuration;

use differential_dataflow::input::{Input, InputSession};
use differential_dataflow::operators::join::{Join, JoinCore};
use differential_dataflow::operators::{Iterate, Threshold, Consolidate, Count, CountTotal};

use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, PyObjectProtocol, PyTypeInfo, PySequenceProtocol, PyNativeType, exceptions};
use pyo3::types::{PyInt, PyList, PyAny, PyDict, PyIterator, PyTuple, PyString, PyBool, PyFloat};

use enum_dispatch::enum_dispatch;

use indexmap::IndexSet;
use rayon::prelude::*;
use std::{fs, fmt};
use std::path::PathBuf;
use std::convert::TryInto;
use std::borrow::{Borrow, BorrowMut};
use std::string::String;
use std::iter::*;
use std::collections::{HashMap, HashSet};
use std::any::Any;
use pyo3::derive_utils::IntoPyResult;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use pyo3::class::basic::CompareOp;
use std::io::Error;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter, Display};
use std::sync::Arc;
use std::ops::Deref;
use std::cell::RefCell;
use std::rc::Rc;
use differential_dataflow::{ExchangeData, Collection};
use abomonation::Abomonation;


#[pyclass]
#[derive(Clone)]
struct BasicType {
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    labels: Option<Vec<String>>,
    #[pyo3(get, set)]
    types: Vec<String>,
    #[pyo3(get, set)]
    refs: Vec<Option<String>>
}

#[pymethods]
impl BasicType {
    #[new]
    fn new(obj: &PyRawObject, name: String, labels: Option<Vec<String>>, types: Vec<String>, refs: Vec<Option<String>>) {
        obj.init(BasicType {
            name,
            labels,
            types,
            refs
        });
    }
}

#[pyclass]
struct BuiltInType {
    name: String
}

#[pymethods]
impl BuiltInType {
    #[new]
    fn new(obj: &PyRawObject, name: String) {
        obj.init(BuiltInType {
            name
        })
    }

    #[staticmethod]
    fn get_types() -> PyResult<HashMap<String, BuiltInType>> {
        let mut map = HashMap::new();
        map.insert("Boolean".to_string(), BuiltInType {name: String::from("Boolean")});
        map.insert("Integer".to_string(), BuiltInType {name: String::from("Integer")});
        map.insert("String".to_string(), BuiltInType {name: String::from("String")});
        Ok(map)
    }
}

#[enum_dispatch]
enum FormulaType {
    BasicType,
    BuiltInType
}


#[enum_dispatch]
trait TermMethods {
    fn variables_native(&self) -> HashSet<Variable>;
    fn get_bindings_native(&self, term: &Term) -> HashMap<Variable, Term>;
    fn is_groundterm_native(&self) -> bool;
    fn propagate_bindings_native(&self, map: &HashMap<Variable, Term>) -> Term;
}

#[enum_dispatch(TermMethods)]
#[derive(Clone, Hash, PartialEq, PartialOrd, Ord, Eq)]
enum Term {
    Composite,
    Variable,
    Atom
}

// Add this trait implementation to make Term accepted by differential dataflow.
impl Abomonation for Term {}

//impl ExchangeData for Term {}

impl Debug for Term {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Term::Composite(c) => write!(f, "{:?}", c),
            Term::Variable(v) => write!(f, "{:?}", v),
            Term::Atom(a) => write!(f, "{:?}", a)
        }
    }
}


#[pyclass]
struct Composite {
    sort: BasicType,
    arguments: Vec<Arc<Term>>,
    alias: Option<String>,
}

impl Hash for Composite {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sort.name.hash(state);
        for arg in self.arguments.iter() {
            arg.as_ref().hash(state);
        }
    }
}

impl Debug for Composite {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let type_name: String = self.sort.name.to_string();
        let mut arg_strs: Vec<String> = vec![];
        for boxed_arg in self.arguments.iter() {
            let arg = boxed_arg.as_ref();
            arg_strs.push(format!("{:?}", arg))
        }
        let arg_str = arg_strs.join(",");
        write!(f, "{}({})", type_name, arg_str)
    }
}

impl PartialOrd for Composite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.sort.name.cmp(&other.sort.name))
    }
}

impl Ord for Composite {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort.name.cmp(&other.sort.name)
    }
}

impl PartialEq for Composite {
    fn eq(&self, other: &Self) -> bool {
        if self.sort.name != other.sort.name {
            return false;
        }

        for (i, item) in self.arguments.iter().enumerate() {
            let arg1 = item.as_ref();
            let arg2 = other.arguments.get(i).unwrap().as_ref();
            if *arg1 != *arg2 {
                return false;
            }
        }

        true
    }
}

impl Eq for Composite {}

impl Clone for Composite {
    fn clone(&self) -> Self {
        let mut copied_args = vec![];
        for arg in &self.arguments {
            copied_args.push(Arc::clone(arg));
        }

        Composite {
            sort: self.sort.clone(),
            arguments: copied_args,
            alias: self.alias.clone()
        }
    }
}


impl Composite {
    fn get_bindings_helper(&self, groundterm: &Term, bindings: &mut HashMap<Variable, Term>) -> bool {
        match groundterm {
            Term::Composite(cterm) => {
                if self.sort.name != cterm.sort.name {
                    return false;
                }

                for (i, arg) in self.arguments.iter().enumerate() {
                    let self_native_arg = arg.as_ref();
                    let groundterm_native_arg = cterm.arguments.get(i).unwrap().as_ref();
                    match self_native_arg {
                        Term::Composite(self_arg_composite) => {
                            self_arg_composite.get_bindings_helper(groundterm_native_arg, bindings);
                        },
                        Term::Variable(self_arg_var) => {
                            bindings.insert(self_arg_var.clone(), groundterm_native_arg.clone());
                        },
                        _ => {
                            if self_native_arg != groundterm_native_arg {
                                return false;
                            }
                        }
                    }
                }
            },
            other => {
                return false;
            }
        }

        true
    }

    fn replace_variables_in_place_helper(term: &mut Composite, alias_to_term_map: &HashMap<String, Term>) {
        for mut arg in term.arguments.iter_mut() {
            let arg_term = arg.as_ref();
            match arg_term {
                Term::Composite(c) => {
                    let mut new_arg: Composite = c.clone();
                    Composite::replace_variables_in_place_helper(&mut new_arg, alias_to_term_map);
                    arg = &mut Arc::new(new_arg.into());
                },
                Term::Variable(v) => {
                    if alias_to_term_map.contains_key(&v.var) {
                        arg = &mut Arc::new(alias_to_term_map.get(&v.var).unwrap().clone());
                    }
                },
                _ => {}
            }
        }
    }
}


impl TermMethods for Composite {
    fn variables_native(&self) -> HashSet<Variable> {
        let mut vars: HashSet<Variable> = HashSet::new();
        for arg in self.arguments.iter() {
            let term = arg.as_ref();
            match term {
                Term::Composite(c) => vars.union(&c.variables_native()),
                Term::Variable(v) => vars.union(&v.variables_native()),
                Term::Atom(a) => vars.union(&a.variables_native())
            };
        }
        vars
    }

    fn get_bindings_native(&self, term: &Term) -> HashMap<Variable, Term> {
        let mut map = HashMap::new();
        self.get_bindings_helper(term, &mut map);
        map
    }

    fn is_groundterm_native(&self) -> bool {
        for boxed_arg in self.arguments.iter() {
            let arg = boxed_arg.as_ref();
            if !arg.is_groundterm_native() {
                return false
            }
        }
        true
    }

    fn propagate_bindings_native(&self, map: &HashMap<Variable, Term>) -> Term {
        let mut new_args: Vec<Arc<Term>> = vec![];
        for arg in self.arguments.iter() {
            // Make clone for each argument in composite term
            match arg.as_ref() {
                Term::Composite(cterm) => {
                    let new_term = cterm.propagate_bindings_native(map);
                    new_args.push(Arc::new(new_term));
                },
                other => {
                    new_args.push(Arc::new(other.clone()))
                }
            }
        }

        let new_composite = Composite {
            sort: self.sort.clone(),
            arguments: new_args,
            alias: self.alias.clone()
        };

        new_composite.into()
    }
}

#[pymethods]
impl Composite {
    #[new]
    fn new(obj: &PyRawObject, relation: PyObject, args: &PyList) {
        let mut native_arguments: Vec<Arc<Term>> = Vec::new();
        for arg in args.iter() {
            if let Ok(var) = arg.cast_as::<Variable>() {
                let boxed_var = Arc::new(var.clone().into());
                native_arguments.push(boxed_var);
            } else if let Ok(atom) = arg.cast_as::<Atom>() {
                let boxed_atom = Arc::new(atom.clone().into());
                native_arguments.push(boxed_atom);
            } else if let Ok(composite) = arg.cast_as::<Composite>() {
                let boxed_composite = Arc::new(composite.clone().into());
                native_arguments.push(boxed_composite);
            } else {
                let msg = format!("Expect Variable, Atom or Composite but found {}", arg.get_type().name());
            }
        }

        let gil = Python::acquire_gil();
        let py = gil.python();
        let basic_type: String = relation.getattr(py, "name").unwrap().extract(py).unwrap();

        obj.init(Composite {
            sort: relation.cast_as::<BasicType>(py).unwrap().clone(),
            arguments: native_arguments,
            alias: None
        });
    }

    #[getter]
    fn get_sort(&self, py: Python) -> PyResult<BasicType> {
        Ok(self.sort.clone())
    }

    #[getter]
    fn get_alias(&self, py: Python) -> PyResult<PyObject> {
        Ok(self.alias.to_object(py))
    }

    #[setter]
    fn set_alias(&mut self, value: Option<&str>) -> PyResult<()> {
        self.alias = Some(value.unwrap().to_string());
        Ok(())
    }

    fn get_variables(&self) -> PyResult<Vec<Variable>> {
        let native_vars: HashSet<Variable> = self.variables_native();
        let mut vars = vec![];
        for var in native_vars {
            vars.push(var);
        }
        Ok(vars)
    }

    fn is_ground_term(&self) -> PyResult<bool> {
        Ok(self.is_groundterm_native())
    }

    fn get_bindings(&self, ground_term: &PyAny) -> PyResult<HashMap<Variable, PyObject>> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut map: HashMap<Variable, PyObject> = HashMap::new();
        let native_ground_term: Term;
        if let Ok(composite) = ground_term.cast_as::<Composite>() {
            native_ground_term = composite.clone().into();
        } else if let Ok(atom) = ground_term.cast_as::<Atom>() {
            native_ground_term = atom.clone().into();
        } else {
            return Err(exceptions::AssertionError::py_err("Ground term cannot be a variable"));
        }

        let native_map: HashMap<Variable, Term> = self.get_bindings_native(&native_ground_term);
        Ok(map)
    }

    fn propagate_bindings(&self, bindings: &PyAny) -> PyResult<PyObject> {
        let mut map: HashMap<Variable, Term> = HashMap::new();
        let gil = Python::acquire_gil();
        let py = gil.python();
        if let Ok(dict) = bindings.cast_as::<PyDict>() {
            for key in dict.keys() {
                let value = dict.get_item(key).unwrap();
                if let Ok(var) = key.cast_as::<Variable>() {
                    if let Ok(value_composite) = value.cast_as::<Composite>() {
                        map.insert(var.clone(), value_composite.clone().into());
                    } else if let Ok(value_variable) = value.cast_as::<Variable>() {
                        map.insert(var.clone(), value_variable.clone().into());
                    } else if let Ok(value_atom) = value.cast_as::<Atom>() {
                        map.insert(var.clone(), value_atom.clone().into());
                    }
                } else {
                    return Err(exceptions::AssertionError::py_err("Key is not a variable"))
                }
            }
        }

        let new_term = self.propagate_bindings_native(&map);
        match new_term {
            Term::Composite(c) => return Ok(c.into_py(py)),
            Term::Variable(v) => return Ok(v.into_py(py)),
            Term::Atom(a) => return Ok(a.into_py(py))
        }
    }

    fn replace_variables_in_place(&mut self, map: &PyDict) -> PyResult<()> {
        let mut native_map: HashMap<String, Term> = HashMap::new();
        for (k, v) in map.iter() {
            let key = k.extract::<String>().unwrap().clone();
            if let Ok(composite) = v.cast_as::<Composite>() {
                native_map.insert(key, composite.clone().into());
            } else if let Ok(atom) = v.cast_as::<Atom>() {
                native_map.insert(key, atom.clone().into());
            }
        }
        Composite::replace_variables_in_place_helper(self, &native_map);
        Ok(())
    }
}

#[pyproto]
impl PyObjectProtocol for Composite {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        Ok(s.finish() as isize)
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<bool> {
        if let Ok(composite) = other.cast_as::<Composite>() {
            match op {
                CompareOp::Eq => {
                    if self == composite { return Ok(true) }
                    else { return Ok(false) }
                },
                CompareOp::Ne => {
                    if self == composite { return Ok(false) }
                    else { return Ok(true) }
                },
                _ => Err(exceptions::NotImplementedError::py_err("Operator is not supported"))
            }
        } else {
            Ok(false)
        }
    }
}

#[pyproto]
impl PySequenceProtocol for Composite {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.arguments.len())
    }
}




#[pyclass]
#[derive(Clone, PartialOrd, Ord)]
struct Variable {
    var: String,
    fragments: Vec<String>,
}

impl TermMethods for Variable {
    fn variables_native(&self) -> HashSet<Variable> {
        let mut set = HashSet::new();
        set.insert(self.clone());
        set
    }

    fn get_bindings_native(&self, term: &Term) -> HashMap<Variable, Term> {
        let mut map = HashMap::new();
        map.insert(self.clone(), term.clone());
        map
    }

    fn is_groundterm_native(&self) -> bool {
        false
    }

    fn propagate_bindings_native(&self, map: &HashMap<Variable, Term>) -> Term {
        if map.contains_key(self) {
            let value = map.get(self).unwrap().clone();
            return value.clone();
        } else {
            return self.clone().into();
        }
    }
}

impl Debug for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "<{}>", self.var)
    }
}

impl Hash for Variable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.var.hash(state);
        self.fragments.hash(state);
    }
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        if self.var == other.var { return true; }
        else { return false; }
    }
}

impl Eq for Variable {}


#[pymethods]
impl Variable {
    #[new]
    fn new(obj: &PyRawObject, var_str: String, var_sort: PyObject) {
        let mut frags: Vec<String> = var_str.split('.').map(|x| x.to_string()).collect();
        let first = frags.remove(0).to_string();
        obj.init(Variable {
            var: first,
            fragments: frags
        });
    }

    #[getter]
    fn get_var(&self) -> PyResult<String> {
        Ok(self.var.clone())
    }

    #[getter]
    fn get_fragments(&self) -> PyResult<Vec<String>> {
        Ok(self.fragments.clone())
    }

    fn get_variables(&self) -> PyResult<Vec<Variable>> {
        Ok(vec![self.clone()])
    }

    fn is_ground_term(&self) -> PyResult<bool> {
        Ok(self.is_groundterm_native())
    }

    /*fn get_bindings(&self, ground_term: &Term) -> PyResult<HashMap<Variable, Term>> {
        let native_map: HashMap<Variable, Term> = self.get_bindings_native(ground_term);
        Ok(map)
    }*/

    fn get_bindings(&self, ground_term: &PyAny) -> PyResult<HashMap<Variable, PyObject>> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut map: HashMap<Variable, PyObject> = HashMap::new();
        let native_ground_term: Term;
        if let Ok(composite) = ground_term.cast_as::<Composite>() {
            native_ground_term = composite.clone().into();
        } else if let Ok(atom) = ground_term.cast_as::<Atom>() {
            native_ground_term = atom.clone().into();
        } else {
            return Err(exceptions::AssertionError::py_err("Ground term cannot be a variable"));
        }

        let native_map: HashMap<Variable, Term> = self.get_bindings_native(&native_ground_term);
        Ok(map)
    }

    fn propagate_bindings(&self, bindings: &PyAny) -> PyResult<PyObject> {
        let mut map: HashMap<Variable, Term> = HashMap::new();
        let gil = Python::acquire_gil();
        let py = gil.python();
        if let Ok(dict) = bindings.cast_as::<PyDict>() {
            for key in dict.keys() {
                let value = dict.get_item(key).unwrap();
                if let Ok(var) = key.cast_as::<Variable>() {
                    if let Ok(value_composite) = value.cast_as::<Composite>() {
                        map.insert(var.clone(), value_composite.clone().into());
                    } else if let Ok(value_variable) = value.cast_as::<Variable>() {
                        map.insert(var.clone(), value_variable.clone().into());
                    } else if let Ok(value_atom) = value.cast_as::<Atom>() {
                        map.insert(var.clone(), value_atom.clone().into());
                    }
                } else {
                    return Err(exceptions::AssertionError::py_err("Key is not a variable"))
                }
            }
        }

        let new_term = self.propagate_bindings_native(&map);
        match new_term {
            Term::Composite(c) => return Ok(c.into_py(py)),
            Term::Variable(v) => return Ok(v.into_py(py)),
            Term::Atom(a) => return Ok(a.into_py(py))
        }
    }
}

#[pyproto]
impl PyObjectProtocol for Variable {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        Ok(s.finish() as isize)
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<bool> {
        if let Ok(variable) = other.cast_as::<Variable>() {
            match op {
                CompareOp::Eq => {
                    if self == variable {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                },
                CompareOp::Ne => {
                    if self == variable {
                        Ok(false)
                    } else {
                        Ok(true)
                    }
                },
                _ => Err(exceptions::NotImplementedError::py_err("Operator is not supported"))
            }
        } else {
            Ok(false)
        }
    }
}




#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
enum AtomValue {
    String(String),
    Integer(usize),
    Boolean(bool),
}


#[pyclass]
#[derive(Clone, PartialOrd, Eq, Ord)]
struct Atom {
    value: AtomValue,
}

impl TermMethods for Atom {
    fn variables_native(&self) -> HashSet<Variable> {
        HashSet::new()
    }

    fn get_bindings_native(&self, term: &Term) -> HashMap<Variable, Term> {
        HashMap::new()
    }

    fn is_groundterm_native(&self) -> bool {
        true
    }

    fn propagate_bindings_native(&self, map: &HashMap<Variable, Term>) -> Term {
        self.clone().into()
    }
}

impl Hash for Atom {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.value {
            AtomValue::String(a) => a.hash(state),
            AtomValue::Boolean(b) => b.hash(state),
            AtomValue::Integer(d) => d.hash(state),
        };
    }
}

impl PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        if self.value == other.value {
            return true;
        } else {
            return false;
        }
    }
}

impl Debug for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let result = match &self.value {
            AtomValue::String(a) => format!("{:?}", a),
            AtomValue::Boolean(b) => format!("{:?}", b),
            AtomValue::Integer(d) => format!("{:?}", d),
        };
        write!(f, "{}", result)
    }
}

#[pymethods]
impl Atom {
    #[new]
    fn new(obj: &PyRawObject, value: &PyAny, atom_sort: Option<PyObject>) {
        let gil = Python::acquire_gil();
        let py = gil.python();
        //let relation = py.import("executer.relation").unwrap();
        //let BuiltInType = relation.get("BuiltInType").unwrap();
        let mut atom_value: AtomValue = AtomValue::String("HELLO".to_string());
        if let Ok(v) = value.cast_as::<PyString>() {
            atom_value = AtomValue::String(v.to_string_lossy().to_string());
        } else if let Ok(v) = value.cast_as::<PyBool>() {
            atom_value = AtomValue::Boolean(v.is_true());
        } else if let Ok(v) = value.cast_as::<PyInt>() {
            atom_value = AtomValue::Integer(v.extract().unwrap())
        } else {}

        obj.init(Atom {
            value: atom_value
        })
    }

    fn get_variables(&self) -> PyResult<Vec<Variable>> {
        Ok(vec![])
    }

    fn is_ground_term(&self) -> PyResult<bool> {
        Ok(self.is_groundterm_native())
    }

    fn get_bindings(&self, ground_term: &PyAny) -> PyResult<HashMap<Variable, PyObject>> {
        Err(exceptions::NotImplementedError::py_err("An Atom term does not support bindings checking with other terms"))
    }

    fn propagate_bindings(&self, bindings: &PyAny) -> PyResult<PyObject> {
        Err(exceptions::NotImplementedError::py_err("Cannot propagate bindings to an atom term"))
    }
}

#[pyproto]
impl PyObjectProtocol for Atom {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        Ok(s.finish() as isize)
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<bool> {
        if let Ok(atom) = other.cast_as::<Atom>() {
            match op {
                CompareOp::Eq => {
                    if self == atom {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                },
                CompareOp::Ne => {
                    if self == atom {
                        Ok(false)
                    } else {
                        Ok(true)
                    }
                },
                _ => Err(exceptions::NotImplementedError::py_err("Operator is not supported"))
            }
        }
        else {
            Ok(false)
        }
    }
}


struct TermIndex {

}

impl TermIndex {

}


#[pyclass(module = "ddengine")]
struct DDExecuter {
    model: PyObject,
    facts: Vec<Composite>,
}

#[pymethods]
impl DDExecuter {
    #[new]
    fn new(obj: &PyRawObject, model: PyObject) {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let fact_list_obj = model.getattr(py, "initial_facts").unwrap();
        let fact_list = fact_list_obj.cast_as::<PyList>(py).unwrap();
        let mut initial_facts: Vec<Composite> = Vec::new();
        for fact in fact_list {
            if let Ok(term) = fact.cast_as::<Composite>() {
                initial_facts.push(term.clone());
            }
        }

        obj.init(DDExecuter {
            model: model,
            facts: initial_facts
        });
    }

    #[getter]
    fn get_facts(&self) -> PyResult<Vec<Composite>> {
        Ok(self.facts.clone())
    }

    fn add_changes(&self, changes: &PyDict) -> PyResult<()> {
        Ok(())
    }

    fn add_rule(&self, rule: &'static PyAny) -> PyResult<()> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let head = rule.getattr("head").unwrap();
        let disjunctions_native = rule.getattr("body").unwrap();
        let disjunctions = disjunctions_native.cast_as::<PyList>().unwrap();
        let body = disjunctions.get_item(0).cast_as::<PyList>().unwrap();

        let mut term_list = Vec::new();
        for pred in body.iter() {
            let term = pred.getattr("term").unwrap();
            let constraint = term.cast_as::<Composite>().unwrap();
            term_list.push(constraint);
        }

        timely::execute_directly(move |worker| {
            let mut input = InputSession::new();

            worker.dataflow(|scope| {
                let models = input.to_collection(scope);
                let mut body_term_iterator = term_list.into_iter();
                let cur = body_term_iterator.next().unwrap();
                let cur_variables_rest: Arc<RefCell<IndexSet<Variable>>> = Arc::new(RefCell::new(IndexSet::new()));
                for var in cur.variables_native() {
                    cur_variables_rest.as_ref().borrow_mut().insert(var);
                }
                let mut cur_collection = models.filter(move |model: &Composite| {
                    cur.sort.name == model.sort.name
                }).map(move |model| {
                    let bindings: HashMap<Variable, Term> = cur.get_bindings_native(&model.into());
                    let mut collection = vec![];
                    for (key, value) in bindings {
                        collection.push(value);
                    }
                    collection
                });

                //let mut cur_models = None;
                for next in body_term_iterator {
                    let cur_variables_rest_clone = cur_variables_rest.clone();
                    let mut next_variables_rest: Arc<RefCell<HashSet<Variable>>> = Arc::new(RefCell::new(next.variables_native()));
                    let next_variables_rest_clone = next_variables_rest.clone();

                    // Find intersection and differences between two hash set.
                    let mut same_variables: Arc<RefCell<HashSet<Variable>>> = Arc::new(RefCell::new(HashSet::new()));
                    // Variable has to be cloned because it is moved into closure.
                    let same_variables_clone = same_variables.clone();
                    let same_variables_clone2 = same_variables.clone();

                    for var in cur_variables_rest_clone.as_ref().borrow().iter() {
                        if next_variables_rest.as_ref().borrow().contains(var) {
                            next_variables_rest.as_ref().borrow_mut().take(var);
                            let removed = cur_variables_rest_clone.as_ref().borrow_mut().take(var).unwrap();
                            same_variables.as_ref().borrow_mut().insert(removed);
                        }
                    }

                    if same_variables.as_ref().borrow().len() == 0 {
                        // A production rule so far with no variable joins needed.
                        let next_collection = models.filter(move |model| {
                           next.sort.name == model.sort.name
                        }).map(move |model| {
                            let bindings: HashMap<Variable, Term> = cur.get_bindings_native(&model.into());
                            let mut collection = vec![];
                            for (key, value) in bindings {
                                collection.push(value);
                            }
                            collection
                        });

                        cur_collection = cur_collection.concat(&next_collection);

                        // Add variables from new term into existing variable hash set.
                        for var in next_variables_rest.as_ref().borrow().iter() {
                            cur_variables_rest_clone.as_ref().borrow_mut().insert(var.clone());
                        }

                    } else {
                        // There are overlap in variables in two terms.
                        let cur_split_collection = cur_collection.map(move |terms| {
                            let mut intersection = vec![];
                            let mut others = vec![];
                            for i in 0..terms.len() {
                                // TODO: Avoid clone function for deep copy
                                let var = cur_variables_rest_clone.as_ref().borrow().get_index(i).unwrap().clone();
                                if same_variables.as_ref().borrow().contains(&var) {
                                    intersection.push(terms.get(i).unwrap().clone());
                                } else {
                                    others.push(terms.get(i).unwrap().clone());
                                }
                            }
                            (intersection, others)
                        });

                        let next_split_collection = models.filter(move |model: &Composite| {
                            next.sort.name == model.sort.name
                        }).map(move |model| {
                            let bindings: HashMap<Variable, Term> = next.get_bindings_native(&model.into());
                            let mut intersection = vec![];
                            let mut others = vec![];

                            for var in same_variables_clone.as_ref().borrow().iter() {
                                if bindings.contains_key(var) {
                                    intersection.push(bindings.get(var).unwrap().clone());
                                }
                            }

                            for var in next_variables_rest.as_ref().borrow().iter() {
                                others.push(bindings.get(var).unwrap().clone());
                            }

                            (intersection, others)
                        });

                        cur_collection = cur_split_collection.join(&next_split_collection).map(|(a, (b, c))| {
                            let mut result = vec![];
                            result.extend(a);
                            result.extend(b);
                            result.extend(c);
                            result
                        });

                        // Collect all variables without duplicates.
                        let mut new_cur_variables = IndexSet::new();
                        for var in same_variables_clone2.as_ref().borrow().iter() {
                            new_cur_variables.insert(var.clone());
                        }

                        for var in cur_variables_rest.as_ref().borrow().iter() {
                            new_cur_variables.insert(var.clone());
                        }

                        for var in next_variables_rest_clone.as_ref().borrow().iter() {
                            new_cur_variables.insert(var.clone());
                        }

                        cur_variables_rest.as_ref().borrow_mut().clear();
                        cur_variables_rest.as_ref().borrow_mut().extend(new_cur_variables);
                    }
                }

            });

            input.advance_to(0);
            let v1 = Variable {
                var: String::from("a"),
                fragments: vec![]
            };

            let a1 = Atom {
                value: AtomValue::Integer(1)
            };

            let term = Composite {
                sort: BasicType {
                    name: "".to_string(),
                    labels: None,
                    types: vec![],
                    refs: vec![None]
                },
                arguments: vec![Arc::new(Term::from(a1)), Arc::new(Term::from(v1))],
                alias: None
            };

            input.insert(term);

        });

        Ok(())
    }
}

#[pymodule(ddengine)]
fn ddengine(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<BasicType>()?;
    m.add_class::<BuiltInType>()?;
    m.add_class::<Atom>()?;
    m.add_class::<Variable>()?;
    m.add_class::<Composite>()?;
    m.add_class::<DDExecuter>()?;

    Ok(())
}