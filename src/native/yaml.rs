// Aly YAML Parser Module — parse, query, and manipulate YAML data

use std::collections::HashMap;
use crate::native::std::{arg as std_arg, split_args};
use crate::native::types::{Validator, ValueData};

fn ok_str(s: impl Into<String>) -> Box<dyn Validator> {
    Box::new(ValueData::String(s.into()))
}

#[derive(Debug, Clone)]
enum YamlValue {
    Scalar(String),
    List(Vec<YamlValue>),
    Map(Vec<(String, YamlValue)>),
}

impl YamlValue {
    fn as_str(&self) -> String {
        match self {
            YamlValue::Scalar(s) => s.clone(),
            YamlValue::List(items) => {
                let strs: Vec<String> = items.iter().map(|v| v.as_str()).collect();
                format!("[{}]", strs.join(", "))
            }
            YamlValue::Map(pairs) => {
                let strs: Vec<String> = pairs.iter()
                    .map(|(k, v)| format!("{}: {}", k, v.as_str()))
                    .collect();
                format!("{{{}}}", strs.join(", "))
            }
        }
    }

    fn to_yaml(&self, indent: usize) -> String {
        let pad = "  ".repeat(indent);
        match self {
            YamlValue::Scalar(s) => s.clone(),
            YamlValue::List(items) => {
                items.iter()
                    .map(|v| format!("{}- {}", pad, v.to_yaml(indent + 1)))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
            YamlValue::Map(pairs) => {
                pairs.iter()
                    .map(|(k, v)| {
                        match v {
                            YamlValue::Scalar(s) => format!("{}{}: {}", pad, k, s),
                            _ => format!("{}{}:\n{}", pad, k, v.to_yaml(indent + 1)),
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        }
    }
}

fn parse_yaml(input: &str) -> YamlValue {
    let lines: Vec<&str> = input.lines().collect();
    let (val, _) = parse_yaml_block(&lines, 0, 0);
    val
}

fn parse_yaml_block(lines: &[&str], start: usize, min_indent: usize) -> (YamlValue, usize) {
    let mut pairs: Vec<(String, YamlValue)> = Vec::new();
    let mut list_items: Vec<YamlValue> = Vec::new();
    let mut is_list = false;
    let mut i = start;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        // skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            i += 1;
            continue;
        }

        let indent = line.len() - line.trim_start().len();
        if indent < min_indent && i > start {
            break;
        }

        if trimmed.starts_with("- ") {
            is_list = true;
            let content = trimmed[2..].trim();
            if content.contains(": ") {
                // list item is a map
                let (key, val) = split_kv(content);
                let mut sub_pairs = vec![(key, YamlValue::Scalar(val))];
                // look for continuation at deeper indent
                let next_indent = indent + 2;
                let mut j = i + 1;
                while j < lines.len() {
                    let nl = lines[j].trim();
                    if nl.is_empty() || nl.starts_with('#') { j += 1; continue; }
                    let ni = lines[j].len() - lines[j].trim_start().len();
                    if ni < next_indent { break; }
                    if nl.contains(": ") {
                        let (k, v) = split_kv(nl);
                        sub_pairs.push((k, YamlValue::Scalar(v)));
                    }
                    j += 1;
                }
                list_items.push(YamlValue::Map(sub_pairs));
                i = j;
            } else {
                list_items.push(YamlValue::Scalar(unquote(content)));
                i += 1;
            }
        } else if trimmed.contains(": ") || trimmed.ends_with(':') {
            let (key, val_str) = split_kv(trimmed);
            if val_str.is_empty() {
                // block value — parse indented block
                let next_indent = indent + 2;
                let (child, consumed) = parse_yaml_block(lines, i + 1, next_indent);
                pairs.push((key, child));
                i = consumed;
            } else {
                pairs.push((key, YamlValue::Scalar(unquote(&val_str))));
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    if is_list {
        (YamlValue::List(list_items), i)
    } else if !pairs.is_empty() {
        (YamlValue::Map(pairs), i)
    } else {
        (YamlValue::Scalar(String::new()), i)
    }
}

fn split_kv(s: &str) -> (String, String) {
    if let Some(idx) = s.find(": ") {
        (s[..idx].trim().to_string(), s[idx + 2..].trim().to_string())
    } else if s.ends_with(':') {
        (s[..s.len() - 1].trim().to_string(), String::new())
    } else {
        (s.to_string(), String::new())
    }
}

fn unquote(s: &str) -> String {
    let s = s.trim();
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

fn find_key<'a>(val: &'a YamlValue, key: &str) -> Option<&'a YamlValue> {
    match val {
        YamlValue::Map(pairs) => {
            // support dotted keys: "a.b.c"
            let parts: Vec<&str> = key.splitn(2, '.').collect();
            for (k, v) in pairs {
                if k == parts[0] {
                    if parts.len() == 1 {
                        return Some(v);
                    } else {
                        return find_key(v, parts[1]);
                    }
                }
            }
            None
        }
        _ => None,
    }
}

/// Parse YAML string
pub fn yaml_parse(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 1);
    let input = std_arg(&args, 0);
    let val = parse_yaml(&input);
    ok_str(val.as_str())
}

/// Convert key-value pairs to YAML format
pub fn yaml_stringify(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let keys_str = std_arg(&args, 0);
    let values_str = std_arg(&args, 1);
    let keys: Vec<&str> = keys_str.split(',').collect();
    let values: Vec<&str> = values_str.split(',').collect();
    let pairs: Vec<(String, YamlValue)> = keys.iter().zip(values.iter())
        .map(|(k, v)| (k.trim().to_string(), YamlValue::Scalar(v.trim().to_string())))
        .collect();
    let val = YamlValue::Map(pairs);
    ok_str(val.to_yaml(0))
}

/// Get value by key (supports dotted paths like "server.port")
pub fn yaml_get(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let input = std_arg(&args, 0);
    let key = std_arg(&args, 1);
    let val = parse_yaml(&input);
    if let Some(found) = find_key(&val, &key) {
        ok_str(found.as_str())
    } else {
        ok_str(format!("YAML key '{}' not found", key))
    }
}

/// Set value for key
pub fn yaml_set(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let input = std_arg(&args, 0);
    let key = std_arg(&args, 1);
    let value = std_arg(&args, 2);
    let mut val = parse_yaml(&input);
    set_key(&mut val, &key, &value);
    ok_str(val.to_yaml(0))
}

fn set_key(val: &mut YamlValue, key: &str, value: &str) {
    match val {
        YamlValue::Map(pairs) => {
            let parts: Vec<&str> = key.splitn(2, '.').collect();
            for (k, v) in pairs.iter_mut() {
                if k == parts[0] {
                    if parts.len() == 1 {
                        *v = YamlValue::Scalar(value.to_string());
                    } else {
                        set_key(v, parts[1], value);
                    }
                    return;
                }
            }
            pairs.push((parts[0].to_string(), YamlValue::Scalar(value.to_string())));
        }
        _ => {}
    }
}

/// List all top-level keys
pub fn yaml_keys(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 1);
    let input = std_arg(&args, 0);
    let val = parse_yaml(&input);
    match val {
        YamlValue::Map(pairs) => {
            let keys: Vec<String> = pairs.iter().map(|(k, _)| k.clone()).collect();
            ok_str(keys.join(", "))
        }
        _ => ok_str("Not a YAML mapping".to_string()),
    }
}
