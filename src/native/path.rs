mod path_lib {
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};

    use crate::native::create_object::Object;
    use crate::native::std::{arg, split_args};
    use crate::native::types::{Validator, ValueData};
    use crate::validators::str::put_quoted_str;

    fn ok_str(s: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str(s))
    }

    // path.join(...segments) -> string
    pub fn path_join(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 0);
        let parts: Vec<String> = (0..args.len())
            .map(|i| arg(&args, i))
            .filter(|s| !s.is_empty())
            .collect();
        let path: PathBuf = parts.iter().collect();
        ok_str(path.display().to_string())
    }

    // path.dirname(path) -> string
    pub fn path_dirname(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let p = arg(&args, 0);
        match Path::new(&p).parent() {
            Some(dir) => ok_str(dir.display().to_string()),
            None => ok_str(String::new()),
        }
    }

    // path.basename(path) -> string
    pub fn path_basename(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let p = arg(&args, 0);
        match Path::new(&p).file_name() {
            Some(name) => ok_str(name.to_string_lossy().to_string()),
            None => ok_str(String::new()),
        }
    }

    // path.extname(path) -> string
    pub fn path_extname(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let p = arg(&args, 0);
        match Path::new(&p).extension() {
            Some(ext) => ok_str(format!(".{}", ext.to_string_lossy())),
            None => ok_str(String::new()),
        }
    }

    // path.resolve(...segments) -> absolute path
    pub fn path_resolve(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 0);
        let parts: Vec<String> = (0..args.len()).map(|i| arg(&args, i)).collect();
        let mut path = if parts.is_empty() || Path::new(&parts[0]).is_relative() {
            std::env::current_dir().unwrap_or_default()
        } else {
            PathBuf::new()
        };
        for part in parts {
            path.push(part);
        }
        ok_str(path.display().to_string())
    }

    // path.parse(path) -> object { root, dir, base, ext, name }
    pub fn path_parse(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let p = arg(&args, 0);
        let path = Path::new(&p);

        let mut obj = Object::new(vec![], HashMap::new());

        let root = path
            .components()
            .next()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .unwrap_or_default();
        obj.set_item("root".to_owned(), ValueData::String(root));

        let dir = path
            .parent()
            .map(|d| d.display().to_string())
            .unwrap_or_default();
        obj.set_item("dir".to_owned(), ValueData::String(dir));

        let base = path
            .file_name()
            .map(|b| b.to_string_lossy().to_string())
            .unwrap_or_default();

        let ext = path
            .extension()
            .map(|e| format!(".{}", e.to_string_lossy()))
            .unwrap_or_default();

        let name = if base.is_empty() {
            String::new()
        } else if ext.is_empty() {
            base.clone()
        } else {
            base.strip_suffix(&ext).unwrap_or(&base).to_string()
        };

        obj.set_item("base".to_owned(), ValueData::String(base));
        obj.set_item("ext".to_owned(), ValueData::String(ext));
        obj.set_item("name".to_owned(), ValueData::String(name));

        Box::new(ValueData::Object(obj))
    }
}

pub use path_lib::*;
