// Author: ContentGamer

use std::{any::{Any, TypeId}, collections::HashMap, process::Command};

use alloc::{string::{String, ToString}, vec::Vec};

use crate::runtime::values::{BooleanVal, NumberVal, StringVal, ValueType};

pub fn isint(character: String) -> bool {
    let valid = character.parse::<i32>();
    match valid {
        Ok(_) => true,
        Err(_) => false
    }
}
pub fn isalphabet(character: &String) -> bool {
    character.to_uppercase() != character.to_lowercase() || character == "_"
}
pub fn isempty(character: String) -> bool {
    character == "" || character == " " || character == "\t" || character == "\n" || character == "\r"
}

pub fn shift<T>(array: &mut Vec<T>) -> Option<T> {
    let remove = array.remove(0);
    Some(remove)
}

pub fn parse_value(value: &dyn Any) -> ValueType {
    match value.type_id() {
        typeid if typeid == TypeId::of::<f64>() => {
            ValueType::Number(NumberVal {
                base: "NumberVal".to_string(),
                value: value.downcast_ref::<f64>().unwrap_or(&0.0).clone()
            })
        }
        typeid if typeid == TypeId::of::<String>() => {
            ValueType::String(StringVal {
                base: "StringVal".to_string(),
                value: value.downcast_ref::<String>().unwrap_or(&"None".to_string()).clone()
            })
        }
        typeid if typeid == TypeId::of::<bool>() => {
            ValueType::Boolean(BooleanVal {
                base: "BooleanVal".to_string(),
                value: value.downcast_ref::<bool>().unwrap_or(&true).clone()
            })
        }

        _ => {
            ValueType::None()
        }
    }
}

pub fn clear_terminal() {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd").arg("/c").arg("cls").status();
    }
    #[cfg(not(target_os = "windows"))]
    {
        Command::new("clear");
    }
}

pub fn escape_seq(raw_str: &str) -> String {
    let mut seq_n: String = String::new();
    let mut chars = raw_str.chars();
    while let Some(ch) = chars.next() {
        if (ch == '\\')
        {
            match chars.next() {
                Some('n') => seq_n.push('\n'),
                Some('t') => seq_n.push('\t'),
                Some('r') => seq_n.push('\r'),
                Some('"') => seq_n.push('"'),
                Some('\\') => seq_n.push('\\'),
                _ => seq_n.push('\\')
            }
        } else {
            seq_n.push(ch);
        }
    }
    seq_n
}