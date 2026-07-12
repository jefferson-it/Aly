mod gzip_mod {
    use std::io::{Read, Write};

    use crate::native::types::{Validator, ValueData};
    use crate::validators::str::put_quoted_str;
    use crate::native::std::{split_args, arg as std_arg};

    // gzip.compress(string) -> compressed bytes as string
    pub fn gzip_compress(x: String) -> Box<dyn Validator> {
        let input = std_arg(&split_args(&x, 1), 0);
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        match encoder.write_all(input.as_bytes()) {
            Ok(_) => {},
            Err(e) => panic!("gzip.compress error: {}", e),
        }
        let compressed = match encoder.finish() {
            Ok(c) => c,
            Err(e) => panic!("gzip.compress error: {}", e),
        };
        Box::new(put_quoted_str(String::from_utf8_lossy(&compressed).to_string()))
    }

    // gzip.decompress(bytes) -> decompressed string
    pub fn gzip_decompress(x: String) -> Box<dyn Validator> {
        let input = std_arg(&split_args(&x, 1), 0);
        let mut decoder = flate2::read::GzDecoder::new(input.as_bytes());
        let mut decoded = String::new();
        match decoder.read_to_string(&mut decoded) {
            Ok(_) => Box::new(put_quoted_str(decoded)),
            Err(e) => panic!("gzip.decompress error: {}", e),
        }
    }
}

pub use gzip_mod::*;
