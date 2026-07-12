use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use fltk::prelude::*;
use fltk::enums::Event;

use crate::lexer::Lexer;
use crate::tokens::Tokens;
use crate::native::types::ValueData;

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
    TabPanel(fltk::group::Tabs),
    Select(fltk::button::ToggleButton),
    Gauge(fltk::misc::Gauge),
    Scale(fltk::valuator::Slider),
    ColorPicker(fltk::button::Button),
    FileInput(fltk::input::Input),
    DatePicker(fltk::input::Input),
    TimePicker(fltk::input::Input),
    Separator(fltk::frame::Frame),
    StatusBar(fltk::frame::Frame),
    ProgressBarCustom(fltk::group::Frame),
}

pub type SharedWidget = Rc<RefCell<FltkWidget>>;

#[derive(Clone, Default, Debug)]
pub struct StyleRule {
    pub background_color: Option<fltk::enums::Color>,
    pub text_color: Option<fltk::enums::Color>,
    pub border_radius: Option<i32>,
    pub font_size: Option<i32>,
    pub padding: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub text_align: Option<String>,
    pub font_weight: Option<i32>,
    pub border_color: Option<fltk::enums::Color>,
    pub border_width: Option<i32>,
    pub display: Option<String>,
    pub opacity: Option<f64>,
}

thread_local! {
    static GUI_WIDGETS: RefCell<HashMap<String, SharedWidget>> = RefCell::new(HashMap::new());
    static WIDGET_CALLBACKS: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
    static WIDGET_EVENT_CALLBACKS: RefCell<HashMap<String, HashMap<String, String>>> = RefCell::new(HashMap::new());
    static WIDGET_COUNTER: RefCell<usize> = RefCell::new(0);
    static STYLESHEET: RefCell<HashMap<String, StyleRule>> = RefCell::new(HashMap::new());
}

fn next_widget_id() -> String {
    let mut counter = 0;
    WIDGET_COUNTER.with(|c| {
        let mut b = c.borrow_mut();
        *b += 1;
        counter = *b;
    });
    format!("widget_{}", counter)
}

