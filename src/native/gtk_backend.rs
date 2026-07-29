use gtk4::prelude::*;
use gtk4::{
    self, glib,
    Application, ApplicationWindow, Box as GtkBox, Button, CssProvider,
    CheckButton, DropDown, Entry, Label, Notebook, Orientation,
    ProgressBar, Scale, Separator, TextView,
};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::gui::{
    self as gui_crate,
    backend::{GuiBackend, BackendType},
    widget::{WidgetId, next_widget_id, PROP_LABEL, PROP_TEXT, PROP_VALUE, PROP_CHECKED, PROP_ENABLED, PROP_VISIBLE},
};

#[derive(Clone)]
pub enum GtkWidget {
    Window(ApplicationWindow),
    Button(Button),
    Label(Label),
    Div(GtkBox),
    Input(Entry),
    TextArea(TextView),
    PasswordField(Entry),
    Checkbox(CheckButton),
    Radio(CheckButton),
    Slider(Scale),
    ProgressBar(ProgressBar),
    Dropdown(DropDown),
    Tabs(Notebook),
    Separator(Separator),
}

impl GtkWidget {
    fn as_gtk_widget(&self) -> &gtk4::Widget {
        match self {
            GtkWidget::Window(w) => w.upcast_ref(),
            GtkWidget::Button(b) => b.upcast_ref(),
            GtkWidget::Label(l) => l.upcast_ref(),
            GtkWidget::Div(b) => b.upcast_ref(),
            GtkWidget::Input(e) => e.upcast_ref(),
            GtkWidget::TextArea(tv) => tv.upcast_ref(),
            GtkWidget::PasswordField(e) => e.upcast_ref(),
            GtkWidget::Checkbox(c) => c.upcast_ref(),
            GtkWidget::Radio(c) => c.upcast_ref(),
            GtkWidget::Slider(s) => s.upcast_ref(),
            GtkWidget::ProgressBar(p) => p.upcast_ref(),
            GtkWidget::Dropdown(d) => d.upcast_ref(),
            GtkWidget::Tabs(n) => n.upcast_ref(),
            GtkWidget::Separator(s) => s.upcast_ref(),
        }
    }
}

pub type SharedGtkWidget = Rc<RefCell<GtkWidget>>;

thread_local! {
    static GTK_APP: RefCell<Option<Application>> = RefCell::new(None);
    static GTK_WIDGETS: RefCell<HashMap<String, SharedGtkWidget>> = RefCell::new(HashMap::new());
    static GTK_CSS: RefCell<String> = RefCell::new(String::new());
    static GTK_CSS_PROVIDER: RefCell<Option<CssProvider>> = RefCell::new(None);
    static GTK_RUN: RefCell<bool> = RefCell::new(false);
}

fn find_widget(id: &str) -> Option<SharedGtkWidget> {
    GTK_WIDGETS.with(|w| w.borrow().get(id).cloned())
}

fn store_widget(id: &str, widget: GtkWidget) {
    let shared = Rc::new(RefCell::new(widget));
    GTK_WIDGETS.with(|w| {
        w.borrow_mut().insert(id.to_owned(), shared);
    });
}

fn get_or_create_app() -> Application {
    GTK_APP.with(|a| {
        if a.borrow().is_none() {
            let app = Application::builder()
                .application_id("com.aly.lang.app")
                .build();
            a.borrow_mut().replace(app);
        }
        a.borrow().as_ref().cloned().expect("GTK app should exist")
    })
}

