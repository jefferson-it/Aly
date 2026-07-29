// Aly XML Parser Module — parse, query, and manipulate XML data

use std::collections::HashMap;
use crate::native::std::{arg as std_arg, split_args};
use crate::native::types::{Validator, ValueData};

fn ok_str(s: impl Into<String>) -> Box<dyn Validator> {
    Box::new(ValueData::String(s.into()))
}

#[derive(Debug, Clone)]
struct XmlNode {
    tag: String,
    attributes: HashMap<String, String>,
    text: String,
    children: Vec<XmlNode>,
}

fn parse_xml_nodes(input: &str) -> Vec<XmlNode> {
    let mut nodes = Vec::new();
    let mut chars = input.chars().peekable();

    while chars.peek().is_some() {
        skip_whitespace(&mut chars);
        if chars.peek() == Some(&'<') {
            chars.next(); // consume '<'
            if chars.peek() == Some(&'/') {
                // closing tag — return to parent
                while chars.peek().is_some() && chars.peek() != Some(&'>') {
                    chars.next();
                }
                if chars.peek() == Some(&'>') { chars.next(); }
                return nodes;
            }
            if chars.peek() == Some(&'?') || chars.peek() == Some(&'!') {
                // processing instruction or comment — skip
                while chars.peek().is_some() && chars.peek() != Some(&'>') {
                    chars.next();
                }
                if chars.peek() == Some(&'>') { chars.next(); }
                continue;
            }
            // opening tag
            let tag = read_until_any(&mut chars, &[' ', '>', '/']);
            let mut attrs = HashMap::new();
            loop {
                skip_whitespace(&mut chars);
                match chars.peek() {
                    Some(&'>') => { chars.next(); break; }
                    Some(&'/') => {
                        chars.next();
                        if chars.peek() == Some(&'>') { chars.next(); }
                        nodes.push(XmlNode { tag, attributes: attrs, text: String::new(), children: Vec::new() });
                        break;
                    }
                    Some(_) => {
                        let key = read_until_any(&mut chars, &['=', ' ', '>', '/']);
                        if key.is_empty() { break; }
                        if chars.peek() == Some(&'=') {
                            chars.next();
                            let val = read_attr_value(&mut chars);
                            attrs.insert(key, val);
                        } else {
                            attrs.insert(key, String::new());
                        }
                    }
                    None => break,
                }
            }
            if chars.peek().is_none() && nodes.last().map(|n: &XmlNode| n.tag == tag).unwrap_or(false) {
                continue;
            }
            // collect text content
            let mut text = String::new();
            let mut children = Vec::new();
            loop {
                match chars.peek() {
                    Some(&'<') => {
                        // peek ahead to check for closing tag
                        let rest: String = chars.clone().collect();
                        if rest.starts_with(&format!("</{}", tag)) {
                            // closing tag for this element
                            while chars.peek().is_some() && chars.peek() != Some(&'>') {
                                chars.next();
                            }
                            if chars.peek() == Some(&'>') { chars.next(); }
                            break;
                        } else {
                            // child element — recurse
                            let sub: String = chars.clone().collect();
                            let sub_input = format!("<{}", &sub[1..]);
                            // parse one child manually
                            let child_nodes = parse_xml_nodes(&format!("<{}", &rest[1..]));
                            children.extend(child_nodes);
                            // advance past what we consumed
                            skip_past_matching_close(&mut chars);
                        }
                    }
                    Some(&c) => {
                        text.push(c);
                        chars.next();
                    }
                    None => break,
                }
            }
            nodes.push(XmlNode { tag, attributes: attrs, text: text.trim().to_string(), children });
        } else {
            chars.next();
        }
    }
    nodes
}

fn skip_whitespace(chars: &mut std::iter::Peekable<std::str::Chars>) {
    while chars.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
        chars.next();
    }
}

fn read_until_any(chars: &mut std::iter::Peekable<std::str::Chars>, stops: &[char]) -> String {
    let mut s = String::new();
    while let Some(&c) = chars.peek() {
        if stops.contains(&c) { break; }
        s.push(c);
        chars.next();
    }
    s
}

fn read_attr_value(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    skip_whitespace(chars);
    let quote = match chars.peek() {
        Some(&'"') | Some(&'\'') => { let q = *chars.peek().unwrap(); chars.next(); q }
        _ => return read_until_any(chars, &[' ', '>', '/']),
    };
    let mut val = String::new();
    while let Some(&c) = chars.peek() {
        if c == quote { chars.next(); break; }
        val.push(c);
        chars.next();
    }
    val
}

