extern crate differential_formula;
extern crate clap;

use std::fs;
use std::io;
use std::io::Write;

use clap::{Arg, ArgMatches, App, SubCommand};

use differential_formula::term::*;
use differential_formula::engine::*;
use differential_formula::module::*;
use differential_formula::type_system::*;

fn main() {
    
    let mut engine = DDEngine::new();
    engine.inspect = true;

    let mut app = App::new("Differential Formula")
            .version("1.0")
            .author("Qishen Zhang <qishen23@gmail.com>")
            .about("Formal Specifications for Verification and Synthesis")
            .subcommand(SubCommand::with_name("load")
                        .about("Load a Formula file ending with .4ml")
                        .arg(Arg::with_name("FILE")
                            .required(true)
                            .index(1)
                            .help("Load a FORMULA file ending with .4ml")))
            .subcommand(SubCommand::with_name("query")
                        .about("Query a model")
                        .arg(Arg::with_name("MODEL")
                            .required(true)
                            .index(1)
                            .help("Query a model by matching patterns")))
            .subcommand(SubCommand::with_name("apply")
                        .about("Apply a model transformation given models and input terms")
                        .arg(Arg::with_name("CMD")
                            .required(true)
                            .index(1)
                            .help("Apply a model transformation given models and input terms")));

    // Start the interactive CLI
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut s = String::new();
        io::stdin()
            .read_line(&mut s)
            .ok()
            .expect("failed to read line");

        // Examples: 
        // query Path10, query LittleCycle, apply r = Add(100, LittleCycle)
        let mut argv: Vec<_> = s.split_whitespace().collect();
        let cmd = argv[1..argv.len()].join("");
        if argv.contains(&"apply") {
            argv = vec!["apply", &cmd];
        }
        // you have to insert the binary name since clap expects it
        argv.insert(0, "differential-formula");

        match app.get_matches_from_safe_borrow(argv) {
            Ok(matches) => {
                // println!("{:?}", matches);
                execute(matches, &mut engine);
            },
            Err(e) => {
                // handle error, display message, etc.
                println!("{:?}", e);
                continue
            }
        }
    }
        
}

fn execute(matches: ArgMatches, engine: &mut DDEngine) {
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
    }
}