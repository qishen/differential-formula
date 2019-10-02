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

use rayon::prelude::*;
use std::rc::Rc;
use std::fs;
use std::path::PathBuf;
use std::convert::TryInto;
use std::borrow::Borrow;
use std::string::String;
use std::iter::*;
use std::collections::HashMap;
use std::any::Any;
use pyo3::derive_utils::IntoPyResult;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use pyo3::class::basic::CompareOp;
use std::io::Error;


trait Term {
    fn get_variables(&self) -> PyResult<Vec<&Variable>>;
    //fn get_bindings() -> PyResult<()>;
}


#[derive(Clone)]
enum TermObject {
    Atom(Atom),
    Variable(Variable),
    Composite(Composite),
}

impl PartialEq for TermObject {
    fn eq(&self, other: &Self) -> bool {
        match self {
            TermObject::Atom(a) => {
                match other {
                    TermObject::Atom(a1) => {
                        if a == a1 { return true; }
                        else { return false; }
                    },
                    _ => return false
                }
            },
            TermObject::Variable(v) => {
                match other {
                    TermObject::Variable(v1) => {
                        if v == v1 { return true; }
                        else { return false; }
                    },
                    _ => return false
                }
            },
            TermObject::Composite(c) => {
                match other {
                    TermObject::Composite(c1) => {

                        if c == c1 { return true; }
                        else { return false; }
                    },
                    _ => return false
                }
            },
            _ => return false
        }
    }
}


#[pyclass]
#[derive(Clone)]
struct Composite {
    type_name: String,
    sort: Rc<PyObject>,
    args: Vec<Box<TermObject>>,
    alias: Option<String>,
}

impl Composite {
    // term can be either variable or composite that has variables, ground term can be either atom or composite.
    fn bind_helper(term: &TermObject, ground_term: &TermObject, bindings: &mut HashMap<Variable, TermObject>) -> bool {
        match term {
            TermObject::Composite(c) => {
                match ground_term {
                    TermObject::Composite(g) => {
                        if c.type_name != g.type_name {
                            return false;
                        }
                        // Check each argument in composite term and immediately return false when mismatch is found.
                        for i in 0..c.args.len() {
                            let arg_term = c.args.get(i).unwrap().as_ref().clone();
                            let arg_ground_term = g.args.get(i).unwrap().as_ref().clone();
                            let equals = Composite::bind_helper(&arg_term, &arg_ground_term, bindings);
                            if equals == false { return false; }
                        }
                    },
                    _ => return false
                }
            },
            TermObject::Atom(a) => {
                // Both terms have to be the same atom term.
                match ground_term {
                    TermObject::Atom(arg_ground_term_atom) => {
                        if a != arg_ground_term_atom { return false; }
                    },
                    _ => return false
                }
                return true;
            },
            TermObject::Variable(v) => {
                // Directly add ground term into bindings indexed by current variable.
                bindings.insert(v.clone(), ground_term.clone());
                return true;
            }
        }
        return true;
    }

    fn _get_bindings(&self, ground_term: TermObject) -> Result<HashMap<Variable, TermObject>, PyErr> {
        let mut map: HashMap<Variable, TermObject> = HashMap::new();
        let equals = Composite::bind_helper(&TermObject::Composite(self.clone()), &ground_term, &mut map);
        if equals {
            Ok(map)
        } else {
            Ok(HashMap::new())
        }
    }

    fn _propagate_bindings(&self, bindings: &HashMap<Variable, TermObject>) -> Result<TermObject, PyErr> {
        let mut new_args: Vec<Box<TermObject>> = Vec::new();
        for i in 0..self.args.len() {
            let arg = self.args.get(i).unwrap().as_ref();
            match arg {
                TermObject::Composite(c) => {
                    let new_c_obj = c._propagate_bindings(bindings).unwrap();
                    new_args.insert(i, Box::new(new_c_obj));
                },
                TermObject::Variable(v) => {
                    if bindings.contains_key(v) {
                        let value = bindings.get(v).unwrap().clone();
                        new_args.insert(i, Box::new(value));
                    } else {
                        new_args.insert(i, Box::new(TermObject::Variable(v.clone())))
                    }
                },
                TermObject::Atom(a) => {
                    new_args.insert(i, Box::new(TermObject::Atom(a.clone())))
                }
            }
        }

        let new_composite = Composite {
            type_name: self.type_name.clone(),
            sort: self.sort.clone(),
            args: new_args,
            alias: self.alias.clone()
        };

        Ok(TermObject::Composite(new_composite))
    }
}

