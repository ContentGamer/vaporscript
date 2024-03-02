use core::hash;
use std::{any::Any, collections::HashMap, hash::Hash, rc::Rc};

use alloc::{boxed::Box, string::ToString, vec::Vec};

use crate::{ast::ast::{ArrayLiteral, AssignmentExpr, BinaryExpr, CallExpr, FuncDeclaration, Identifier, MemberExpr, NullLiteral, NumericLiteral, ObjectLiteral, Program, Statment, VarDeclaration}, logger::logger::{log, LogLevel}, utils::shift};
use super::{env::{assign_var, Environment}, values::{ArrayVal, FuncVal, MemberVal, NativeFn, NullVal, NumberVal, ObjectVal, StringVal, ValueType}};

enum BinaryExprEvaluate {
    NumericLiteral(NumericLiteral),
    BinaryExpr(BinaryExpr)
}

fn evaluate_num(left: f64, right: f64, operation: &str) -> f64 {
    match operation {
        "+" => left + right,
        "*" => left * right,
        "/" => {
            if (left == 0.0 || right == 0.0) {
                log(LogLevel::Error, "Cannot divide by 0");
                return 0.0;
            }
            left / right
        },
        "-" => left - right,
        "%" => left % right,
        _ => left % right
    }
}

fn evaluate_fn(declaration: FuncDeclaration, env: &mut Environment) -> ValueType {
    let func: ValueType = ValueType::Function(FuncVal {
        base: "FuncVal".to_string(),
        name: declaration.name.clone(),
        parameters: declaration.parameters,
        body: declaration.body
    });

    env.declare_var(&declaration.name, &func, true)
}

fn evaluate_call(obj: CallExpr, env: &mut Environment) -> ValueType {
    let args: Vec<ValueType> = obj.args.iter().map(|x| evaluate(&x, env)).collect();
    let func = evaluate(&obj.caller, env);

    match func {
        ValueType::NativeFn(native) => {
            (native.call)(args, env)
        }
        ValueType::Function(func) => {
            let mut result: ValueType = ValueType::None();
            let mut scope = Environment::new();
            let mut constants: &Vec<String> = &env.constants;

            for (varname, value) in &env.variables {
                scope.declare_var(varname, value, constants.contains(varname));
            }

            for i in 0..func.parameters.len() {
                let varname = func.parameters[i].clone();
                scope.declare_var(&varname, &args[i], false);
            }
            for stmt in func.body {
                result = evaluate(&stmt, &mut scope);
            }

            result
        }
        _ => {
            log(LogLevel::Error, "Cannot call a function that is not a native-fn type.");
            ValueType::None()
        }
    }
}

fn evaluate_member(memexpr: MemberExpr, env: &mut Environment) -> ValueType {
    let mut eval: ValueType = ValueType::None();
    let object: ValueType = evaluate(&memexpr.object, env);
    let property: ValueType = evaluate(&memexpr.property, env);

    match &object {
        ValueType::Array(arr) => {
            let prop = match &property {
                ValueType::Number(num) => num.value.to_string().parse::<usize>().unwrap_or(0),
                _ => {
                    log(LogLevel::Error, "Expected a number while indexing an array.");
                    0
                }
            };
            eval = arr.contents[prop].clone();
        }
        ValueType::Object(obj) => {
            let prop = match &property {
                ValueType::String(num) => num.value.to_string(),
                _ => {
                    log(LogLevel::Error, "Expected a string while indexing an object.");
                    "Err".to_string()
                }
            };
            eval = obj.properties.get(&prop).unwrap_or(&ValueType::None()).clone();
        }
        _ => {}
    }

    eval
}

fn evaluate_array(arr: ArrayLiteral, env: &mut Environment) -> ValueType {
    let mut array = ArrayVal {
        base: "ArrayVal".to_string(),
        contents: Vec::new()
    };

    for content in arr.contents {
        match &content {
            Statment::Identifier(iden) => {
                array.contents.push(env.lookup_var(&iden.symbol));
            }
            Statment::StringLiteral(iden) => {
                array.contents.push(evaluate(&content, env));
            }
            _ => {
                array.contents.push(evaluate(&content, env));
            }
        }
    }

    ValueType::Array(array)
}

