extern crate differential_formula;
extern crate clap;

use std::fs;
use std::io;
use std::io::Write;

use clap::{Arg, ArgMatches, App, AppSettings, crate_version};

use differential_formula::term::*;
use differential_formula::engine::*;
use differential_formula::module::*;
use differential_formula::type_system::*;

fn main() {
    
    let mut engine = DDEngine::new();
    engine.inspect = true;

    // Start the interactive CLI
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut s = String::new();
        io::stdin()
            .read_line(&mut s)
            .ok()
            .expect("failed to read line");

        run_command(&s, &mut engine);
    }
        
}

fn run_command(cmd: &str, engine: &mut DDEngine) {
    let mut app = App::new("Differential Formula")
            .setting(AppSettings::WaitOnError)
            .version(crate_version!())
            .author("Qishen Zhang <qishen23@gmail.com>")
            .about("Formal Specifications for Verification and Synthesis")
            .subcommand(App::new("execute")
                        .about("Read a file and all the commands in the file.")
                        .arg(Arg::with_name("FILE")
                            .required(true)
                            .index(1)))
            .subcommand(App::new("load")
                        .about("Load a Formula file ending with .4ml")
                        .arg(Arg::with_name("FILE")
                            .required(true)
                            .index(1)))
            .subcommand(App::new("print")
                        .about("Print out the content of a module in current environment.")
                        .arg(Arg::with_name("MODULE")
                        .required(true)
                        .index(1)))
            .subcommand(App::new("query")
                        .about("Query a model")
                        .arg(Arg::with_name("MODEL")
                            .required(true)
                            .index(1)))
            .subcommand(App::new("apply")
                        .about("Apply a model transformation given models and input terms")
                        .arg(Arg::with_name("CMD")
                            .required(true)
                            .index(1)));

    // Examples: 
    // query Path10, query LittleCycle, 
    // apply r = Add(100, LittleCycle)
    let mut argv: Vec<_> = cmd.split_whitespace().collect();
    let cmd = argv[1..argv.len()].join("");
    if argv.contains(&"apply") {
        argv = vec!["apply", &cmd];
    }

    // you have to insert the binary name since clap expects it
    argv.insert(0, "differential-formula");

    match app.try_get_matches_from_mut(argv) {
        Ok(matches) => {
            execute_matches(matches, engine);
        },
        Err(e) => {
            // TODO: find a way to print out the err properly in terminal.
            println!("{}", e);
        }
    }
}

fn execute_matches(matches: ArgMatches, engine: &mut DDEngine) {
    if let Some(matches) = matches.subcommand_matches("load") {
        let file_path = matches.value_of("FILE").unwrap();
        println!("FORUMULA file path: {:?}", file_path);
        let content = fs::read_to_string(file_path).unwrap();
        engine.install(content);

    } else if let Some(matches) = matches.subcommand_matches("query") {
        let model_name = matches.value_of("MODEL").unwrap();
        let model = engine.env.get_model_by_name(&model_name).unwrap().clone();
        let mut session = Session::new(model, &engine);
        session.load();

    } else if let Some(matches) = matches.subcommand_matches("apply") {
        let transform_cmd = matches.value_of("CMD").unwrap();
        let transformation = engine.create_model_transformation(transform_cmd);
        let mut session = Session::new(transformation, &engine);
        session.load();
    } else if let Some(matches) = matches.subcommand_matches("print") {
        let module_name = matches.value_of("MODULE").unwrap();
        if engine.env.domain_map.contains_key(module_name) {
            let domain = engine.env.domain_map.get(module_name).unwrap();
            println!("{:?}", domain);
        } else if engine.env.model_map.contains_key(module_name) {
            let model = engine.env.model_map.get(module_name).unwrap();
            println!("{:?}", model);
        } else if engine.env.transform_map.contains_key(module_name) {
            let transform = engine.env.transform_map.get(module_name).unwrap();
            println!("{:?}", transform);
        } else {
            println!("No module with name `{}` is found in current environment.", module_name);
        }
    } else if let Some(matches) = matches.subcommand_matches("execute") {
        let file_path = matches.value_of("FILE").unwrap();
        println!("Path of the file with commands to be executed: {:?}", file_path);
        let content = fs::read_to_string(file_path).unwrap();
        for line in content.split("\n") {
            println!("---- Starting to run command: {} ----", line);
            run_command(line, engine);
        }
    }
}