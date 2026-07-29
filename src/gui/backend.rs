use std::sync::atomic::{AtomicU8, Ordering};

use super::widget::WidgetId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BackendType {
    Fltk = 0,
    Gtk4 = 1,
    Dom = 2,
    Qt = 3,
    Wayland = 4,
}

impl BackendType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "gtk" | "gtk4" => Self::Gtk4,
            "dom" | "web" | "html" => Self::Dom,
            "qt" | "qt6" => Self::Qt,
            "wayland" | "wl" => Self::Wayland,
            _ => Self::Fltk,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Fltk => "FLTK",
            Self::Gtk4 => "GTK4",
            Self::Dom => "DOM/Web",
            Self::Qt => "Qt",
            Self::Wayland => "Wayland",
        }
    }
}

static CURRENT_BACKEND: AtomicU8 = AtomicU8::new(0);

pub fn set_backend_type(bt: BackendType) {
    CURRENT_BACKEND.store(bt as u8, Ordering::Relaxed);
}

pub fn get_backend_type() -> BackendType {
    match CURRENT_BACKEND.load(Ordering::Relaxed) {
        1 => BackendType::Gtk4,
        2 => BackendType::Dom,
        3 => BackendType::Qt,
        4 => BackendType::Wayland,
        _ => BackendType::Fltk,
    }
}

pub trait GuiBackend {
    fn backend_type(&self) -> BackendType;

    fn create_window(&mut self, title: &str, width: i32, height: i32) -> WidgetId;
    fn create_button(&mut self, label: &str) -> WidgetId;
    fn create_label(&mut self, text: &str) -> WidgetId;
    fn create_div(&mut self, direction: &str) -> WidgetId;
    fn create_input(&mut self, placeholder: &str) -> WidgetId;
    fn create_textarea(&mut self) -> WidgetId;
    fn create_password(&mut self) -> WidgetId;
    fn create_checkbox(&mut self, label: &str) -> WidgetId;
    fn create_radio(&mut self, label: &str) -> WidgetId;
    fn create_slider(&mut self, min: f64, max: f64, val: f64) -> WidgetId;
    fn create_progressbar(&mut self, val: f64) -> WidgetId;
    fn create_dropdown(&mut self, items: &[String]) -> WidgetId;
    fn create_separator(&mut self) -> WidgetId;
    fn create_spinner(&mut self, min: f64, max: f64, step: f64, val: f64) -> WidgetId;
    fn create_container(&mut self, width: i32, height: i32, direction: &str) -> WidgetId;

    fn insert_widget(&mut self, parent: &WidgetId, child: &WidgetId);

    fn set_property(&mut self, id: &WidgetId, prop: &str, value: &str);
    fn get_property(&mut self, id: &WidgetId, prop: &str) -> String;

    fn on_event(&mut self, id: &WidgetId, event: &str, callback: String);

    // System integration
    fn set_tray_icon(&mut self, window_id: &WidgetId, icon_path: &str, tooltip: &str) -> bool;
    fn show_notification(&mut self, title: &str, body: &str, icon: Option<&str>);
    fn clipboard_set(&mut self, text: &str);
    fn clipboard_get(&mut self) -> String;
    fn drag_drop_init(&mut self, widget_id: &WidgetId, data: &str);
    fn drag_drop_accept(&mut self, widget_id: &WidgetId, types: &[&str]);

    // Graphics APIs
    fn gl_context_create(&mut self, window_id: &WidgetId) -> bool;
    fn gl_make_current(&mut self, window_id: &WidgetId) -> bool;
    fn gl_swap_buffers(&mut self, window_id: &WidgetId);
    fn gl_get_proc_address(&mut self, proc_name: &str) -> *const std::ffi::c_void;

    fn run(&mut self);
}

use std::cell::RefCell;

thread_local! {
    static BACKEND: RefCell<Option<Box<dyn GuiBackend>>> = RefCell::new(None);
}

pub fn register_backend(backend: Box<dyn GuiBackend>) {
    BACKEND.with(|b| {
        let bt = backend.backend_type();
        set_backend_type(bt);
        b.borrow_mut().replace(backend);
    });
}

pub fn with_backend<F, R>(f: F) -> R
where
    F: FnOnce(&mut dyn GuiBackend) -> R,
{
    BACKEND.with(|b| {
        let mut guard = b.borrow_mut();
        let backend = guard.as_mut().expect("No GUI backend registered. Call gui.useBackend() first.");
        f(backend.as_mut())
    })
}

pub fn is_backend_registered() -> bool {
    BACKEND.with(|b| b.borrow().is_some())
}
