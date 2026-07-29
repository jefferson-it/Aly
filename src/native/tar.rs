mod tar_mod {
    use std::collections::HashMap;
    use std::io::{Cursor, Read};

    use crate::native::create_object::Object;
    use crate::native::types::{Validator, ValueData};
    use crate::native::vector::Vector;
    use crate::validators::str::remove_quoted_str;
    use crate::native::std::split_args;

    pub fn tar_parse(x: String) -> Box<dyn Validator> {
        let input = if x.trim().starts_with('"') || x.trim().starts_with('\'') {
            remove_quoted_str(x.trim().to_string())
        } else {
            x.trim().to_string()
        };

        let bytes = input.as_bytes();
        let mut cursor = Cursor::new(bytes);

        let mut files = Vec::new();

        loop {
            let mut header = [0u8; 512];
            match cursor.read_exact(&mut header) {
                Ok(_) => {}
                Err(_) => break,
            }

            let name = String::from_utf8_lossy(&header[0..100]).trim_end_matches('\0').to_string();
            if name.is_empty() {
                break;
            }

            let size_str = String::from_utf8_lossy(&header[124..136]).trim_end_matches('\0').trim().to_string();
            let size: u64 = size_str.parse().unwrap_or(0);

            let mut data = vec![0u8; size as usize];
            if size > 0 {
                match cursor.read_exact(&mut data) {
                    Ok(_) => {}
                    Err(_) => break,
                }
            }

            let padding = (512 - (size % 512)) % 512;
            if padding > 0 {
                let mut pad = vec![0u8; padding as usize];
                let _ = cursor.read_exact(&mut pad);
            }

            let mut file_obj = Object::new(vec![], HashMap::new());
            file_obj.set_item("name".to_string(), ValueData::String(name.clone()));
            file_obj.set_item("size".to_string(), ValueData::Int(size as i32));
            file_obj.set_item("data".to_string(), ValueData::String(String::from_utf8_lossy(&data).to_string()));

            let mode_str = String::from_utf8_lossy(&header[100..108]).trim_end_matches('\0').trim().to_string();
            file_obj.set_item("mode".to_string(), ValueData::String(mode_str));

            let mtime_str = String::from_utf8_lossy(&header[136..148]).trim_end_matches('\0').trim().to_string();
            file_obj.set_item("mtime".to_string(), ValueData::String(mtime_str));

            let typeflag = header[156];
            file_obj.set_item("type".to_string(), ValueData::String(String::from_utf8_lossy(&[typeflag]).to_string()));

            let linkname = String::from_utf8_lossy(&header[157..257]).trim_end_matches('\0').to_string();
            file_obj.set_item("linkname".to_string(), ValueData::String(linkname));

            files.push(ValueData::Object(file_obj));
        }

        Box::new(ValueData::Vec(Vector::new(files)))
    }

    pub fn tar_list(x: String) -> Box<dyn Validator> {
        let input = if x.trim().starts_with('"') || x.trim().starts_with('\'') {
            remove_quoted_str(x.trim().to_string())
        } else {
            x.trim().to_string()
        };

        let bytes = input.as_bytes();
        let mut cursor = Cursor::new(bytes);

        let mut names = Vec::new();

        loop {
            let mut header = [0u8; 512];
            match cursor.read_exact(&mut header) {
                Ok(_) => {}
                Err(_) => break,
            }

            let name = String::from_utf8_lossy(&header[0..100]).trim_end_matches('\0').to_string();
            if name.is_empty() {
                break;
            }

            let size_str = String::from_utf8_lossy(&header[124..136]).trim_end_matches('\0').trim().to_string();
            let size: u64 = size_str.parse().unwrap_or(0);

            let mut data = vec![0u8; size as usize];
            if size > 0 {
                match cursor.read_exact(&mut data) {
                    Ok(_) => {}
                    Err(_) => break,
                }
            }

            let padding = (512 - (size % 512)) % 512;
            if padding > 0 {
                let mut pad = vec![0u8; padding as usize];
                let _ = cursor.read_exact(&mut pad);
            }

            names.push(name);
        }

        let vec = Vector::new(names.into_iter().map(ValueData::String).collect());
        Box::new(ValueData::Vec(vec))
    }

    pub fn tar_extract(x: String) -> Box<dyn Validator> {
        let input = if x.trim().starts_with('"') || x.trim().starts_with('\'') {
            remove_quoted_str(x.trim().to_string())
        } else {
            x.trim().to_string()
        };

        let bytes = input.as_bytes();
        let mut cursor = Cursor::new(bytes);

        let mut extracted = Vec::new();

        loop {
            let mut header = [0u8; 512];
            match cursor.read_exact(&mut header) {
                Ok(_) => {}
                Err(_) => break,
            }

            let name = String::from_utf8_lossy(&header[0..100]).trim_end_matches('\0').to_string();
            if name.is_empty() {
                break;
            }

            let size_str = String::from_utf8_lossy(&header[124..136]).trim_end_matches('\0').trim().to_string();
            let size: u64 = size_str.parse().unwrap_or(0);

            let mut data = vec![0u8; size as usize];
            if size > 0 {
                match cursor.read_exact(&mut data) {
                    Ok(_) => {}
                    Err(_) => break,
                }
            }

            let padding = (512 - (size % 512)) % 512;
            if padding > 0 {
                let mut pad = vec![0u8; padding as usize];
                let _ = cursor.read_exact(&mut pad);
            }

            let mut file_obj = Object::new(vec![], HashMap::new());
            file_obj.set_item("name".to_string(), ValueData::String(name.clone()));
            file_obj.set_item("size".to_string(), ValueData::Int(size as i32));
            file_obj.set_item("data".to_string(), ValueData::String(String::from_utf8_lossy(&data).to_string()));

            extracted.push(ValueData::Object(file_obj));
        }

        Box::new(ValueData::Vec(Vector::new(extracted)))
    }

    pub fn tar_create(x: String) -> Box<dyn Validator> {
        let args: Vec<String> = x.split(|c: char| c == ' ' || c == '\t')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        let mut files = Vec::new();

        for arg in &args {
            let parts: Vec<&str> = arg.splitn(2, ':').collect();
            if parts.len() == 2 {
                let name = parts[0].trim().to_string();
                let content = parts[1].trim().to_string();
                files.push((name, content.into_bytes()));
            } else {
                let name = arg.trim().to_string();
                files.push((name, Vec::new()));
            }
        }

        let mut tar_data = Vec::new();

        for (name, data) in files {
            let mut header = [0u8; 512];

            let name_bytes = name.as_bytes();
            let name_len = name_bytes.len().min(100);
            header[0..name_len].copy_from_slice(&name_bytes[0..name_len]);

            let mode = b"0000644\0";
            header[100..108].copy_from_slice(mode);

            let uid = b"0000000\0";
            header[108..116].copy_from_slice(uid);

            let gid = b"0000000\0";
            header[116..124].copy_from_slice(gid);

            let size_str = format!("{:011o}\0", data.len());
            let size_bytes = size_str.as_bytes();
            header[124..136].copy_from_slice(&size_bytes[0..12]);

            let mtime = b"00000000000\0";
            header[136..148].copy_from_slice(mtime);

            let mut chksum = 0u32;
            for byte in &header {
                chksum += *byte as u32;
            }
            let chksum_str = format!("{:06o}\0 ", chksum);
            header[148..156].copy_from_slice(chksum_str.as_bytes());

            header[156] = b'0';

            tar_data.extend_from_slice(&header);
            tar_data.extend_from_slice(&data);

            let padding = (512 - (data.len() % 512)) % 512;
            for _ in 0..padding {
                tar_data.push(0);
            }
        }

        tar_data.extend_from_slice(&[0u8; 1024]);

        Box::new(crate::validators::str::put_quoted_str(String::from_utf8_lossy(&tar_data).to_string()))
    }
}

pub use tar_mod::*;
