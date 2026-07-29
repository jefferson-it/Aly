// Aly INI Parser Module — parse, query, and manipulate INI data

use std::collections::HashMap;
use crate::native::std::{arg as std_arg, split_args};
use crate::native::types::{Validator, ValueData};

fn ok_str(s: impl Into<String>) -> Box<dyn Validator> {
    Box::new(ValueData::String(s.into()))
}

#[derive(Debug, Clone)]
struct IniData {
    sections: Vec<(String, Vec<(String, String)>)>,
}

impl IniData {
    fn new() -> Self { IniData { sections: Vec::new() } }

    fn get_section(&self, section: &str) -> Option<&Vec<(String, String)>> {
        self.sections.iter().find(|(s, _)| s == section).map(|(_, kv)| kv)
    }

    fn get_section_mut(&mut self, section: &str) -> &mut Vec<(String, String)> {
        if !self.sections.iter().any(|(s, _)| s == section) {
            self.sections.push((section.to_string(), Vec::new()));
        }
        &mut self.sections.iter_mut().find(|(s, _)| s == section).unwrap().1
    }

    fn get(&self, section: &str, key: &str) -> Option<&str> {
        self.get_section(section)
            .and_then(|kv| kv.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str()))
    }

    fn set(&mut self, section: &str, key: &str, value: &str) {
        let kv = self.get_section_mut(section);
        if let Some(entry) = kv.iter_mut().find(|(k, _)| k == key) {
            entry.1 = value.to_string();
        } else {
            kv.push((key.to_string(), value.to_string()));
        }
    }

    fn to_ini(&self) -> String {
        let mut out = String::new();
        for (section, pairs) in &self.sections {
            if !section.is_empty() {
                out.push_str(&format!("[{}]\n", section));
            }
            for (k, v) in pairs {
                out.push_str(&format!("{}={}\n", k, v));
            }
            out.push('\n');
        }
        out.trim_end().to_string()
    }
}

fn parse_ini(input: &str) -> IniData {
    let mut data = IniData::new();
    let mut current_section = String::new();

    for line in input.lines() {
        let trimmed = line.trim();

        // skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with('#') {
            continue;
        }

        // section header
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            current_section = trimmed[1..trimmed.len() - 1].trim().to_string();
            continue;
        }

        // key=value or key = value
        if let Some(eq_pos) = trimmed.find('=') {
            let key = trimmed[..eq_pos].trim().to_string();
            let value = trimmed[eq_pos + 1..].trim().to_string();
            // strip inline comments
            let value = if let Some(comment_pos) = value.find(" ;") {
                value[..comment_pos].trim().to_string()
            } else if let Some(comment_pos) = value.find(" #") {
                value[..comment_pos].trim().to_string()
            } else {
                value
            };
            // unquote
            let value = if (value.starts_with('"') && value.ends_with('"'))
                || (value.starts_with('\'') && value.ends_with('\''))
            {
                value[1..value.len() - 1].to_string()
            } else {
                value
            };
            data.set(&current_section, &key, &value);
        }
    }
    data
}

/// Parse INI string
pub fn ini_parse(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 1);
    let input = std_arg(&args, 0);
    let data = parse_ini(&input);
    ok_str(data.to_ini())
}

/// Convert key-value data to INI format
pub fn ini_stringify(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let section = std_arg(&args, 0);
    let keys_str = std_arg(&args, 1);
    let values_str = std_arg(&args, 2);
    let keys: Vec<&str> = keys_str.split(',').collect();
    let values: Vec<&str> = values_str.split(',').collect();
    let mut data = IniData::new();
    for (k, v) in keys.iter().zip(values.iter()) {
        data.set(&section, k.trim(), v.trim());
    }
    ok_str(data.to_ini())
}

/// Get value from INI (args: ini_string, section, key)
pub fn ini_get(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let input = std_arg(&args, 0);
    let section = std_arg(&args, 1);
    let key = std_arg(&args, 2);
    let data = parse_ini(&input);
    match data.get(&section, &key) {
        Some(val) => ok_str(val.to_string()),
        None => ok_str(format!("INI key '{}' not found in section '[{}]'", key, section)),
    }
}

/// Set value in INI (args: ini_string, section, key, value)
pub fn ini_set(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 4);
    let input = std_arg(&args, 0);
    let section = std_arg(&args, 1);
    let key = std_arg(&args, 2);
    let value = std_arg(&args, 3);
    let mut data = parse_ini(&input);
    data.set(&section, &key, &value);
    ok_str(data.to_ini())
}

/// List all sections
pub fn ini_sections(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 1);
    let input = std_arg(&args, 0);
    let data = parse_ini(&input);
    let sections: Vec<String> = data.sections.iter().map(|(s, _)| s.clone()).collect();
    ok_str(sections.join(", "))
}

/// List keys in a section
pub fn ini_keys(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let input = std_arg(&args, 0);
    let section = std_arg(&args, 1);
    let data = parse_ini(&input);
    match data.get_section(&section) {
        Some(pairs) => {
            let keys: Vec<String> = pairs.iter().map(|(k, _)| k.clone()).collect();
            ok_str(keys.join(", "))
        }
        None => ok_str(format!("INI section '[{}]' not found", section)),
    }
}