pub fn create_widget(widget_type: &str, args: Vec<ValueData>) -> ValueData {
    let id = next_widget_id();
    
    fltk::app::App::default();
    
    let fltk_widget = match widget_type {
        "Window" => {
            let title = args.get(0).map(|v| v.to_string(false)).unwrap_or_else(|| "Aly Window".to_owned());
            let width = args.get(1).and_then(|v| match v {
                ValueData::Int(i) => Some(*i),
                _ => None,
            }).unwrap_or(400);
            let height = args.get(2).and_then(|v| match v {
                ValueData::Int(i) => Some(*i),
                _ => None,
            }).unwrap_or(500);
            
            let win = fltk::window::Window::default().with_size(width as i32, height as i32).with_label(&title);
            let mut flex = fltk::group::Flex::default().size_of(&win).column();
            flex.set_margin(15);
            flex.set_pad(12);
            flex.end();
            win.end();
            
            FltkWidget::Window(win, flex)
        }
        "Button" => {
            let label = args.get(0).map(|v| v.to_string(false)).unwrap_or_else(|| "Button".to_owned());
            let mut btn = fltk::button::Button::default().with_size(380, 45).with_label(&label);
            btn.set_frame(fltk::enums::FrameType::RFlatBox);
            btn.set_color(fltk::enums::Color::from_rgb(0, 122, 255));
            btn.set_label_color(fltk::enums::Color::White);
            FltkWidget::Button(btn)
        }
        "Label" => {
            let text = args.get(0).map(|v| v.to_string(false)).unwrap_or_default();
            let lbl = fltk::frame::Frame::default().with_size(380, 30).with_label(&text);
            FltkWidget::Label(lbl)
        }
        "Header" => {
            let text = args.get(0).map(|v| v.to_string(false)).unwrap_or_default();
            let mut hdr = fltk::frame::Frame::default().with_size(380, 50).with_label(&text);
            hdr.set_label_size(24);
            hdr.set_label_type(fltk::enums::LabelType::Shadow);
            FltkWidget::Header(hdr)
        }
        "Div" => {
            let dir = args.get(0).map(|v| v.to_string(false)).unwrap_or_else(|| "vertical".to_owned());
            let mut flex = fltk::group::Flex::default().with_size(380, 100);
            if dir == "horizontal" {
                flex = flex.row();
            } else {
                flex = flex.column();
            }
            flex.set_pad(10);
            flex.end();
            FltkWidget::Div(flex)
        }
        "Radius" => {
            let mut grp = fltk::group::Group::default().with_size(380, 100);
            grp.set_frame(fltk::enums::FrameType::RFlatBox);
            grp.end();
            FltkWidget::Radius(grp)
        }
        "Input" => {
            let val = args.get(0).map(|v| v.to_string(false)).unwrap_or_default();
            let mut input = fltk::input::Input::default().with_size(380, 40);
            input.set_value(&val);
            FltkWidget::Input(input)
        }
        "TextArea" => {
            let val = args.get(0).map(|v| v.to_string(false)).unwrap_or_default();
            let mut input = fltk::input::MultilineInput::default().with_size(380, 100);
            input.set_value(&val);
            FltkWidget::TextArea(input)
        }
        "PasswordField" => {
            let val = args.get(0).map(|v| v.to_string(false)).unwrap_or_default();
            let mut input = fltk::input::SecretInput::default().with_size(380, 40);
            input.set_value(&val);
            FltkWidget::PasswordField(input)
        }
        "Slider" => {
            let min = args.get(0).and_then(|v| match v { ValueData::Float(f) => Some(*f as f64), ValueData::Int(i) => Some(*i as f64), _ => None }).unwrap_or(0.0);
            let max = args.get(1).and_then(|v| match v { ValueData::Float(f) => Some(*f as f64), ValueData::Int(i) => Some(*i as f64), _ => None }).unwrap_or(100.0);
            let val = args.get(2).and_then(|v| match v { ValueData::Float(f) => Some(*f as f64), ValueData::Int(i) => Some(*i as f64), _ => None }).unwrap_or(50.0);
            let mut slider = fltk::valuator::Slider::default().with_size(380, 30);
            slider.set_range(min, max);
            slider.set_value(val);
            FltkWidget::Slider(slider)
        }
        "Checkbox" => {
            let label = args.get(0).map(|v| v.to_string(false)).unwrap_or_else(|| "Checkbox".to_owned());
            let checked = args.get(1).and_then(|v| match v { ValueData::Bool(b) => Some(*b), _ => None }).unwrap_or(false);
            let mut chk = fltk::button::CheckButton::default().with_size(380, 30).with_label(&label);
            chk.set_value(checked);
            FltkWidget::Checkbox(chk)
        }
        "Radio" => {
            let label = args.get(0).map(|v| v.to_string(false)).unwrap_or_else(|| "Radio".to_owned());
            let mut radio = fltk::button::RadioRoundButton::default().with_size(380, 30).with_label(&label);
            FltkWidget::Radio(radio)
        }
        "Progress" => {
            let max = args.get(0).and_then(|v| match v {
                ValueData::Float(f) => Some(*f as f64),
                ValueData::Int(i) => Some(*i as f64),
                _ => None,
            }).unwrap_or(100.0);
            let mut progress = fltk::misc::Progress::default().with_size(380, 30);
            progress.set_maximum(max);
            let value = args.get(1).and_then(|v| match v {
                ValueData::Float(f) => Some(*f as f64),
                ValueData::Int(i) => Some(*i as f64),
                _ => None,
            }).unwrap_or(0.0);
            progress.set_value(value);
            FltkWidget::Progress(progress)
        }
        "ProgressBar" => {
            let max = args.get(0).and_then(|v| match v {
                ValueData::Float(f) => Some(*f as f64),
                ValueData::Int(i) => Some(*i as f64),
                _ => None,
            }).unwrap_or(100.0);
            let mut prog = fltk::misc::Progress::default().with_size(380, 30);
            prog.set_maximum(max);
            prog.set_value(0.0);
            FltkWidget::ProgressBar(prog)
        }
        "Container" => {
            let width = args.get(0).and_then(|v| match v {
                ValueData::Int(i) => Some(*i),
                _ => None,
            }).unwrap_or(380);
            let height = args.get(1).and_then(|v| match v {
                ValueData::Int(i) => Some(*i),
                _ => None,
            }).unwrap_or(300);
            let dir = args.get(2).map(|v| v.to_string(false)).unwrap_or_else(|| "vertical".to_owned());
            let mut flex = fltk::group::Flex::default().with_size(width as i32, height as i32);
            if dir == "horizontal" {
                flex = flex.row();
            } else {
                flex = flex.column();
            }
            flex.set_pad(10);
            flex.end();
            FltkWidget::Div(flex)
        }
        "Dropdown" => {
            let items_str = args.get(0).map(|v| v.to_string(false)).unwrap_or_default();
            let mut choice = fltk::menu::Choice::default().with_size(380, 30);
            for item in items_str.split(',') {
                let trimmed = item.trim();
                if !trimmed.is_empty() {
                    choice.add_choice(&trimmed);
                }
            }
            FltkWidget::Dropdown(choice)
        }
        "TabPanel" => {
            let height = args.get(0).and_then(|v| match v {
                ValueData::Int(i) => Some(*i),
                _ => None,
            }).unwrap_or(400);
            let mut tab_group = fltk::group::Tabs::default().with_size(380, height);
            let mut flex = fltk::group::Flex::default().parent(&tab_group).of_size(380, height - 30).column();
            flex.set_margin(5);
            FltkWidget::Div(flex)
        }
        "Select" => {
            let options_str = args.get(0).map(|v| v.to_string(false)).unwrap_or_default();
            let mut select = fltk::button::ToggleButton::default().with_size(380, 30);
            select.set_label(&options_str);
            FltkWidget::Checkbox(select)
        }
        "Gauge" => {
            let min = args.get(0).and_then(|v| match v {
                ValueData::Float(f) => Some(*f as f64),
                ValueData::Int(i) => Some(*i as f64),
                _ => None,
            }).unwrap_or(0.0);
            let max = args.get(1).and_then(|v| match v {
                ValueData::Float(f) => Some(*f as f64),
                ValueData::Int(i) => Some(*i as f64),
                _ => None,
            }).unwrap_or(100.0);
            let mut gauge = fltk::misc::Gauge::default().with_size(380, 40);
            gauge.set_value(0.0);
            gauge.set_range(min, max);
            FltkWidget::Slider(gauge)
        }
        "Scale" => {
            let min = args.get(0).and_then(|v| match v {
                ValueData::Float(f) => Some(*f as f64),
                ValueData::Int(i) => Some(*i as f64),
                _ => None,
            }).unwrap_or(0.0);
            let max = args.get(1).and_then(|v| match v {
                ValueData::Float(f) => Some(*f as f64),
                ValueData::Int(i) => Some(*i as f64),
                _ => None,
            }).unwrap_or(100.0);
            let mut scale = fltk::valuator::Slider::default().with_size(380, 30);
            scale.set_range(min, max);
            scale.set_value((min + max) / 2.0);
            scale.set_slider_size(20.0);
            FltkWidget::Slider(scale)
        }
        "ColorPicker" => {
            let color_str = args.get(0).map(|v| v.to_string(false)).unwrap_or_default();
            let mut btn = fltk::button::Button::default().with_size(380, 40);
            btn.set_color(parse_color(&color_str).unwrap_or(fltk::enums::Color::White));
            btn.set_label(&color_str);
            FltkWidget::Button(btn)
        }
        "FileInput" => {
            let val = args.get(0).map(|v| v.to_string(false)).unwrap_or_default();
            let mut input = fltk::input::Input::default().with_size(380, 40);
            input.set_value(&val);
            FltkWidget::Input(input)
        }
        "DatePicker" => {
            let val = args.get(0).map(|v| v.to_string(false)).unwrap_or_default();
            let mut input = fltk::input::Input::default().with_size(380, 40);
            input.set_value(&val);
            FltkWidget::Input(input)
        }
        "TimePicker" => {
            let val = args.get(0).map(|v| v.to_string(false)).unwrap_or_default();
            let mut input = fltk::input::Input::default().with_size(380, 40);
            input.set_value(&val);
            FltkWidget::Input(input)
        }
        "Separator" => {
            let height = args.get(0).and_then(|v| match v {
                ValueData::Int(i) => Some(*i),
                _ => None,
            }).unwrap_or(5);
            let mut separator = fltk::frame::Frame::default().with_size(380, height);
            separator.set_frame(fltk::enums::FrameType::EngravedBox);
            separator.set_color(fltk::enums::Color::from_rgb(200, 200, 200));
            FltkWidget::Label(separator)
        }
        "StatusBar" => {
            let height = args.get(0).and_then(|v| match v {
                ValueData::Int(i) => Some(*i),
                _ => None,
            }).unwrap_or(25);
            let mut status = fltk::frame::Frame::default().with_size(380, height);
            status.set_frame(fltk::enums::FrameType::DownBox);
            status.set_color(fltk::enums::Color::from_rgb(240, 240, 240));
            FltkWidget::Label(status)
        }
        "ProgressBarCustom" => {
            let width = args.get(0).and_then(|v| match v {
                ValueData::Int(i) => Some(*i),
                _ => None,
            }).unwrap_or(380);
            let height = args.get(1).and_then(|v| match v {
                ValueData::Int(i) => Some(*i),
                _ => None,
            }).unwrap_or(30);
            let border = args.get(2).and_then(|v| match v {
                ValueData::Int(i) => Some(*i),
                _ => None,
            }).unwrap_or(1);
            let mut border_frame = fltk::frame::Frame::default().with_size(width as i32, height as i32);
            border_frame.set_frame(fltk::enums::FrameType::BorderBox);
            let mut progress = fltk::misc::Progress::default().with_size(width as i32 - 4, height as i32 - 4).parent(&border_frame);
            progress.set_maximum(100.0);
            progress.set_value(50.0);
            FltkWidget::ProgressBar(progress)
        }
        "Spinner" => {
            let min = args.get(0).and_then(|v| match v { ValueData::Float(f) => Some(*f as f64), ValueData::Int(i) => Some(*i as f64), _ => None }).unwrap_or(0.0);
            let max = args.get(1).and_then(|v| match v { ValueData::Float(f) => Some(*f as f64), ValueData::Int(i) => Some(*i as f64), _ => None }).unwrap_or(100.0);
            let step = args.get(2).and_then(|v| match v { ValueData::Float(f) => Some(*f as f64), ValueData::Int(i) => Some(*i as f64), _ => None }).unwrap_or(1.0);
            let val = args.get(3).and_then(|v| match v { ValueData::Float(f) => Some(*f as f64), ValueData::Int(i) => Some(*i as f64), _ => None }).unwrap_or(0.0);
            let mut spinner = fltk::misc::Spinner::default().with_size(380, 30);
            spinner.set_range(min, max);
            spinner.set_step(step);
            spinner.set_value(val);
            FltkWidget::Spinner(spinner)
        }
        _ => panic!("Unknown widget type: {}", widget_type),
    };
    
    let shared = Rc::new(RefCell::new(fltk_widget));
    GUI_WIDGETS.with(|w| {
        w.borrow_mut().insert(id.clone(), shared);
    });
    
    ValueData::Widget(id)
}