fn apply_css(css_content: &str) {
    if css_content.trim().is_empty() {
        return;
    }
    let provider = CssProvider::new();
    provider.load_from_data(css_content);
    GTK_CSS_PROVIDER.with(|p| p.borrow_mut().replace(provider.clone()));
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

pub struct Gtk4Backend;

impl GuiBackend for Gtk4Backend {
    fn backend_type(&self) -> BackendType {
        BackendType::Gtk4
    }

    fn create_window(&mut self, title: &str, width: i32, height: i32) -> WidgetId {
        let id = next_widget_id();
        let app = get_or_create_app();
        let win = ApplicationWindow::builder()
            .application(&app)
            .title(title)
            .default_width(width)
            .default_height(height)
            .build();
        store_widget(&id, GtkWidget::Window(win));
        id
    }

    fn create_button(&mut self, label: &str) -> WidgetId {
        let id = next_widget_id();
        let btn = Button::with_label(label);
        store_widget(&id, GtkWidget::Button(btn));
        id
    }

    fn create_label(&mut self, text: &str) -> WidgetId {
        let id = next_widget_id();
        let lbl = Label::new(Some(text));
        store_widget(&id, GtkWidget::Label(lbl));
        id
    }

    fn create_div(&mut self, _direction: &str) -> WidgetId {
        let id = next_widget_id();
        let box_widget = GtkBox::new(Orientation::Vertical, 0);
        store_widget(&id, GtkWidget::Div(box_widget));
        id
    }

    fn create_input(&mut self, placeholder: &str) -> WidgetId {
        let id = next_widget_id();
        let entry = Entry::new();
        if !placeholder.is_empty() {
            entry.set_placeholder_text(Some(placeholder));
        }
        store_widget(&id, GtkWidget::Input(entry));
        id
    }

    fn create_textarea(&mut self) -> WidgetId {
        let id = next_widget_id();
        let tv = TextView::new();
        store_widget(&id, GtkWidget::TextArea(tv));
        id
    }

    fn create_password(&mut self) -> WidgetId {
        let id = next_widget_id();
        let entry = Entry::new();
        entry.set_visibility(false);
        store_widget(&id, GtkWidget::PasswordField(entry));
        id
    }

    fn create_checkbox(&mut self, label: &str) -> WidgetId {
        let id = next_widget_id();
        let cb = CheckButton::with_label(label);
        store_widget(&id, GtkWidget::Checkbox(cb));
        id
    }

    fn create_radio(&mut self, label: &str) -> WidgetId {
        let id = next_widget_id();
        let radio = CheckButton::with_label(label);
        store_widget(&id, GtkWidget::Radio(radio));
        id
    }

    fn create_slider(&mut self, min: f64, max: f64, val: f64) -> WidgetId {
        let id = next_widget_id();
        let adj = gtk4::Adjustment::new(val, min, max, 1.0, 10.0, 0.0);
        let scale = Scale::new(Orientation::Horizontal, Some(&adj));
        scale.set_value(val);
        store_widget(&id, GtkWidget::Slider(scale));
        id
    }

    fn create_progressbar(&mut self, val: f64) -> WidgetId {
        let id = next_widget_id();
        let pb = ProgressBar::new();
        pb.set_fraction(val);
        store_widget(&id, GtkWidget::ProgressBar(pb));
        id
    }

    fn create_dropdown(&mut self, items: &[String]) -> WidgetId {
        let id = next_widget_id();
        let item_refs: Vec<&str> = items.iter().map(|s| s.as_str()).collect();
        let dd = DropDown::from_strings(&item_refs);
        store_widget(&id, GtkWidget::Dropdown(dd));
        id
    }

    fn create_separator(&mut self) -> WidgetId {
        let id = next_widget_id();
        let sep = Separator::new(Orientation::Horizontal);
        store_widget(&id, GtkWidget::Separator(sep));
        id
    }

    fn create_spinner(&mut self, _min: f64, _max: f64, _step: f64, _val: f64) -> WidgetId {
        let id = next_widget_id();
        let entry = Entry::new();
        store_widget(&id, GtkWidget::Input(entry));
        id
    }

    fn create_container(&mut self, _width: i32, _height: i32, _direction: &str) -> WidgetId {
        self.create_div("vertical")
    }

    fn insert_widget(&mut self, parent: &WidgetId, child: &WidgetId) {
        let pw = match find_widget(parent) {
            Some(w) => w,
            None => { return; }
        };
        let cw = match find_widget(child) {
            Some(w) => w,
            None => { return; }
        };

        let p = pw.borrow();
        match &*p {
            GtkWidget::Window(win) => {
                win.set_child(Some(&cw.borrow().as_gtk_widget().clone()));
            }
            GtkWidget::Div(box_widget) => {
                box_widget.append(&cw.borrow().as_gtk_widget().clone());
            }
            GtkWidget::Tabs(nb) => {
                let label = Label::new(Some("Tab"));
                let child_clone = cw.borrow().as_gtk_widget().clone();
                nb.append_page(&child_clone, Some(&label));
            }
            _ => {}
        }
    }

    fn set_property(&mut self, id: &WidgetId, prop: &str, value: &str) {
        let shared = match find_widget(id) {
            Some(w) => w,
            None => { return; }
        };
        let mut borrow = shared.borrow_mut();

        match prop {
            PROP_LABEL | PROP_TEXT | PROP_VALUE => match &mut *borrow {
                GtkWidget::Button(b) => b.set_label(value),
                GtkWidget::Label(l) => l.set_label(value),
                GtkWidget::Input(e) => e.set_text(value),
                GtkWidget::TextArea(tv) => {
                    let buf = tv.buffer();
                    buf.set_text(value);
                }
                GtkWidget::PasswordField(e) => e.set_text(value),
                GtkWidget::Checkbox(c) => c.set_label(Some(value)),
                GtkWidget::Radio(r) => r.set_label(Some(value)),
                _ => {}
            },
            PROP_CHECKED => match &mut *borrow {
                GtkWidget::Checkbox(c) => c.set_active(value == "true" || value == "1"),
                GtkWidget::Radio(r) => r.set_active(value == "true" || value == "1"),
                _ => {}
            },
            PROP_ENABLED => match &mut *borrow {
                GtkWidget::Button(b) => b.set_sensitive(value != "false"),
                GtkWidget::Input(e) => e.set_sensitive(value != "false"),
                GtkWidget::Slider(s) => s.set_sensitive(value != "false"),
                GtkWidget::Checkbox(c) => c.set_sensitive(value != "false"),
                _ => {}
            },
            PROP_VISIBLE => match &mut *borrow {
                GtkWidget::Window(w) => w.set_visible(value != "false"),
                _ => {}
            },
            "fraction" => match &mut *borrow {
                GtkWidget::ProgressBar(p) => {
                    if let Ok(v) = value.parse::<f64>() {
                        p.set_fraction(v);
                    }
                }
                _ => {}
            },
            "class" => {
                drop(borrow);
                if let Some(w) = find_widget(id) {
                    let b = w.borrow();
                    b.as_gtk_widget().add_css_class(value);
                }
            }
            _ => {}
        }
    }

    fn get_property(&mut self, id: &WidgetId, prop: &str) -> String {
        let shared = match find_widget(id) {
            Some(w) => w,
            None => { return "None".to_owned(); }
        };
        let borrow = shared.borrow();

        match prop {
            PROP_LABEL | PROP_TEXT | PROP_VALUE => match &*borrow {
                GtkWidget::Button(b) => b.label().map(|s| s.to_string()).unwrap_or_default(),
                GtkWidget::Label(l) => l.label().to_string(),
                GtkWidget::Input(e) => e.text().to_string(),
                GtkWidget::TextArea(tv) => {
                    let buf = tv.buffer();
                    let start = buf.start_iter();
                    let end = buf.end_iter();
                    buf.text(&start, &end, false).to_string()
                }
                GtkWidget::PasswordField(e) => e.text().to_string(),
                GtkWidget::Checkbox(c) => c.label().map(|s| s.to_string()).unwrap_or_default(),
                GtkWidget::Radio(r) => r.label().map(|s| s.to_string()).unwrap_or_default(),
                _ => "None".to_owned(),
            },
            PROP_CHECKED => match &*borrow {
                GtkWidget::Checkbox(c) => if c.is_active() { "true" } else { "false" }.to_owned(),
                GtkWidget::Radio(r) => if r.is_active() { "true" } else { "false" }.to_owned(),
                _ => "false".to_owned(),
            },
            "selected" => match &*borrow {
                GtkWidget::Dropdown(d) => {
                    let selected = d.selected();
                    let text = d
                        .selected_item()
                        .and_then(|item| item.downcast::<gtk4::StringObject>().ok())
                        .map(|obj| obj.string().to_string())
                        .unwrap_or_default();
                    format!("{}:{}", text, selected)
                }
                _ => "None".to_owned(),
            },
            "id" => id.clone(),
            _ => "None".to_owned(),
        }
    }

    fn on_event(&mut self, id: &WidgetId, event: &str, callback: String) {
        let shared = match find_widget(id) {
            Some(w) => w,
            None => { return; }
        };

        gui_crate::event::set_callback(id, event, callback);

        match event {
            "onClick" => {
                let borrow = shared.borrow();
                if let GtkWidget::Button(btn) = &*borrow {
                    let wid = id.to_owned();
                    btn.connect_clicked(move |_| {
                        gui_crate::event::fire_callback(&wid, "onClick");
                    });
                }
            }
            "onChange" => {
                let wid = id.to_owned();
                let borrow = shared.borrow();
                if let GtkWidget::Input(e) = &*borrow {
                    let w = wid.clone();
                    e.connect_changed(move |_| {
                        gui_crate::event::fire_callback(&w, "onChange");
                    });
                }
            }
            "onMouseOver" | "onMouseOut" => {
                let wid = id.to_owned();
                let event_name = event.to_owned();
                let controller = gtk4::EventControllerMotion::new();
                let w2 = wid.clone();
                let _en2 = event_name.clone();
                if event_name == "onMouseOver" {
                    controller.connect_enter(move |_, _, _| {
                        gui_crate::event::fire_callback(&w2, "onMouseOver");
                    });
                }
                controller.connect_leave(move |_| {
                    gui_crate::event::fire_callback(&wid, "onMouseOut");
                });
                let borrow = shared.borrow();
                borrow.as_gtk_widget().add_controller(controller);
            }
            "onClose" => {
                let wid = id.to_owned();
                let borrow = shared.borrow();
                if let GtkWidget::Window(win) = &*borrow {
                    win.connect_close_request(move |_| {
                        gui_crate::event::fire_callback(&wid, "onClose");
                        glib::Propagation::Proceed
                    });
                }
            }
            _ => {}
        }
    }

    fn run(&mut self) {
        let already_ran = GTK_RUN.with(|r| *r.borrow());
        if already_ran {
            return;
        }
        GTK_RUN.with(|r| *r.borrow_mut() = true);

        let css_content = GTK_CSS.with(|c| c.borrow().clone());
        apply_css(&css_content);

        let app = get_or_create_app();
        app.connect_activate(move |_| {
            GTK_WIDGETS.with(|widgets| {
                for (_, shared) in widgets.borrow().iter() {
                    let widget_borrow = shared.borrow();
                    if let GtkWidget::Window(win) = &*widget_borrow {
                        win.present();
                    }
                }
            });
        });

        let args: Vec<String> = vec![];
        app.run_with_args(&args);

        GTK_RUN.with(|r| *r.borrow_mut() = false);
    }

    // System tray
    fn set_tray_icon(&mut self, window_id: &WidgetId, icon_path: &str, tooltip: &str) -> bool {
        #[cfg(target_os = "linux")]
        {
            use gtk4::gio::Menu;
            use gtk4::StatusIcon;
            
            if let Some(shared) = find_widget(window_id) {
                if let GtkWidget::Window(win) = &*shared.borrow() {
                    let status_icon = StatusIcon::new();
                    if std::path::Path::new(icon_path).exists() {
                        status_icon.set_from_file(icon_path);
                    }
                    status_icon.set_tooltip_text(Some(tooltip));
                    status_icon.set_visible(true);
                    
                    // Create a simple menu
                    let menu = Menu::new();
                    status_icon.set_menu(Some(&menu));
                    
                    // Store status icon (we'd need a map for this)
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            false
        }
    }

    fn show_notification(&mut self, title: &str, body: &str, icon: Option<&str>) {
        #[cfg(target_os = "linux")]
        {
            use gtk4::Notification;
            let notification = Notification::new(Some(title));
            notification.set_body(Some(body));
            if let Some(icon_path) = icon {
                if std::path::Path::new(icon_path).exists() {
                    notification.set_image(Some(&gtk4::gdk::Texture::from_file(&gio::File::for_path(icon_path)).unwrap()));
                }
            }
            notification.show();
        }
        #[cfg(not(target_os = "linux"))]
        {
            eprintln!("Notifications only supported on Linux with GTK4");
        }
    }

    fn clipboard_set(&mut self, text: &str) {
        use gtk4::gdk::Display;
        if let Some(display) = Display::default() {
            let clipboard = display.clipboard();
            clipboard.set_text(text);
        }
    }

    fn clipboard_get(&mut self) -> String {
        use gtk4::gdk::Display;
        if let Some(display) = Display::default() {
            let clipboard = display.clipboard();
            if let Ok(text) = clipboard.read_text_future().blocking_wait() {
                return text.to_string();
            }
        }
        String::new()
    }

    fn drag_drop_init(&mut self, widget_id: &WidgetId, data: &str) {
        use gtk4::gdk::ContentProvider;
        if let Some(shared) = find_widget(widget_id) {
            let mut borrow = shared.borrow_mut();
            if let GtkWidget::Button(btn) = &mut *borrow {
                let provider = ContentProvider::for_value(&data.to_value());
                btn.drag_source_set_content_provider(Some(&provider));
            }
        }
    }

    fn drag_drop_accept(&mut self, widget_id: &WidgetId, types: &[&str]) {
        use gtk4::gdk::ContentFormats;
        if let Some(shared) = find_widget(widget_id) {
            let mut borrow = shared.borrow_mut();
            let formats = ContentFormats::new(types.iter().map(|s| gdk::Atom::intern(s)).collect::<Vec<_>>().as_slice());
            if let GtkWidget::Button(btn) = &mut *borrow {
                btn.set_drop_target_async(&formats, |_, _| {});
            }
        }
    }

    // OpenGL
    fn gl_context_create(&mut self, window_id: &WidgetId) -> bool {
        #[cfg(target_os = "linux")]
        {
            if let Some(shared) = find_widget(window_id) {
                if let GtkWidget::Window(win) = &*shared.borrow() {
                    // GTK4 uses GdkGLContext
                    if let Some(gl_ctx) = win.gdk_surface().and_then(|s| s.create_gl_context()) {
                        win.set_gl_context(Some(&gl_ctx));
                        return true;
                    }
                }
            }
        }
        false
    }

    fn gl_make_current(&mut self, window_id: &WidgetId) -> bool {
        #[cfg(target_os = "linux")]
        {
            if let Some(shared) = find_widget(window_id) {
                if let GtkWidget::Window(win) = &*shared.borrow() {
                    if let Some(gl_ctx) = win.gl_context() {
                        return gl_ctx.make_current();
                    }
                }
            }
        }
        false
    }

    fn gl_swap_buffers(&mut self, window_id: &WidgetId) {
        #[cfg(target_os = "linux")]
        {
            if let Some(shared) = find_widget(window_id) {
                if let GtkWidget::Window(win) = &*shared.borrow() {
                    if let Some(gl_ctx) = win.gl_context() {
                        gl_ctx.swap_buffers();
                    }
                }
            }
        }
    }

    fn gl_get_proc_address(&mut self, proc_name: &str) -> *const std::ffi::c_void {
        #[cfg(target_os = "linux")]
        {
            use std::ffi::CString;
            let cname = CString::new(proc_name).unwrap();
            use gdk4::gl::GetProcAddress;
            GetProcAddress::get_proc_address(cname.as_ptr()) as *const std::ffi::c_void
        }
        #[cfg(not(target_os = "linux"))]
        {
            std::ptr::null()
        }
    }
}
