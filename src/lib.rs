#[macro_use]
extern crate nom;
extern crate num;
extern crate abomonation_derive;
extern crate abomonation;

pub mod constraint;
pub mod converter;
pub mod engine;
pub mod expression;
pub mod module;
pub mod parser;
pub mod rule;
pub mod term;
pub mod type_system;
pub mod util;

// use parser::combinator::parse_program;
// use term::*;
// use module::Env;

// use pyo3::prelude::*;
// use pyo3::wrap_pyfunction;

// #[pyfunction]
// /// Formats the sum of two numbers as string.
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

// #[pyclass]
// struct MyClass {
//     #[pyo3(get)]
//     weight: usize,
//     name: String,
// }

// #[pymethods]
// impl MyClass {
//     #[new]
//     fn new(weight: usize, name: String) -> Self {
//         MyClass { weight, name }
//     }
// }

// #[derive(FromPyObject)]
// enum RustyEnum {
//     #[pyo3(transparent, annotation = "str")]
//     String(String),
//     #[pyo3(transparent, annotation = "int")]
//     Int(isize),
// }

// impl RustyEnum {
//     fn new(s: String) -> Self {
//         RustyEnum::String(s)
//     }
// }

// #[pyfunction]
// fn create_env(path: String) -> PyResult<Env> {
//     let path = std::path::Path::new("./tests/testcase/p0.4ml");
//     let content = std::fs::read_to_string(path).unwrap() + "EOF";
//     let (_, program_ast) = parse_program(&content);
        
//     let env: Env = program_ast.build_env();
//     Ok(env)
// }

// #[pymodule]
// /// A Python module implemented in Rust.
// fn differential_formula(py: Python, m: &PyModule) -> PyResult<()> {
//     // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
//     // m.add_function(wrap_pyfunction!(create_env, m)?)?;
//     m.add_class::<MyClass>()?;
//     // m.add_class::<type_system::CompositeType>()?;
//     Ok(())
// }