mod path {
    use std::path::Path;

    pub fn validation_file(file: &str) -> String{
        let path = Path::new(file);
        let ext = path.extension().and_then(|ex| ex.to_str());

        let filename = match ext {
            Some(ex) => {
                if ex == "aly" {
                    return file.to_owned();
                } else {
                    let example = file.replace(ex, "aly");
                    eprintln!("ImportError [path]: '{}' não é um arquivo válido de extensão .aly! Tente algo como '{}'", file, example);
                    return example;
                }
            },
            None => {
                format!("{}.aly", file).to_string()
            }
        };
        
        String::from(filename)
    }
}

pub use path::*;