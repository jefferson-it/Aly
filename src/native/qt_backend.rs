use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use qtrs::prelude::*;

use crate::gui::{
    self as gui_crate,
    backend::{GuiBackend, BackendType},
    widget::{WidgetId, next_widget_id, PROP_LABEL, PROP_TEXT, PROP_VALUE, PROP_CHECKED, PROP_ENABLED, PROP_VISIBLE},
};

pub enum QtWidget {
    Window(Widget, Option<VBoxLayout>, Option<HBoxLayout>),
    Button(PushButton),
    Label(Label),
    Div(Widget, Option<VBoxLayout>, Option<HBoxLayout>),
    Input(LineEdit),
    TextArea(PlainTextEdit),
    PasswordField(LineEdit),
    Checkbox(CheckBox),
    Radio(RadioButton),
    Slider(Slider),
    ProgressBar(ProgressBar),
    Dropdown(ComboBox),
    Separator(Frame),
    Spinner(SpinBox),
    Container(Widget, Option<VBoxLayout>, Option<HBoxLayout>),
}

impl QtWidget {
    fn widget_ptr(&self) -> *mut qtrs::ffi::ffi_inner::QWidget {
        match self {
            QtWidget::Window(w, _, _) => w.widget_ptr(),
            QtWidget::Button(w) => w.widget_ptr(),
            QtWidget::Label(w) => w.widget_ptr(),
            QtWidget::Div(w, _, _) => w.widget_ptr(),
            QtWidget::Input(w) => w.widget_ptr(),
            QtWidget::TextArea(w) => w.widget_ptr(),
            QtWidget::PasswordField(w) => w.widget_ptr(),
            QtWidget::Checkbox(w) => w.widget_ptr(),
            QtWidget::Radio(w) => w.widget_ptr(),
            QtWidget::Slider(w) => w.widget_ptr(),
            QtWidget::ProgressBar(w) => w.widget_ptr(),
            QtWidget::Dropdown(w) => w.widget_ptr(),
            QtWidget::Separator(w) => w.widget_ptr(),
            QtWidget::Spinner(w) => w.widget_ptr(),
            QtWidget::Container(w, _, _) => w.widget_ptr(),
        }
    }

    fn set_has_parent(&mut self) {
        match self {
            QtWidget::Window(w, _, _) => w.set_has_parent(),
            QtWidget::Button(w) => w.set_has_parent(),
            QtWidget::Label(w) => w.set_has_parent(),
            QtWidget::Div(w, _, _) => w.set_has_parent(),
            QtWidget::Input(w) => w.set_has_parent(),
            QtWidget::TextArea(w) => w.set_has_parent(),
            QtWidget::PasswordField(w) => w.set_has_parent(),
            QtWidget::Checkbox(w) => w.set_has_parent(),
            QtWidget::Radio(w) => w.set_has_parent(),
            QtWidget::Slider(w) => w.set_has_parent(),
            QtWidget::ProgressBar(w) => w.set_has_parent(),
            QtWidget::Dropdown(w) => w.set_has_parent(),
            QtWidget::Separator(w) => w.set_has_parent(),
            QtWidget::Spinner(w) => w.set_has_parent(),
            QtWidget::Container(w, _, _) => w.set_has_parent(),
        }
    }
}

pub type SharedQtWidget = Rc<RefCell<QtWidget>>;

thread_local! {
    static QT_WIDGETS: RefCell<HashMap<String, SharedQtWidget>> = RefCell::new(HashMap::new());
    static QT_APP: RefCell<Option<Application>> = RefCell::new(None);
}

fn find_widget(id: &str) -> Option<SharedQtWidget> {
    QT_WIDGETS.with(|w| w.borrow().get(id).cloned())
}

fn store_widget(id: &str, widget: QtWidget) {
    let shared = Rc::new(RefCell::new(widget));
    QT_WIDGETS.with(|w| {
        w.borrow_mut().insert(id.to_owned(), shared);
    });
}

fn ensure_app() {
    QT_APP.with(|a| {
        if a.borrow().is_none() {
            let app = Application::new();
            a.borrow_mut().replace(app);
        }
    });
}

struct WidgetPtrWrapper {
    ptr: *mut qtrs::ffi::ffi_inner::QWidget,
}

