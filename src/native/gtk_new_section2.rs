// ─────────────────────────────────────────────────────────────────────────────
// Widget Operations
// ─────────────────────────────────────────────────────────────────────────────

fn get_widget(id: &str) -> Option<SharedGtkWidget> {
    GTK_WIDGETS.with(|w| w.borrow().get(id).cloned())
}

pub fn gtk_insert(parent: String, child: String) -> Box<dyn Validator> {
    let parent_widget = match get_widget(&parent) {
        Some(w) => w,
        None => {
            eprintln!("RuntimeError [gtk]: parent widget '{}' not found.", parent);
            return Box::new(put_quoted_str("None".to_string()));
        }
    };
    let child_widget = match get_widget(&child) {
        Some(w) => w,
        None => {
            eprintln!("RuntimeError [gtk]: child widget '{}' not found.", child);
            return Box::new(put_quoted_str("None".to_string()));
        }
    };

    let p = parent_widget.borrow();
    match &*p {
        GtkWidget::Window(win) => {
            win.set_child(Some(&child_widget.borrow().as_gtk_widget().clone()));
        }
        GtkWidget::Div(box_widget) => {
            box_widget.append(&child_widget.borrow().as_gtk_widget().clone());
        }
        GtkWidget::Tabs(nb) => {
            let label = Label::new(Some("Tab"));
            let child_clone = child_widget.borrow().as_gtk_widget().clone();
            nb.append_page(&child_clone, Some(&label));
        }
        _ => {
            eprintln!("RuntimeError [gtk]: parent widget '{}' is not a container.", parent);
        }
    }

    Box::new(put_quoted_str("OK".to_string()))
}

pub fn gtk_set_prop(id: String, prop: String, value: String) -> Box<dyn Validator> {
    let widget_opt = get_widget(&id);
    if widget_opt.is_none() {
        eprintln!("RuntimeError [gtk]: widget '{}' not found.", id);
        return Box::new(put_quoted_str("None".to_string()));
    }
    let shared = widget_opt.unwrap();
    let mut borrow = shared.borrow_mut();

    match prop.as_str() {
        "label" | "innerText" | "value" => match &mut *borrow {
            GtkWidget::Button(b) => b.set_label(&value),
            GtkWidget::Label(l) => l.set_label(&value),
            GtkWidget::Header(hdr) => hdr.set_label(&value),
            GtkWidget::Input(e) => e.set_text(&value),
            GtkWidget::TextArea(tv) => {
                let buf = tv.buffer();
                buf.set_text(&value);
            }
            GtkWidget::PasswordField(e) => e.set_text(&value),
            GtkWidget::Checkbox(c) => c.set_label(&value),
            GtkWidget::Radio(r) => r.set_label(&value),
            _ => {}
        },
        "checked" => match &mut *borrow {
            GtkWidget::Checkbox(c) => c.set_active(value == "true" || value == "1"),
            GtkWidget::Radio(r) => r.set_active(value == "true" || value == "1"),
            _ => {}
        },
        "enabled" => match &mut *borrow {
            GtkWidget::Button(b) => b.set_sensitive(value != "false"),
            GtkWidget::Input(e) => e.set_sensitive(value != "false"),
            GtkWidget::Slider(s) => s.set_sensitive(value != "false"),
            GtkWidget::Checkbox(c) => c.set_sensitive(value != "false"),
            _ => {}
        },
        "visible" => match &mut *borrow {
            GtkWidget::Window(w) => w.set_visible(value != "false"),
            _ => {}
        },
        "fraction" => match &mut *borrow {
            GtkWidget::ProgressBar(p) => {
                if let Ok(v) = value.parse::<f64>() { p.set_fraction(v); }
            }
            _ => {}
        },
        _ => {}
    }

    Box::new(put_quoted_str("OK".to_string()))
}

