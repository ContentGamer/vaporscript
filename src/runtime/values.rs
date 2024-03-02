use std::{cell::RefCell, collections::HashMap};

use crate::ast::ast::Statment;

use super::env::Environment;

#[derive(Debug, Clone)]
pub enum ValueType {
    Null(NullVal),
    Number(NumberVal),
    String(StringVal),
    Boolean(BooleanVal),
    Object(ObjectVal),
    Array(ArrayVal),
    Member(Box<MemberVal>),

    NativeFn(NativeFn),
    Function(FuncVal),

    FnVoid(FuncVoid),
    FnString(StringVal),
    FnNumber(NumberVal),
    FnObject(ObjectVal),

    None()
}

#[derive(Debug, Clone)]
pub struct FuncVoid {}

#[derive(Debug, Clone)]
pub struct NullVal {
    pub base: String,
    pub value: Option<String>
}
#[derive(Debug, Clone)]
pub struct NumberVal {
    pub base: String,
    pub value: f64
}
#[derive(Debug, Clone)]
pub struct StringVal {
    pub base: String,
    pub value: String
}
#[derive(Debug, Clone)]
pub struct BooleanVal {
    pub base: String,
    pub value: bool
}
#[derive(Debug, Clone)]
pub struct ObjectVal {
    pub base: String,
    pub properties: HashMap<String, ValueType>
}
#[derive(Debug, Clone)]
pub struct ArrayVal {
    pub base: String,
    pub contents: Vec<ValueType>
}
#[derive(Debug, Clone)]
pub struct NativeFn {
    pub base: String,
    pub call: fn(args: Vec<ValueType>, env: &mut Environment) -> ValueType
}
#[derive(Debug, Clone)]
pub struct FuncVal {
    pub base: String,
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Vec<Statment>
}
#[derive(Debug, Clone)]
pub struct MemberVal {
    pub base: String,
    pub object: ValueType,
    pub property: ValueType,
    pub eval: ValueType,
    pub computed: bool
}