fn insert_widget(parent: &SharedWidget, child: &SharedWidget) {
    let mut parent_borrow = parent.borrow_mut();
    let child_borrow = child.borrow();
    
    fn add_to_group<W: WidgetExt + Clone>(group: &mut fltk::group::Group, child_widget: &W) {
        group.add(child_widget);
        let mut w = child_widget.clone();
        w.show();
    }
    fn add_to_flex<W: WidgetExt + Clone>(flex: &mut fltk::group::Flex, child_widget: &W) {
        flex.add(child_widget);
        let mut w = child_widget.clone();
        w.show();
    }
    
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
            }
            group.redraw();
            if let Some(mut w) = group.window() {
                w.redraw();
            }
        }
        _ => {}
    }
}

fn apply_class(widget: &mut FltkWidget, class_name: &str) {
    let rule_opt = STYLESHEET.with(|s| {
        s.borrow().get(class_name).cloned()
    });

    if let Some(rule) = rule_opt {
        apply_style_rule(widget, &rule);
        return;
    }

    match class_name {
        "window" => {
            if let FltkWidget::Window(w, _) = widget {
                w.set_color(fltk::enums::Color::from_rgb(240, 240, 245));
            }
        }
        "btn-primary" => {
            if let FltkWidget::Button(b) = widget {
                b.set_color(fltk::enums::Color::from_rgb(0, 122, 255));
                b.set_label_color(fltk::enums::Color::White);
                b.set_frame(fltk::enums::FrameType::RFlatBox);
            }
        }
        "btn-danger" => {
            if let FltkWidget::Button(b) = widget {
                b.set_color(fltk::enums::Color::from_rgb(255, 59, 48));
                b.set_label_color(fltk::enums::Color::White);
                b.set_frame(fltk::enums::FrameType::RFlatBox);
            }
        }
        "dark-mode" => {
            match widget {
                FltkWidget::Window(w, _) => w.set_color(fltk::enums::Color::from_rgb(30, 30, 35)),
                FltkWidget::Button(b) => {
                    b.set_color(fltk::enums::Color::from_rgb(50, 50, 55));
                    b.set_label_color(fltk::enums::Color::White);
                }
                FltkWidget::Label(l) => l.set_label_color(fltk::enums::Color::White),
                FltkWidget::Header(h) => h.set_label_color(fltk::enums::Color::White),
                _ => {}
            }
        }
        _ => {}
    }
}