fn skip_past_matching_close(chars: &mut std::iter::Peekable<std::str::Chars>) {
    let mut depth = 0i32;
    while let Some(c) = chars.next() {
        if c == '<' {
            if chars.peek() == Some(&'/') { depth -= 1; }
            else if chars.peek() != Some(&'!') && chars.peek() != Some(&'?') { depth += 1; }
        }
        if c == '>' && depth < 0 { return; }
    }
}

fn node_to_string(node: &XmlNode, indent: usize) -> String {
    let pad = "  ".repeat(indent);
    let mut s = format!("{}<{}", pad, node.tag);
    for (k, v) in &node.attributes {
        s.push_str(&format!(" {}=\"{}\"", k, v));
    }
    if node.children.is_empty() && node.text.is_empty() {
        s.push_str("/>");
        return s;
    }
    s.push('>');
    if !node.text.is_empty() {
        s.push_str(&node.text);
    }
    for child in &node.children {
        s.push('\n');
        s.push_str(&node_to_string(child, indent + 1));
    }
    if !node.children.is_empty() {
        s.push('\n');
        s.push_str(&pad);
    }
    s.push_str(&format!("</{}>", node.tag));
    s
}

fn find_nodes_by_tag<'a>(nodes: &'a [XmlNode], tag: &str) -> Vec<&'a XmlNode> {
    let mut result = Vec::new();
    for node in nodes {
        if node.tag == tag { result.push(node); }
        result.extend(find_nodes_by_tag(&node.children, tag));
    }
    result
}

/// Parse XML string
pub fn xml_parse(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 1);
    let input = std_arg(&args, 0);
    let nodes = parse_xml_nodes(&input);
    let result: Vec<String> = nodes.iter().map(|n| node_to_string(n, 0)).collect();
    ok_str(result.join("\n"))
}

/// Convert key-value pairs to XML
pub fn xml_stringify(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let root_tag = std_arg(&args, 0);
    let keys_str = std_arg(&args, 1);
    let values_str = std_arg(&args, 2);
    let keys: Vec<&str> = keys_str.split(',').collect();
    let values: Vec<&str> = values_str.split(',').collect();
    let mut xml = format!("<{}>", root_tag);
    for (k, v) in keys.iter().zip(values.iter()) {
        xml.push_str(&format!("<{}>{}</{}>", k.trim(), v.trim(), k.trim()));
    }
    xml.push_str(&format!("</{}>", root_tag));
    ok_str(xml)
}

/// Get element by tag name
pub fn xml_get(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let input = std_arg(&args, 0);
    let tag = std_arg(&args, 1);
    let nodes = parse_xml_nodes(&input);
    let found = find_nodes_by_tag(&nodes, &tag);
    if let Some(node) = found.first() {
        ok_str(node_to_string(node, 0))
    } else {
        ok_str(format!("XML element '{}' not found", tag))
    }
}

/// Get attribute value
pub fn xml_attr(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let input = std_arg(&args, 0);
    let tag = std_arg(&args, 1);
    let attr_name = std_arg(&args, 2);
    let nodes = parse_xml_nodes(&input);
    let found = find_nodes_by_tag(&nodes, &tag);
    if let Some(node) = found.first() {
        if let Some(val) = node.attributes.get(&attr_name) {
            ok_str(val.clone())
        } else {
            ok_str(format!("Attribute '{}' not found on '{}'", attr_name, tag))
        }
    } else {
        ok_str(format!("XML element '{}' not found", tag))
    }
}

/// Get text content of element
pub fn xml_text(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let input = std_arg(&args, 0);
    let tag = std_arg(&args, 1);
    let nodes = parse_xml_nodes(&input);
    let found = find_nodes_by_tag(&nodes, &tag);
    if let Some(node) = found.first() {
        ok_str(node.text.clone())
    } else {
        ok_str(format!("XML element '{}' not found", tag))
    }
}

/// Count elements matching tag
pub fn xml_count(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let input = std_arg(&args, 0);
    let tag = std_arg(&args, 1);
    let nodes = parse_xml_nodes(&input);
    let found = find_nodes_by_tag(&nodes, &tag);
    ok_str(format!("{}", found.len()))
}