fn evaluate_object(obj: ObjectLiteral, env: &mut Environment) -> ValueType {
    let mut object = ObjectVal {
        base: "ObjectVal".to_string(),
        properties: HashMap::new()
    };

    for prop in obj.properties {
        let key = &prop.key;
        match &prop.value {
            Statment::None() => {
                object.properties.insert(key.clone(), env.lookup_var(key));
            }
            _ => {
                object.properties.insert(key.clone(), evaluate(&prop.value, env));
            }
        }
    }

    ValueType::Object(object)
}

fn evaluate_assignment(assignment: AssignmentExpr, env: &mut Environment) -> ValueType {
    let value: &ValueType = &evaluate(&assignment.value, env);
    let assigne: String = match assignment.assigne {
        Statment::Identifier(iden) => {
            iden.symbol
        }
        _ => {
            log(LogLevel::Error, format!("Unknown Statment {:#?} while evaluating an assignment expression.", assignment.assigne).as_str());
            "Err".to_string()
        }
    };
    assign_var(env, &assigne, value)
}

fn evaluate_declaration(declaration: VarDeclaration, env: &mut Environment) -> ValueType {
    match &declaration.value {
        Statment::None() => {
            env.declare_var(&declaration.identifier, &&ValueType::Null(NullVal { base: "NullVal".to_string(), value: None }), false)
        }
        _ => {
            let statment = evaluate(&declaration.value, env);
            env.declare_var(&declaration.identifier, &statment, declaration.constant)
        }
    }
}

fn evaluate_program(program: Program, env: &mut Environment) -> ValueType {
    let mut last_eval: ValueType = ValueType::Null(NullVal { base: "NullVal".to_string(), value: None });
    for stmt in program.body {
        last_eval = evaluate(&stmt, env);
    }
    last_eval
}

fn evaluate_binexpr(binary_expr: &mut Box<BinaryExpr>, env: &mut Environment) -> ValueType {
    let mut binop: BinaryExpr = *binary_expr.clone();

    let left: ValueType = evaluate(&binop.left, env);
    let right: ValueType = evaluate(&binop.right, env);
    
    match (left, right) {
        (ValueType::Number(l_num), ValueType::Number(r_num)) => {
            ValueType::Number(NumberVal {
                base: "NumberVal".to_string(),
                value: evaluate_num(l_num.value, r_num.value, binop.operator.as_str())
            })
        },
        (_, _) => {
            ValueType::Null(NullVal {
                base: "NullVar".to_string(),
                value: None
            })
        }
    }
}

pub fn evaluate(ast_node: &Statment, env: &mut Environment) -> ValueType {
    match ast_node {
        Statment::NumericLiteral(ident) => {
            ValueType::Number(NumberVal {
                base: "NumberVal".to_string(),
                value: ident.value
            })
        }
        Statment::NullLiteral(ident) => {
            ValueType::Null(NullVal {
                base: "NullVal".to_string(),
                value: None
            })
        }
        Statment::StringLiteral(ident) => {
            ValueType::String(StringVal {
                base: "StringVal".to_string(),
                value: ident.value.clone()
            })
        }

        Statment::BinaryExpr(ident) => {
            evaluate_binexpr(&mut ident.clone(), env)
        }
        Statment::Program(ident) => {
            evaluate_program(ident.clone(), env)
        }
        Statment::VarDeclaration(ident) => {
            evaluate_declaration(*ident.clone(), env)
        }
        Statment::Identifier(ident) => {
            env.lookup_var(&ident.symbol)
        }
        Statment::AssignmentExpr(ident) => {
            evaluate_assignment(*ident.clone(), env)
        }
        Statment::ObjectLiteral(ident) => {
            evaluate_object(ident.clone(), env)
        }
        Statment::ArrayLiteral(ident) => {
            evaluate_array(*ident.clone(), env)
        }
        Statment::CallExpr(ident) => {
            evaluate_call(*ident.clone(), env)
        }
        Statment::FuncDeclaration(func) => {
            evaluate_fn(*func.clone(), env)
        }

        Statment::MemberExpr(expr) => {
            evaluate_member(*expr.clone(), env)
        }

        _ => {
            log(LogLevel::Warn, format!("This Runtime Interperter did not regonize a AST Node: {:#?}", ast_node).as_str());
            ValueType::None()
        }
    }
}