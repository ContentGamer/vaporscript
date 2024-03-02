use std::collections::HashMap;
use crate::{funcs, logger::logger::{log, LogLevel}, runtime::values::FuncVoid, utils::parse_value};

use super::values::{NativeFn, NullVal, NumberVal, ObjectVal, ValueType};

pub fn setup_fn(env: &mut Environment) -> () {
    env.declare_var(&"print".to_string(), &ValueType::NativeFn(NativeFn {
        base: "NativeFn".to_string(),
        call: funcs::print
    }), true);
}

pub fn setup_scopes(env: &mut Environment) -> () {
    env.declare_var(&"true".to_string(), &parse_value(&true), true);
    env.declare_var(&"false".to_string(), &parse_value(&false), true);
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub constants: Vec<String>,
    pub variables: HashMap<String, ValueType>
}

impl Environment {
    pub fn new() -> Self {
        Self {
            constants: Vec::new(),
            variables: HashMap::new()
        }
    }

    pub fn declare_var(&mut self, varname: &String, value: &ValueType, constant: bool) -> ValueType {
        if (self.variables.contains_key(varname)) {
            log(LogLevel::Error, format!("Cannot declare a variable '{}' as it is already declared.", varname).as_str());
            return ValueType::None();
        }
        self.variables.insert(varname.clone(), value.clone());
        if (constant) {
            self.constants.push(varname.clone());
        }
        value.clone()
    }
    
    pub fn assign_var(&mut self, varname: &String, value: &ValueType) -> ValueType {
        if (!self.variables.contains_key(varname)) {
            log(LogLevel::Error, format!("Cannot reassign variable '{}', as it is undefined (NotDefinedErr).", varname).as_str());
            return ValueType::None();
        }
        if (self.constants.contains(varname)) {
            log(LogLevel::Error, format!("Cannot reassign variable '{}', as it is a constant variable.", varname).as_str());
            return ValueType::None();
        }
        self.variables.remove(varname);
        self.variables.insert(varname.clone(), value.clone());
        value.clone()
    }
    
    pub fn lookup_var(&mut self, varname: &String) -> ValueType {
        if (!self.variables.contains_key(varname)) {
            log(LogLevel::Error, format!("Cannot lookup for variable '{}', as it does not exist.", varname).as_str());
            return ValueType::None();
        }
        self.variables.get(varname).unwrap_or(&ValueType::None()).clone()
    }
}

pub fn assign_var(env: &mut Environment, varname: &String, value: &ValueType) -> ValueType {
    if (!env.variables.contains_key(varname)) {
        log(LogLevel::Error, format!("Cannot reassign variable '{}', as it is undefined (NotDefinedErr).", varname).as_str());
        return ValueType::None();
    }
    if (env.constants.contains(varname)) {
        log(LogLevel::Error, format!("Cannot reassign variable '{}', as it is a constant variable.", varname).as_str());
        return ValueType::None();
    }
    env.variables.remove(varname);
    env.variables.insert(varname.clone(), value.clone());
    value.clone()
}