impl Hash for Composite {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for arg in &self.args {
            let y = arg.as_ref();
            match y {
                TermObject::Variable(v) => v.hash(state),
                TermObject::Atom(a) => a.hash(state),
                TermObject::Composite(c) => c.hash(state),
            }
        }

        self.type_name.hash(state);
    }
}

impl PartialEq for Composite {
    fn eq(&self, other: &Self) -> bool {
        // TODO: Compare hash first instead of recursively compare each argument.
        if self.type_name != other.type_name { return false; }
        else if self.args.len() != other.args.len() { return false; }
        else {
            for i in 0..self.args.len() {
                if self.args.get(i).unwrap().as_ref() !=
                    other.args.get(i).unwrap().as_ref() {
                    return false;
                }
            }
        }
        return true;
    }
}


#[pymethods]
impl Composite {
    #[new]
    fn new(obj: &PyRawObject, relation: PyObject, args: &PyList) {
        let mut arguments: Vec<Box<TermObject>> = Vec::new();
        for arg in args.iter() {
            if let Ok(var) = arg.cast_as::<Variable>() {
                let boxed_var = Box::new(TermObject::Variable(var.clone()));
                arguments.push(boxed_var);
            } else if let Ok(atom) = arg.cast_as::<Atom>() {
                let boxed_atom = Box::new(TermObject::Atom(atom.clone()));
                arguments.push(boxed_atom);
            } else if let Ok(c) = arg.cast_as::<Composite>() {
                let boxed_composite = Box::new(TermObject::Composite(c.clone()));
                arguments.push(boxed_composite);
            } else {
                let msg = format!("Expect Variable, Atom or Composite but found {}", arg.get_type().name());
                //return TypeError::into(msg);
            }
        }

        let gil = Python::acquire_gil();
        let py = gil.python();
        let basic_type: String = relation.getattr(py, "name").unwrap().extract(py).unwrap();

        obj.init(Composite {
            type_name: basic_type,
            sort: Rc::new(relation),
            args: arguments,
            alias: Some("hi, alias".into()),
        });
    }

    #[getter]
    fn get_sort(&self, py: Python) -> PyResult<PyObject> {
        Ok(self.sort.to_object(py))
    }

    fn get_variables(&self) -> PyResult<Vec<Variable>> {
        let mut result = vec![];
        for boxed_termObj in self.args.iter() {
            let termObj = boxed_termObj.as_ref();
            match termObj {
                TermObject::Variable(v) => {
                    result.push(v.clone());
                },
                TermObject::Composite(c) => {
                    let vars = c.get_variables().unwrap();
                    for var in vars {
                        result.push(var);
                    }
                },
                _ => {}
            }
        }
        Ok(result)
    }

    fn check_ground_term(&self) -> PyResult<bool> {
        let vars = self.get_variables().unwrap();
        if vars.len() == 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn get_bindings(&self, ground_term: &PyAny) -> PyResult<HashMap<Variable, PyObject>> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        if let Ok(composite) = ground_term.cast_as::<Composite>() {
            let _map: HashMap<Variable, TermObject> = self._get_bindings(TermObject::Composite(composite.clone())).unwrap();
            let mut bindings: HashMap<Variable, PyObject> = HashMap::new();
            for (key, value) in _map {
                match value {
                    TermObject::Variable(v) => bindings.insert(key.clone(), v.into_py(py)),
                    TermObject::Atom(a) => bindings.insert(key.clone(), a.into_py(py)),
                    TermObject::Composite(c) => bindings.insert(key.clone(), c.into_py(py))
                };
            }
            Ok(bindings)
        } else {
            Err(exceptions::AssertionError::py_err("Can't compare to a term that is not a composite ground term"))
        }
    }

    fn propagate_bindings(&self, bindings: &PyDict) -> PyResult<PyObject> {
        // Have to turn Python object bindings into hash map in Rust
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut _bindings: HashMap<Variable, TermObject> = HashMap::new();
        for (key, value) in bindings {
            if let Ok(_key) = key.cast_as::<Variable>() {
                if let Ok(_value) = value.cast_as::<Atom>() {
                    _bindings.insert(_key.clone(), TermObject::Atom(_value.clone()));
                } else if let Ok(_value) = value.cast_as::<Composite>() {
                    _bindings.insert(_key.clone(), TermObject::Composite(_value.clone()));
                } else {
                    return Err(exceptions::AssertionError::py_err("The value must be either atom or composite"))
                }
            } else {
                return Err(exceptions::AssertionError::py_err("The key must be a variable"))
            }
        }

        let termObj = self._propagate_bindings(&_bindings).unwrap();
        match termObj {
            TermObject::Composite(c) => {
                return Ok(c.into_py(py));
            },
            _ => {
                return Err(exceptions::AssertionError::py_err("It must be a composite term"));
            }
        }
    }
}