impl AsWidget for WidgetPtrWrapper {
    fn widget_ptr(&self) -> *mut qtrs::ffi::ffi_inner::QWidget {
        self.ptr
    }
    fn set_has_parent(&mut self) {}
}

fn do_insert(parent: &SharedQtWidget, child: &SharedQtWidget) {
    let mut parent_borrow = parent.borrow_mut();
    let mut child_borrow = child.borrow_mut();

    let child_ptr = child_borrow.widget_ptr();
    child_borrow.set_has_parent();

    match &mut *parent_borrow {
        QtWidget::Window(_, Some(vbox), None) => {
            vbox.add(WidgetPtrWrapper { ptr: child_ptr });
        }
        QtWidget::Window(_, None, Some(hbox)) => {
            hbox.add(WidgetPtrWrapper { ptr: child_ptr });
        }
        QtWidget::Div(_, Some(vbox), None) => {
            vbox.add(WidgetPtrWrapper { ptr: child_ptr });
        }
        QtWidget::Div(_, None, Some(hbox)) => {
            hbox.add(WidgetPtrWrapper { ptr: child_ptr });
        }
        QtWidget::Container(_, Some(vbox), None) => {
            vbox.add(WidgetPtrWrapper { ptr: child_ptr });
        }
        QtWidget::Container(_, None, Some(hbox)) => {
            hbox.add(WidgetPtrWrapper { ptr: child_ptr });
        }
        _ => {
            eprintln!("Warning: parent widget cannot accept children or has no layout installed.");
        }
    }
}

fn leak_int<F: Fn(i32) + 'static>(f: F) -> u64 {
    let thin: *mut F = Box::into_raw(Box::new(f));
    let fat: *mut dyn Fn(i32) = thin;
    let inner: Box<dyn Fn(i32)> = unsafe { std::mem::transmute(fat) };
    Box::into_raw(Box::new(inner)) as u64
}

pub struct QtBackend;

impl GuiBackend for QtBackend {
    fn backend_type(&self) -> BackendType {
        BackendType::Qt
    }

    fn create_window(&mut self, title: &str, width: i32, height: i32) -> WidgetId {
        ensure_app();
        let id = next_widget_id();
        let mut win = Widget::new().title(title).size(width, height).build();
        let vbox = VBoxLayout::new();
        vbox.set_contents_margins(15, 15, 15, 15);
        vbox.set_spacing(12);
        win.set_vlayout(vbox.layout_ptr());
        store_widget(&id, QtWidget::Window(win, Some(vbox), None));
        id
    }

    fn create_button(&mut self, label: &str) -> WidgetId {
        ensure_app();
        let id = next_widget_id();
        let btn = PushButton::new(label).build();
        store_widget(&id, QtWidget::Button(btn));
        id
    }

    fn create_label(&mut self, text: &str) -> WidgetId {
        ensure_app();
        let id = next_widget_id();
        let lbl = Label::new(text).build();
        store_widget(&id, QtWidget::Label(lbl));
        id
    }

    fn create_div(&mut self, direction: &str) -> WidgetId {
        ensure_app();
        let id = next_widget_id();
        let mut win = Widget::new().build();
        let (vbox, hbox) = if direction == "horizontal" {
            let l = HBoxLayout::new();
            l.set_contents_margins(0, 0, 0, 0);
            l.set_spacing(10);
            win.set_hlayout(l.layout_ptr());
            (None, Some(l))
        } else {
            let l = VBoxLayout::new();
            l.set_contents_margins(0, 0, 0, 0);
            l.set_spacing(10);
            win.set_vlayout(l.layout_ptr());
            (Some(l), None)
        };
        store_widget(&id, QtWidget::Div(win, vbox, hbox));
        id
    }

    fn create_input(&mut self, placeholder: &str) -> WidgetId {
        ensure_app();
        let id = next_widget_id();
        let inp = LineEdit::new(placeholder).build();
        store_widget(&id, QtWidget::Input(inp));
        id
    }

    fn create_textarea(&mut self) -> WidgetId {
        ensure_app();
        let id = next_widget_id();
        let ta = PlainTextEdit::new().build();
        store_widget(&id, QtWidget::TextArea(ta));
        id
    }

    fn create_password(&mut self) -> WidgetId {
        ensure_app();
        let id = next_widget_id();
        let pf = LineEdit::new("").build();
        store_widget(&id, QtWidget::PasswordField(pf));
        id
    }

