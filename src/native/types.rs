mod types {
    use core::fmt;
    use std::rc::Rc;
    use std::cell::RefCell;

    use crate::{lexer::Lexer, native::{create_object::Object, enumeration::Enumeration, linked_list::LinkedList, queue::Queue, set::Set, stack::Stack, tuple::Tuple, vector::Vector}, validators::{conversor_to_bool, conversor_to_float, conversor_to_int, is_bool, is_num, numeric::{is_float, is_int}, str::{is_any_str, put_quoted_str, remove_quoted_str}}};

    #[derive(Clone, PartialEq, Debug)]
    pub enum Type {
        Int,
        Float,
        String,
        Bool,
        Char,
        Void,
        Vec,
        Tuple,
        Set,
        Stack,
        Queue,
        LinkedList,
        Enum,
        Optional(Box<Type>),
        Union(Vec<Type>),
        Result,
        Obj,
        None,
        Struct(String),
        Model(String),
        Function,
        NativeFunction,
        PluginFunction,
        Shared(Box<Type>),
        Fixed(FixedIntTy),
        Pointer(Box<Type>),
    }

    impl fmt::Display for Type {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let res = match self {
                Type::Int => "int".to_string(),
                Type::Float => "float".to_string(),
                Type::String => "string".to_string(),
                Type::Bool => "boolean".to_string(),
                Type::Char => "char".to_string(),
                Type::Void => "void".to_string(),
                Type::Vec => "vector".to_string(),
                Type::Tuple => "tuple".to_string(),
                Type::Set => "set".to_string(),
                Type::Stack => "stack".to_string(),
                Type::Queue => "queue".to_string(),
                Type::LinkedList => "linkedlist".to_string(),
                Type::Enum => "enum".to_string(),
                Type::Optional(t) => format!("{}?", t),
                Type::Union(ts) => format!("({})", ts.iter().map(|t| t.to_string()).collect::<Vec<_>>().join("|")),
                Type::Shared(t) => format!("Shared<{}>", t),
                Type::Result => "result".to_string(),
                Type::None => "None".to_string(),
                Type::Fixed(t) => format!("{}", t),
                Type::Pointer(t) => {
                    if **t == Type::None {
                        "ptr".to_string()
                    } else {
                        format!("ptr<{}>", t)
                    }
                },

                Type::Obj => "obj".to_string(),
                Type::Struct(s) => format!("Struct({})", s),
                Type::Model(m) => format!("Model({})", m),
                Type::Function => "Function".to_string(),
                Type::NativeFunction => "NativeFunction".to_string(),
                Type::PluginFunction => "PluginFunction".to_string(),
            };

            write!(f, "{}", res)
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum FixedIntTy {
        I8,
        I16,
        I32,
        I64,
        U8,
        U16,
        U32,
        U64,
        F32,
        F64,
    }

    impl FixedIntTy {
        pub fn from_str(s: &str) -> Option<FixedIntTy> {
            match s.trim() {
                "i8" => Some(FixedIntTy::I8),
                "i16" => Some(FixedIntTy::I16),
                "i32" => Some(FixedIntTy::I32),
                "i64" => Some(FixedIntTy::I64),
                "u8" => Some(FixedIntTy::U8),
                "u16" => Some(FixedIntTy::U16),
                "u32" => Some(FixedIntTy::U32),
                "u64" => Some(FixedIntTy::U64),
                "f32" => Some(FixedIntTy::F32),
                "f64" => Some(FixedIntTy::F64),
                _ => None,
            }
        }

        pub fn bits(&self) -> u32 {
            match self {
                FixedIntTy::I8 | FixedIntTy::U8 => 8,
                FixedIntTy::I16 | FixedIntTy::U16 => 16,
                FixedIntTy::I32 | FixedIntTy::U32 | FixedIntTy::F32 => 32,
                FixedIntTy::I64 | FixedIntTy::U64 | FixedIntTy::F64 => 64,
            }
        }

        pub fn is_signed(&self) -> bool {
            matches!(self, FixedIntTy::I8 | FixedIntTy::I16 | FixedIntTy::I32 | FixedIntTy::I64)
        }

        pub fn is_float(&self) -> bool {
            matches!(self, FixedIntTy::F32 | FixedIntTy::F64)
        }

        pub fn min_int(&self) -> i64 {
            match self {
                FixedIntTy::I8 => i8::MIN as i64,
                FixedIntTy::I16 => i16::MIN as i64,
                FixedIntTy::I32 => i32::MIN as i64,
                FixedIntTy::I64 => i64::MIN,
                FixedIntTy::U8 => 0,
                FixedIntTy::U16 => 0,
                FixedIntTy::U32 => 0,
                FixedIntTy::U64 => 0,
                FixedIntTy::F32 | FixedIntTy::F64 => 0,
            }
        }

        pub fn max_int(&self) -> i64 {
            match self {
                FixedIntTy::I8 => i8::MAX as i64,
                FixedIntTy::I16 => i16::MAX as i64,
                FixedIntTy::I32 => i32::MAX as i64,
                FixedIntTy::I64 => i64::MAX,
                FixedIntTy::U8 => u8::MAX as i64,
                FixedIntTy::U16 => u16::MAX as i64,
                FixedIntTy::U32 => u32::MAX as i64,
                FixedIntTy::U64 => u64::MAX as i64,
                FixedIntTy::F32 | FixedIntTy::F64 => 0,
            }
        }
    }

    impl fmt::Display for FixedIntTy {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = match self {
                FixedIntTy::I8 => "i8",
                FixedIntTy::I16 => "i16",
                FixedIntTy::I32 => "i32",
                FixedIntTy::I64 => "i64",
                FixedIntTy::U8 => "u8",
                FixedIntTy::U16 => "u16",
                FixedIntTy::U32 => "u32",
                FixedIntTy::U64 => "u64",
                FixedIntTy::F32 => "f32",
                FixedIntTy::F64 => "f64",
            };
            write!(f, "{}", s)
        }
    }

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum FixedRepr {
        Int(i64),
        Float(f64),
    }

    /// Um ponteiro real: endereço opaco no heap do Aly.
    #[derive(Clone, PartialEq, Debug)]
    pub struct Pointer {
        pub address: usize,
        pub ty: Option<Type>,
        pub is_null: bool,
    }

    impl Pointer {
        pub fn new(address: usize) -> Pointer {
            Pointer {
                address,
                ty: None,
                is_null: address == 0,
            }
        }
    }

pub enum ValueData {
        Int(i32),
        Float(f32),
        String(String),
        Bool(bool),
        Char(char),
        Void,
        Vec(Vector),
        Tuple(Tuple),
        Set(Set),
        Stack(Stack),
        Queue(Queue),
        LinkedList(LinkedList),
        Enum(Enumeration),
        Option(Option<Box<ValueData>>),
        Result(Box<ValueData>, Box<ValueData>),
        Object(Object),
        Function(Vec<Lexer>),   
        NativeFunction(fn(String) -> Box<dyn Validator>),
        PluginFunction { namespace: String, func_name: String },
        Shared(Rc<RefCell<ValueData>>),
        JotInstance(Box<dyn crate::runtime::jot::JotInstanceValidator>),
        FixedInt { ty: FixedIntTy, val: FixedRepr },
        Pointer(Pointer),
        NativeData(fn(&[ValueData]) -> ValueData),
    }

    impl ValueData {
        pub fn new_shared(value: ValueData) -> ValueData {
            ValueData::Shared(Rc::new(RefCell::new(value)))
        }

        pub fn get_shared_value(&self) -> Option<ValueData> {
            match self {
                ValueData::Shared(rc) => Some(rc.borrow().clone()),
                _ => None,
            }
        }

        pub fn set_shared_value(&self, value: ValueData) {
            if let ValueData::Shared(rc) = self {
                *rc.borrow_mut() = value;
            }
        }
        pub fn set_property(&mut self, prop_path: &[String], new_val: ValueData) -> Result<(), String> {
            if prop_path.is_empty() {
                return Ok(());
            }
            if prop_path.len() == 1 {
                match self {
                    ValueData::Object(obj) => {
                        obj.set_item(prop_path[0].clone(), new_val);
                        return Ok(());
                    }
                    _ => return Err(format!("O tipo não é um objeto.")),
                }
            }
            match self {
                ValueData::Object(obj) => {
                    if let Some(child) = obj.get_item_mut(&prop_path[0]) {
                        child.set_property(&prop_path[1..], new_val)?;
                    } else {
                        let new_obj = Object::from_map(linked_hash_map::LinkedHashMap::new());
                        let mut val = ValueData::Object(new_obj);
                        val.set_property(&prop_path[1..], new_val)?;
                        obj.set_item(prop_path[0].clone(), val);
                    }
                    Ok(())
                }
                _ => Err(format!("O tipo não é um objeto.")),
            }
        }

        pub fn to_string(&self, qt: bool) -> String {
            match self {
                ValueData::Int(int) => int.to_string(),
                ValueData::Float(f) => f.to_string(),
                ValueData::String(s) => {
                    if qt {
                        put_quoted_str(s.to_string())
                    } else {
                        s.to_string()
                    }
                },
                ValueData::Bool(bool)  => bool.to_string(),
                ValueData::Char(c) => c.to_string(),
                ValueData::Void => "void".to_string(),
                ValueData::Vec(vec) => vec.to_json(0),
                ValueData::Tuple(tup) => tup.to_json(0),
                ValueData::Set(set) => set.to_json(0),
                ValueData::Stack(stack) => stack.to_json(0),
                ValueData::Queue(queue) => queue.to_json(0),
                ValueData::LinkedList(list) => list.to_json(0),
                ValueData::Enum(en) => en.to_string(false),
                ValueData::Option(opt) => match opt {
                    Some(v) => format!("Some({})", v.to_string(false)),
                    None => "None".to_string(),
                },
                ValueData::Shared(s) => format!("Shared({})", s.borrow().to_string(false)),
                ValueData::Result(ok, err) => format!("Result(Ok: {}, Err: {})", ok.to_string(false), err.to_string(false)),
                ValueData::Function(_) => "Function".to_owned(),

                ValueData::NativeFunction(_) => "NativeFunction".to_owned(),
                ValueData::PluginFunction { .. } => "PluginFunction".to_owned(),
                ValueData::Object(obj) => obj.to_string(false),
                ValueData::JotInstance(jot) => jot.jot_to_string(None),
                ValueData::FixedInt { ty, val } => format_fixed_value(*ty, *val),
                ValueData::Pointer(p) => {
                    if p.is_null {
                        "null".to_string()
                    } else {
                        format!("&{}", p.address)
                    }
                },
                ValueData::NativeData(_) => "NativeFunction".to_owned(),
            }
        }

        pub fn literal(&self) -> String{
            match self {
                ValueData::Int(int) => int.to_string(),
                ValueData::Float(f) => f.to_string(),
                ValueData::String(s) => s.clone(),
                ValueData::Bool(bool)  => bool.to_string(),
                ValueData::Char(c) => c.to_string(),
                ValueData::Void => "void".to_string(),
                ValueData::Vec(vec) => vec.to_json(0),
                ValueData::Tuple(tup) => tup.to_json(0),
                ValueData::Set(set) => set.to_json(0),
                ValueData::Stack(stack) => stack.to_json(0),
                ValueData::Queue(queue) => queue.to_json(0),
                ValueData::LinkedList(list) => list.to_json(0),
                ValueData::Enum(en) => en.to_string(false),
                ValueData::Option(opt) => match opt {
                    Some(v) => format!("Some({})", v.to_string(false)),
                    None => "None".to_string(),
                },
                ValueData::Shared(s) => format!("Shared({})", s.borrow().to_string(false)),
                ValueData::Result(ok, err) => format!("Result(Ok: {}, Err: {})", ok.to_string(false), err.to_string(false)),
                ValueData::Function(_) => "Function".to_owned(),

                ValueData::NativeFunction(_) => "NativeFunction".to_owned(),
                ValueData::PluginFunction { .. } => "PluginFunction".to_owned(),
                ValueData::Object(obj) => obj.to_string(false),
                ValueData::JotInstance(jot) => jot.jot_to_string(None),
                ValueData::FixedInt { ty, val } => format_fixed_value(*ty, *val),
                ValueData::Pointer(p) => {
                    if p.is_null {
                        "null".to_string()
                    } else {
                        format!("&{}", p.address)
                    }
                },
                ValueData::NativeData(_) => "NativeFunction".to_owned(),
            }
        }

        fn extract(&self) -> Box<dyn Validator> {
            match self {
                ValueData::Int(i) => Box::new(i.to_string()),
                ValueData::Float(f) => Box::new(f.to_string()),
                ValueData::String(s) => Box::new(s.to_string()),
                ValueData::Bool(b) => Box::new(b.to_string()),
                ValueData::Char(c) => Box::new(c.to_string()),
                ValueData::Void => Box::new("void".to_string()),
                ValueData::Object(o) => Box::new(o.clone()),
                ValueData::Function(_) |
                ValueData::NativeFunction(_) |
                ValueData::PluginFunction { .. } => Box::new("PluginFunction".to_string()),
                ValueData::Vec(_) => Box::new("Vec".to_string()),
                ValueData::Tuple(_) => Box::new("Tuple".to_string()),
                ValueData::Set(_) => Box::new("Set".to_string()),
                ValueData::Stack(_) => Box::new("Stack".to_string()),
                ValueData::Queue(_) => Box::new("Queue".to_string()),
                ValueData::LinkedList(_) => Box::new("LinkedList".to_string()),
                ValueData::Enum(_) => Box::new("Enum".to_string()),
                ValueData::Option(_) => Box::new("Option".to_string()),
                ValueData::Shared(_) => Box::new("Shared".to_string()),
                ValueData::Result(_, _) => Box::new("Result".to_string()),
                ValueData::JotInstance(jot) => Box::new(jot.jot_to_string(None)),
                ValueData::FixedInt { .. } => Box::new("FixedInt".to_string()),
                ValueData::Pointer(_) => Box::new("Pointer".to_string()),
                ValueData::NativeData(_) => Box::new("NativeFunction".to_string()),
            }
        } 

        pub fn type_name(&self) -> &'static str {
            match self {
                ValueData::Int(_) => "int",
                ValueData::Float(_) => "float",
                ValueData::String(_) => "string",
                ValueData::Bool(_) => "bool",
                ValueData::Char(_) => "char",
                ValueData::Void => "void",
                ValueData::Vec(_) => "vector",
                ValueData::Tuple(_) => "tuple",
                ValueData::Set(_) => "set",
                ValueData::Stack(_) => "stack",
                ValueData::Queue(_) => "queue",
                ValueData::LinkedList(_) => "linkedlist",
                ValueData::Enum(_) => "enum",
                ValueData::Option(_) => "option",
                ValueData::Shared(_) => "shared",
                ValueData::Result(_, _) => "result",
                ValueData::Object(_) => "object",
                ValueData::Function(_) => "function",
                ValueData::NativeFunction(_) => "native",
                ValueData::PluginFunction { .. } => "plugin",
                ValueData::JotInstance(_) => "jot",
                ValueData::FixedInt { .. } => "fixed",
                ValueData::Pointer(_) => "pointer",
                ValueData::NativeData(_) => "native",
            }
        }

        pub fn get_prop(&self, is_mut: bool, props: Vec<Lexer>) -> Box<dyn Validator> {
            let mut props_vec = vec![];
            
            let data = self.extract();
            let line = props[0].line;

            for prop in props {
                if prop.literal != "." {
                    props_vec.push(prop.literal.clone())
                }
            }

            return Box::new(put_quoted_str(if props_vec.len() == 1 {
                self.prop(&props_vec[0], is_mut, line, data).to_string(false)
            } else {
                let mut tmp = self.prop(props_vec[0].as_str(), is_mut, line, data);

                for prop in &props_vec[1..] {
                    tmp = match tmp {
                        ValueData::String(ref str) => {
                            if is_any_str(&str) {
                                tmp
                            } else {
                                ValueData::String(put_quoted_str(tmp.to_string(false)))
                            }
                        },
                        _ => tmp
                    };

                    let boxed_tmp: Box<dyn Validator> = Box::new(tmp);
                    
                    tmp = self.prop(prop.as_str(), is_mut, line, boxed_tmp);
                }
                
                tmp.to_string(false)                            
            }));
        }

        /// Reflection: retorna metadados sobre o objeto
        pub fn reflect(&self) -> ValueData {
            use linked_hash_map::LinkedHashMap;
            let mut map = LinkedHashMap::new();
            map.insert("type".to_string(), ValueData::String(self.type_name().to_string()));
            map.insert("value".to_string(), ValueData::String(self.to_string(false)));
            map.insert("is_mutable".to_string(), ValueData::Bool(true)); // Simplified for now
            ValueData::Object(Object::from_map(map))
        }

        pub fn prop(&self, prop: &str, is_mut: bool, line: i32, data: Box<dyn Validator>) -> ValueData {
            let (type_data, value) = data.valid();

        
            match prop {
                "reflect" => self.reflect(),
                "type" => ValueData::String(type_data.to_string()),
                "len" => {
                    let len_value  = match value {
                        ValueData::Vec(vector) => vector.len(),
                        ValueData::Object(obj) => {
                            obj.len()
                        }
                        _ => value.to_string(false).len().try_into().unwrap()
                    };

                    ValueData::Int(len_value.try_into().unwrap())
                },
                "is_mut" => ValueData::Bool(is_mut),
                "to_int" => {
                    let mut val = value.to_string(false).trim().to_string();
                    if val.contains(".") {
                        val = val.split(".").collect::<Vec<&str>>()[0].trim().to_owned();
                    }
                    ValueData::Int(conversor_to_int(val))
                },
                "to_str" => {
                    ValueData::String(match self {
                        ValueData::Object(obj) => obj.to_json(0),
                        _ => put_quoted_str(self.to_string(false))
                    })
                },
                "to_float" => {
                    let mut val = value.to_string(false).trim().to_owned().replace(" ", "");
                    if !val.contains(".") {
                        val = format!("{}.00", val);
                    }
                    ValueData::Float(conversor_to_float(val))
                },
                item => {
                    if is_num(item) || item == "_" {
                        match type_data {
                            Type::String => {
                                let x = conversor_to_int(item.to_owned());
                                let val = value.to_string(false);
                                if x <= -1 {
                                    if let Some(ch) = val.chars().last() {
                                        return ValueData::String(ch.to_string());
                                    } else {
                                        return ValueData::String("None".to_owned());
                                    }
                                }
                                if let Ok(idx) = usize::try_from(x) {
                                    if let Some(ch) = val.chars().nth(idx) {
                                        return ValueData::String(ch.to_string());
                                    }
                                }
                                return ValueData::String("None".to_owned());
                            },
                            _ => {
                                match self {
                                    ValueData::Vec(vec) => {
                                        let x = conversor_to_int(item.to_owned());
                                        if let Ok(idx) = usize::try_from(x) {
                                            vec.get_index(idx)
                                        } else {
                                            ValueData::String("None".to_owned())
                                        }
                                    },
                                    ValueData::Tuple(tup) => {
                                        let x = conversor_to_int(item.to_owned());
                                        if let Ok(idx) = usize::try_from(x) {
                                            tup.get_index(idx)
                                        } else {
                                            ValueData::String("None".to_owned())
                                        }
                                    },
                                    _ => {
                                        eprintln!("RuntimeError [types]: o tipo {} não é indexável na linha {}.", type_data, line);
                                        ValueData::String("None".to_owned())
                                    }
                                }
                            }
                        }
                    } else if type_data.to_string() == "obj" {
                        match value {
                            ValueData::Object(obj) => obj.get_item(prop.to_owned()),
                            _ => {
                                eprintln!("RuntimeError [types]: o tipo {} não é um objeto na linha {}.", type_data, line);
                                ValueData::String("None".to_owned())
                            }
                        }
                    } else {
                        ValueData::String("None".to_owned())
                    }
                }
            }
        }        

    }

    impl PartialEq for ValueData {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (ValueData::Int(a), ValueData::Int(b)) => a == b,
                (ValueData::Float(a), ValueData::Float(b)) => a == b,
                (ValueData::String(a), ValueData::String(b)) => a == b,
                (ValueData::Bool(a), ValueData::Bool(b)) => a == b,
                (ValueData::Vec(a), ValueData::Vec(b)) => a == b,
                (ValueData::Tuple(a), ValueData::Tuple(b)) => a == b,
                (ValueData::Set(a), ValueData::Set(b)) => a == b,
                (ValueData::Stack(a), ValueData::Stack(b)) => a == b,
                (ValueData::Queue(a), ValueData::Queue(b)) => a == b,
                (ValueData::LinkedList(a), ValueData::LinkedList(b)) => a == b,
                (ValueData::Enum(a), ValueData::Enum(b)) => a == b,
                (ValueData::Shared(a), ValueData::Shared(b)) => *a.borrow() == *b.borrow(),
                (ValueData::Object(a), ValueData::Object(b)) => a == b,
                (ValueData::JotInstance(_), ValueData::JotInstance(_)) => false,
                (ValueData::FixedInt { ty: a, val: b }, ValueData::FixedInt { ty: c, val: d }) => {
                    a == c && b == d
                }
                (ValueData::Pointer(a), ValueData::Pointer(b)) => a == b,
                (ValueData::NativeData(a), ValueData::NativeData(b)) => {
                    *a as usize == *b as usize
                }
                _ => false,
            }
        }
    }

    impl Clone for ValueData {
        fn clone(&self) -> Self {
            match self {
                ValueData::Int(d) => ValueData::Int(d.clone()),
                ValueData::Float(d) => ValueData::Float(d.clone()),
                ValueData::String(d) => ValueData::String(d.clone()),
                ValueData::Bool(d) => ValueData::Bool(d.clone()),
                ValueData::Char(d) => ValueData::Char(d.clone()),
                ValueData::Void => ValueData::Void,
                ValueData::Vec(d) => ValueData::Vec(d.clone()),
                ValueData::Tuple(d) => ValueData::Tuple(d.clone()),
                ValueData::Set(d) => ValueData::Set(d.clone()),
                ValueData::Stack(d) => ValueData::Stack(d.clone()),
                ValueData::Queue(d) => ValueData::Queue(d.clone()),
                ValueData::LinkedList(d) => ValueData::LinkedList(d.clone()),
                ValueData::Enum(d) => ValueData::Enum(d.clone()),
                ValueData::Option(d) => ValueData::Option(d.clone()),
                ValueData::Shared(d) => ValueData::Shared(d.clone()),
                ValueData::Result(ok, err) => ValueData::Result(ok.clone(), err.clone()),
                ValueData::Object(obj) => ValueData::Object(obj.clone()),
                ValueData::Function(fun) => ValueData::Function(fun.clone()),
                ValueData::NativeFunction(fun) => ValueData::NativeFunction(fun.clone()),
                ValueData::PluginFunction { namespace, func_name } => ValueData::PluginFunction {
                    namespace: namespace.clone(),
                    func_name: func_name.clone(),
                },
                ValueData::Object(obj) => ValueData::Object(obj.clone()),
                ValueData::Shared(rc) => ValueData::Shared(rc.clone()),
                ValueData::JotInstance(_) => {
                    // JotInstance is immutable, return a new reference to the same data
                    // This should not be called in practice since JOT is immutable
                    panic!("JotInstance clone not supported directly")
                }
                ValueData::FixedInt { ty, val } => ValueData::FixedInt { ty: *ty, val: *val },
                ValueData::Pointer(p) => ValueData::Pointer(p.clone()),
                ValueData::NativeData(fun) => ValueData::NativeData(*fun),
            }
        }
    }

    // Validator value
    
    pub trait Validator {
        fn valid(&self) -> (Type, ValueData);
    }

    impl Validator for () {
        fn valid(&self) -> (Type, ValueData) {            
            (Type::None, ValueData::String("None".to_owned())) 
        }
    }

    impl Validator for Object {
        fn valid(&self) -> (Type, ValueData) {            
            (Type::Obj, ValueData::Object(self.clone()))
        }
    }

    impl Validator for ValueData {
        fn valid(&self) -> (Type, ValueData) {            
            match self {
                ValueData::Int(int) => (Type::Int, ValueData::Int(*int)),
                ValueData::Float(float) => (Type::Float, ValueData::Float(*float)),
                ValueData::String(str) => {
                    str.valid()
                },
                ValueData::Bool(bool) => (Type::Bool, ValueData::Bool(*bool)),
                ValueData::Char(c) => (Type::Char, ValueData::Char(*c)),
                ValueData::Void => (Type::Void, ValueData::Void),
                ValueData::Vec(vec) => (Type::Vec, ValueData::Vec(vec.clone())),
                ValueData::Tuple(tup) => (Type::Tuple, ValueData::Tuple(tup.clone())),
                ValueData::Set(set) => (Type::Set, ValueData::Set(set.clone())),
                ValueData::Stack(stack) => (Type::Stack, ValueData::Stack(stack.clone())),
                ValueData::Queue(queue) => (Type::Queue, ValueData::Queue(queue.clone())),
                ValueData::LinkedList(list) => (Type::LinkedList, ValueData::LinkedList(list.clone())),
                ValueData::Enum(en) => (Type::Enum, ValueData::Enum(en.clone())),
                ValueData::Option(opt) => (Type::Optional(Box::new(Type::None)), ValueData::Option(opt.clone())),
                ValueData::Shared(s) => (Type::Shared(Box::new(Type::None)), ValueData::Shared(s.clone())),
                ValueData::Result(ok, err) => (Type::Result, ValueData::Result(ok.clone(), err.clone())),
                ValueData::Object(obj) => (Type::Obj, ValueData::Object(obj.clone())),
                ValueData::Function(fun) => (Type::Function, ValueData::Function(fun.clone())),
                ValueData::NativeFunction(fun) => (Type::NativeFunction, ValueData::NativeFunction(*fun)),
                ValueData::PluginFunction { namespace, func_name } => {
                    (Type::PluginFunction, ValueData::PluginFunction {
                        namespace: namespace.clone(),
                        func_name: func_name.clone(),
                    })
                }
                ValueData::JotInstance(_) => {
                    (Type::Obj, ValueData::String("JotInstance".to_string()))
                }
                ValueData::FixedInt { ty, val } => {
                    (Type::Fixed(*ty), ValueData::FixedInt { ty: *ty, val: *val })
                }
                ValueData::Pointer(p) => {
                    let inner = p.ty.clone().unwrap_or(Type::None);
                    (Type::Pointer(Box::new(inner)), ValueData::Pointer(p.clone()))
                }
                ValueData::NativeData(fun) => (Type::NativeFunction, ValueData::NativeData(*fun)),
            }
        }
    }

    
    impl Validator for fn(String) -> Box<dyn Validator> {
        fn valid(&self) -> (Type, ValueData) {
            (Type::NativeFunction, ValueData::NativeFunction(*self)) 
        }
    }

    impl Validator for fn(&[ValueData]) -> ValueData {
        fn valid(&self) -> (Type, ValueData) {
            (Type::NativeFunction, ValueData::NativeData(*self))
        }
    }

    impl Validator for bool {
        fn valid(&self) -> (Type, ValueData) {
            (Type::Bool, ValueData::Bool(*self))
        }
    }

    impl Validator for String {
        fn valid(&self) -> (Type, ValueData) {
            if is_any_str(self) {
                (Type::String, ValueData::String(remove_quoted_str(self.clone()))) 
            } else if is_bool(&self) {
                (Type::Bool, ValueData::Bool(conversor_to_bool(self.clone())))
            } else if is_int(self) {
                (Type::Int, ValueData::Int(conversor_to_int(self.clone())))
            } else if is_float(self) {
                (Type::Int, ValueData::Float(conversor_to_float(self.clone())))
            } else {
                if self.chars().all(|c| c.is_alphanumeric() || c.is_ascii_punctuation()) {
                    return (Type::String, ValueData::String(remove_quoted_str(self.clone()))); 
                }

                (Type::None, ValueData::String("None".to_owned())) 
            }
        }
    }
    

    pub fn is_valid_data<T: Validator>(data: T) -> (Type, ValueData) {
        data.valid()
    }

    impl ValueData {
        pub fn as_string(&self) -> Option<String> {
            match self {
                ValueData::String(s) => Some(s.clone()),
                _ => None,
            }
        }

        pub fn as_bool(&self) -> Option<bool> {
            match self {
                ValueData::Bool(b) => Some(*b),
                _ => None,
            }
        }

        pub fn as_int(&self) -> Option<i32> {
            match self {
                ValueData::Int(i) => Some(*i),
                ValueData::FixedInt { ty: _, val: FixedRepr::Int(i) } => Some(*i as i32),
                _ => None,
            }
        }

        pub fn as_float(&self) -> Option<f32> {
            match self {
                ValueData::Float(f) => Some(*f),
                ValueData::FixedInt { ty: _, val: FixedRepr::Float(f) } => Some(*f as f32),
                _ => None,
            }
        }

        pub fn as_jot_instance(&self) -> Option<&dyn crate::runtime::jot::JotInstanceValidator> {
            match self {
                ValueData::JotInstance(jot) => Some(jot.as_ref()),
                _ => None,
            }
        }
    }

    /// Interpreta uma anotação de tipo da linguagem (`i8`, `u32`, `ptr`, ...).
    pub fn parse_type_annotation(s: &str) -> Option<Type> {
        match s.trim() {
            "int" => Some(Type::Int),
            "float" => Some(Type::Float),
            "string" => Some(Type::String),
            "boolean" | "bool" => Some(Type::Bool),
            "vector" => Some(Type::Vec),
            "ptr" => Some(Type::Pointer(Box::new(Type::None))),
            _ => FixedIntTy::from_str(s).map(Type::Fixed),
        }
    }

    pub fn format_fixed_value(ty: FixedIntTy, val: FixedRepr) -> String {
        match (ty, val) {
            (FixedIntTy::F32, FixedRepr::Float(f)) => (f as f32).to_string(),
            (FixedIntTy::F64, FixedRepr::Float(f)) => f.to_string(),
            (_, FixedRepr::Float(f)) => (f as i64).to_string(),
            (FixedIntTy::I8, FixedRepr::Int(i)) => (i as i8).to_string(),
            (FixedIntTy::I16, FixedRepr::Int(i)) => (i as i16).to_string(),
            (FixedIntTy::I32, FixedRepr::Int(i)) => (i as i32).to_string(),
            (FixedIntTy::I64, FixedRepr::Int(i)) => i.to_string(),
            (FixedIntTy::U8, FixedRepr::Int(i)) => (i as u8).to_string(),
            (FixedIntTy::U16, FixedRepr::Int(i)) => (i as u16).to_string(),
            (FixedIntTy::U32, FixedRepr::Int(i)) => (i as u32).to_string(),
            (FixedIntTy::U64, FixedRepr::Int(i)) => (i as u64).to_string(),
            (FixedIntTy::F32 | FixedIntTy::F64, FixedRepr::Int(i)) => i.to_string(),
        }
    }

    /// Coage um `ValueData` para um tipo-alvo, validando intervalos
    /// (overflow de i8/u32/etc. gera erro em vez de wrap).
    pub fn coerce(value: ValueData, ty: &Type) -> Result<ValueData, String> {
        match ty {
            Type::Fixed(ft) => coerce_fixed(value, *ft),
            Type::Pointer(_) => match value {
                ValueData::Pointer(p) => Ok(ValueData::Pointer(p)),
                other => Err(format!(
                    "TypeError: não é possível converter {} para ptr.",
                    other.type_name()
                )),
            },
            Type::Int => match value {
                ValueData::Int(i) => Ok(ValueData::Int(i)),
                ValueData::FixedInt { ty: _, val: FixedRepr::Int(i) } => {
                    if i > i32::MAX as i64 || i < i32::MIN as i64 {
                        return Err(format!("TypeError: overflow int (i32) — valor {}.", i));
                    }
                    Ok(ValueData::Int(i as i32))
                }
                ValueData::Float(f) => Ok(ValueData::Int(f as i32)),
                other => Err(format!(
                    "TypeError: não é possível converter {} para int.",
                    other.type_name()
                )),
            },
            Type::Float => match value {
                ValueData::Float(f) => Ok(ValueData::Float(f)),
                ValueData::FixedInt { ty: _, val: FixedRepr::Float(f) } => Ok(ValueData::Float(f as f32)),
                ValueData::Int(i) => Ok(ValueData::Float(i as f32)),
                other => Err(format!(
                    "TypeError: não é possível converter {} para float.",
                    other.type_name()
                )),
            },
            Type::String => match value {
                ValueData::String(s) => Ok(ValueData::String(s)),
                other => Ok(ValueData::String(other.to_string(false))),
            },
            _ => Ok(value),
        }
    }

    fn coerce_fixed(value: ValueData, ft: FixedIntTy) -> Result<ValueData, String> {
        let numeric = match value {
            ValueData::FixedInt { ty: _, val } => val,
            ValueData::Int(i) => FixedRepr::Int(i as i64),
            ValueData::Float(f) => FixedRepr::Float(f as f64),
            ValueData::String(s) => {
                let cleaned = s.trim();
                if ft.is_float() {
                    match cleaned.parse::<f64>() {
                        Ok(f) => FixedRepr::Float(f),
                        Err(_) => {
                            return Err(format!(
                                "TypeError: '{}' não é um número válido para {}.",
                                cleaned, ft
                            ));
                        }
                    }
                } else {
                    match cleaned.parse::<i64>() {
                        Ok(i) => FixedRepr::Int(i),
                        Err(_) => {
                            return Err(format!(
                                "TypeError: '{}' não é um número válido para {}.",
                                cleaned, ft
                            ));
                        }
                    }
                }
            }
            other => {
                return Err(format!(
                    "TypeError: não é possível converter {} para {}.",
                    other.type_name(),
                    ft
                ));
            }
        };

        match numeric {
            FixedRepr::Float(f) if ft.is_float() => match ft {
                FixedIntTy::F32 => {
                    let cast = f as f32;
                    if f.is_finite() && cast.is_infinite() {
                        return Err(format!(
                            "TypeError: overflow f32 — valor {} fora do intervalo.",
                            f
                        ));
                    }
                    Ok(ValueData::FixedInt { ty: ft, val: FixedRepr::Float(cast as f64) })
                }
                FixedIntTy::F64 => Ok(ValueData::FixedInt { ty: ft, val: FixedRepr::Float(f) }),
                _ => unreachable!(),
            },
            FixedRepr::Float(f) if !ft.is_float() => {
                let i = f.trunc() as i64;
                if i < ft.min_int() || i > ft.max_int() {
                    return Err(format!(
                        "TypeError: overflow {} — valor {} fora do intervalo [{}, {}].",
                        ft,
                        f,
                        ft.min_int(),
                        ft.max_int()
                    ));
                }
                Ok(ValueData::FixedInt { ty: ft, val: FixedRepr::Int(i) })
            }
            FixedRepr::Int(i) if !ft.is_float() => {
                if i < ft.min_int() || i > ft.max_int() {
                    return Err(format!(
                        "TypeError: overflow {} — valor {} fora do intervalo [{}, {}].",
                        ft,
                        i,
                        ft.min_int(),
                        ft.max_int()
                    ));
                }
                Ok(ValueData::FixedInt { ty: ft, val: FixedRepr::Int(i) })
            }
            FixedRepr::Int(i) if ft.is_float() => match ft {
                FixedIntTy::F32 => Ok(ValueData::FixedInt { ty: ft, val: FixedRepr::Float(i as f32 as f64) }),
                FixedIntTy::F64 => Ok(ValueData::FixedInt { ty: ft, val: FixedRepr::Float(i as f64) }),
                _ => unreachable!(),
            },
            _ => Err(format!("TypeError: não é possível converter para {}.", ft)),
        }
    }
}

pub use types::*;
