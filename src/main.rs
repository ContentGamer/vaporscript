#![allow(unused_parens)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused)]

extern crate alloc;

mod lexer;
mod utils;
mod ast;
mod runtime;
mod logger;
mod funcs;

use alloc::string::{String, ToString};
use lexer::lexer::tokenize;
use logger::logger::{log, LogLevel};
use runtime::env::{setup_fn, setup_scopes, Environment};
use std::{collections::HashMap, fs::File, hash::Hash, io::{self, BufReader, Read, Write}};

use crate::{ast::{ast::{Program, Statment}, parser::{self, produce_ast}}, runtime::{interpreter::evaluate, values::{BooleanVal, NullVal, ValueType}}, utils::{clear_terminal, parse_value}};

fn main() {
    let mut input = String::new();
    let mut env: Environment = Environment::new();

    setup_scopes(&mut env);
    setup_fn(&mut env);

    log(LogLevel::Warn, "VaporScript 0.1 (ALPHA)\n");
    loop {
        input = String::new();

        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap_or(2);

        if (input.contains(&"exit".to_string())) {
            std::process::exit(0);
        }

        if (input.contains(&".cls".to_string())) {
            clear_terminal();
        } else {
            let program: Statment = parser::produce_ast(&input);
            let result: ValueType = evaluate(&program, &mut env);
            println!("\n{}", format!("{:#?}", result));

	    /*
            let mut file = File::open("./lang/test.txt").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents);

            let program: Statment = parser::produce_ast(&contents);
            let result: ValueType = evaluate(&program, &mut env);
            println!("\n{}", format!("{:#?}", result));
	    */
        }
    }
}