    fn create_checkbox(&mut self, label: &str) -> WidgetId {
        ensure_app();
        let id = next_widget_id();
        let cb = CheckBox::new(label).build();
        store_widget(&id, QtWidget::Checkbox(cb));
        id
    }

    fn create_radio(&mut self, label: &str) -> WidgetId {
        ensure_app();
        let id = next_widget_id();
        let rb = RadioButton::new(label).build();
        store_widget(&id, QtWidget::Radio(rb));
        id
    }

    fn create_slider(&mut self, min: f64, max: f64, val: f64) -> WidgetId {
        ensure_app();
        let id = next_widget_id();
        let slider = Slider::horizontal()
            .range(min as i32, max as i32)
            .build();
        slider.set_value(val as i32);
        store_widget(&id, QtWidget::Slider(slider));
        id
    }

    fn create_progressbar(&mut self, val: f64) -> WidgetId {
        ensure_app();
        let id = next_widget_id();
        let pb = ProgressBar::new()
            .range(0, 100)
            .value((val * 100.0) as i32)
            .build();
        store_widget(&id, QtWidget::ProgressBar(pb));
        id
    }

    fn create_dropdown(&mut self, items: &[String]) -> WidgetId {
        ensure_app();
        let id = next_widget_id();
        let item_refs: Vec<&str> = items.iter().map(|s| s.as_str()).collect();
        let cb = ComboBox::new().items(&item_refs).build();
        store_widget(&id, QtWidget::Dropdown(cb));
        id
    }

    fn create_separator(&mut self) -> WidgetId {
        ensure_app();
        let id = next_widget_id();
        let frame = Frame::new()
            .set_frame_shape(qtrs::frame::H_LINE)
            .set_frame_shadow(qtrs::frame::SUNKEN)
            .build();
        store_widget(&id, QtWidget::Separator(frame));
        id
    }

    fn create_spinner(&mut self, min: f64, max: f64, _step: f64, val: f64) -> WidgetId {
        ensure_app();
        let id = next_widget_id();
        let sb = SpinBox::new()
            .range(min as i32, max as i32)
            .value(val as i32)
            .build();
        store_widget(&id, QtWidget::Spinner(sb));
        id
    }

    fn create_container(&mut self, width: i32, height: i32, direction: &str) -> WidgetId {
        ensure_app();
        let id = next_widget_id();
        let mut win = Widget::new().size(width, height).build();
        let (vbox, hbox) = if direction == "horizontal" {
            let l = HBoxLayout::new();
            l.set_contents_margins(0, 0, 0, 0);
            l.set_spacing(10);
            win.set_hlayout(l.layout_ptr());
            (None, Some(l))
        } else {
            let l = VBoxLayout::new();
            l.set_contents_margins(0, 0, 0, 0);
            l.set_spacing(10);
            win.set_vlayout(l.layout_ptr());
            (Some(l), None)
        };
        store_widget(&id, QtWidget::Container(win, vbox, hbox));
        id
    }

    fn insert_widget(&mut self, parent: &WidgetId, child: &WidgetId) {
        let p = find_widget(parent);
        let c = find_widget(child);
        match (p, c) {
            (Some(pw), Some(cw)) => do_insert(&pw, &cw),
            _ => eprintln!("RuntimeError [gui]: parent or child widget not found."),
        }
    }

