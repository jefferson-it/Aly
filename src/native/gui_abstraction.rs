use crate::gui::{with_backend, register_backend, is_backend_registered};
use crate::native::{
    std::{arg, split_args},
    types::Validator,
};
use crate::validators::str::put_quoted_str;

#[cfg(target_os = "ios")]
use super::ios::IosBackend;
use super::gui_fltk::FltkBackend;
use super::gtk_backend::Gtk4Backend;
use super::dom_backend::DomBackend;
use super::qt_backend::QtBackend;
#[cfg(feature = "wayland-backend")]
fn ensure_backend() {
    if !is_backend_registered() {
        #[cfg(target_os = "ios")]
        register_backend(Box::new(IosBackend));
        #[cfg(not(target_os = "ios"))]
        register_backend(Box::new(FltkBackend));
    }
}

pub fn gui_use_backend(params: String) -> Box<dyn Validator> {
    let args = split_args(&params, 1);
    let backend_name = arg(&args, 0);
    match backend_name.to_lowercase().as_str() {
        "fltk" => {
            register_backend(Box::new(FltkBackend));
        }
        "gtk" | "gtk4" => {
            register_backend(Box::new(Gtk4Backend));
        }
        "dom" | "web" | "html" => {
            register_backend(Box::new(DomBackend));
        }
        "qt" | "qt6" => {
            register_backend(Box::new(QtBackend));
        }
        #[cfg(target_os = "ios")]
        "ios" | "cocoa" | "uikit" => {
            register_backend(Box::new(IosBackend));
        }
        #[cfg(feature = "wayland-backend")]
        "wayland" | "wl" => {
            register_backend(Box::new(WaylandBackend));
        }
        _ => {
            #[cfg(target_os = "ios")]
            register_backend(Box::new(IosBackend));
            #[cfg(not(target_os = "ios"))]
            register_backend(Box::new(FltkBackend));
        }
    }
    Box::new(put_quoted_str("OK".to_string()))
}

pub fn gui_window(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 3);
    let title = arg(&args, 0);
    let width: i32 = arg(&args, 1).parse().unwrap_or(400);
    let height: i32 = arg(&args, 2).parse().unwrap_or(500);
    let id = with_backend(|b| b.create_window(&title, width, height));
    Box::new(put_quoted_str(id))
}

pub fn gui_button(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 1);
    let label = arg(&args, 0);
    let id = with_backend(|b| b.create_button(&label));
    Box::new(put_quoted_str(id))
}

pub fn gui_label(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 1);
    let text = arg(&args, 0);
    let id = with_backend(|b| b.create_label(&text));
    Box::new(put_quoted_str(id))
}

pub fn gui_div(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 1);
    let direction = arg(&args, 0);
    let dir = if direction.is_empty() { "vertical" } else { &direction };
    let id = with_backend(|b| b.create_div(dir));
    Box::new(put_quoted_str(id))
}

pub fn gui_input(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 1);
    let placeholder = arg(&args, 0);
    let id = with_backend(|b| b.create_input(&placeholder));
    Box::new(put_quoted_str(id))
}

pub fn gui_textarea(_params: String) -> Box<dyn Validator> {
    ensure_backend();
    let id = with_backend(|b| b.create_textarea());
    Box::new(put_quoted_str(id))
}

pub fn gui_password(_params: String) -> Box<dyn Validator> {
    ensure_backend();
    let id = with_backend(|b| b.create_password());
    Box::new(put_quoted_str(id))
}

pub fn gui_checkbox(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 1);
    let label = arg(&args, 0);
    let id = with_backend(|b| b.create_checkbox(&label));
    Box::new(put_quoted_str(id))
}

pub fn gui_radio(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 1);
    let label = arg(&args, 0);
    let id = with_backend(|b| b.create_radio(&label));
    Box::new(put_quoted_str(id))
}

pub fn gui_slider(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 3);
    let min: f64 = arg(&args, 0).parse().unwrap_or(0.0);
    let max: f64 = arg(&args, 1).parse().unwrap_or(100.0);
    let val: f64 = arg(&args, 2).parse().unwrap_or(50.0);
    let id = with_backend(|b| b.create_slider(min, max, val));
    Box::new(put_quoted_str(id))
}

pub fn gui_progressbar(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 1);
    let val: f64 = arg(&args, 0).parse().unwrap_or(0.0);
    let id = with_backend(|b| b.create_progressbar(val));
    Box::new(put_quoted_str(id))
}

pub fn gui_dropdown(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 0);
    let items: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    let id = with_backend(|b| b.create_dropdown(&items));
    Box::new(put_quoted_str(id))
}

