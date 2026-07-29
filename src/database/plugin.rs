use std::collections::HashMap;
use std::cell::RefCell;

thread_local! {
    static DRIVER_FACTORIES: RefCell<HashMap<String, fn(&str) -> Result<Box<dyn super::driver::DatabaseDriver>, String>>> =
        RefCell::new(HashMap::new());
}

pub fn register_factory(
    scheme: &str,
    factory: fn(&str) -> Result<Box<dyn super::driver::DatabaseDriver>, String>,
) {
    DRIVER_FACTORIES.with(|f| {
        f.borrow_mut().insert(scheme.to_string(), factory);
    });
}

pub fn has_factory(scheme: &str) -> bool {
    DRIVER_FACTORIES.with(|f| f.borrow().contains_key(scheme))
}

pub fn list_factories() -> Vec<String> {
    DRIVER_FACTORIES.with(|f| f.borrow().keys().cloned().collect())
}

pub fn create_from_factory(scheme: &str, uri: &str) -> Option<Result<Box<dyn super::driver::DatabaseDriver>, String>> {
    DRIVER_FACTORIES.with(|f| {
        let factories = f.borrow();
        factories.get(scheme).map(|factory| factory(uri))
    })
}
