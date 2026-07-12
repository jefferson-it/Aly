mod uuid_mod {
    use crate::native::types::{Validator, ValueData};
    use crate::validators::str::put_quoted_str;

    // uuid.v4() -> "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
    pub fn uuid_v4(_x: String) -> Box<dyn Validator> {
        let id = uuid::Uuid::new_v4().to_string();
        Box::new(put_quoted_str(id))
    }
}

pub use uuid_mod::*;