pub fn gui_spinner(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 4);
    let min: f64 = arg(&args, 0).parse().unwrap_or(0.0);
    let max: f64 = arg(&args, 1).parse().unwrap_or(100.0);
    let step: f64 = arg(&args, 2).parse().unwrap_or(1.0);
    let val: f64 = arg(&args, 3).parse().unwrap_or(0.0);
    let id = with_backend(|b| b.create_spinner(min, max, step, val));
    Box::new(put_quoted_str(id))
}

pub fn gui_container(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 3);
    let width: i32 = arg(&args, 0).parse().unwrap_or(380);
    let height: i32 = arg(&args, 1).parse().unwrap_or(300);
    let direction = arg(&args, 2);
    let dir = if direction.is_empty() { "vertical" } else { &direction };
    let id = with_backend(|b| b.create_container(width, height, dir));
    Box::new(put_quoted_str(id))
}

pub fn gui_insert(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 2);
    let parent = arg(&args, 0);
    let child = arg(&args, 1);
    with_backend(|b| b.insert_widget(&parent, &child));
    Box::new(put_quoted_str("OK".to_string()))
}

pub fn gui_set(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 3);
    let id = arg(&args, 0);
    let prop = arg(&args, 1);
    let value = arg(&args, 2);
    with_backend(|b| b.set_property(&id, &prop, &value));
    Box::new(put_quoted_str("OK".to_string()))
}

pub fn gui_get(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 2);
    let id = arg(&args, 0);
    let prop = arg(&args, 1);
    let value = with_backend(|b| b.get_property(&id, &prop));
    Box::new(put_quoted_str(value))
}

pub fn gui_on(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 3);
    let id = arg(&args, 0);
    let event = arg(&args, 1);
    let callback = arg(&args, 2);
    with_backend(|b| b.on_event(&id, &event, callback));
    Box::new(put_quoted_str("OK".to_string()))
}

pub fn gui_run(_params: String) -> Box<dyn Validator> {
    ensure_backend();
    with_backend(|b| b.run());
    Box::new(put_quoted_str("OK".to_string()))
}

// System tray
pub fn gui_tray_icon(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 3);
    let window_id = arg(&args, 0);
    let icon_path = arg(&args, 1);
    let tooltip = arg(&args, 2);
    let result = with_backend(|b| b.set_tray_icon(&window_id, &icon_path, &tooltip));
    Box::new(put_quoted_str(if result { "true" } else { "false" }))
}

// Notifications
pub fn gui_notify(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 3);
    let title = arg(&args, 0);
    let body = arg(&args, 1);
    let icon = arg(&args, 2);
    let icon_opt = if icon.is_empty() { None } else { Some(icon.as_str()) };
    with_backend(|b| b.show_notification(&title, &body, icon_opt));
    Box::new(put_quoted_str("OK".to_string()))
}

// Clipboard
pub fn gui_clipboard_set(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 1);
    let text = arg(&args, 0);
    with_backend(|b| b.clipboard_set(&text));
    Box::new(put_quoted_str("OK".to_string()))
}

pub fn gui_clipboard_get(_params: String) -> Box<dyn Validator> {
    ensure_backend();
    let text = with_backend(|b| b.clipboard_get());
    Box::new(put_quoted_str(text))
}

// Drag and Drop
pub fn gui_drag_init(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 2);
    let widget_id = arg(&args, 0);
    let data = arg(&args, 1);
    with_backend(|b| b.drag_drop_init(&widget_id, &data));
    Box::new(put_quoted_str("OK".to_string()))
}

pub fn gui_drag_accept(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 0);
    let widget_id = arg(&args, 0);
    let types: Vec<&str> = args.iter().skip(1).map(|s| s.as_str()).collect();
    with_backend(|b| b.drag_drop_accept(&widget_id, &types));
    Box::new(put_quoted_str("OK".to_string()))
}

// OpenGL
pub fn gui_gl_create(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 1);
    let window_id = arg(&args, 0);
    let result = with_backend(|b| b.gl_context_create(&window_id));
    Box::new(put_quoted_str(if result { "true" } else { "false" }))
}

pub fn gui_gl_make_current(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 1);
    let window_id = arg(&args, 0);
    let result = with_backend(|b| b.gl_make_current(&window_id));
    Box::new(put_quoted_str(if result { "true" } else { "false" }))
}

pub fn gui_gl_swap(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 1);
    let window_id = arg(&args, 0);
    with_backend(|b| b.gl_swap_buffers(&window_id));
    Box::new(put_quoted_str("OK".to_string()))
}

pub fn gui_gl_proc(params: String) -> Box<dyn Validator> {
    ensure_backend();
    let args = split_args(&params, 1);
    let proc_name = arg(&args, 0);
    let ptr = with_backend(|b| b.gl_get_proc_address(&proc_name));
    Box::new(put_quoted_str(format!("{:p}", ptr)))
}