    fn set_property(&mut self, id: &WidgetId, prop: &str, value: &str) {
        let widget_opt = find_widget(id);
        if let Some(shared) = widget_opt {
            let mut borrow = shared.borrow_mut();
            match prop {
                PROP_LABEL | PROP_TEXT | PROP_VALUE => {
                    match &mut *borrow {
                        QtWidget::Window(w, _, _) => w.set_title(value),
                        QtWidget::Button(b) => b.set_text(value),
                        QtWidget::Label(l) => l.set_text(value),
                        QtWidget::Input(i) => i.set_text(value),
                        QtWidget::TextArea(t) => t.set_plain_text(value),
                        QtWidget::PasswordField(p) => p.set_text(value),
                        QtWidget::Slider(s) => {
                            if let Ok(v) = value.parse::<i32>() {
                                s.set_value(v);
                            }
                        }
                        QtWidget::ProgressBar(p) => {
                            if let Ok(v) = value.parse::<i32>() {
                                p.set_value(v);
                            }
                        }
                        QtWidget::Spinner(s) => {
                            if let Ok(v) = value.parse::<i32>() {
                                s.set_value(v);
                            }
                        }
                        _ => {}
                    }
                }
                PROP_CHECKED => {
                    let checked = value == "true" || value == "1";
                    match &mut *borrow {
                        QtWidget::Checkbox(c) => c.set_checked(checked),
                        QtWidget::Radio(r) => r.set_checked(checked),
                        _ => {}
                    }
                }
                PROP_ENABLED => {
                    let enabled = value != "false";
                    let ptr = borrow.widget_ptr();
                    unsafe { qtrs::ffi::QWidget_setEnabled(ptr, enabled); }
                }
                PROP_VISIBLE => {
                    let visible = value != "false";
                    let ptr = borrow.widget_ptr();
                    unsafe { qtrs::ffi::QWidget_setVisible(ptr, visible); }
                }
                _ => {}
            }
        }
    }

    fn get_property(&mut self, id: &WidgetId, prop: &str) -> String {
        let widget_opt = find_widget(id);
        if let Some(shared) = widget_opt {
            let mut borrow = shared.borrow_mut();
            match prop {
                PROP_LABEL | PROP_TEXT | PROP_VALUE => {
                    return match &mut *borrow {
                        QtWidget::Window(_, _, _) => "Window".to_owned(),
                        QtWidget::Button(b) => b.text().to_owned(),
                        QtWidget::Label(l) => l.text().to_owned(),
                        QtWidget::Input(i) => {
                            i.refresh_text();
                            i.text().to_owned()
                        }
                        QtWidget::PasswordField(p) => {
                            p.refresh_text();
                            p.text().to_owned()
                        }
                        QtWidget::TextArea(t) => t.plain_text(),
                        QtWidget::Checkbox(c) => c.is_checked().to_string(),
                        QtWidget::Radio(r) => r.is_checked().to_string(),
                        QtWidget::Slider(s) => s.value().to_string(),
                        QtWidget::ProgressBar(p) => p.value().to_string(),
                        QtWidget::Spinner(s) => s.value().to_string(),
                        QtWidget::Dropdown(d) => d.current_text(),
                        _ => "None".to_owned(),
                    };
                }
                PROP_CHECKED => {
                    return match &mut *borrow {
                        QtWidget::Checkbox(c) => c.is_checked().to_string(),
                        QtWidget::Radio(r) => r.is_checked().to_string(),
                        _ => "false".to_owned(),
                    };
                }
                _ => {}
            }
        }
        "None".to_owned()
    }

