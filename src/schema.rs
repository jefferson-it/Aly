mod schema {
    
    use crate::lexer::Lexer;
    use crate::native::types::ValueData;

    /// Definition of a Schema (class blueprint)
    #[derive(Clone)]
    pub struct SchemaDef {
        pub name: String,
        pub parent: Option<String>,
        /// Fields and their default value expressions (Vec<Lexer>)
        pub fields: Vec<(String, Vec<Lexer>)>,
        /// Methods: name -> (params, body_tokens)
        pub methods: Vec<(String, Vec<String>, Vec<Lexer>)>,
        /// Static methods: name -> (params, body_tokens)
        pub static_methods: Vec<(String, Vec<String>, Vec<Lexer>)>,
        /// Generics: name -> (params)
        pub generics: Vec<String>,
        /// Constructor body (custom init, called after field initialization)
        pub constructor_body: Option<Vec<Lexer>>,
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
                parent: None,
                fields: vec![],
                methods: vec![],
                static_methods: vec![],
                generics: vec![],
                constructor_body: None,
            }
        }

        pub fn add_field(&mut self, field_name: String, default_expr: Vec<Lexer>) {
            self.fields.push((field_name, default_expr));
        }

        pub fn add_method(&mut self, method_name: String, params: Vec<String>, body: Vec<Lexer>) {
            self.methods.push((method_name, params, body));
        }

        pub fn add_static_method(&mut self, method_name: String, params: Vec<String>, body: Vec<Lexer>) {
            self.static_methods.push((method_name, params, body));
        }

        /// Get method from this schema or parent schemas
        pub fn get_method(&self, name: &str, schemas: &[SchemaDef]) -> Option<(Vec<String>, Vec<Lexer>)> {
            for (mname, params, body) in &self.methods {
                if mname == name {
                    return Some((params.clone(), body.clone()));
                }
            }
            // Check parent
            if let Some(ref parent_name) = self.parent {
                if let Some(parent) = schemas.iter().find(|s| s.name == *parent_name) {
                    return parent.get_method(name, schemas);
                }
            }
            None
        }

        /// Get static method from this schema or parent schemas
        pub fn get_static_method(&self, name: &str, schemas: &[SchemaDef]) -> Option<(Vec<String>, Vec<Lexer>)> {
            for (mname, params, body) in &self.static_methods {
                if mname == name {
                    return Some((params.clone(), body.clone()));
                }
            }
            // Check parent
            if let Some(ref parent_name) = self.parent {
                if let Some(parent) = schemas.iter().find(|s| s.name == *parent_name) {
                    return parent.get_static_method(name, schemas);
                }
            }
            None
        }

        /// Get all field names including parent fields
        pub fn get_all_fields(&self, schemas: &[SchemaDef]) -> Vec<(String, Vec<Lexer>)> {
            let mut result = vec![];
            if let Some(ref parent_name) = self.parent {
                if let Some(parent) = schemas.iter().find(|s| s.name == *parent_name) {
                    result = parent.get_all_fields(schemas);
                }
            }
            for field in &self.fields {
                if !result.iter().any(|(n, _)| n == &field.0) {
                    result.push(field.clone());
                }
            }
            result
        }

        /// Get all methods including parent methods (own methods override)
        pub fn get_all_methods(&self, schemas: &[SchemaDef]) -> Vec<(String, Vec<String>, Vec<Lexer>)> {
            let mut result = vec![];
            if let Some(ref parent_name) = self.parent {
                if let Some(parent) = schemas.iter().find(|s| s.name == *parent_name) {
                    result = parent.get_all_methods(schemas);
                }
            }
            for method in &self.methods {
                // Remove any method with same name from parent (override)
                result.retain(|(n, _, _)| n != &method.0);
                result.push(method.clone());
            }
            result
        }

        /// Check if a field name is private (starts with _)
        pub fn is_private_field(name: &str) -> bool {
            name.starts_with('_')
        }
        /// Verifica se um método é estático
        pub fn is_static_method(&self, name: &str) -> bool {
            self.static_methods.iter().any(|(mname, _, _)| mname == name)
        }

        /// Executa método estático
        pub fn call_static_method(&self, name: &str, _params: Vec<ValueData>, schemas: &[SchemaDef]) -> ValueData {
            if let Some((_params, _body)) = self.get_static_method(name, schemas) {
                // Runtime execution logic would go here
                ValueData::String(format!("<static method {} executed>", name))
            } else {
                ValueData::String("None".to_string())
            }
        }
        pub fn get_destructor(&self, schemas: &[SchemaDef]) -> Option<Vec<Lexer>> {
            self.get_method("__drop__", schemas).map(|(_, body)| body)
        }
    }
}

pub use schema::*;