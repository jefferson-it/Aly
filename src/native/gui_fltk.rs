use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use fltk::prelude::*;
use fltk::enums::Event;

use crate::gui::{
    self as gui_crate,
    backend::{GuiBackend, BackendType},
    widget::{WidgetId, next_widget_id, PROP_LABEL, PROP_TEXT, PROP_VALUE, PROP_CHECKED, PROP_ENABLED, PROP_VISIBLE},
};

#[derive(Clone)]
pub enum FltkWidget {
    Window(fltk::window::Window, fltk::group::Flex),
    Button(fltk::button::Button),
    Label(fltk::frame::Frame),
    Header(fltk::frame::Frame),
    Div(fltk::group::Flex),
    Radius(fltk::group::Group),
    Input(fltk::input::Input),
    TextArea(fltk::input::MultilineInput),
    PasswordField(fltk::input::SecretInput),
    Slider(fltk::valuator::Slider),
    Checkbox(fltk::button::CheckButton),
    Radio(fltk::button::RadioRoundButton),
    Dropdown(fltk::menu::Choice),
    ProgressBar(fltk::misc::Progress),
    Spinner(fltk::misc::Spinner),
    Progress(fltk::misc::Progress),
    Container(fltk::group::Flex),
}

pub type SharedWidget = Rc<RefCell<FltkWidget>>;

thread_local! {
    static FLTK_WIDGETS: RefCell<HashMap<String, SharedWidget>> = RefCell::new(HashMap::new());
}

fn find_widget(id: &str) -> Option<SharedWidget> {
    FLTK_WIDGETS.with(|w| w.borrow().get(id).cloned())
}

fn store_widget(id: &str, widget: FltkWidget) {
    let shared = Rc::new(RefCell::new(widget));
    FLTK_WIDGETS.with(|w| {
        w.borrow_mut().insert(id.to_owned(), shared);
    });
}

fn add_to_flex<W: WidgetExt + Clone>(flex: &mut fltk::group::Flex, child: &W) {
    flex.add(child);
    let mut w = child.clone();
    w.show();
}

fn add_to_group<W: WidgetExt + Clone>(group: &mut fltk::group::Group, child: &W) {
    group.add(child);
    let mut w = child.clone();
    w.show();
}

