pub mod module_system {
    use std::collections::HashMap;
    use crate::native::types::ValueData;

    #[derive(Clone)]
    pub struct Module {
        pub name: String,
        pub public_vars: HashMap<String, ValueData>,
        pub private_vars: HashMap<String, ValueData>,
    }

    impl Module {
        pub fn new(name: String) -> Self {
            Module {
                name,
                public_vars: HashMap::new(),
                private_vars: HashMap::new(),
            }
        }

        pub fn export(&mut self, name: String, value: ValueData) {
            self.public_vars.insert(name, value);
        }
    }
}
pub use module_system::*;
