mod reference {
    use regex::Regex;

    pub fn is_reference(item: &str) -> bool {
        let re = Regex::new(r"^[^\d][\w\d]*$").unwrap();

        re.is_match(item)
    }
}

pub use reference::*;