fn do_insert(parent: &SharedWidget, child: &SharedWidget) {
    let mut parent_borrow = parent.borrow_mut();
    let child_borrow = child.borrow();

    match &mut *parent_borrow {
        FltkWidget::Window(_, flex) => {
            match &*child_borrow {
                FltkWidget::Window(w, _) => add_to_flex(flex, w),
                FltkWidget::Button(b) => add_to_flex(flex, b),
                FltkWidget::Label(l) => add_to_flex(flex, l),
                FltkWidget::Header(h) => add_to_flex(flex, h),
                FltkWidget::Div(d) => add_to_flex(flex, d),
                FltkWidget::Radius(r) => add_to_flex(flex, r),
                FltkWidget::Input(i) => add_to_flex(flex, i),
                FltkWidget::TextArea(t) => add_to_flex(flex, t),
                FltkWidget::PasswordField(p) => add_to_flex(flex, p),
                FltkWidget::Slider(s) => add_to_flex(flex, s),
                FltkWidget::Checkbox(c) => add_to_flex(flex, c),
                FltkWidget::Radio(r) => add_to_flex(flex, r),
                FltkWidget::Dropdown(d) => add_to_flex(flex, d),
                FltkWidget::ProgressBar(p) => add_to_flex(flex, p),
                FltkWidget::Spinner(s) => add_to_flex(flex, s),
                FltkWidget::Progress(p) => add_to_flex(flex, p),
                FltkWidget::Container(c) => add_to_flex(flex, c),
            }
            if let FltkWidget::Window(win, flex) = &mut *parent_borrow {
                flex.layout();
                win.redraw();
            }
        }
        FltkWidget::Div(flex) => {
            match &*child_borrow {
                FltkWidget::Window(w, _) => add_to_flex(flex, w),
                FltkWidget::Button(b) => add_to_flex(flex, b),
                FltkWidget::Label(l) => add_to_flex(flex, l),
                FltkWidget::Header(h) => add_to_flex(flex, h),
                FltkWidget::Div(d) => add_to_flex(flex, d),
                FltkWidget::Radius(r) => add_to_flex(flex, r),
                FltkWidget::Input(i) => add_to_flex(flex, i),
                FltkWidget::TextArea(t) => add_to_flex(flex, t),
                FltkWidget::PasswordField(p) => add_to_flex(flex, p),
                FltkWidget::Spinner(s) => add_to_flex(flex, s),
                FltkWidget::ProgressBar(p) => add_to_flex(flex, p),
                FltkWidget::Radio(r) => add_to_flex(flex, r),
                FltkWidget::Checkbox(c) => add_to_flex(flex, c),
                FltkWidget::Dropdown(d) => add_to_flex(flex, d),
                FltkWidget::Slider(sl) => add_to_flex(flex, sl),
                FltkWidget::Progress(progress) => add_to_flex(flex, progress),
                FltkWidget::Container(container) => add_to_flex(flex, container),
            }
            flex.layout();
            if let Some(mut w) = flex.window() {
                w.redraw();
            }
        }
        FltkWidget::Radius(group) => {
            match &*child_borrow {
                FltkWidget::Window(w, _) => add_to_group(group, w),
                FltkWidget::Button(b) => add_to_group(group, b),
                FltkWidget::Label(l) => add_to_group(group, l),
                FltkWidget::Header(h) => add_to_group(group, h),
                FltkWidget::Div(d) => add_to_group(group, d),
                FltkWidget::Radius(r) => add_to_group(group, r),
                FltkWidget::Input(i) => add_to_group(group, i),
                FltkWidget::TextArea(t) => add_to_group(group, t),
                FltkWidget::PasswordField(p) => add_to_group(group, p),
                FltkWidget::Slider(s) => add_to_group(group, s),
                FltkWidget::Checkbox(c) => add_to_group(group, c),
                FltkWidget::Radio(r) => add_to_group(group, r),
                FltkWidget::Dropdown(d) => add_to_group(group, d),
                FltkWidget::ProgressBar(p) => add_to_group(group, p),
                FltkWidget::Spinner(s) => add_to_group(group, s),
                FltkWidget::Progress(p) => add_to_group(group, p),
                FltkWidget::Container(c) => add_to_group(group, c),
            }
            group.redraw();
            if let Some(mut w) = group.window() {
                w.redraw();
            }
        }
        FltkWidget::Container(flex) => {
            match &*child_borrow {
                FltkWidget::Button(b) => add_to_flex(flex, b),
                FltkWidget::Label(l) => add_to_flex(flex, l),
                FltkWidget::Div(d) => add_to_flex(flex, d),
                FltkWidget::Input(i) => add_to_flex(flex, i),
                FltkWidget::Dropdown(d) => add_to_flex(flex, d),
                _ => {}
            }
            flex.layout();
        }
        _ => {}
    }
}

macro_rules! setup_handle_event {
    ($widget:expr, $target_event:expr, $id_clone:expr, $event_name:expr) => {{
        let id = $id_clone.clone();
        let evt = $target_event;
        let ename = $event_name.clone();
        $widget.handle(move |_w, ev| {
            if ev == evt {
                gui_crate::event::fire_callback(&id, &ename);
            }
            false
        });
    }};
}

fn setup_event_handler(widget_id: &str, shared: &SharedWidget, fltk_event: Event, event_name: &str) {
    let id_clone = widget_id.to_owned();
    let ename = event_name.to_owned();
    let mut borrow = shared.borrow_mut();

    match &mut *borrow {
        FltkWidget::Button(b) => { setup_handle_event!(b, fltk_event, id_clone, ename); }
        FltkWidget::Label(l) => { setup_handle_event!(l, fltk_event, id_clone, ename); }
        FltkWidget::Header(h) => { setup_handle_event!(h, fltk_event, id_clone, ename); }
        FltkWidget::Input(i) => { setup_handle_event!(i, fltk_event, id_clone, ename); }
        FltkWidget::TextArea(t) => { setup_handle_event!(t, fltk_event, id_clone, ename); }
        FltkWidget::PasswordField(p) => { setup_handle_event!(p, fltk_event, id_clone, ename); }
        FltkWidget::Slider(s) => { setup_handle_event!(s, fltk_event, id_clone, ename); }
        FltkWidget::Checkbox(c) => { setup_handle_event!(c, fltk_event, id_clone, ename); }
        FltkWidget::Radio(r) => { setup_handle_event!(r, fltk_event, id_clone, ename); }
        FltkWidget::Dropdown(d) => { setup_handle_event!(d, fltk_event, id_clone, ename); }
        FltkWidget::ProgressBar(p) => { setup_handle_event!(p, fltk_event, id_clone, ename); }
        FltkWidget::Spinner(s) => { setup_handle_event!(s, fltk_event, id_clone, ename); }
        FltkWidget::Window(w, _) => { setup_handle_event!(w, fltk_event, id_clone, ename); }
        FltkWidget::Div(d) => { setup_handle_event!(d, fltk_event, id_clone, ename); }
        FltkWidget::Radius(r) => { setup_handle_event!(r, fltk_event, id_clone, ename); }
        FltkWidget::Container(f) => { setup_handle_event!(f, fltk_event, id_clone, ename); }
        _ => {}
    }
}

