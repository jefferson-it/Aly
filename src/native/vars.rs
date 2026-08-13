mod vars {
    use core::fmt;
    

    use crate::{
        lexer::Lexer, native::types::{coerce, Type, Validator, ValueData}, tokens::Tokens
    };

    #[derive(Clone)]
    pub struct Var {
        name: String,
        value: ValueData,
        data_type: Type,
        mutable: bool,
        borrow_count: usize,
    }

    
    impl Var {
        pub fn new<T: Validator>(name: String, value: T, mut_: bool) -> Var {
            let (d_type, val) = value.valid();

            Var {
                name,
                mutable: mut_,
                value: val,
                data_type: d_type,
                borrow_count: 0,
            }
        }

        /// Cria uma variável com tipo fixo declarado (`let x : i8 = 5`),
        /// coagindo o valor inicial para o tipo alvo (com checagem de overflow).
        pub fn new_typed<T: Validator>(name: String, value: T, mut_: bool, ty: Type) -> Result<Var, String> {
            let (_, val) = value.valid();
            let coerced = coerce(val, &ty)?;
            Ok(Var {
                name,
                mutable: mut_,
                value: coerced,
                data_type: ty,
                borrow_count: 0,
            })
        }

        pub fn borrow(&mut self) -> Result<(), String> {
            self.borrow_count += 1;
            Ok(())
        }

        pub fn unborrow(&mut self) -> Result<(), String> {
            if self.borrow_count > 0 {
                self.borrow_count -= 1;
                Ok(())
            } else {
                Err(format!("Variable {} not borrowed", self.name))
            }
        }

        pub fn is_borrowed(&self) -> bool {
            self.borrow_count > 0
        }

        pub fn compare_var(&self, name: String) -> bool {
            &self.name == name.trim()
        }
        // Setters
        pub fn change_value<T: Validator>(&mut self, new_value: T) -> Result<(), String>{
            if self.is_borrowed() {
                return Err(format!("Variable {} is currently borrowed, cannot change value", self.name));
            }
            if !self.mutable {
                return Err(
                    String::from(
                        format!(
                            "The variable {} is constant, can't change your value",
                            self.name,
                        )
                    )
                );
            }
            let (d_type, val) = new_value.valid();
   
            match self.data_type {
                Type::None => {
                    self.data_type = d_type;
                    if let ValueData::Shared(rc) = &self.value {
                        *rc.borrow_mut() = val;
                    } else {
                        self.value = val;
                    }
                },
                _ => {
                    if self.data_type.to_string() == d_type.to_string() {
                        if let ValueData::Shared(rc) = &self.value {
                            *rc.borrow_mut() = val;
                        } else {
                            self.value = val;
                        }
                    } else {
                        return Err(
                            String::from(
                                format!(
                                    "The variable {} is a {}, {} is a {}, not a {}.",
                                    self.name,
                                    self.data_type,
                                    val.to_string(false),
                                    d_type,
                                    self.data_type
                                )
                            )
                        );
                    }
                }
            };
            
            Ok(())
        }

        pub fn set_prop(&mut self, prop_path: &[String], new_val: ValueData) -> Result<(), String> {
            self.value.set_property(prop_path, new_val)
        }

        pub fn in_mut(&mut self) -> Result<(), String> {
            if !self.mutable {
                return Err(
                    String::from(
                        format!(
                            "The variable {} is constant, can't change your mutability",
                            self.name,
                        )
                    )
                );
            }

            self.mutable = false;

            Ok(())
        }

        // Getters
        pub fn get_name(&self) -> String {
            self.name.clone()
        }

        pub fn get_value(&self) -> ValueData {
            match &self.value {
                ValueData::Shared(rc) => rc.borrow().clone(),
                v => v.clone(),
            }
        }

        /// Substituição interna sem checagens — usada para boxar a variável
        /// numa célula do heap quando `&nome` é tomado.
        pub fn set_raw_value(&mut self, value: ValueData) {
            self.value = value;
        }

                pub fn get_type(&self) -> &Type {
            &self.data_type
        }

        pub fn is_mutable(&self) -> bool {
            self.mutable
        }

        pub fn get_prop(&self, props: Vec<Lexer>) -> Box<dyn Validator>  {
            self.get_value().get_prop(self.mutable, props)
        }
    }

    impl fmt::Display for Var {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{} debug: Reference: {}, Value: {}, Type: {}",
                if self.mutable { "Variable" } else { "Constant" },
                self.name,
                self.value.to_string(true),
                self.data_type
            )
        }
    }

    pub fn is_var_declaration(tk: Tokens) -> bool {
        match tk {
            Tokens::Let | Tokens::Reference | Tokens::Identifier | Tokens::Value => true,
            _ => false,
        }
    }

    pub fn is_const_declaration(tk: Tokens) -> bool {
        match tk {
            Tokens::Const | Tokens::Reference | Tokens::Identifier | Tokens::Value => true,
            _ => false,
        }
    }
}

pub use vars::*;
