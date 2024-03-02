// Author: ContentGamer

use core::ptr::null;
use std::{any::Any, rc::Rc};

use alloc::{boxed::Box, string::{String, ToString}, vec::{self, Vec}};

use super::{ast::{AssignmentExpr, BinaryExpr, Expression, FuncDeclaration, Identifier, NullLiteral, NumericLiteral, ObjectLiteral, Program, PropertyLiteral, Statment, StringLiteral, VarDeclaration}, caller::{parse_args, parse_call_member}};
use crate::{ast::ast::ArrayLiteral, funcs::print, lexer::lexer::{tokenize, Token, TokenBase}, logger::logger::{log, LogLevel}, runtime::{interpreter::evaluate, values::ValueType}, utils::shift};

pub fn expect(tokens: &mut Vec<Token>, base: TokenBase, err: String) -> Token {
    let prev: Option<Token> = shift(tokens);
    match prev {
        Some(token) => {
            if (token.base != base) {
                log(LogLevel::Error, err.as_str())
            }
            token
        },
        None => {
            log(LogLevel::Error, err.as_str());
            Token { value: "NotAToken".to_string(), base: TokenBase::EoF }
        }
    }
}

pub fn parse_stmt(tokens: &mut Vec<Token>) -> Statment {
    match tokens[0].base {
        TokenBase::Let => parse_declaration(tokens),
        TokenBase::Const => parse_declaration(tokens),
        TokenBase::Function => parse_fn(tokens),

        TokenBase::ForLoop => parse_for_loop(tokens),
        _ => parse_expr(tokens)
    }
}

pub fn parse_for_loop(tokens: &mut Vec<Token>) -> Statment {
    shift(tokens);
    let name: String = expect(tokens, TokenBase::Identifier, "Expected a for loop identifier while parsing.".to_string()).value;

    expect(tokens, TokenBase::Identifier, "Expected an EQ identifier sign.".to_string());
    let start_indx: String = expect(tokens, TokenBase::Identifier, "Expected an identifier sign.".to_string()).value;
    expect(tokens, TokenBase::ToKeyword, "Expected a 'to' keyword next to the equals sign.".to_string());
    let end_indx: String = expect(tokens, TokenBase::Identifier, "Expected an end index identifier while parsing a for loop.".to_string()).value;

    println!("{:#?} {:#?} {:#?}", name, start_indx, end_indx);

    Statment::None()
}

pub fn parse_fn(tokens: &mut Vec<Token>) -> Statment {
    shift(tokens);
    let name: String = expect(tokens, TokenBase::Identifier, "Expected a function identifier while parsing.".to_string()).value;
    let args: Vec<Statment> = parse_args(tokens);
    let mut params: Vec<String> = Vec::new();

    for arg in args {
        match arg {
            Statment::Identifier(iden) => {
                params.push(iden.symbol);
            }
            _ => {
                log(LogLevel::Info, format!("Got Statment: {:#?}", arg).as_str());
                log(LogLevel::Error, "Expected an identifier inside the function declaration parameters.");
            }
        };
    }

    expect(tokens, TokenBase::OpenBrace, "Expected an FnBody while parsing the function.".to_string());
    let mut body: Vec<Statment> = Vec::new();

    while (tokens[0].base != TokenBase::EoF && tokens[0].base != TokenBase::CloseBrace) {
        body.push(parse_stmt(tokens));
    }

    expect(tokens, TokenBase::CloseBrace, "Expected a closing brace while building a function body.".to_string());
    let func: Statment = Statment::FuncDeclaration(Box::new(FuncDeclaration {
        kind: "FuncDeclaration".to_string(),
        body,
        parameters: params,
        name,
        sync: false,
        arrow: false
    }));

    func
}

pub fn parse_declaration(tokens: &mut Vec<Token>) -> Statment {
    let is_constant: bool = shift(tokens).unwrap_or(Token { value: "EndOfFile".to_string(), base: TokenBase::EoF }).base == TokenBase::Const;
    let identifier: String = expect(tokens, TokenBase::Identifier, "Expected an identifier while building a variable declaration.".to_string()).value;
    
    if (tokens[0].base == TokenBase::Semicolon) {
        shift(tokens);
        if (is_constant)
        {
            log(LogLevel::Error, "Must assign a value to a constant expression, No value was provided while searching.");
            return Statment::None();
        }

        return Statment::VarDeclaration(Box::new(VarDeclaration {
            kind: "VarDeclaration".to_string(),
            identifier,
            constant: false,
            value: Statment::None()
        }));
    }

    expect(tokens, TokenBase::Equals, "Expected an equals token while declaring a variable.".to_string());
    let declaration = Statment::VarDeclaration(Box::new(VarDeclaration {
        kind: "VarDeclaration".to_string(),
        identifier,
        constant: is_constant,
        value: parse_expr(tokens)
    }));
    expect(tokens, TokenBase::Semicolon, "Expected a semicolon (;) while building a variable declaration.".to_string());

    declaration
}

pub fn parse_expr(tokens: &mut Vec<Token>) -> Statment {
    parse_assignment_expr(tokens)
}