fn setup_change_callback(widget_id: &str, shared: &SharedWidget) {
    let id_clone = widget_id.to_owned();
    let mut borrow = shared.borrow_mut();

    match &mut *borrow {
        FltkWidget::Input(inp) => {
            inp.set_callback(move |_| {
                gui_crate::event::fire_callback(&id_clone, EVENT_CHANGE);
            });
        }
        FltkWidget::TextArea(ta) => {
            let id2 = id_clone.clone();
            ta.set_callback(move |_| {
                gui_crate::event::fire_callback(&id2, EVENT_CHANGE);
            });
        }
        FltkWidget::Slider(sl) => {
            let id2 = id_clone.clone();
            sl.set_callback(move |_| {
                gui_crate::event::fire_callback(&id2, EVENT_CHANGE);
            });
        }
        FltkWidget::Checkbox(cb) => {
            let id2 = id_clone.clone();
            cb.set_callback(move |_| {
                gui_crate::event::fire_callback(&id2, EVENT_CHANGE);
            });
        }
        FltkWidget::Dropdown(ch) => {
            let id2 = id_clone.clone();
            ch.set_callback(move |_| {
                gui_crate::event::fire_callback(&id2, EVENT_CHANGE);
            });
        }
        FltkWidget::Spinner(sp) => {
            let id2 = id_clone.clone();
            sp.set_callback(move |_| {
                gui_crate::event::fire_callback(&id2, EVENT_CHANGE);
            });
        }
        _ => {}
    }
}

const EVENT_CLICK: &str = "onClick";
const EVENT_CHANGE: &str = "onChange";
const EVENT_MOUSEOVER: &str = "onMouseOver";
const EVENT_MOUSEOUT: &str = "onMouseOut";
const EVENT_FOCUS: &str = "onFocus";
const EVENT_BLUR: &str = "onBlur";
const EVENT_KEYPRESS: &str = "onKeyPress";
const EVENT_MOUSEWHEEL: &str = "onMouseWheel";
const EVENT_RESIZE: &str = "onResize";

pub struct FltkBackend;

impl GuiBackend for FltkBackend {
    fn backend_type(&self) -> BackendType {
        BackendType::Fltk
    }

    fn create_window(&mut self, title: &str, width: i32, height: i32) -> WidgetId {
        let id = next_widget_id();
        let win = fltk::window::Window::default().with_size(width, height).with_label(title);
        let mut flex = fltk::group::Flex::default().size_of(&win).column();
        flex.set_margin(15);
        flex.set_pad(12);
        flex.end();
        win.end();
        store_widget(&id, FltkWidget::Window(win, flex));
        id
    }

    fn create_button(&mut self, label: &str) -> WidgetId {
        let id = next_widget_id();
        let mut btn = fltk::button::Button::default().with_size(380, 45).with_label(label);
        btn.set_frame(fltk::enums::FrameType::RFlatBox);
        btn.set_color(fltk::enums::Color::from_rgb(0, 122, 255));
        btn.set_label_color(fltk::enums::Color::White);
        store_widget(&id, FltkWidget::Button(btn));
        id
    }

    fn create_label(&mut self, text: &str) -> WidgetId {
        let id = next_widget_id();
        let lbl = fltk::frame::Frame::default().with_size(380, 30).with_label(text);
        store_widget(&id, FltkWidget::Label(lbl));
        id
    }

    fn create_div(&mut self, direction: &str) -> WidgetId {
        let id = next_widget_id();
        let mut flex = fltk::group::Flex::default().with_size(380, 100);
        if direction == "horizontal" {
            flex = flex.row();
        } else {
            flex = flex.column();
        }
        flex.set_pad(10);
        flex.end();
        store_widget(&id, FltkWidget::Div(flex));
        id
    }

