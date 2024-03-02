use std::{any::Any, rc::Rc};

use crate::{logger::logger::{log, LogLevel}, runtime::{env::Environment, values::{FuncVoid, StringVal, ValueType}}};

fn base_fn(fnn: &str, args: &Vec<ValueType>, arguments: usize) -> () {
    if (args.len() < arguments) {
        log(LogLevel::Error, format!("{}_fn: Expected an argument.", fnn).as_str());
    }
}
fn oftype(fnn: &str, indx: usize, args: &Vec<ValueType>, t: &str) -> () {
    let mut arg_t: String = String::new();
    let arg: ValueType = args[indx].clone();
    match arg {
        ValueType::String(v) => arg_t = "string".to_string(),
        ValueType::Number(v) => arg_t = "number".to_string(),
        ValueType::Object(v) => arg_t = "object".to_string(),
        _ => arg_t = "err".to_string()
    };
    if (arg_t.to_lowercase() != t.to_string().to_lowercase()) {
        log(LogLevel::Error, format!("{}_fn: Expected argument {} of the function to be a {}.", fnn, indx, t).as_str());
    }
}

pub fn print(args: Vec<ValueType>, env: &mut Environment) -> ValueType {
    base_fn("print", &args, 1);
    println!("{:?}", &args[0]);

    ValueType::FnVoid(FuncVoid {})
}