    fn on_event(&mut self, id: &WidgetId, event: &str, callback: String) {
        let shared = match find_widget(id) {
            Some(w) => w,
            None => {
                eprintln!("RuntimeError [gui]: widget '{}' not found for event.", id);
                return;
            }
        };

        gui_crate::event::set_callback(id, event, callback);

        let id_clone = id.to_owned();

        match event {
            "onClick" => {
                let mut borrow = shared.borrow_mut();
                if let QtWidget::Button(btn) = &mut *borrow {
                    btn.connect_clicked(move || {
                        gui_crate::event::fire_callback(&id_clone, "onClick");
                    });
                }
            }
            "onChange" => {
                let mut borrow = shared.borrow_mut();
                match &mut *borrow {
                    QtWidget::Input(inp) => {
                        inp.connect_return_pressed(move || {
                            gui_crate::event::fire_callback(&id_clone, "onChange");
                        });
                    }
                    QtWidget::PasswordField(inp) => {
                        inp.connect_return_pressed(move || {
                            gui_crate::event::fire_callback(&id_clone, "onChange");
                        });
                    }
                    QtWidget::Checkbox(cb) => {
                        cb.connect_toggled(move |_checked| {
                            gui_crate::event::fire_callback(&id_clone, "onChange");
                        });
                    }
                    QtWidget::Radio(rb) => {
                        rb.connect_toggled(move |_checked| {
                            gui_crate::event::fire_callback(&id_clone, "onChange");
                        });
                    }
                    QtWidget::Dropdown(cb) => {
                        cb.connect_current_index_changed(move |_idx| {
                            gui_crate::event::fire_callback(&id_clone, "onChange");
                        });
                    }
                    QtWidget::Slider(s) => {
                        let id_c = id_clone.clone();
                        let token = leak_int(move |_val| {
                            gui_crate::event::fire_callback(&id_c, "onChange");
                        });
                        unsafe {
                            qtrs::ffi::QSlider_onValueChanged(s.widget_ptr() as *mut _, token);
                        }
                    }
                    QtWidget::Spinner(s) => {
                        let id_c = id_clone.clone();
                        let token = leak_int(move |_val| {
                            gui_crate::event::fire_callback(&id_c, "onChange");
                        });
                        unsafe {
                            qtrs::ffi::QSpinBox_onValueChanged(s.widget_ptr() as *mut _, token);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn run(&mut self) {
        ensure_app();
        QT_APP.with(|a| {
            if let Some(app) = &*a.borrow() {
                app.exec();
            }
        });
    }

    // System tray
    fn set_tray_icon(&mut self, window_id: &WidgetId, icon_path: &str, tooltip: &str) -> bool {
        use qtrs::SystemTrayIcon;
        if let Some(shared) = find_widget(window_id) {
            if let QtWidget::Window(_, _, _) = &*shared.borrow() {
                let tray = SystemTrayIcon::new();
                if std::path::Path::new(icon_path).exists() {
                    tray.set_icon(icon_path);
                }
                tray.set_tool_tip(tooltip);
                tray.set_visible(true);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn show_notification(&mut self, title: &str, body: &str, icon: Option<&str>) {
        // Qt doesn't have native notification API, use system tray
        use qtrs::SystemTrayIcon;
        let tray = SystemTrayIcon::new();
        if let Some(icon_path) = icon {
            if std::path::Path::new(icon_path).exists() {
                tray.set_icon(icon_path);
            }
        }
        tray.show_message(title, body, qtrs::MessageIcon::Information, 5000);
    }

    fn clipboard_set(&mut self, text: &str) {
        use qtrs::Clipboard;
        Clipboard::set_text(text);
    }

    fn clipboard_get(&mut self) -> String {
        use qtrs::Clipboard;
        Clipboard::text().unwrap_or_default()
    }

    fn drag_drop_init(&mut self, widget_id: &WidgetId, data: &str) {
        use qtrs::MimeData;
        if let Some(shared) = find_widget(widget_id) {
            let mut borrow = shared.borrow_mut();
            if let QtWidget::Button(btn) = &mut *borrow {
                let mime = MimeData::new();
                mime.set_text(data);
                btn.set_mime_data(mime);
            }
        }
    }

    fn drag_drop_accept(&mut self, widget_id: &WidgetId, types: &[&str]) {
        // Qt handles drag and drop through event filters
        if let Some(shared) = find_widget(widget_id) {
            let mut borrow = shared.borrow_mut();
            if let QtWidget::Button(btn) = &mut *borrow {
                btn.set_accept_drops(true);
            }
        }
    }

    // OpenGL
    fn gl_context_create(&mut self, window_id: &WidgetId) -> bool {
        use qtrs::OpenGLContext;
        if let Some(shared) = find_widget(window_id) {
            if let QtWidget::Window(win, _, _) = &*shared.borrow() {
                let ctx = OpenGLContext::new();
                ctx.set_widget(win);
                ctx.create();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn gl_make_current(&mut self, window_id: &WidgetId) -> bool {
        use qtrs::OpenGLContext;
        if let Some(shared) = find_widget(window_id) {
            if let QtWidget::Window(win, _, _) = &*shared.borrow() {
                if let Some(ctx) = OpenGLContext::current_context() {
                    ctx.make_current(win);
                    return true;
                }
            }
        }
        false
    }

    fn gl_swap_buffers(&mut self, window_id: &WidgetId) {
        use qtrs::OpenGLContext;
        if let Some(shared) = find_widget(window_id) {
            if let QtWidget::Window(win, _, _) = &*shared.borrow() {
                if let Some(ctx) = OpenGLContext::current_context() {
                    ctx.swap_buffers(win);
                }
            }
        }
    }

    fn gl_get_proc_address(&mut self, proc_name: &str) -> *const std::ffi::c_void {
        use qtrs::OpenGLContext;
        OpenGLContext::get_proc_address(proc_name) as *const std::ffi::c_void
    }
}