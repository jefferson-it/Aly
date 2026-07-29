use std::collections::HashMap;
use std::cell::RefCell;

thread_local! {
    static CALLBACKS: RefCell<HashMap<String, HashMap<String, String>>> = RefCell::new(HashMap::new());
}

pub fn set_callback(widget_id: &str, event: &str, func_name: String) {
    CALLBACKS.with(|c| {
        c.borrow_mut()
            .entry(widget_id.to_owned())
            .or_default()
            .insert(event.to_owned(), func_name);
    });
}

pub fn get_callback(widget_id: &str, event: &str) -> Option<String> {
    CALLBACKS.with(|c| {
        c.borrow().get(widget_id)
            .and_then(|m| m.get(event).cloned())
    })
}

pub fn remove_callback(widget_id: &str, event: &str) {
    CALLBACKS.with(|c| {
        c.borrow_mut()
            .entry(widget_id.to_owned())
            .or_default()
            .remove(event);
    });
}

pub fn clear_callbacks(widget_id: &str) {
    CALLBACKS.with(|c| {
        c.borrow_mut().remove(widget_id);
    });
}

pub fn fire_callback(widget_id: &str, event: &str) {
    let func_name = get_callback(widget_id, event);
    if let Some(name) = func_name {
        let run = crate::aly::get_runtime();
        let fake_lexer = vec![
            crate::lexer::Lexer::new(crate::tokens::Tokens::Identifier, name, 0),
        ];
        let _ = run.function_run(fake_lexer);
    }
}