fn trigger_event_callback(widget_id: &str, event_type: &str) {
    let func_name_opt = WIDGET_EVENT_CALLBACKS.with(|c| {
        c.borrow().get(widget_id)
            .and_then(|events| events.get(event_type))
            .cloned()
    });
    if let Some(func_name) = func_name_opt {
        let run = crate::aly::get_runtime();
        let fake_lexer = vec![
            Lexer::new(Tokens::Reference, func_name, 0),
            Lexer::new(Tokens::LeftParenthesis, "(".to_string(), 0),
            Lexer::new(Tokens::RightParenthesis, ")".to_string(), 0),
        ];
        run.function_run(fake_lexer);
    }
}

macro_rules! setup_handle_event {
    ($widget:expr, $target_event:expr, $id_clone:expr, $event_name:expr) => {{
        let id = $id_clone.clone();
        let evt = $target_event;
        let ename = $event_name.clone();
        $widget.handle(move |_w, ev| {
            if ev == evt {
                trigger_event_callback(&id, &ename);
            }
            false
        });
    }};
}

fn setup_event_handler(widget_id: &str, shared: &SharedWidget, event_type: &str, fltk_event: Event) {
    let id_clone = widget_id.to_owned();
    let event_name = event_type.to_owned();
    let mut borrow = shared.borrow_mut();
    
    match &mut *borrow {
        FltkWidget::Button(b) => { setup_handle_event!(b, fltk_event, id_clone, event_name); }
        FltkWidget::Label(l) => { setup_handle_event!(l, fltk_event, id_clone, event_name); }
        FltkWidget::Header(h) => { setup_handle_event!(h, fltk_event, id_clone, event_name); }
        FltkWidget::Input(i) => { setup_handle_event!(i, fltk_event, id_clone, event_name); }
        FltkWidget::TextArea(t) => { setup_handle_event!(t, fltk_event, id_clone, event_name); }
        FltkWidget::PasswordField(p) => { setup_handle_event!(p, fltk_event, id_clone, event_name); }
        FltkWidget::Slider(s) => { setup_handle_event!(s, fltk_event, id_clone, event_name); }
        FltkWidget::Checkbox(c) => { setup_handle_event!(c, fltk_event, id_clone, event_name); }
        FltkWidget::Radio(r) => { setup_handle_event!(r, fltk_event, id_clone, event_name); }
        FltkWidget::Dropdown(d) => { setup_handle_event!(d, fltk_event, id_clone, event_name); }
        FltkWidget::ProgressBar(p) => { setup_handle_event!(p, fltk_event, id_clone, event_name); }
        FltkWidget::Spinner(s) => { setup_handle_event!(s, fltk_event, id_clone, event_name); }
        FltkWidget::Window(w, _) => { setup_handle_event!(w, fltk_event, id_clone, event_name); }
        FltkWidget::Div(d) => { setup_handle_event!(d, fltk_event, id_clone, event_name); }
        FltkWidget::Radius(r) => { setup_handle_event!(r, fltk_event, id_clone, event_name); }
    }
}

fn setup_on_change_callback(widget_id: &str, shared: &SharedWidget) {
    let id_clone = widget_id.to_owned();
    let mut borrow = shared.borrow_mut();
    
    match &mut *borrow {
        FltkWidget::Input(inp) => {
            inp.set_callback(move |_| {
                trigger_event_callback(&id_clone, "onChange");
            });
        }
        FltkWidget::TextArea(ta) => {
            let id2 = id_clone.clone();
            ta.set_callback(move |_| {
                trigger_event_callback(&id2, "onChange");
            });
        }
        FltkWidget::Slider(sl) => {
            let id2 = id_clone.clone();
            sl.set_callback(move |_| {
                trigger_event_callback(&id2, "onChange");
            });
        }
        FltkWidget::Checkbox(cb) => {
            let id2 = id_clone.clone();
            cb.set_callback(move |_| {
                trigger_event_callback(&id2, "onChange");
            });
        }
        FltkWidget::Dropdown(ch) => {
            let id2 = id_clone.clone();
            ch.set_callback(move |_| {
                trigger_event_callback(&id2, "onChange");
            });
        }
        FltkWidget::Spinner(sp) => {
            let id2 = id_clone.clone();
            sp.set_callback(move |_| {
                trigger_event_callback(&id2, "onChange");
            });
        }
        _ => {}
    }
}

