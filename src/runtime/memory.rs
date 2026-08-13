mod memory {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    use crate::native::types::{coerce, FixedIntTy, Pointer, Type, ValueData};
    use crate::aly::Aly;

    /// Uma célula mutável do heap — equivalente ao `Cell` do projeto Python,
    /// porém em Rust seguro: o endereço é apenas uma chave do HashMap.
    #[derive(Clone)]
    pub struct Cell {
        pub value: Rc<RefCell<ValueData>>,
        pub mutable: bool,
        pub name: Option<String>,
        pub ty: Option<Type>,
    }

    pub struct Heap {
        next_addr: usize,
        cells: HashMap<usize, Cell>,
        names: HashMap<String, usize>,
    }

    impl Heap {
        pub fn new() -> Heap {
            Heap {
                next_addr: 1,
                cells: HashMap::new(),
                names: HashMap::new(),
            }
        }

        pub fn alloc(
            &mut self,
            value: ValueData,
            mutable: bool,
            name: Option<String>,
            ty: Option<Type>,
        ) -> (usize, Rc<RefCell<ValueData>>) {
            let addr = self.next_addr;
            self.next_addr += 1;
            let rc = Rc::new(RefCell::new(value));
            self.cells.insert(
                addr,
                Cell {
                    value: rc.clone(),
                    mutable,
                    name: name.clone(),
                    ty,
                },
            );
            if let Some(n) = name {
                self.names.insert(n, addr);
            }
            (addr, rc)
        }

        pub fn pointer_for(&self, addr: usize) -> Pointer {
            let ty = self.cells.get(&addr).and_then(|c| c.ty.clone());
            Pointer {
                address: addr,
                ty,
                is_null: addr == 0,
            }
        }

        pub fn exists(&self, addr: usize) -> bool {
            addr != 0 && self.cells.contains_key(&addr)
        }

        pub fn load(&self, addr: usize) -> Result<ValueData, String> {
            if addr == 0 {
                return Err("ponteiro nulo (null) — não é possível ler.".to_string());
            }
            match self.cells.get(&addr) {
                Some(c) => Ok(c.value.borrow().clone()),
                None => Err(format!(
                    "ponteiro pendurado — endereço {} não existe no heap.",
                    addr
                )),
            }
        }

        pub fn store(&self, addr: usize, value: ValueData) -> Result<(), String> {
            if addr == 0 {
                return Err("ponteiro nulo (null) — não é possível escrever.".to_string());
            }
            let cell = self.cells.get(&addr).ok_or_else(|| {
                format!("ponteiro pendurado — endereço {} não existe no heap.", addr)
            })?;
            if !cell.mutable {
                return Err(format!(
                    "a célula no endereço {} é imutável (const ou tomb()).",
                    addr
                ));
            }
            let coerced = match &cell.ty {
                Some(t) => coerce(value, t)?,
                None => value,
            };
            *cell.value.borrow_mut() = coerced;
            Ok(())
        }

        pub fn free(&mut self, addr: usize) -> Result<(), String> {
            match self.cells.remove(&addr) {
                Some(cell) => {
                    if let Some(n) = cell.name {
                        self.names.remove(&n);
                    }
                    Ok(())
                }
                None => Err(format!(
                    "ponteiro pendurado — endereço {} não existe no heap.",
                    addr
                )),
            }
        }

        pub fn set_mutable(&mut self, addr: usize, mutable: bool) -> Result<(), String> {
            match self.cells.get_mut(&addr) {
                Some(cell) => {
                    cell.mutable = mutable;
                    Ok(())
                }
                None => Err(format!(
                    "ponteiro pendurado — endereço {} não existe no heap.",
                    addr
                )),
            }
        }

        pub fn forget_name(&mut self, name: &str) {
            self.names.remove(name);
        }
    }

    thread_local! {
        static HEAP: RefCell<Heap> = RefCell::new(Heap::new());
    }

    pub fn with_heap<T>(f: impl FnOnce(&mut Heap) -> T) -> T {
        HEAP.with(|h| f(&mut h.borrow_mut()))
    }

    /// O `&var` real: boxa o valor da variável numa célula do heap e devolve o
    /// ponteiro. A partir daí, escrita por nome (`x = v`) e por ponteiro
    /// (`store(<&x>, v)`) mutam a mesma célula (write-through).
    pub fn address_of_var(run: &mut Aly, name: String, line: i32) -> Result<ValueData, String> {
        with_heap(|heap| {
            if let Some(&addr) = heap.names.get(&name) {
                if heap.exists(addr) {
                    return Ok(ValueData::Pointer(heap.pointer_for(addr)));
                }
            }

            let var = run
                .get_var_per_name(name.clone())
                .map_err(|e| format!("ReferenceError: {} na linha {}.", e, line))?;

            let raw = match var.get_value() {
                ValueData::Shared(rc) => rc.borrow().clone(),
                v => v,
            };
            let mutable = var.is_mutable();
            let ty = var.get_type().clone();

            let (addr, rc) = heap.alloc(raw, mutable, Some(name.clone()), Some(ty));
            var.set_raw_value(ValueData::Shared(rc));

            Ok(ValueData::Pointer(heap.pointer_for(addr)))
        })
    }

    pub fn heap_alloc(value: ValueData) -> ValueData {
        with_heap(|h| {
            let (addr, _) = h.alloc(value, true, None, None);
            ValueData::Pointer(h.pointer_for(addr))
        })
    }

    pub fn heap_alloc_typed(ty: &str, value: ValueData) -> ValueData {
        let ty = match FixedIntTy::from_str(ty) {
            Some(t) => Type::Fixed(t),
            None => {
                eprintln!("TypeError: tipo fixo desconhecido '{}'.", ty);
                return ValueData::String("None".to_owned());
            }
        };
        let coerced = match coerce(value, &ty) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("TypeError: {}", e);
                return ValueData::String("None".to_owned());
            }
        };
        with_heap(|h| {
            let (addr, _) = h.alloc(coerced, true, None, Some(ty));
            ValueData::Pointer(h.pointer_for(addr))
        })
    }

    pub fn heap_load(addr: usize) -> Result<ValueData, String> {
        with_heap(|h| h.load(addr))
    }

    pub fn heap_store(addr: usize, value: ValueData) -> Result<(), String> {
        with_heap(|h| h.store(addr, value))
    }

    pub fn heap_free(addr: usize) -> Result<(), String> {
        with_heap(|h| h.free(addr))
    }

    pub fn heap_set_mutable(addr: usize, mutable: bool) -> Result<(), String> {
        with_heap(|h| h.set_mutable(addr, mutable))
    }

    pub fn heap_pointer(addr: usize) -> ValueData {
        with_heap(|h| ValueData::Pointer(h.pointer_for(addr)))
    }

    pub fn heap_exists(addr: usize) -> bool {
        with_heap(|h| h.exists(addr))
    }
}

pub use memory::*;
