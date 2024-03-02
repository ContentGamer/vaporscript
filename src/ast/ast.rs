use std::{any::Any, rc::Rc};

use alloc::{boxed::Box, vec::Vec};
use crate::runtime::values::ValueType;

#[derive(Debug, Clone)]
pub enum Statment {
    Program(Program),
    Statment(Box<Statment>),
    Identifier(Identifier),
    VarDeclaration(Box<VarDeclaration>),
    AssignmentExpr(Box<AssignmentExpr>),
    BinaryExpr(Box<BinaryExpr>),
    MemberExpr(Box<MemberExpr>),
    CallExpr(Box<CallExpr>),
    FuncDeclaration(Box<FuncDeclaration>),
    ArrayLiteral(Box<ArrayLiteral>),

    PropertyLiteral(Box<PropertyLiteral>),
    ObjectLiteral(ObjectLiteral),
    NumericLiteral(NumericLiteral),
    NullLiteral(NullLiteral),
    StringLiteral(StringLiteral),

    None()
}
#[derive(Debug, Clone)]
pub struct Program {
    pub body: Vec<Statment>
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub kind: String
}
#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub kind: String,

    pub left: Statment,
    pub right: Statment,
    pub operator: String
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub kind: String,
    pub symbol: String
}
#[derive(Debug, Clone)]
pub struct NumericLiteral {
    pub kind: String,
    pub value: f64
}
#[derive(Debug, Clone)]
pub struct NullLiteral {
    pub kind: String,
    pub value: String
}
#[derive(Debug, Clone)]
pub struct VarDeclaration {
    pub kind: String,
    pub constant: bool,
    pub identifier: String,
    pub value: Statment
}
#[derive(Debug, Clone)]
pub struct AssignmentExpr {
    pub kind: String,
    pub assigne: Statment,
    pub value: Statment
}
#[derive(Debug, Clone)]
pub struct PropertyLiteral {
    pub kind: String,
    pub key: String,
    pub value: Statment
}
#[derive(Debug, Clone)]
pub struct ObjectLiteral {
    pub kind: String,
    pub properties: Vec<PropertyLiteral>
}
#[derive(Debug, Clone)]
pub struct CallExpr {
    pub kind: String,
    pub args: Vec<Statment>,
    pub caller: Statment
}
#[derive(Debug, Clone)]
pub struct MemberExpr {
    pub kind: String,
    pub object: Statment,
    pub property: Statment,
    pub computed: bool
}
#[derive(Debug, Clone)]
pub struct FuncDeclaration {
    pub kind: String,
    pub parameters: Vec<String>,
    pub name: String,
    pub body: Vec<Statment>,
    pub sync: bool,
    pub arrow: bool
}
#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub kind: String,
    pub value: String
}
#[derive(Debug, Clone)]
pub struct ArrayLiteral {
    pub kind: String,
    pub contents: Vec<Statment>
}
#[derive(Debug, Clone)]
pub struct WhileLoop {
    pub kind: String,
    pub eval_body: Vec<Statment>,
    pub check_body: Vec<Statment>
}