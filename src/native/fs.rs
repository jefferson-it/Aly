#![allow(unused)]

mod files {
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    pub fn read_file(path: String) -> String  {
        let file = match File::open(path) {
            Ok(it) => it,
            Err(err) => {
                println!("Error: {err}");

                return "".to_owned();
            }
        };
        
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        
        match buf_reader.read_to_string(&mut contents) {
            Ok(it) => it,
            Err(err) => {
                println!("Error: {err}");

                return "".to_owned();
            }
        };
        
        return contents;
    }
}

pub use files::*;