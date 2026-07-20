// ─────────────────────────────────────────────────────────────────────────────
// Generic GTK Widget System
// ─────────────────────────────────────────────────────────────────────────────

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow,
    Box as GtkBox, Button, CheckButton, CssProvider, DropDown,
    Entry, Label, Notebook, Orientation, ProgressBar, Scale,
    Stack, StackTransitionType, TextView, gdk::Paintable,
};

use crate::native::types::{Validator, ValueData};
use crate::native::std::{arg, split_args};
use crate::validators::str::put_quoted_str;

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
        }
    }
}

pub type SharedGtkWidget = Rc<RefCell<GtkWidget>>;

thread_local! {
    static GTK_APP: RefCell<Option<gtk4::Application>> = RefCell::new(None);
    static GTK_WIDGETS: RefCell<HashMap<String, SharedGtkWidget>> = RefCell::new(HashMap::new());
    static GTK_CALLBACKS: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
    static GTK_EVENT_CALLBACKS: RefCell<HashMap<String, HashMap<String, String>>> = RefCell::new(HashMap::new());
    static GTK_COUNTER: RefCell<usize> = RefCell::new(0);
    static GTK_CSS: RefCell<String> = RefCell::new(String::new());
    static GTK_CSS_PROVIDER: RefCell<Option<CssProvider>> = RefCell::new(None);
    static GTK_RUN: RefCell<bool> = RefCell::new(false);
}

fn next_gtk_id() -> String {
    let mut counter = 0;
    GTK_COUNTER.with(|c| {
        let mut b = c.borrow_mut();
        *b += 1;
        counter = *b;
    });
    format!("widget_{}", counter)
}