#[pyproto]
impl PyObjectProtocol for Composite {
    fn __str__(&self) -> PyResult<String> {
        let type_name: String = self.type_name.to_string();
        let mut arg_strs = vec![];
        for boxed_arg in self.args.iter() {
            let arg = boxed_arg.as_ref();
            match arg {
                TermObject::Variable(v) => {
                    arg_strs.push(v.__str__().unwrap());
                },
                TermObject::Atom(a) => {
                    arg_strs.push(a.__str__().unwrap());
                },
                TermObject::Composite(c) => {
                    arg_strs.push(c.__str__().unwrap());
                }
            }
        }
        let arg_str = arg_strs.join(",");
        Ok(format!("{}({})", type_name, arg_str))
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
        Ok(self.args.len())
    }
}



#[derive(Clone)]
enum AtomObject {
    String(String),
    Integer(usize),
    Boolean(bool),
    Float(f64),
}

impl PartialEq for AtomObject {
    fn eq(&self, other: &Self) -> bool {
        match self {
            AtomObject::String(s) => {
                match other {
                    AtomObject::String(s2) => {
                        if s == s2 { return true; }
                        else { return false; }
                    },
                    _ => false
                }
            },
            AtomObject::Integer(i) => {
                match other {
                    AtomObject::Integer(i2) => {
                        if i == i2 { return true; }
                        else { return false; }
                    },
                    _ => false
                }
            },
            AtomObject::Boolean(b) => {
                match other {
                    AtomObject::Boolean(b2) => {
                        if b == b2 { return true; }
                        else { return false; }
                    },
                    _ => false
                }
            },
            AtomObject::Float(f) => {
                match other {
                    AtomObject::Float(f2) => {
                        if f == f2 { return true; }
                        else {return false}
                    },
                    _ => false
                }
            }
            _ => false
        }
    }
}

#[pyclass]
#[derive(Clone)]
struct Atom {
    val: AtomObject,
    sort: Rc<PyObject>,
}

impl Hash for Atom {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.val {
            AtomObject::String(a) => a.hash(state),
            AtomObject::Boolean(b) => b.hash(state),
            AtomObject::Float(c) => c.to_bits().hash(state),
            AtomObject::Integer(d) => d.hash(state),
        };
    }
}

impl PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        if self.val == other.val { return true; }
        else { return false; }
    }
}

#[pymethods]
impl Atom {
    #[new]
    fn new(obj: &PyRawObject, value: &PyAny, atom_sort: PyObject) {
        let mut atomObj = AtomObject::Boolean(false);
        if let Ok(v) = value.cast_as::<PyString>() {
            atomObj = AtomObject::String(v.to_string_lossy().to_string());
        } else if let Ok(v) = value.cast_as::<PyBool>() {
            atomObj = AtomObject::Boolean(v.is_true());
        } else if let Ok(v) = value.cast_as::<PyFloat>() {
            atomObj = AtomObject::Float(v.value());
        } else if let Ok(v) = value.cast_as::<PyInt>() {
            atomObj = AtomObject::Integer(v.extract().unwrap())
        }

        obj.init(Atom {
            val: atomObj,
            sort: Rc::new(atom_sort),
        })
    }

    fn get_variables(&self) -> PyResult<Vec<Variable>> {
        Ok(vec![])
    }