pub fn call_widget_method(widget_id: &str, method: &str, args: Vec<ValueData>) -> ValueData {
    let widget_opt = GUI_WIDGETS.with(|w| {
        w.borrow().get(widget_id).cloned()
    });
    
    if let Some(shared) = widget_opt {
        match method {
            "insert" => {
                if let Some(ValueData::Widget(child_id)) = args.get(0) {
                    let child_opt = GUI_WIDGETS.with(|w| {
                        w.borrow().get(child_id).cloned()
                    });
                    if let Some(child_shared) = child_opt {
                        insert_widget(&shared, &child_shared);
                    }
                }
            }
            "loop" => {
                let mut borrow = shared.borrow_mut();
                if let FltkWidget::Window(win, flex) = &mut *borrow {
                    flex.layout();
                    win.show();
                }
                fltk::app::App::default().run().unwrap();
            }
            "addClass" => {
                if let Some(class_val) = args.get(0) {
                    let class_name = class_val.to_string(false);
                    let mut borrow = shared.borrow_mut();
                    apply_class(&mut *borrow, &class_name);
                }
            }
            "style" => {
                if let Some(style_val) = args.get(0) {
                    let style_str = style_val.to_string(false);
                    apply_css_rules_to_selector("temp_style", &style_str);
                    let temp_rule = STYLESHEET.with(|s| {
                        s.borrow().get("temp_style").cloned().unwrap_or_default()
                    });
                    let mut borrow = shared.borrow_mut();
                    apply_style_rule(&mut *borrow, &temp_rule);
                }
            }
            "setStyle" => {
                if args.len() == 2 {
                    let prop = args[0].to_string(false);
                    let val = args[1].to_string(false);
                    let style_str = format!("{}: {}", prop, val);
                    apply_css_rules_to_selector("temp_style", &style_str);
                    let temp_rule = STYLESHEET.with(|s| {
                        s.borrow().get("temp_style").cloned().unwrap_or_default()
                    });
                    let mut borrow = shared.borrow_mut();
                    apply_style_rule(&mut *borrow, &temp_rule);
                }
            }
            "importStyle" => {
                if let Some(path_val) = args.get(0) {
                    let path = path_val.to_string(false);
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        parse_css(&content);
                    }
                }
            }
            "onClick" => {
                if let Some(func_val) = args.get(0) {
                    let func_name = match func_val {
                        ValueData::Function(_, _, _) => {
                            let run = crate::aly::get_runtime();
                            let mut found_name = None;
                            for var in run.get_vars() {
                                if let ValueData::Function(_, _, ref body) = var.get_value() {
                                    if let ValueData::Function(_, _, ref target_body) = func_val {
                                        if body == target_body {
                                            found_name = Some(var.get_name());
                                            break;
                                        }
                                    }
                                }
                            }
                            found_name.unwrap_or_else(|| "anonymous".to_owned())
                        }
                        other => other.to_string(false),
                    };
                    
                    WIDGET_CALLBACKS.with(|c| {
                        c.borrow_mut().insert(widget_id.to_owned(), func_name);
                    });
                    
                    let mut borrow = shared.borrow_mut();
                    if let FltkWidget::Button(btn) = &mut *borrow {
                        let id_clone = widget_id.to_owned();
                        btn.set_callback(move |_| {
                            let func_name_opt = WIDGET_CALLBACKS.with(|c| {
                                c.borrow().get(&id_clone).cloned()
                            });
                            if let Some(func_name) = func_name_opt {
                                let run = crate::aly::get_runtime();
                                let fake_lexer = vec![
                                    Lexer::new(Tokens::Reference, func_name, 0),
                                    Lexer::new(Tokens::LeftParenthesis, "(".to_string(), 0),
                                    Lexer::new(Tokens::RightParenthesis, ")".to_string(), 0),
                                ];
                                run.function_run(fake_lexer);
                            }
                        });
                    }
                }
            }
            "onChange" => {
                if let Some(func_val) = args.get(0) {
                    let func_name = match func_val {
                        ValueData::Function(_, _, _) => {
                            let run = crate::aly::get_runtime();
                            let mut found_name = None;
                            for var in run.get_vars() {
                                if let ValueData::Function(_, _, ref body) = var.get_value() {
                                    if let ValueData::Function(_, _, ref target_body) = func_val {
                                        if body == target_body {
                                            found_name = Some(var.get_name());
                                            break;
                                        }
                                    }
                                }
                            }
                            found_name.unwrap_or_else(|| "anonymous".to_owned())
                        }
                        other => other.to_string(false),
                    };
                    
                    WIDGET_EVENT_CALLBACKS.with(|c| {
                        c.borrow_mut()
                            .entry(widget_id.to_owned())
                            .or_insert_with(HashMap::new)
                            .insert("onChange".to_owned(), func_name);
                    });
                    
                    setup_on_change_callback(widget_id, &shared);
                }
            }
            "onMouseOver" | "onMouseOut" | "onFocus" | "onBlur" | "onKeyPress" | "onMouseWheel" | "onResize" => {
                if let Some(func_val) = args.get(0) {
                    let func_name = match func_val {
                        ValueData::Function(_, _, _) => {
                            let run = crate::aly::get_runtime();
                            let mut found_name = None;
                            for var in run.get_vars() {
                                if let ValueData::Function(_, _, ref body) = var.get_value() {
                                    if let ValueData::Function(_, _, ref target_body) = func_val {
                                        if body == target_body {
                                            found_name = Some(var.get_name());
                                            break;
                                        }
                                    }
                                }
                            }
                            found_name.unwrap_or_else(|| "anonymous".to_owned())
                        }
                        other => other.to_string(false),
                    };
                    
                    let event_name = method.to_owned();
                    WIDGET_EVENT_CALLBACKS.with(|c| {
                        c.borrow_mut()
                            .entry(widget_id.to_owned())
                            .or_insert_with(HashMap::new)
                            .insert(event_name.clone(), func_name);
                    });
                    
                    let fltk_event = match method {
                        "onMouseOver" => Event::Enter,
                        "onMouseOut" => Event::Leave,
                        "onFocus" => Event::Focus,
                        "onBlur" => Event::Unfocus,
                        "onKeyPress" => Event::KeyDown,
                        "onMouseWheel" => Event::MouseWheel,
                        "onResize" => Event::Resize,
                        _ => return ValueData::String("None".to_owned()),
                    };
                    
                    setup_event_handler(widget_id, &shared, &event_name, fltk_event);
                }
            }
            _ => {}
        }
    }
    
    ValueData::String("None".to_owned())
}

