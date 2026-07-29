mod pdf_mod {
    use std::collections::HashMap;

    use crate::native::create_object::Object;
    use crate::native::types::{Validator, ValueData};
    use crate::validators::str::remove_quoted_str;

    pub fn pdf_parse(x: String) -> Box<dyn Validator> {
        let input = if x.trim().starts_with('"') || x.trim().starts_with('\'') {
            remove_quoted_str(x.trim().to_string())
        } else {
            x.trim().to_string()
        };

        let bytes = input.as_bytes();

        if bytes.len() < 8 || &bytes[0..8] != b"%PDF-" {
            eprintln!("RuntimeError [pdf.parse]: not a valid PDF file (missing %PDF- header)");
            return Box::new(ValueData::Object(Object::new(vec![], HashMap::new())));
        }

        let mut obj = Object::new(vec![], HashMap::new());

        let version = String::from_utf8_lossy(&bytes[5..8]).to_string();
        obj.set_item("version".to_string(), ValueData::String(version));

        let text = extract_pdf_text(bytes);
        obj.set_item("text".to_string(), ValueData::String(text));

        let page_count = count_pdf_pages(bytes);
        obj.set_item("pages".to_string(), ValueData::Int(page_count));

        let metadata = extract_pdf_metadata(bytes);
        for (k, v) in metadata {
            obj.set_item(k, ValueData::String(v));
        }

        Box::new(ValueData::Object(obj))
    }

    pub fn pdf_info(x: String) -> Box<dyn Validator> {
        let input = if x.trim().starts_with('"') || x.trim().starts_with('\'') {
            remove_quoted_str(x.trim().to_string())
        } else {
            x.trim().to_string()
        };

        let bytes = input.as_bytes();

        if bytes.len() < 8 || &bytes[0..8] != b"%PDF-" {
            eprintln!("RuntimeError [pdf.info]: not a valid PDF file");
            return Box::new(ValueData::String("None".to_owned()));
        }

        let mut info = HashMap::new();

        let version = String::from_utf8_lossy(&bytes[5..8]).to_string();
        info.insert("version".to_string(), version);

        let page_count = count_pdf_pages(bytes);
        info.insert("pages".to_string(), page_count.to_string());

        let metadata = extract_pdf_metadata(bytes);
        for (k, v) in metadata {
            info.insert(k, v);
        }

        let mut obj = Object::new(vec![], HashMap::new());
        for (k, v) in info {
            obj.set_item(k, ValueData::String(v));
        }

        Box::new(ValueData::Object(obj))
    }

    fn extract_pdf_text(bytes: &[u8]) -> String {
        let mut text = String::new();

        let mut i = 0;
        while i < bytes.len() {
            if i + 4 <= bytes.len() && &bytes[i..i + 4] == b"BT" {
                let mut end = i + 2;
                while end + 3 <= bytes.len() && &bytes[end..end + 3] != b"ET" {
                    end += 1;
                }
                if end + 3 <= bytes.len() {
                    let content = &bytes[i..end + 3];
                    let content_str = String::from_utf8_lossy(content);
                    for part in content_str.split_whitespace() {
                        if part.starts_with('(') && part.ends_with(')') {
                            let inner = &part[1..part.len() - 1];
                            text.push_str(inner);
                            text.push(' ');
                        }
                    }
                }
                i = end + 3;
            } else {
                i += 1;
            }
        }

        text.trim().to_string()
    }

    fn count_pdf_pages(bytes: &[u8]) -> i32 {
        let mut count = 0;
        let mut i = 0;
        while i + 6 <= bytes.len() {
            if &bytes[i..i + 6] == b"/Type " {
                let rest = &bytes[i + 6..];
                let end = rest.iter().position(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b']' || b == b'}').unwrap_or(rest.len());
                let val = String::from_utf8_lossy(&rest[..end]);
                if val.trim() == "/Page" || val.trim() == "/Page\n" {
                    count += 1;
                }
            }
            i += 1;
        }
        count
    }

    fn extract_pdf_metadata(bytes: &[u8]) -> HashMap<String, String> {
        let mut metadata = HashMap::new();

        let content = String::from_utf8_lossy(bytes);

        let patterns = [
            ("/Title", "title"),
            ("/Author", "author"),
            ("/Subject", "subject"),
            ("/Creator", "creator"),
            ("/Producer", "producer"),
            ("/CreationDate", "created"),
            ("/ModDate", "modified"),
            ("/Keywords", "keywords"),
        ];

        for (pdf_key, meta_key) in patterns {
            if let Some(pos) = content.find(pdf_key) {
                let rest = &content[pos + pdf_key.len()..];
                let end = rest.find(|c: char| c == '\n' || c == '\r' || c == '/' || c == '>').unwrap_or(rest.len());
                let value = rest[..end].trim().trim_matches(|c: char| c == '(' || c == ')' || c == '<' || c == '>').trim();
                if !value.is_empty() {
                    metadata.insert(meta_key.to_string(), value.to_string());
                }
            }
        }

        metadata
    }

    pub fn pdf_text(x: String) -> Box<dyn Validator> {
        let input = if x.trim().starts_with('"') || x.trim().starts_with('\'') {
            remove_quoted_str(x.trim().to_string())
        } else {
            x.trim().to_string()
        };

        let bytes = input.as_bytes();

        if bytes.len() < 8 || &bytes[0..8] != b"%PDF-" {
            eprintln!("RuntimeError [pdf.text]: not a valid PDF file");
            return Box::new(ValueData::String("None".to_owned()));
        }

        let text = extract_pdf_text(bytes);
        Box::new(ValueData::String(text))
    }

    pub fn pdf_pages(x: String) -> Box<dyn Validator> {
        let input = if x.trim().starts_with('"') || x.trim().starts_with('\'') {
            remove_quoted_str(x.trim().to_string())
        } else {
            x.trim().to_string()
        };

        let bytes = input.as_bytes();

        if bytes.len() < 8 || &bytes[0..8] != b"%PDF-" {
            eprintln!("RuntimeError [pdf.pages]: not a valid PDF file");
            return Box::new(ValueData::Int(0));
        }

        let count = count_pdf_pages(bytes);
        Box::new(ValueData::Int(count))
    }
}

pub use pdf_mod::*;