pub fn gtk_get_prop(id: String, prop: String) -> Box<dyn Validator> {
    let widget_opt = get_widget(&id);
    if widget_opt.is_none() {
        return Box::new(put_quoted_str("None".to_owned()));
    }
    let shared = widget_opt.unwrap();
    let borrow = shared.borrow();

    match prop.as_str() {
        "label" | "innerText" | "value" => match &*borrow {
            GtkWidget::Button(b) => Box::new(put_quoted_str(b.label())),
            GtkWidget::Label(l) => Box::new(put_quoted_str(l.label())),
            GtkWidget::Input(e) => Box::new(put_quoted_str(e.text())),
            GtkWidget::TextArea(tv) => {
                let buf = tv.buffer();
                let start = buf.start_iter();
                let end = buf.end_iter();
                let text = buf.text(&start, &end, true);
                Box::new(put_quoted_str(text.to_string()))
            }
            GtkWidget::PasswordField(e) => Box::new(put_quoted_str(e.text())),
            GtkWidget::Checkbox(c) => Box::new(put_quoted_str(c.label())),
            GtkWidget::Radio(r) => Box::new(put_quoted_str(r.label())),
            _ => Box::new(put_quoted_str("None".to_owned())),
        },
        "checked" => match &*borrow {
            GtkWidget::Checkbox(c) => Box::new(ValueData::Bool(c.is_active())),
            GtkWidget::Radio(r) => Box::new(ValueData::Bool(r.is_active())),
            _ => Box::new(put_quoted_str("None".to_owned())),
        },
        "selected" => match &*borrow {
            GtkWidget::Dropdown(d) => {
                if let Some(idx) = d.selected() {
                    let text = d.selected_string().unwrap_or_default();
                    Box::new(put_quoted_str(format!("{}:{}", text, idx)))
                } else {
                    Box::new(put_quoted_str("None".to_owned()))
                }
            }
            _ => Box::new(put_quoted_str("None".to_owned())),
        },
        "value_int" => match &*borrow {
            GtkWidget::Slider(s) => Box::new(ValueData::Int(s.value() as i32)),
            GtkWidget::ProgressBar(p) => Box::new(ValueData::Int((p.fraction() * 100.0).round() as i32)),
            _ => Box::new(put_quoted_str("None".to_owned())),
        },
        "value_float" => match &*borrow {
            GtkWidget::Slider(s) => Box::new(ValueData::Float(s.value() as f32)),
            GtkWidget::ProgressBar(p) => Box::new(ValueData::Float(p.fraction() as f32)),
            _ => Box::new(put_quoted_str("None".to_owned())),
        },
        "id" => Box::new(put_quoted_str(id)),
        _ => Box::new(put_quoted_str("None".to_owned())),
    }
}

pub fn gtk_add_class(id: String, class: String) -> Box<dyn Validator> {
    if let Some(shared) = get_widget(&id) {
        let borrow = shared.borrow();
        borrow.as_gtk_widget().add_css_class(&class);
    }
    Box::new(put_quoted_str("OK".to_string()))
}

pub fn gtk_set_style(id: String, style_str: String) -> Box<dyn Validator> {
    let class_name = format!("aly-inline-{}", id.replace("widget_", ""));
    if let Some(shared) = get_widget(&id) {
        let borrow = shared.borrow();
        borrow.as_gtk_widget().add_css_class(&class_name);
    }
    let css_rule = format!(".{} {{ {} }}\n", class_name, style_str);
    GTK_CSS.with(|css| css.borrow_mut().push_str(&css_rule));
    Box::new(put_quoted_str("OK".to_string()))
}

pub fn gtk_import_style(args: Vec<ValueData>) -> Box<dyn Validator> {
    let path = args.get(0).map(|v| v.to_string(false)).unwrap_or_else(|| "".to_string());
    if path.is_empty() {
        return Box::new(put_quoted_str("None".to_string()));
    }
    match std::fs::read_to_string(&path) {
        Ok(content) => {
            GTK_CSS.with(|css| css.borrow_mut().push_str(&content));
            Box::new(put_quoted_str("OK".to_string()))
        }
        Err(e) => {
            eprintln!("RuntimeError [gtk]: failed to import CSS from '{}': {}", path, e);
            Box::new(put_quoted_str("None".to_string()))
        }
    }
}