pub fn set_widget_property(widget_id: &str, property: &str, value: ValueData) {
    let widget_opt = GUI_WIDGETS.with(|w| {
        w.borrow().get(widget_id).cloned()
    });
    
    if let Some(shared) = widget_opt {
        let mut borrow = shared.borrow_mut();
        let val_str = value.to_string(false);
        match property {
            "innerText" | "value" | "label" => {
                match &mut *borrow {
                    FltkWidget::Window(w, _) => { w.set_label(&val_str); w.redraw(); }
                    FltkWidget::Button(b) => { b.set_label(&val_str); b.redraw(); }
                    FltkWidget::Label(l) => { l.set_label(&val_str); l.redraw(); }
                    FltkWidget::Header(h) => { h.set_label(&val_str); h.redraw(); }
                    FltkWidget::Input(i) => { i.set_value(&val_str); i.redraw(); }
                    FltkWidget::TextArea(t) => { t.set_value(&val_str); t.redraw(); }
                    FltkWidget::PasswordField(p) => { p.set_value(&val_str); p.redraw(); }
                    FltkWidget::Slider(s) => { s.set_label(&val_str); s.redraw(); }
                    FltkWidget::Checkbox(c) => { c.set_label(&val_str); c.redraw(); }
                    FltkWidget::Radio(r) => { r.set_label(&val_str); r.redraw(); }
                    FltkWidget::Dropdown(d) => { d.set_label(&val_str); d.redraw(); }
                    FltkWidget::ProgressBar(p) => { p.set_label(&val_str); p.redraw(); }
                    FltkWidget::Spinner(s) => { s.set_label(&val_str); s.redraw(); }
                    FltkWidget::Div(_) | FltkWidget::Radius(_) => {}
                }
            }
            "value_int" => {
                if let ValueData::Int(n) = value {
                    match &mut *borrow {
                        FltkWidget::Slider(s) => { s.set_value(n as f64); s.redraw(); }
                        FltkWidget::Spinner(sp) => { sp.set_value(n as f64); sp.redraw(); }
                        FltkWidget::ProgressBar(p) => { p.set_value(n as f64); p.redraw(); }
                        _ => {}
                    }
                }
            }
            "value_float" => {
                if let ValueData::Float(f) = value {
                    match &mut *borrow {
                        FltkWidget::Slider(s) => { s.set_value(f); s.redraw(); }
                        FltkWidget::Spinner(sp) => { sp.set_value(f); sp.redraw(); }
                        FltkWidget::ProgressBar(p) => { p.set_value(f); p.redraw(); }
                        _ => {}
                    }
                }
            }
            "checked" => {
                let checked = val_str == "true" || val_str == "1";
                if let FltkWidget::Checkbox(c) = &mut *borrow {
                    c.set_value(checked);
                    c.redraw();
                }
            }
            "id" => {}
            _ => {}
        }
    }
}

pub fn get_widget_property(widget_id: &str, property: &str) -> ValueData {
    let widget_opt = GUI_WIDGETS.with(|w| {
        w.borrow().get(widget_id).cloned()
    });
    
    if let Some(shared) = widget_opt {
        let borrow = shared.borrow();
        match property {
            "innerText" | "value" | "label" => {
                let val_str = match &*borrow {
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
                    FltkWidget::Div(_) | FltkWidget::Radius(_) => return ValueData::String("None".to_owned()),
                };
                return ValueData::String(val_str);
            }
            "value_int" => {
                let n = match &*borrow {
                    FltkWidget::Slider(s) => s.value() as i64,
                    FltkWidget::Spinner(sp) => sp.value() as i64,
                    FltkWidget::ProgressBar(p) => p.value() as i64,
                    _ => return ValueData::String("None".to_owned()),
                };
                return ValueData::Int(n);
            }
            "value_float" => {
                let f = match &*borrow {
                    FltkWidget::Slider(s) => s.value(),
                    FltkWidget::Spinner(sp) => sp.value(),
                    FltkWidget::ProgressBar(p) => p.value(),
                    _ => return ValueData::String("None".to_owned()),
                };
                return ValueData::Float(f);
            }
            "checked" => {
                if let FltkWidget::Checkbox(c) = &*borrow {
                    return ValueData::Bool(c.value());
                }
                return ValueData::Bool(false);
            }
            "selected" => {
                if let FltkWidget::Dropdown(d) = &*borrow {
                    let text = d.text(0).unwrap_or_default();
                    let idx = d.value();
                    return ValueData::String(format!("{}:{}", text, idx));
                }
                return ValueData::String("None".to_owned());
            }
            "id" => {
                return ValueData::String(widget_id.to_owned());
            }
            _ => {}
        }
    }
    
    ValueData::String("None".to_owned())
}