pub fn parse_array(tokens: &mut Vec<Token>) -> Statment {
    if (tokens[0].base != TokenBase::OpenBracket) {
        return parse_additive_expr(tokens);
    }

    shift(tokens);
    let mut contents: Vec<Statment> = Vec::new();

    while (tokens[0].base != TokenBase::EoF && tokens[0].base != TokenBase::CloseBracket) {
        let value = parse_expr(tokens);

        if (tokens[0].base == TokenBase::Comma) {
            shift(tokens);
            contents.push(value);
            continue;
        } else if (tokens[0].base == TokenBase::CloseBracket) {
            contents.push(value);
            continue;
        }
    }
    expect(tokens, TokenBase::CloseBracket, "Array litreal missing a closing bracket [.".to_string());

    Statment::ArrayLiteral(Box::new(ArrayLiteral {
        kind: "ArrayLiteral".to_string(),
        contents
    }))
}

pub fn parse_object_expr(tokens: &mut Vec<Token>) -> Statment {
    if (tokens[0].base != TokenBase::OpenBrace) {
        return parse_array(tokens);
    }

    shift(tokens);
    let mut properties: Vec<PropertyLiteral> = Vec::new();

    while (tokens[0].base != TokenBase::EoF && tokens[0].base != TokenBase::CloseBrace) {
        let key = expect(tokens, TokenBase::Identifier, "Object literal was missing a key.".to_string()).value;

        if (tokens[0].base == TokenBase::Comma) {
            shift(tokens);
            properties.push(PropertyLiteral {
                kind: "PropertyLiteral".to_string(),
                key,
                value: Statment::None()
            });
            continue;
        } else if (tokens[0].base == TokenBase::CloseBrace) {
            properties.push(PropertyLiteral {
                kind: "PropertyLiteral".to_string(),
                key,
                value: Statment::None()
            });
            continue;
        }

        expect(tokens, TokenBase::Colon, "Expected a colon while building an object property.".to_string());
        let value = parse_expr(tokens);

        properties.push(PropertyLiteral {
            kind: "PropertyLiteral".to_string(),
            value,
            key
        });
        if (tokens[0].base != TokenBase::CloseBrace) {
            expect(tokens, TokenBase::Comma, "Expected a comma or closing bracket while building an object property.".to_string());
        }
    }
    expect(tokens, TokenBase::CloseBrace, "Object litreal missing a closing brace.".to_string());

    Statment::ObjectLiteral(ObjectLiteral {
        kind: "ObjectLiteral".to_string(),
        properties
    })
}

pub fn parse_assignment_expr(tokens: &mut Vec<Token>) -> Statment {
    let left = parse_object_expr(tokens);

    if (tokens[0].base == TokenBase::Equals) {
        shift(tokens);
        let value = parse_assignment_expr(tokens);

        return Statment::AssignmentExpr(Box::new(AssignmentExpr {
            kind: "AssignmentExpr".to_string(),
            value,
            assigne: left,
        }));
    }

    left
}

pub fn parse_multiplicative_expr(tokens: &mut Vec<Token>) -> Statment {
    let mut left_stat = parse_call_member(tokens);

    while tokens.get(0).map_or(false, |token| {
        token.base == TokenBase::Asterick || token.base == TokenBase::Divide || token.base == TokenBase::Modulus
    }) {
        let operator = shift(tokens).unwrap();
        let right = parse_call_member(tokens);
        left_stat = Statment::BinaryExpr(Box::new(BinaryExpr {
            kind: "BinaryExpr".to_string(),
            left: left_stat,
            right,
            operator: operator.value,
        }));
    }

    left_stat
}

pub fn parse_additive_expr(tokens: &mut Vec<Token>) -> Statment {
    let mut left_stat = parse_multiplicative_expr(tokens);

    while tokens.get(0).map_or(false, |token| {
        token.base == TokenBase::Plus || token.base == TokenBase::Minus
    }) {
        let operator = shift(tokens).unwrap();
        let right = parse_multiplicative_expr(tokens);
        left_stat = Statment::BinaryExpr(Box::new(BinaryExpr {
            kind: "BinaryExpr".to_string(),
            left: left_stat,
            right,
            operator: operator.value,
        }));
    }

    left_stat
}

pub fn parse_primary_expr(tokens: &mut Vec<Token>) -> Statment {
    let token: Token = shift(tokens).unwrap_or(Token { value: "EndOfFile".to_string(), base: TokenBase::EoF });

    match token.base {
        TokenBase::Identifier => {
            Statment::Identifier(Identifier {
                kind: "Identifier".to_string(),
                symbol: token.value
            })
        }
        TokenBase::Number => {
            Statment::NumericLiteral(NumericLiteral {
                kind: "NumericLiteral".to_string(),
                value: token.value.parse::<f64>().unwrap_or(0.0)
            })
        }
        TokenBase::Null => {
            Statment::NullLiteral(NullLiteral {
                kind: "NullLiteral".to_string(),
                value: "null".to_string()
            })
        }
        TokenBase::String => {
            Statment::StringLiteral(StringLiteral {
                kind: "StringLiteral".to_string(),
                value: token.value
            })
        }
        _ => {
            log(LogLevel::Warn, format!("Unexpected token while parsing! {:#?}", token).as_str());
            Statment::None()
        }
    }
}

pub fn produce_ast(source_code: &String) -> Statment {
    let mut tokens = tokenize(source_code);
    let mut program: Program = Program { 
        body: vec![],
    };

    while (tokens[0].base != TokenBase::EoF) {
        program.body.push(parse_stmt(&mut tokens));
    }

    Statment::Program(program)
}