    fn create_input(&mut self, placeholder: &str) -> WidgetId {
        let id = next_widget_id();
        let mut inp = fltk::input::Input::default().with_size(380, 40);
        inp.set_value(placeholder);
        store_widget(&id, FltkWidget::Input(inp));
        id
    }

    fn create_textarea(&mut self) -> WidgetId {
        let id = next_widget_id();
        let ta = fltk::input::MultilineInput::default().with_size(380, 100);
        store_widget(&id, FltkWidget::TextArea(ta));
        id
    }

    fn create_password(&mut self) -> WidgetId {
        let id = next_widget_id();
        let pf = fltk::input::SecretInput::default().with_size(380, 40);
        store_widget(&id, FltkWidget::PasswordField(pf));
        id
    }

    fn create_checkbox(&mut self, label: &str) -> WidgetId {
        let id = next_widget_id();
        let mut chk = fltk::button::CheckButton::default().with_size(380, 30).with_label(label);
        chk.set_value(false);
        store_widget(&id, FltkWidget::Checkbox(chk));
        id
    }

    fn create_radio(&mut self, label: &str) -> WidgetId {
        let id = next_widget_id();
        let radio = fltk::button::RadioRoundButton::default().with_size(380, 30).with_label(label);
        store_widget(&id, FltkWidget::Radio(radio));
        id
    }

    fn create_slider(&mut self, min: f64, max: f64, val: f64) -> WidgetId {
        let id = next_widget_id();
        let mut slider = fltk::valuator::Slider::default().with_size(380, 30);
        slider.set_range(min, max);
        slider.set_value(val);
        store_widget(&id, FltkWidget::Slider(slider));
        id
    }

    fn create_progressbar(&mut self, val: f64) -> WidgetId {
        let id = next_widget_id();
        let mut pb = fltk::misc::Progress::default().with_size(380, 30);
        pb.set_maximum(100.0);
        pb.set_value(val);
        store_widget(&id, FltkWidget::ProgressBar(pb));
        id
    }

    fn create_dropdown(&mut self, items: &[String]) -> WidgetId {
        let id = next_widget_id();
        let mut choice = fltk::menu::Choice::default().with_size(380, 30);
        for item in items {
            choice.add_choice(item);
        }
        store_widget(&id, FltkWidget::Dropdown(choice));
        id
    }

    fn create_separator(&mut self) -> WidgetId {
        let id = next_widget_id();
        let sep = fltk::frame::Frame::default().with_size(380, 5);
        store_widget(&id, FltkWidget::Label(sep));
        id
    }

    fn create_spinner(&mut self, min: f64, max: f64, step: f64, val: f64) -> WidgetId {
        let id = next_widget_id();
        let mut spinner = fltk::misc::Spinner::default().with_size(380, 30);
        spinner.set_range(min, max);
        spinner.set_step(step);
        spinner.set_value(val);
        store_widget(&id, FltkWidget::Spinner(spinner));
        id
    }

    fn create_container(&mut self, width: i32, height: i32, direction: &str) -> WidgetId {
        let id = next_widget_id();
        let mut flex = fltk::group::Flex::default().with_size(width, height);
        if direction == "horizontal" {
            flex = flex.row();
        } else {
            flex = flex.column();
        }
        flex.set_pad(10);
        flex.end();
        store_widget(&id, FltkWidget::Container(flex));
        id
    }

    fn insert_widget(&mut self, parent: &WidgetId, child: &WidgetId) {
        let p = find_widget(parent);
        let c = find_widget(child);
        match (p, c) {
            (Some(pw), Some(cw)) => do_insert(&pw, &cw),
            _ => eprintln!("RuntimeError [gui]: widget parent or child not found."),
        }
    }

