mod trait_system {
    

    #[derive(Clone)]
    pub struct TraitDef {
        pub name: String,
        pub methods: Vec<(String, Vec<String>)>, // name -> params
    }

    impl TraitDef {
        pub fn new(name: String) -> Self {
            TraitDef { name, methods: vec![] }
        }

        pub fn add_method(&mut self, name: String, params: Vec<String>) {
            self.methods.push((name, params));
        }

        pub fn is_implemented_by(&self, schema: &crate::schema::SchemaDef, schemas: &[crate::schema::SchemaDef]) -> bool {
            // Check if all methods of the trait exist in the schema (including inherited ones)
            let schema_methods = schema.get_all_methods(schemas);
            for (t_name, _) in &self.methods {
                if !schema_methods.iter().any(|(s_name, _, _)| s_name == t_name) {
                    return false;
                }
            }
            true
        }
    }
}

pub use trait_system::*;