fn get_or_create_app() -> gtk4::Application {
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

fn apply_accumulated_css() {
    GTK_CSS.with(|css| {
        let css_content = css.borrow().clone();
        if css_content.trim().is_empty() {
            return;
        }
        let provider = CssProvider::new();
        if let Err(e) = provider.load_from_data(css_content.as_bytes()) {
            eprintln!("RuntimeError [gtk]: failed to load CSS: {}", e);
        }
        GTK_CSS_PROVIDER.with(|p| p.borrow_mut().replace(provider.clone()));
        if let Some(display) = gtk4::gdk::Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    });
}

pub fn gtk_run(_x: String) -> Box<dyn Validator> {
    GTK_RUN.with(|r| {
        if *r.borrow() {
            return Box::new(put_quoted_str("OK".to_string()));
        }
        r.borrow_mut().replace(true);
    });

    let app = get_or_create_app();
    apply_accumulated_css();

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

    GTK_RUN.with(|r| r.borrow_mut().replace(false));

    Box::new(put_quoted_str("OK".to_string()))
}

// ─────────────────────────────────────────────────────────────────────────────
// Widget Constructors
// ─────────────────────────────────────────────────────────────────────────────

pub fn gtk_new_window(args: Vec<ValueData>) -> ValueData {
    let title = args.get(0).map(|v| v.to_string(false)).unwrap_or_else(|| "Aly Window".to_string());
    let width = args.get(1).and_then(|v| match v { ValueData::Int(i) => Some(*i), _ => None }).unwrap_or(400);
    let height = args.get(2).and_then(|v| match v { ValueData::Int(i) => Some(*i), _ => None }).unwrap_or(500);

    let app = get_or_create_app();
    let win = ApplicationWindow::builder()
        .application(&app)
        .title(&title)
        .default_width(width as i32)
        .default_height(height as i32)
        .build();

    let id = next_gtk_id();
    let shared = Rc::new(RefCell::new(GtkWidget::Window(win)));
    GTK_WIDGETS.with(|w| w.borrow_mut().insert(id.clone(), shared));
    ValueData::String(id)
}

pub fn gtk_new_button(args: Vec<ValueData>) -> ValueData {
    let label = args.get(0).map(|v| v.to_string(false)).unwrap_or_else(|| "Button".to_string());
    let btn = Button::with_label(&label);
    let id = next_gtk_id();
    let shared = Rc::new(RefCell::new(GtkWidget::Button(btn)));
    GTK_WIDGETS.with(|w| w.borrow_mut().insert(id.clone(), shared));
    ValueData::String(id)
}

pub fn gtk_new_label(args: Vec<ValueData>) -> ValueData {
    let text = args.get(0).map(|v| v.to_string(false)).unwrap_or_else(|| "".to_string());
    let lbl = Label::new(Some(&text));
    let id = next_gtk_id();
    let shared = Rc::new(RefCell::new(GtkWidget::Label(lbl)));
    GTK_WIDGETS.with(|w| w.borrow_mut().insert(id.clone(), shared));
    ValueData::String(id)
}

pub fn gtk_new_div(_args: Vec<ValueData>) -> ValueData {
    let box_widget = GtkBox::new(Orientation::Vertical, 0);
    let id = next_gtk_id();
    let shared = Rc::new(RefCell::new(GtkWidget::Div(box_widget)));
    GTK_WIDGETS.with(|w| w.borrow_mut().insert(id.clone(), shared));
    ValueData::String(id)
}

pub fn gtk_new_input(args: Vec<ValueData>) -> ValueData {
    let placeholder = args.get(0).map(|v| v.to_string(false)).unwrap_or_else(|| "".to_string());
    let entry = Entry::new();
    if !placeholder.is_empty() {
        entry.set_placeholder_text(Some(&placeholder));
    }
    let id = next_gtk_id();
    let shared = Rc::new(RefCell::new(GtkWidget::Input(entry)));
    GTK_WIDGETS.with(|w| w.borrow_mut().insert(id.clone(), shared));
    ValueData::String(id)
}

pub fn gtk_new_textarea(_args: Vec<ValueData>) -> ValueData {
    let tv = TextView::new();
    let id = next_gtk_id();
    let shared = Rc::new(RefCell::new(GtkWidget::TextArea(tv)));
    GTK_WIDGETS.with(|w| w.borrow_mut().insert(id.clone(), shared));
    ValueData::String(id)
}

pub fn gtk_new_password(_args: Vec<ValueData>) -> ValueData {
    let entry = Entry::new();
    entry.set_visibility(false);
    let id = next_gtk_id();
    let shared = Rc::new(RefCell::new(GtkWidget::PasswordField(entry)));
    GTK_WIDGETS.with(|w| w.borrow_mut().insert(id.clone(), shared));
    ValueData::String(id)
}

pub fn gtk_new_checkbox(args: Vec<ValueData>) -> ValueData {
    let label = args.get(0).map(|v| v.to_string(false)).unwrap_or_else(|| "Checkbox".to_string());
    let cb = CheckButton::with_label(&label);
    let id = next_gtk_id();
    let shared = Rc::new(RefCell::new(GtkWidget::Checkbox(cb)));
    GTK_WIDGETS.with(|w| w.borrow_mut().insert(id.clone(), shared));
    ValueData::String(id)
}

pub fn gtk_new_radio(args: Vec<ValueData>) -> ValueData {
    let label = args.get(0).map(|v| v.to_string(false)).unwrap_or_else(|| "Radio".to_string());
    let radio = CheckButton::with_label(&label);
    radio.join_group(None::<&CheckButton>);
    let id = next_gtk_id();
    let shared = Rc::new(RefCell::new(GtkWidget::Radio(radio)));
    GTK_WIDGETS.with(|w| w.borrow_mut().insert(id.clone(), shared));
    ValueData::String(id)
}

pub fn gtk_new_slider(args: Vec<ValueData>) -> ValueData {
    let min = args.get(0).and_then(|v| match v { ValueData::Int(i) => Some(*i), _ => None }).unwrap_or(0);
    let max = args.get(1).and_then(|v| match v { ValueData::Int(i) => Some(*i), _ => None }).unwrap_or(100);
    let val = args.get(2).and_then(|v| match v { ValueData::Int(i) => Some(*i), _ => None }).unwrap_or(50);

    let scale = Scale::new(Orientation::Horizontal, min as f64, max as f64, 1.0);
    scale.set_value(val as f64);

    let id = next_gtk_id();
    let shared = Rc::new(RefCell::new(GtkWidget::Slider(scale)));
    GTK_WIDGETS.with(|w| w.borrow_mut().insert(id.clone(), shared));
    ValueData::String(id)
}

pub fn gtk_new_progressbar(args: Vec<ValueData>) -> ValueData {
    let val = args.get(0).and_then(|v| match v {
        ValueData::Float(f) => Some(*f as f64),
        ValueData::Int(i) => Some(*i as f64),
        _ => None,
    }).unwrap_or(0.0);
    let pb = ProgressBar::new();
    pb.set_fraction(val);
    let id = next_gtk_id();
    let shared = Rc::new(RefCell::new(GtkWidget::ProgressBar(pb)));
    GTK_WIDGETS.with(|w| w.borrow_mut().insert(id.clone(), shared));
    ValueData::String(id)
}

pub fn gtk_new_dropdown(args: Vec<ValueData>) -> ValueData {
    let items: Vec<String> = args.iter().map(|v| v.to_string(false)).collect();
    let item_refs: Vec<&str> = items.iter().map(|s| s.as_str()).collect();
    let dd = DropDown::from(item_refs);
    let id = next_gtk_id();
    let shared = Rc::new(RefCell::new(GtkWidget::Dropdown(dd)));
    GTK_WIDGETS.with(|w| w.borrow_mut().insert(id.clone(), shared));
    ValueData::String(id)
}

pub fn gtk_new_tabs(_args: Vec<ValueData>) -> ValueData {
    let nb = Notebook::new();
    let id = next_gtk_id();
    let shared = Rc::new(RefCell::new(GtkWidget::Tabs(nb)));
    GTK_WIDGETS.with(|w| w.borrow_mut().insert(id.clone(), shared));
    ValueData::String(id)
}