    fn set_property(&mut self, id: &WidgetId, prop: &str, value: &str) {
        let widget_opt = find_widget(id);
        if let Some(shared) = widget_opt {
            let mut borrow = shared.borrow_mut();
            match prop {
                PROP_LABEL | PROP_TEXT | PROP_VALUE => {
                    match &mut *borrow {
                        FltkWidget::Window(w, _) => { w.set_label(value); w.redraw(); }
                        FltkWidget::Button(b) => { b.set_label(value); b.redraw(); }
                        FltkWidget::Label(l) => { l.set_label(value); l.redraw(); }
                        FltkWidget::Header(h) => { h.set_label(value); h.redraw(); }
                        FltkWidget::Input(i) => { i.set_value(value); i.redraw(); }
                        FltkWidget::TextArea(t) => { t.set_value(value); t.redraw(); }
                        FltkWidget::PasswordField(p) => { p.set_value(value); p.redraw(); }
                        FltkWidget::Slider(s) => { s.set_label(value); s.redraw(); }
                        FltkWidget::Checkbox(c) => { c.set_label(value); c.redraw(); }
                        FltkWidget::Radio(r) => { r.set_label(value); r.redraw(); }
                        FltkWidget::Dropdown(d) => { d.set_label(value); d.redraw(); }
                        FltkWidget::ProgressBar(p) => { p.set_label(value); p.redraw(); }
                        FltkWidget::Spinner(s) => { s.set_label(value); s.redraw(); }
                        _ => {}
                    }
                }
                PROP_CHECKED => {
                    let checked = value == "true" || value == "1";
                    if let FltkWidget::Checkbox(c) = &mut *borrow {
                        c.set_value(checked);
                        c.redraw();
                    }
                }
                PROP_ENABLED => {
                    let enabled = value != "false";
                    match &mut *borrow {
                        FltkWidget::Button(b) => if enabled { b.activate(); } else { b.deactivate(); },
                        FltkWidget::Input(i) => if enabled { i.activate(); } else { i.deactivate(); },
                        FltkWidget::Slider(s) => if enabled { s.activate(); } else { s.deactivate(); },
                        _ => {}
                    }
                }
                PROP_VISIBLE => {
                    let visible = value != "false";
                    match &mut *borrow {
                        FltkWidget::Window(w, _) => if visible { w.show() } else { w.hide() },
                        FltkWidget::Button(b) => if visible { b.show() } else { b.hide() },
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    fn get_property(&mut self, id: &WidgetId, prop: &str) -> String {
        let widget_opt = find_widget(id);
        if let Some(shared) = widget_opt {
            let borrow = shared.borrow();
            match prop {
                PROP_LABEL | PROP_TEXT | PROP_VALUE => {
                    return match &*borrow {
                        FltkWidget::Window(w, _) => w.label(),
                        FltkWidget::Button(b) => b.label(),
                        FltkWidget::Label(l) => l.label(),
                        FltkWidget::Header(h) => h.label(),
                        FltkWidget::Input(i) => i.value(),
                        FltkWidget::TextArea(t) => t.value(),
                        FltkWidget::PasswordField(p) => p.value(),
                        FltkWidget::Slider(s) => s.label(),
                        FltkWidget::Checkbox(c) => c.label(),
                        FltkWidget::Radio(r) => r.label(),
                        FltkWidget::Dropdown(d) => d.label(),
                        FltkWidget::ProgressBar(p) => p.label(),
                        FltkWidget::Spinner(sp) => sp.label(),
                        _ => "None".to_owned(),
                    };
                }
                PROP_CHECKED => {
                    if let FltkWidget::Checkbox(c) = &*borrow {
                        return if c.value() { "true" } else { "false" }.to_owned();
                    }
                    return "false".to_owned();
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

        match event {
            EVENT_CLICK => {
                let mut borrow = shared.borrow_mut();
                if let FltkWidget::Button(btn) = &mut *borrow {
                    let id_clone = id.to_owned();
                    btn.set_callback(move |_| {
                        gui_crate::event::fire_callback(&id_clone, EVENT_CLICK);
                    });
                }
            }
            EVENT_CHANGE => {
                setup_change_callback(id, &shared);
            }
            EVENT_MOUSEOVER | EVENT_MOUSEOUT | EVENT_FOCUS | EVENT_BLUR
            | EVENT_KEYPRESS | EVENT_MOUSEWHEEL | EVENT_RESIZE => {
                let fltk_ev = match event {
                    EVENT_MOUSEOVER => Event::Enter,
                    EVENT_MOUSEOUT => Event::Leave,
                    EVENT_FOCUS => Event::Focus,
                    EVENT_BLUR => Event::Unfocus,
                    EVENT_KEYPRESS => Event::KeyDown,
                    EVENT_MOUSEWHEEL => Event::MouseWheel,
                    EVENT_RESIZE => Event::Resize,
                    _ => return,
                };
                setup_event_handler(id, &shared, fltk_ev, event);
            }
            _ => {
                let fltk_ev = match event {
                    "onEnter" => Event::Enter,
                    "onLeave" => Event::Leave,
                    _ => return,
                };
                setup_event_handler(id, &shared, fltk_ev, event);
            }
        }
    }

    fn run(&mut self) {
        fltk::app::App::default().run().unwrap();
    }

    // System tray
    fn set_tray_icon(&mut self, window_id: &WidgetId, icon_path: &str, tooltip: &str) -> bool {
        let widget_opt = find_widget(window_id);
        if let Some(shared) = widget_opt {
            let mut borrow = shared.borrow_mut();
            if let FltkWidget::Window(win, _) = &mut *borrow {
                if std::path::Path::new(icon_path).exists() {
                    let img = fltk::image::PngImage::load(icon_path).ok();
                    if let Some(img) = img {
                        win.set_icon(Some(img));
                        win.set_tooltip(tooltip);
                        return true;
                    }
                }
            }
        }
        false
    }

    // Notifications
    fn show_notification(&mut self, title: &str, body: &str, icon: Option<&str>) {
        // FLTK doesn't have native notifications, use system command
        #[cfg(target_os = "linux")]
        {
            let _ = std::process::Command::new("notify-send")
                .arg(title)
                .arg(body)
                .spawn();
        }
        #[cfg(target_os = "macos")]
        {
            let _ = std::process::Command::new("osascript")
                .arg("-e")
                .arg(format!("display notification \"{}\" with title \"{}\"", body, title))
                .spawn();
        }
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            let ps_cmd = format!(
                "Add-Type -AssemblyName System.Windows.Forms; \
                 $notify = New-Object System.Windows.Forms.NotifyIcon; \
                 $notify.Icon = [System.Drawing.SystemIcons]::Information; \
                 $notify.Visible = $true; \
                 $notify.ShowBalloonTip(5000, '{}', '{}', 'Info')",
                title.replace("'", "''"), body.replace("'", "''")
            );
            let _ = Command::new("powershell").arg("-Command").arg(&ps_cmd).spawn();
        }
    }

    // Clipboard
    fn clipboard_set(&mut self, text: &str) {
        let mut app = fltk::app::App::default();
        app.clipboard_set(text);
    }

    fn clipboard_get(&mut self) -> String {
        let mut app = fltk::app::App::default();
        app.clipboard().unwrap_or_default()
    }

    // Drag and Drop
    fn drag_drop_init(&mut self, widget_id: &WidgetId, data: &str) {
        let widget_opt = find_widget(widget_id);
        if let Some(shared) = widget_opt {
            let mut borrow = shared.borrow_mut();
            match &mut *borrow {
                FltkWidget::Button(b) => {
                    b.set_dnd_data(data);
                }
                FltkWidget::Input(i) => {
                    i.set_dnd_data(data);
                }
                FltkWidget::TextArea(t) => {
                    t.set_dnd_data(data);
                }
                _ => {}
            }
        }
    }

    fn drag_drop_accept(&mut self, widget_id: &WidgetId, types: &[&str]) {
        let widget_opt = find_widget(widget_id);
        if let Some(shared) = widget_opt {
            let mut borrow = shared.borrow_mut();
            match &mut *borrow {
                FltkWidget::Input(i) => {
                    i.set_dnd_types(types.iter().map(|s| s.to_string()).collect());
                }
                FltkWidget::TextArea(t) => {
                    t.set_dnd_types(types.iter().map(|s| s.to_string()).collect());
                }
                _ => {}
            }
        }
    }

    // OpenGL
    fn gl_context_create(&mut self, window_id: &WidgetId) -> bool {
        let widget_opt = find_widget(window_id);
        if let Some(shared) = widget_opt {
            let mut borrow = shared.borrow_mut();
            if let FltkWidget::Window(win, _) = &mut *borrow {
                win.make_gl_context();
                true
            } else { false }
        } else { false }
    }

    fn gl_make_current(&mut self, window_id: &WidgetId) -> bool {
        let widget_opt = find_widget(window_id);
        if let Some(shared) = widget_opt {
            let mut borrow = shared.borrow_mut();
            if let FltkWidget::Window(win, _) = &mut *borrow {
                win.make_gl_context();
                true
            } else { false }
        } else { false }
    }

    fn gl_swap_buffers(&mut self, window_id: &WidgetId) {
        let widget_opt = find_widget(window_id);
        if let Some(shared) = widget_opt {
            let mut borrow = shared.borrow_mut();
            if let FltkWidget::Window(win, _) = &mut *borrow {
                win.swap_buffers();
            }
        }
    }

    fn gl_get_proc_address(&mut self, proc_name: &str) -> *const std::ffi::c_void {
        fltk::app::get_proc_address(proc_name) as *const std::ffi::c_void
    }
}
