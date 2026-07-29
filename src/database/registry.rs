use std::cell::RefCell;
use std::collections::HashMap;

use super::driver::DatabaseDriver;

thread_local! {
    static CONNECTIONS: RefCell<HashMap<String, Box<dyn DatabaseDriver>>> = RefCell::new(HashMap::new());
}

pub fn register_driver(name: &str, driver: Box<dyn DatabaseDriver>) {
    CONNECTIONS.with(|reg| {
        reg.borrow_mut().insert(name.to_string(), driver);
    });
}

pub fn with_driver<F, R>(name: &str, f: F) -> Result<R, String>
where
    F: FnOnce(&mut dyn DatabaseDriver) -> Result<R, String>,
{
    CONNECTIONS.with(|reg| {
        let mut guard = reg.borrow_mut();
        let driver = guard.get_mut(name).ok_or_else(|| {
            format!("No database connection '{}' found. Call db.open() first.", name)
        })?;
        f(driver.as_mut())
    })
}

pub fn has_driver(name: &str) -> bool {
    CONNECTIONS.with(|reg| reg.borrow().contains_key(name))
}

pub fn list_drivers() -> Vec<String> {
    CONNECTIONS.with(|reg| reg.borrow().keys().cloned().collect())
}

pub fn remove_connection(name: &str) {
    CONNECTIONS.with(|reg| {
        reg.borrow_mut().remove(name);
    });
}
