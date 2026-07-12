mod schema {
    use std::collections::HashMap;
    use crate::lexer::Lexer;

    /// Definition of a Schema (class blueprint)
    #[derive(Clone)]
    pub struct SchemaDef {
        pub name: String,
        /// Fields and their default value expressions (Vec<Lexer>)
        pub fields: Vec<(String, Vec<Lexer>)>,
        /// Methods: name -> (params, body_tokens)
        pub methods: Vec<(String, Vec<String>, Vec<Lexer>)>,
    }

    /// Instance of a Schema (an object created from a schema)
    /// Stored as an Object in ValueData for property access
    /// This is just metadata; actual field values are in the Object
    #[derive(Clone)]
    pub struct SchemaInstance {
        pub schema_name: String,
    }

    impl SchemaDef {
        pub fn new(name: String) -> Self {
            SchemaDef {
                name,
                fields: vec![],
                methods: vec![],
            }
        }

        pub fn add_field(&mut self, field_name: String, default_expr: Vec<Lexer>) {
            self.fields.push((field_name, default_expr));
        }

        pub fn add_method(&mut self, method_name: String, params: Vec<String>, body: Vec<Lexer>) {
            self.methods.push((method_name, params, body));
        }
    }
}

pub use schema::*;