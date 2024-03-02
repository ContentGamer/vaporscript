// Author: ContentGamer
// let x = 4 * ( 4 + 4 ) / 1 - 2

use alloc::{string::{String, ToString}, vec::Vec};
use core::clone;
use std::collections::HashMap;

use crate::{logger::logger::{log, LogLevel}, utils::{escape_seq, isalphabet, isempty, isint, shift}};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenBase {
    Comma,
    Colon,
    Semicolon,
    Dot,
    Hashtag,
    
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    
    Plus,
    Minus,
    Divide,
    Modulus,
    Asterick,
    Equals,
    
    Let,
    Const,
    
    Function,
    Number,
    String,
    Identifier,
    EoF,
    Null,

    IfCondition,
    ElseCondition,
    ElseIfCondition,
    ExclamationMark,
    QuestionMark,
    ToKeyword,

    ForLoop,
    WhileLoop,
    BreakLoop,
    ContinueLoop
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub value: String,
    pub base: TokenBase
}

fn token(value: &str, base: TokenBase) -> Token {
    Token { value: String::from(value), base }
}

fn get_keywords() -> HashMap<String, TokenBase> {
    let mut keywords = HashMap::new();

    keywords.insert("let".to_string(), TokenBase::Let);
    keywords.insert("const".to_string(), TokenBase::Const);
    keywords.insert("null".to_string(), TokenBase::Null);

    keywords.insert("fn".to_string(), TokenBase::Function);

    keywords.insert("if".to_string(), TokenBase::IfCondition);
    keywords.insert("else".to_string(), TokenBase::ElseCondition);
    keywords.insert("else if".to_string(), TokenBase::ElseIfCondition);

    keywords.insert("to".to_string(), TokenBase::ToKeyword);

    keywords
}

pub fn tokenize(source_code: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut code: Vec<String> = source_code.split("").map(|s| s.to_string()).collect::<Vec<String>>();

    while (code.len() > 0) {
        if (code[0] == "*") {
            tokens.push(token(shift(&mut code).unwrap_or_default().as_str(), TokenBase::Asterick));
        } else if (code[0] == "+") {
            tokens.push(token(shift(&mut code).unwrap_or_default().as_str(), TokenBase::Plus));
        } else if (code[0] == "-") {
            tokens.push(token(shift(&mut code).unwrap_or_default().as_str(), TokenBase::Minus));
        } else if (code[0] == "/") {
            tokens.push(token(shift(&mut code).unwrap_or_default().as_str(), TokenBase::Divide));
        } else if (code[0] == "%") {
            tokens.push(token(shift(&mut code).unwrap_or_default().as_str(), TokenBase::Modulus));
        } else if (code[0] == "=") {
            tokens.push(token(shift(&mut code).unwrap_or_default().as_str(), TokenBase::Equals))
        }
        
        else if (code[0] == "(") {
            tokens.push(token(shift(&mut code).unwrap_or_default().as_str(), TokenBase::OpenParen))
        } else if (code[0] == ")") {
            tokens.push(token(shift(&mut code).unwrap_or_default().as_str(), TokenBase::CloseParen))
        }
        else if (code[0] == "{") {
            tokens.push(token(shift(&mut code).unwrap_or_default().as_str(), TokenBase::OpenBrace))
        } else if (code[0] == "}") {
            tokens.push(token(shift(&mut code).unwrap_or_default().as_str(), TokenBase::CloseBrace))
        }        
        else if (code[0] == "[") {
            tokens.push(token(shift(&mut code).unwrap_or_default().as_str(), TokenBase::OpenBracket))
        } else if (code[0] == "]") {
            tokens.push(token(shift(&mut code).unwrap_or_default().as_str(), TokenBase::CloseBracket))
        }

        else if (code[0] == ":") {
            tokens.push(token(shift(&mut code).unwrap_or_default().as_str(), TokenBase::Colon))
        } else if (code[0] == ",") {
            tokens.push(token(shift(&mut code).unwrap_or_default().as_str(), TokenBase::Comma))
        } else if (code[0] == ";") {
            tokens.push(token(shift(&mut code).unwrap_or_default().as_str(), TokenBase::Semicolon))
        } else if (code[0] == ".") {
            tokens.push(token(shift(&mut code).unwrap_or_default().as_str(), TokenBase::Dot))
        }

        else if (code[0] == "!") {
            tokens.push(token(shift(&mut code).unwrap_or_default().as_str(), TokenBase::ExclamationMark))
        } else if (code[0] == "?") {
            tokens.push(token(shift(&mut code).unwrap_or_default().as_str(), TokenBase::QuestionMark))
        }

        else if (code[0] == "#") {
            shift(&mut code);
            while (code[0] != "\n") {
                shift(&mut code);
            }
            shift(&mut code);
        }

        else if (code[0] == '"'.to_string()) {
            let mut string: String = String::new();
            shift(&mut code);

            while (code[0].len() > 0 && code[0] != '"'.to_string()) {
                string += code[0].as_str();
                shift(&mut code);
            }
            shift(&mut code);
            
            tokens.push(token(escape_seq(string.as_str()).as_str(), TokenBase::String))
        }

        else {
            if (isint(code[0].clone())) {
                let mut num: String = String::from("");
                while (code.len() > 0 && isint(code[0].clone())) {
                    num += shift(&mut code).unwrap_or_default().as_str();
                }

                tokens.push(token(num.as_str(), TokenBase::Number));
            }
            else if (isempty(code[0].clone())) {
                shift(&mut code);
            }
            else if (isalphabet(&code[0].clone())) {
                let keywords: HashMap<String, TokenBase> = get_keywords();
                let mut ident: String = String::from("");

                while (code.len() > 0 && isalphabet(&code[0].clone())) {
                    ident += shift(&mut code).unwrap_or_default().as_str();
                }

                match &keywords.get(&ident) {
                    Some(keyword) => {
                        let inner = *keyword;
                        tokens.push(token(&ident, inner.clone()));
                    }
                    None => {
                        tokens.push(token(&ident, TokenBase::Identifier));
                    }
                }
            }
            else {
                log(LogLevel::Error, format!("Unregonized token found in source code: {}", code[0]).as_str());
            }
        }
    }

    tokens.push(token("EndOfFile", TokenBase::EoF));
    tokens
}