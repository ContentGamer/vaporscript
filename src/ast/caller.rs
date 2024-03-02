use crate::{lexer::lexer::{Token, TokenBase}, logger::logger::{log, LogLevel}, utils::shift};

use super::{ast::{CallExpr, MemberExpr, Statment}, parser::{expect, parse_assignment_expr, parse_expr, parse_primary_expr}};

pub fn parse_call_member(tokens: &mut Vec<Token>) -> Statment {
    let member = parse_member_expr(tokens);
    if (tokens[0].base == TokenBase::OpenParen) {
        return parse_call_expr(tokens, &member);
    }

    return member;
}

pub fn parse_call_expr(tokens: &mut Vec<Token>, caller: &Statment) -> Statment {
    let mut call: Statment = Statment::CallExpr(Box::new(CallExpr {
        kind: "CallExpr".to_string(),
        caller: caller.clone(),
        args: parse_args(tokens)
    }));

    if (tokens[0].base == TokenBase::OpenParen) {
        call = parse_call_expr(tokens, &call);
    }

    call
}

pub fn parse_args(tokens: &mut Vec<Token>) -> Vec<Statment> {
    expect(tokens, TokenBase::OpenParen, "Expected an open paranthesis while parsing caller arguments.".to_string());

    let args: Vec<Statment> = if (tokens[0].base == TokenBase::CloseParen) { 
        Vec::new() 
    } else { 
        parse_args_list(tokens) 
    };

    expect(tokens, TokenBase::CloseParen, "Expected a closing paranthesis while parsing caller arguments.".to_string()); 
    args   
}
fn parse_args_list(tokens: &mut Vec<Token>) -> Vec<Statment> {
    let mut args: Vec<Statment> = vec![parse_assignment_expr(tokens)];
    
    while (tokens[0].base == TokenBase::Comma && shift(tokens).is_some()) {
        args.push(parse_assignment_expr(tokens));
    }

    args
}

pub fn parse_member_expr(tokens: &mut Vec<Token>) -> Statment {
    let mut obj = parse_primary_expr(tokens);

    while (tokens[0].base == TokenBase::Dot || tokens[0].base == TokenBase::OpenBracket) {
        let mut property: Statment;
        let mut computed: bool;
        let operator = shift(tokens).unwrap_or(Token { value: "EndOfFile".to_string(), base: TokenBase::EoF });

        if (operator.base == TokenBase::Dot) {
            computed = false;
            property = parse_primary_expr(tokens);

            match property {
                Statment::Identifier(_) => {}
                _ => {
                    log(LogLevel::Error, "Cannot use the dot operator without the right hand expression being an identifier.");
                }
            }
        } else {
            computed = true;
            property = parse_expr(tokens);
            expect(tokens, TokenBase::CloseBracket, "Expected a closing bracket while trying to access an object key.".to_string());
        }

        obj = Statment::MemberExpr(Box::new(MemberExpr {
            kind: "MemberExpr".to_string(),
            object: obj,
            property,
            computed
        }));
    }

    obj
}