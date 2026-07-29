pub mod reflection {
    use crate::aly::Aly;
    use crate::native::types::ValueData;

    pub fn list_variables(aly: &Aly) -> Vec<String> {
        aly.get_all_var_names()
    }

    pub fn list_schemas(aly: &Aly) -> Vec<String> {
        aly.schemas.iter().map(|s| s.name.clone()).collect()
    }
}
pub use reflection::*;