// ---------------------------------------------------------------------
// Stylesheet & CSS/SCSS parser implementation
// ---------------------------------------------------------------------

pub fn parse_css(css_content: &str) {
    let mut current_selector = String::new();
    let mut in_block = false;
    let mut block_content = String::new();

    for line in css_content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") || line.starts_with("/*") {
            continue;
        }
        if line.contains('{') {
            let parts: Vec<&str> = line.split('{').collect();
            current_selector = parts[0].trim().replace(".", "");
            in_block = true;
            block_content.clear();
            if parts.len() > 1 && parts[1].contains('}') {
                let inside = parts[1].split('}').next().unwrap_or("");
                apply_css_rules_to_selector(&current_selector, inside);
                in_block = false;
            }
        } else if line.contains('}') {
            apply_css_rules_to_selector(&current_selector, &block_content);
            in_block = false;
        } else if in_block {
            block_content.push_str(line);
            block_content.push(' ');
        }
    }
}

fn apply_css_rules_to_selector(selector: &str, rules_str: &str) {
    let mut rule = StyleRule::default();
    
    for dec in rules_str.split(';') {
        let dec = dec.trim();
        if dec.is_empty() {
            continue;
        }
        let parts: Vec<&str> = dec.split(':').collect();
        if parts.len() == 2 {
            let prop = parts[0].trim();
            let val = parts[1].trim().replace("'", "").replace("\"", "");
            
            match prop {
                "background-color" | "background" => {
                    if let Some(col) = parse_color(&val) {
                        rule.background_color = Some(col);
                    }
                }
                "color" => {
                    if let Some(col) = parse_color(&val) {
                        rule.text_color = Some(col);
                    }
                }
                "font-size" => {
                    let size_str: String = val.chars().filter(|c| c.is_numeric()).collect();
                    if let Ok(size) = size_str.parse::<i32>() {
                        rule.font_size = Some(size);
                    }
                }
                "border-radius" | "radius" => {
                    let rad_str: String = val.chars().filter(|c| c.is_numeric()).collect();
                    if let Ok(rad) = rad_str.parse::<i32>() {
                        rule.border_radius = Some(rad);
                    }
                }
                "padding" => {
                    let pad_str: String = val.chars().filter(|c| c.is_numeric()).collect();
                    if let Ok(pad) = pad_str.parse::<i32>() {
                        rule.padding = Some(pad);
                    }
                }
                "width" => {
                    let w_str: String = val.chars().filter(|c| c.is_numeric()).collect();
                    if let Ok(w) = w_str.parse::<i32>() {
                        rule.width = Some(w);
                    }
                }
                "height" => {
                    let h_str: String = val.chars().filter(|c| c.is_numeric()).collect();
                    if let Ok(h) = h_str.parse::<i32>() {
                        rule.height = Some(h);
                    }
                }
                "text-align" => {
                    rule.text_align = Some(val.to_lowercase());
                }
                "font-weight" => {
                    let fw_str: String = val.chars().filter(|c| c.is_numeric()).collect();
                    if let Ok(fw) = fw_str.parse::<i32>() {
                        rule.font_weight = Some(fw);
                    }
                }
                "border-color" => {
                    if let Some(col) = parse_color(&val) {
                        rule.border_color = Some(col);
                    }
                }
                "border-width" | "border" => {
                    let bw_str: String = val.chars().filter(|c| c.is_numeric()).collect();
                    if let Ok(bw) = bw_str.parse::<i32>() {
                        rule.border_width = Some(bw);
                    }
                }
                "display" => {
                    rule.display = Some(val.to_lowercase());
                }
                "opacity" => {
                    if let Ok(o) = val.parse::<f64>() {
                        rule.opacity = Some(o);
                    }
                }
                _ => {}
            }
        }
    }

    STYLESHEET.with(|s| {
        s.borrow_mut().insert(selector.to_owned(), rule);
    });
}

fn parse_color(val: &str) -> Option<fltk::enums::Color> {
    let val = val.trim().to_lowercase();
    if val.starts_with('#') {
        let hex = val.replace("#", "");
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            return Some(fltk::enums::Color::from_rgb(r, g, b));
        }
    }
    match val.as_str() {
        "white" => Some(fltk::enums::Color::White),
        "black" => Some(fltk::enums::Color::Black),
        "red" => Some(fltk::enums::Color::from_rgb(255, 59, 48)),
        "blue" => Some(fltk::enums::Color::from_rgb(0, 122, 255)),
        "green" => Some(fltk::enums::Color::from_rgb(52, 199, 89)),
        "gray" | "grey" => Some(fltk::enums::Color::from_rgb(142, 142, 147)),
        "yellow" => Some(fltk::enums::Color::from_rgb(255, 204, 0)),
        "orange" => Some(fltk::enums::Color::from_rgb(255, 149, 0)),
        "purple" => Some(fltk::enums::Color::from_rgb(175, 82, 222)),
        "pink" => Some(fltk::enums::Color::from_rgb(255, 45, 85)),
        "brown" => Some(fltk::enums::Color::from_rgb(162, 132, 94)),
        "cyan" => Some(fltk::enums::Color::from_rgb(50, 173, 230)),
        "lime" => Some(fltk::enums::Color::from_rgb(90, 200, 100)),
        "navy" => Some(fltk::enums::Color::from_rgb(0, 31, 84)),
        "teal" => Some(fltk::enums::Color::from_rgb(0, 128, 128)),
        "transparent" => Some(fltk::enums::Color::from_rgb(255, 255, 255)),
        _ => None,
    }
}

fn apply_style_rule(widget: &mut FltkWidget, rule: &StyleRule) {
    // Apply display property (show/hide)
    if let Some(ref display) = rule.display {
        let is_visible = display != "none";
        let show_fn = |w: &mut dyn WidgetExt| {
            if is_visible { w.show(); } else { w.hide(); }
        };
        match widget {
            FltkWidget::Window(w, _) => show_fn(w),
            FltkWidget::Button(b) => show_fn(b),
            FltkWidget::Label(l) => show_fn(l),
            FltkWidget::Header(h) => show_fn(h),
            FltkWidget::Input(i) => show_fn(i),
            FltkWidget::TextArea(t) => show_fn(t),
            FltkWidget::PasswordField(p) => show_fn(p),
            FltkWidget::Slider(s) => show_fn(s),
            FltkWidget::Checkbox(c) => show_fn(c),
            FltkWidget::Radio(r) => show_fn(r),
            FltkWidget::Dropdown(d) => show_fn(d),
            FltkWidget::ProgressBar(p) => show_fn(p),
            FltkWidget::Spinner(sp) => show_fn(sp),
            _ => {}
        }
    }
    
    let bg = rule.background_color;
    let tc = rule.text_color;
    let fs = rule.font_size;
    let br = rule.border_radius;
    let pad = rule.padding;
    let w = rule.width;
    let h = rule.height;
    let ta = rule.text_align.as_deref();
    let fw = rule.font_weight;
    let bc = rule.border_color;
    let bw = rule.border_width;
    
    match widget {
        FltkWidget::Window(win, _) => {
            if let Some(col) = bg { win.set_color(col); }
            if let Some(p) = pad { /* flex margin handled below */ }
        }
        FltkWidget::Button(b) => {
            if let Some(col) = bg { b.set_color(col); b.set_frame(fltk::enums::FrameType::RFlatBox); }
            if let Some(col) = tc { b.set_label_color(col); }
            if let Some(size) = fs { b.set_label_size(size); }
            if br.is_some() { b.set_frame(fltk::enums::FrameType::RFlatBox); }
            if let Some(p) = bw { /* not directly supported */ }
        }
        FltkWidget::Label(l) => {
            if let Some(col) = bg { l.set_color(col); }
            if let Some(col) = tc { l.set_label_color(col); }
            if let Some(size) = fs { l.set_label_size(size); }
            if let Some(a) = ta {
                if a == "center" { l.set_label_type(fltk::enums::LabelType::Normal); }
            }
            if let Some(wt) = fw {
                if wt >= 700 { l.set_label_type(fltk::enums::LabelType::Engraved); }
            }
        }
        FltkWidget::Header(hdr) => {
            if let Some(col) = bg { hdr.set_color(col); }
            if let Some(col) = tc { hdr.set_label_color(col); }
            if let Some(size) = fs { hdr.set_label_size(size); }
        }
        FltkWidget::Div(flex) => {
            if let Some(col) = bg { flex.set_color(col); }
            if let Some(p) = pad { flex.set_margin(p); }
        }
        FltkWidget::Radius(r) => {
            if let Some(col) = bg { r.set_color(col); }
        }
        FltkWidget::Input(inp) => {
            if let Some(col) = bg { inp.set_color(col); }
            if let Some(col) = tc { inp.set_text_color(col); }
            if let Some(size) = fs { inp.set_text_size(size); }
        }
        FltkWidget::TextArea(ta_w) => {
            if let Some(col) = bg { ta_w.set_color(col); }
            if let Some(col) = tc { ta_w.set_text_color(col); }
            if let Some(size) = fs { ta_w.set_text_size(size); }
        }
        FltkWidget::PasswordField(pf) => {
            if let Some(col) = bg { pf.set_color(col); }
            if let Some(col) = tc { pf.set_text_color(col); }
            if let Some(size) = fs { pf.set_text_size(size); }
        }
        FltkWidget::Slider(sl) => {
            if let Some(col) = bg { sl.set_color(col); }
            if let Some(size) = fs { sl.set_label_size(size); }
        }
        FltkWidget::Checkbox(cb) => {
            if let Some(col) = bg { cb.set_color(col); }
            if let Some(col) = tc { cb.set_label_color(col); }
            if let Some(size) = fs { cb.set_label_size(size); }
        }
        FltkWidget::Radio(rb) => {
            if let Some(col) = bg { rb.set_color(col); }
            if let Some(col) = tc { rb.set_label_color(col); }
            if let Some(size) = fs { rb.set_label_size(size); }
        }
        FltkWidget::Dropdown(ch) => {
            if let Some(col) = bg { ch.set_color(col); }
            if let Some(col) = tc { ch.set_label_color(col); }
            if let Some(size) = fs { ch.set_label_size(size); }
        }
        FltkWidget::ProgressBar(pb) => {
            if let Some(col) = bg { pb.set_color(col); }
            if let Some(col) = tc { pb.set_label_color(col); }
        }
        FltkWidget::Spinner(sp) => {
            if let Some(col) = bg { sp.set_color(col); }
            if let Some(col) = tc { sp.set_text_color(col); }
            if let Some(size) = fs { sp.set_text_size(size); }
        }
    }
}
