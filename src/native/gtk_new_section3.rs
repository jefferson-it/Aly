// ─────────────────────────────────────────────────────────────────────────────
// Event Callbacks
// ─────────────────────────────────────────────────────────────────────────────

pub fn gtk_on_click(id: String, func: String) -> Box<dyn Validator> {
    GTK_EVENT_CALLBACKS.with(|c| {
        c.borrow_mut()
            .entry(id.clone())
            .or_default()
            .insert("clicked".to_string(), func);
    });
    if let Some(shared) = get_widget(&id) {
        let widget_id = id.clone();
        let borrow = shared.borrow();
        if let GtkWidget::Button(btn) = &*borrow {
            let wid = widget_id.clone();
            btn.connect_clicked(move |_| {
                fire_callback(&wid, "clicked");
            });
        }
    }
    Box::new(put_quoted_str("OK".to_string()))
}

pub fn gtk_on_change(id: String, func: String) -> Box<dyn Validator> {
    GTK_EVENT_CALLBACKS.with(|c| {
        c.borrow_mut()
            .entry(id.clone())
            .or_default()
            .insert("changed".to_string(), func);
    });
    if let Some(shared) = get_widget(&id) {
        let widget_id = id.clone();
        let borrow = shared.borrow();
        if let GtkWidget::Input(e) = &*borrow {
            let wid = widget_id.clone();
            e.connect_changed(move |_| {
                fire_callback(&wid, "changed");
            });
        } else if let GtkWidget::Dropdown(dd) = &*borrow {
            let wid = widget_id.clone();
            dd.connect_notify(Some("selected"), move |dd, _| {
                fire_callback(&wid, "changed");
            });
        }
    }
    Box::new(put_quoted_str("OK".to_string()))
}

pub fn gtk_on_hover(id: String, entered_func: String, left_func: String) -> Box<dyn Validator> {
    GTK_EVENT_CALLBACKS.with(|c| {
        c.borrow_mut()
            .entry(id.clone())
            .or_default()
            .insert("enter".to_string(), entered_func);
        c.borrow_mut()
            .entry(id.clone())
            .or_default()
            .insert("leave".to_string(), left_func);
    });
    if let Some(shared) = get_widget(&id) {
        let widget_id = id.clone();
        let borrow = shared.borrow();
        let widget_clone = borrow.as_gtk_widget().clone();
        let event_controller = gtk4::EventControllerMotion::new();
        let wid = widget_id.clone();
        event_controller.connect_enter(move |_, _, _| {
            fire_callback(&wid, "enter");
        });
        let wid2 = widget_id.clone();
        event_controller.connect_leave(move |_| {
            fire_callback(&wid2, "leave");
        });
        widget_clone.add_controller(&event_controller);
    }
    Box::new(put_quoted_str("OK".to_string()))
}

pub fn gtk_on_close(id: String, func: String) -> Box<dyn Validator> {
    GTK_EVENT_CALLBACKS.with(|c| {
        c.borrow_mut()
            .entry(id.clone())
            .or_default()
            .insert("close".to_string(), func);
    });
    if let Some(shared) = get_widget(&id) {
        let widget_id = id.clone();
        let borrow = shared.borrow();
        if let GtkWidget::Window(win) = &*borrow {
            let wid = widget_id.clone();
            win.connect_close_request(move |_| {
                fire_callback(&wid, "close");
                gtk4::Inhibit(false)
            });
        }
    }
    Box::new(put_quoted_str("OK".to_string()))
}

pub fn gtk_on_resize(id: String, func: String) -> Box<dyn Validator> {
    GTK_EVENT_CALLBACKS.with(|c| {
        c.borrow_mut()
            .entry(id.clone())
            .or_default()
            .insert("resize".to_string(), func);
    });
    Box::new(put_quoted_str("OK".to_string()))
}

pub fn gtk_on_close(id: String, func: String) -> Box<dyn Validator> {
    GTK_EVENT_CALLBACKS.with(|c| {
        c.borrow_mut()
            .entry(id.clone())
            .or_default()
            .insert("close".to_string(), func);
    });
    Box::new(put_quoted_str("OK".to_string()))
}

fn fire_callback(widget_id: &str, event: &str) {
    let func_name = GTK_EVENT_CALLBACKS.with(|c| {
        c.borrow().get(widget_id).and_then(|m| m.get(event).cloned())
    });
    if let Some(name) = func_name {
        let run = crate::aly::get_runtime();
        let fake_lexer = vec![crate::lexer::Lexer::new(
            crate::tokens::Tokens::Identifier,
            name.clone(),
            0,
        )];
        let _ = run.function_run(fake_lexer);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Native Function Wrappers (fn(String) -> Box<dyn Validator>)
// ─────────────────────────────────────────────────────────────────────────────

pub fn gtk_new_window_str(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let title = arg(&args, 0);
    let width = arg(&args, 1).parse().unwrap_or(400);
    let height = arg(&args, 2).parse().unwrap_or(500);
    Box::new(gtk_new_window(vec![
        ValueData::String(title),
        ValueData::Int(width),
        ValueData::Int(height),
    ]))
}

pub fn gtk_new_button_str(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 1);
    let label = arg(&args, 0);
    Box::new(gtk_new_button(vec![ValueData::String(label)]))
}

pub fn gtk_new_label_str(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 1);
    let text = arg(&args, 0);
    Box::new(gtk_new_label(vec![ValueData::String(text)]))
}

pub fn gtk_new_div_str(_x: String) -> Box<dyn Validator> {
    Box::new(gtk_new_div(vec![]))
}

pub fn gtk_new_input_str(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 1);
    let placeholder = arg(&args, 0);
    Box::new(gtk_new_input(vec![ValueData::String(placeholder)]))
}

pub fn gtk_new_textarea_str(_x: String) -> Box<dyn Validator> {
    Box::new(gtk_new_textarea(vec![]))
}

pub fn gtk_new_checkbox_str(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 1);
    let label = arg(&args, 0);
    Box::new(gtk_new_checkbox(vec![ValueData::String(label)]))
}

pub fn gtk_new_radio_str(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 1);
    let label = arg(&args, 0);
    Box::new(gtk_new_radio(vec![ValueData::String(label)]))
}

pub fn gtk_new_slider_str(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let min = arg(&args, 0).parse().unwrap_or(0);
    let max = arg(&args, 1).parse().unwrap_or(100);
    let val = arg(&args, 2).parse().unwrap_or(50);
    Box::new(gtk_new_slider(vec![
        ValueData::Int(min),
        ValueData::Int(max),
        ValueData::Int(val),
    ]))
}

pub fn gtk_new_progressbar_str(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 1);
    let val = arg(&args, 0).parse().unwrap_or(0.0);
    Box::new(ValueData::Float(gtk_new_progressbar(vec![ValueData::Float(val as f32)]).clone()))
}

pub fn gtk_new_dropdown_str(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 0);
    let values: Vec<ValueData> = args.iter().map(|a| ValueData::String(a.clone())).collect();
    Box::new(gtk_new_dropdown(values))
}

pub fn gtk_new_tabs_str(_x: String) -> Box<dyn Validator> {
    Box::new(gtk_new_tabs(vec![]))
}

pub fn gtk_new_separator_str(_x: String) -> Box<dyn Validator> {
    Box::new(gtk_new_separator(vec![]))
}