    fn check_ground_term(&self) -> PyResult<bool> {
        Ok(true)
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
        let result = match &self.val {
            AtomObject::String(a) => format!("\"{}\"", a),
            AtomObject::Boolean(b) => format!("{}", b),
            AtomObject::Float(c) => format!("{}", c),
            AtomObject::Integer(d) => format!("{}", d),
        };

        Ok(result)
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


#[pyclass]
#[derive(Clone)]
struct Variable {
    var: String,
    fragments: Vec<String>,
    sort: Rc<PyObject>,
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

impl Variable {
    fn _get_bindings(&self, ground_term: TermObject) -> Result<HashMap<Variable, TermObject>, PyErr> {
       let mut map: HashMap<Variable, TermObject> = HashMap::new();
        match ground_term {
            TermObject::Variable(variable) => {
                return Err(exceptions::AssertionError::py_err("Can't find bindings for a variable"));
            },
            TermObject::Atom(atom) => {
                 map.insert(self.clone(), TermObject::Atom(atom));
            },
            TermObject::Composite(composite) => {
                if composite.get_variables().unwrap().len() != 0 {
                    return Err(exceptions::AssertionError::py_err("Can't find bindings for composite having variables inside it"))
                }
                map.insert(self.clone(), TermObject::Composite(composite));
            }
        }
        Ok(map)
    }
}

#[pymethods]
impl Variable {
    #[new]
    fn new(obj: &PyRawObject, var_str: String, var_sort: PyObject) {
        let mut frags: Vec<String> = var_str.split('.').map(|x| x.to_string()).collect();
        let first = frags.remove(0).to_string();

        obj.init(Variable {
            var: first,
            fragments: frags,
            sort: Rc::new(var_sort),
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

    fn check_ground_term(&self) -> PyResult<bool> {
        Ok(false)
    }

    fn get_bindings(&self, ground_term: &PyAny) -> PyResult<HashMap<Variable, PyObject>> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut map: HashMap<Variable, PyObject> = HashMap::new();
        if let Ok(composite) = ground_term.cast_as::<Composite>() {
            // Create a new map with values changed to corresponding PyObject.
            let mut _map: HashMap<Variable, TermObject> = self._get_bindings(TermObject::Composite(composite.clone())).unwrap();
            for (key, value) in _map {
                match value {
                    TermObject::Variable(v) => map.insert(key.clone(), v.into_py(py)),
                    TermObject::Atom(a) => map.insert(key.clone(), a.into_py(py)),
                    TermObject::Composite(c) => map.insert(key.clone(), c.into_py(py))
                };
            }
        } else if let Ok(atom) = ground_term.cast_as::<Atom>() {
            map.insert(self.clone(), atom.clone().into_py(py));
        } else {
            return Err(exceptions::AssertionError::py_err("Ground term cannot be a variable"));
        }
        Ok(map)
    }

    fn propagate_bindings(&self, bindings: &PyAny) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        if let Ok(dict) = bindings.cast_as::<PyDict>() {
            let var: PyObject = self.clone().into_py(py);
            if dict.contains(&var).unwrap() {
                let value = dict.get_item(&var).unwrap();
                return Ok(value.into_py(py));
            } else {
                return Ok(self.clone().into_py(py));
            }
        } else {
            Err(exceptions::AssertionError::py_err("Can't fully propagate all variables"))
        }
    }
}

#[pyproto]
impl PyObjectProtocol for Variable {
    fn __str__(&self) -> PyResult<String> {
        let s = "<".to_string() + &self.var + &">";
        Ok(s)
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




#[pyclass(module = "ddengine")]
struct DDExecuter {
    mode: bool,
}

#[pymethods]
impl DDExecuter {
    #[new]
    fn new(obj: &PyRawObject) {
        obj.init(DDExecuter { mode: true});

    }
}


/// Represents a file that can be searched
#[pyclass(module = "ddengine")]
struct WordCounter {
    path: PathBuf,
}

#[pymethods]
impl WordCounter {
    #[new]
    fn new(obj: &PyRawObject, path: String) {
        obj.init(WordCounter {
            path: PathBuf::from(path),
        });
    }

    #[getter]
    fn path(&self) -> PyResult<String>{
        Ok("hello".to_string())
    }

    /// Searches for the word, parallelized by rayon
    fn search(&self, py: Python<'_>, search: String) -> PyResult<usize> {
        let contents = fs::read_to_string(&self.path)?;

        let count = py.allow_threads(move || {
            contents
                .par_lines()
                .map(|line| count_line(line, &search))
                .sum()
        });
        Ok(count)
    }

    /// Searches for a word in a classic sequential fashion
    fn search_sequential(&self, needle: String) -> PyResult<usize> {
        let contents = fs::read_to_string(&self.path)?;

        let result = contents.lines().map(|line| count_line(line, &needle)).sum();

        Ok(result)
    }
}

fn matches(word: &str, needle: &str) -> bool {
    let mut needle = needle.chars();
    for ch in word.chars().skip_while(|ch| !ch.is_alphabetic()) {
        match needle.next() {
            None => {
                return !ch.is_alphabetic();
            }
            Some(expect) => {
                if ch.to_lowercase().next() != Some(expect) {
                    return false;
                }
            }
        }
    }
    return needle.next().is_none();
}

/// Count the occurences of needle in line, case insensitive
#[pyfunction]
fn count_line(line: &str, needle: &str) -> usize {
    let mut total = 0;
    for word in line.split(' ') {
        if matches(word, needle) {
            total += 1;
        }
    }
    total
}

#[pymodule(ddengine)]
fn ddengine(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(count_line))?;
    m.add_class::<WordCounter>()?;
    m.add_class::<Atom>()?;
    m.add_class::<Variable>()?;
    m.add_class::<Composite>()?;
    m.add_class::<DDExecuter>()?;

    Ok(())
}