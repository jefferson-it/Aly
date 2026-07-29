use std::collections::HashMap;
use std::cell::RefCell;

use crate::gui::{
    self as gui_crate,
    backend::{GuiBackend, BackendType},
    widget::{WidgetId, next_widget_id, PROP_LABEL, PROP_TEXT, PROP_VALUE, PROP_CHECKED, PROP_ENABLED, PROP_VISIBLE},
};

use axum::{
    routing::{get, post},
    Json, Router, response::Html,
};

#[derive(serde::Deserialize)]
struct EventPayload {
    id: String,
    event: String,
    inputs: HashMap<String, String>,
}

#[derive(Clone)]
pub struct DomNode {
    pub id: WidgetId,
    pub tag: String,
    pub attrs: HashMap<String, String>,
    pub children: Vec<WidgetId>,
    pub text: String,
    pub parent: Option<WidgetId>,
    pub events: HashMap<String, String>,
    pub widget_type: String,
}

impl DomNode {
    fn new(id: WidgetId, tag: &str, widget_type: &str) -> Self {
        Self {
            id,
            tag: tag.to_owned(),
            attrs: HashMap::new(),
            children: vec![],
            text: String::new(),
            parent: None,
            events: HashMap::new(),
            widget_type: widget_type.to_owned(),
        }
    }

    fn set_attr(&mut self, key: &str, value: &str) {
        self.attrs.insert(key.to_owned(), value.to_owned());
    }

    fn render_html(&self, nodes: &HashMap<WidgetId, DomNode>, indent: usize) -> String {
        let pad = "  ".repeat(indent);
        let mut html = String::new();

        match self.widget_type.as_str() {
            "Window" => {
                html.push_str(&format!("{}<div id=\"{}\" class=\"aly-window\" style=\"{}\">\n",
                    pad, self.id, self.attrs.get("style").map(|s| s.as_str()).unwrap_or("")));
                for child_id in &self.children {
                    if let Some(child) = nodes.get(child_id) {
                        html.push_str(&child.render_html(nodes, indent + 1));
                    }
                }
                html.push_str(&format!("{}</div>\n", pad));
            }
            "Button" => {
                let style = self.attrs.get("style").map(|s| s.as_str()).unwrap_or("");
                let onclick = self.events.get("onClick").map(|_| {
                    format!(" onclick=\"aly.fire('{}','onClick')\"", self.id)
                }).unwrap_or_default();
                html.push_str(&format!("{}<button id=\"{}\" class=\"aly-btn\" style=\"{}\"{}>{}</button>\n",
                    pad, self.id, style, onclick, escape_html(&self.text)));
            }
            "Label" => {
                let style = self.attrs.get("style").map(|s| s.as_str()).unwrap_or("");
                html.push_str(&format!("{}<span id=\"{}\" class=\"aly-label\" style=\"{}\">{}</span>\n",
                    pad, self.id, style, escape_html(&self.text)));
            }
            "Div" => {
                let style = self.attrs.get("style").map(|s| s.as_str()).unwrap_or("");
                html.push_str(&format!("{}<div id=\"{}\" class=\"aly-div\" style=\"{}\">\n",
                    pad, self.id, style));
                for child_id in &self.children {
                    if let Some(child) = nodes.get(child_id) {
                        html.push_str(&child.render_html(nodes, indent + 1));
                    }
                }
                html.push_str(&format!("{}</div>\n", pad));
            }
            "Input" => {
                let style = self.attrs.get("style").map(|s| s.as_str()).unwrap_or("");
                let value = escape_html(&self.text);
                let onchange = self.events.get("onChange").map(|_| {
                    format!(" onchange=\"aly.fire('{}','onChange')\"", self.id)
                }).unwrap_or_default();
                html.push_str(&format!("{}<input id=\"{}\" type=\"text\" class=\"aly-input\" style=\"{}\" value=\"{}\"{}>\n",
                    pad, self.id, style, value, onchange));
            }
            "TextArea" => {
                let style = self.attrs.get("style").map(|s| s.as_str()).unwrap_or("");
                let value = escape_html(&self.text);
                let onchange = self.events.get("onChange").map(|_| {
                    format!(" onchange=\"aly.fire('{}','onChange')\"", self.id)
                }).unwrap_or_default();
                html.push_str(&format!("{}<textarea id=\"{}\" class=\"aly-textarea\" style=\"{}\"{}>{}</textarea>\n",
                    pad, self.id, style, onchange, value));
            }
            "Password" => {
                let style = self.attrs.get("style").map(|s| s.as_str()).unwrap_or("");
                html.push_str(&format!("{}<input id=\"{}\" type=\"password\" class=\"aly-input\" style=\"{}\">\n",
                    pad, self.id, style));
            }
            "Checkbox" => {
                let checked = if self.attrs.contains_key("checked") { " checked" } else { "" };
                let label = escape_html(&self.text);
                let onchange = self.events.get("onChange").map(|_| {
                    format!(" onchange=\"aly.fire('{}','onChange')\"", self.id)
                }).unwrap_or_default();
                html.push_str(&format!("{}<label id=\"{}\" class=\"aly-checkbox\"{}><input type=\"checkbox\"{}> {}</label>\n",
                    pad, self.id, onchange, checked, label));
            }
            "Radio" => {
                let label = escape_html(&self.text);
                html.push_str(&format!("{}<label class=\"aly-radio\"><input id=\"{}\" type=\"radio\" name=\"aly-radio\"> {}</label>\n",
                    pad, self.id, label));
            }
            "Slider" => {
                let style = self.attrs.get("style").map(|s| s.as_str()).unwrap_or("");
                let min = self.attrs.get("min").map(|s| s.as_str()).unwrap_or("0");
                let max = self.attrs.get("max").map(|s| s.as_str()).unwrap_or("100");
                let val = self.attrs.get("value").map(|s| s.as_str()).unwrap_or("50");
                let onchange = self.events.get("onChange").map(|_| {
                    format!(" onchange=\"aly.fire('{}','onChange')\"", self.id)
                }).unwrap_or_default();
                html.push_str(&format!("{}<input id=\"{}\" type=\"range\" class=\"aly-slider\" style=\"{}\" min=\"{}\" max=\"{}\" value=\"{}\"{}>\n",
                    pad, self.id, style, min, max, val, onchange));
            }
            "ProgressBar" => {
                let style = self.attrs.get("style").map(|s| s.as_str()).unwrap_or("");
                let val = self.attrs.get("value").map(|s| s.as_str()).unwrap_or("0");
                html.push_str(&format!("{}<progress id=\"{}\" class=\"aly-progress\" style=\"{}\" value=\"{}\" max=\"100\"></progress>\n",
                    pad, self.id, style, val));
            }
            "Dropdown" => {
                let style = self.attrs.get("style").map(|s| s.as_str()).unwrap_or("");
                let items_attr = self.attrs.get("items").map(|s| s.as_str()).unwrap_or("");
                let onchange = self.events.get("onChange").map(|_| {
                    format!(" onchange=\"aly.fire('{}','onChange')\"", self.id)
                }).unwrap_or_default();
                html.push_str(&format!("{}<select id=\"{}\" class=\"aly-dropdown\" style=\"{}\"{}>\n", pad, self.id, style, onchange));
                for item in items_attr.split(',') {
                    let trimmed = item.trim();
                    if !trimmed.is_empty() {
                        html.push_str(&format!("{}  <option>{}</option>\n", pad, escape_html(trimmed)));
                    }
                }
                html.push_str(&format!("{}</select>\n", pad));
            }
            "Separator" => {
                html.push_str(&format!("{}<hr id=\"{}\" class=\"aly-separator\">\n", pad, self.id));
            }
            "Container" => {
                let style = self.attrs.get("style").map(|s| s.as_str()).unwrap_or("");
                html.push_str(&format!("{}<div id=\"{}\" class=\"aly-container\" style=\"{}\">\n",
                    pad, self.id, style));
                for child_id in &self.children {
                    if let Some(child) = nodes.get(child_id) {
                        html.push_str(&child.render_html(nodes, indent + 1));
                    }
                }
                html.push_str(&format!("{}</div>\n", pad));
            }
            _ => {
                html.push_str(&format!("{}<!-- unknown widget: {} -->\n", pad, self.widget_type));
            }
        }

        html
    }
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

thread_local! {
    static DOM_NODES: RefCell<HashMap<WidgetId, DomNode>> = RefCell::new(HashMap::new());
    static DOM_ROOT: RefCell<Option<WidgetId>> = RefCell::new(None);
}

fn find_node(id: &str) -> Option<DomNode> {
    DOM_NODES.with(|n| n.borrow().get(id).cloned())
}

fn update_node<F>(id: &str, f: F)
where
    F: FnOnce(&mut DomNode),
{
    DOM_NODES.with(|n| {
        if let Some(node) = n.borrow_mut().get_mut(id) {
            f(node);
        }
    });
}

async fn handle_index() -> Html<String> {
    Html(DomBackend::generate_html())
}

async fn handle_event(Json(payload): Json<EventPayload>) -> Html<String> {
    // 1. Sync input values
    for (input_id, val) in payload.inputs {
        update_node(&input_id, |node| {
            match node.widget_type.as_str() {
                "Checkbox" | "Radio" => {
                    if val == "true" {
                        node.set_attr("checked", "true");
                    } else {
                        node.attrs.remove("checked");
                    }
                }
                "Slider" | "ProgressBar" | "Spinner" => {
                    node.set_attr("value", &val);
                }
                _ => {
                    node.text = val;
                }
            }
        });
    }

    // 2. Fire the callback
    gui_crate::event::fire_callback(&payload.id, &payload.event);

    // 3. Return the updated HTML
    Html(DomBackend::generate_html())
}

pub struct DomBackend;

impl DomBackend {
    pub fn generate_html() -> String {
        let mut html = String::from(
            "<!DOCTYPE html>\n<html>\n<head>\n"
        );
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str("<link href=\"https://fonts.googleapis.com/css2?family=Outfit:wght@300;400;600;700&display=swap\" rel=\"stylesheet\">\n");
        html.push_str("<style>\n");
        html.push_str("body {\n");
        html.push_str("  margin: 0;\n");
        html.push_str("  padding: 0;\n");
        html.push_str("  background-color: #0f0f13;\n");
        html.push_str("  color: #f1f1f7;\n");
        html.push_str("  font-family: 'Outfit', -apple-system, sans-serif;\n");
        html.push_str("  display: flex;\n");
        html.push_str("  justify-content: center;\n");
        html.push_str("  align-items: center;\n");
        html.push_str("  min-height: 100vh;\n");
        html.push_str("}\n");
        html.push_str(".aly-window {\n");
        html.push_str("  width: 100%;\n");
        html.push_str("  max-width: 480px;\n");
        html.push_str("  background: rgba(25, 25, 35, 0.65);\n");
        html.push_str("  backdrop-filter: blur(16px);\n");
        html.push_str("  -webkit-backdrop-filter: blur(16px);\n");
        html.push_str("  border: 1px solid rgba(255, 255, 255, 0.08);\n");
        html.push_str("  border-radius: 20px;\n");
        html.push_str("  padding: 32px;\n");
        html.push_str("  box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.37);\n");
        html.push_str("  display: flex;\n");
        html.push_str("  flex-direction: column;\n");
        html.push_str("  gap: 20px;\n");
        html.push_str("  transition: all 0.3s ease;\n");
        html.push_str("}\n");
        html.push_str(".aly-btn {\n");
        html.push_str("  background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);\n");
        html.push_str("  color: white;\n");
        html.push_str("  border: none;\n");
        html.push_str("  padding: 12px 24px;\n");
        html.push_str("  border-radius: 12px;\n");
        html.push_str("  font-size: 16px;\n");
        html.push_str("  font-weight: 600;\n");
        html.push_str("  cursor: pointer;\n");
        html.push_str("  box-shadow: 0 4px 12px rgba(79, 70, 229, 0.3);\n");
        html.push_str("  transition: all 0.2s ease;\n");
        html.push_str("}\n");
        html.push_str(".aly-btn:hover {\n");
        html.push_str("  transform: translateY(-2px);\n");
        html.push_str("  box-shadow: 0 6px 20px rgba(79, 70, 229, 0.4);\n");
        html.push_str("}\n");
        html.push_str(".aly-btn:active {\n");
        html.push_str("  transform: translateY(1px);\n");
        html.push_str("}\n");
        html.push_str(".aly-label {\n");
        html.push_str("  font-size: 15px;\n");
        html.push_str("  color: #a1a1aa;\n");
        html.push_str("  line-height: 1.5;\n");
        html.push_str("}\n");
        html.push_str(".aly-div {\n");
        html.push_str("  display: flex;\n");
        html.push_str("  flex-direction: column;\n");
        html.push_str("  gap: 16px;\n");
        html.push_str("}\n");
        html.push_str(".aly-input, .aly-textarea, .aly-dropdown {\n");
        html.push_str("  background: rgba(25, 25, 30, 0.5);\n");
        html.push_str("  border: 1px solid rgba(255, 255, 255, 0.1);\n");
        html.push_str("  color: white;\n");
        html.push_str("  padding: 12px 16px;\n");
        html.push_str("  border-radius: 12px;\n");
        html.push_str("  font-size: 15px;\n");
        html.push_str("  outline: none;\n");
        html.push_str("  transition: all 0.2s ease;\n");
        html.push_str("}\n");
        html.push_str(".aly-input:focus, .aly-textarea:focus, .aly-dropdown:focus {\n");
        html.push_str("  border-color: #6366f1;\n");
        html.push_str("  background: rgba(255, 255, 255, 0.08);\n");
        html.push_str("  box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.15);\n");
        html.push_str("}\n");
        html.push_str(".aly-checkbox, .aly-radio {\n");
        html.push_str("  display: flex;\n");
        html.push_str("  align-items: center;\n");
        html.push_str("  gap: 10px;\n");
        html.push_str("  cursor: pointer;\n");
        html.push_str("  font-size: 15px;\n");
        html.push_str("  color: #d1d5db;\n");
        html.push_str("}\n");
        html.push_str(".aly-slider {\n");
        html.push_str("  -webkit-appearance: none;\n");
        html.push_str("  width: 100%;\n");
        html.push_str("  height: 6px;\n");
        html.push_str("  border-radius: 5px;\n");
        html.push_str("  background: rgba(255, 255, 255, 0.1);\n");
        html.push_str("  outline: none;\n");
        html.push_str("}\n");
        html.push_str(".aly-slider::-webkit-slider-thumb {\n");
        html.push_str("  -webkit-appearance: none;\n");
        html.push_str("  appearance: none;\n");
        html.push_str("  width: 18px;\n");
        html.push_str("  height: 18px;\n");
        html.push_str("  border-radius: 50%;\n");
        html.push_str("  background: #6366f1;\n");
        html.push_str("  cursor: pointer;\n");
        html.push_str("  transition: transform 0.1s ease;\n");
        html.push_str("}\n");
        html.push_str(".aly-slider::-webkit-slider-thumb:hover {\n");
        html.push_str("  transform: scale(1.2);\n");
        html.push_str("}\n");
        html.push_str(".aly-progress {\n");
        html.push_str("  -webkit-appearance: none;\n");
        html.push_str("  appearance: none;\n");
        html.push_str("  width: 100%;\n");
        html.push_str("  height: 8px;\n");
        html.push_str("}\n");
        html.push_str(".aly-progress::-webkit-progress-bar {\n");
        html.push_str("  background-color: rgba(255, 255, 255, 0.1);\n");
        html.push_str("  border-radius: 10px;\n");
        html.push_str("}\n");
        html.push_str(".aly-progress::-webkit-progress-value {\n");
        html.push_str("  background: linear-gradient(90deg, #6366f1, #a855f7);\n");
        html.push_str("  border-radius: 10px;\n");
        html.push_str("}\n");
        html.push_str(".aly-separator {\n");
        html.push_str("  border: none;\n");
        html.push_str("  border-top: 1px solid rgba(255, 255, 255, 0.08);\n");
        html.push_str("  margin: 16px 0;\n");
        html.push_str("}\n");
        html.push_str(".aly-container {\n");
        html.push_str("  display: flex;\n");
        html.push_str("  flex-direction: column;\n");
        html.push_str("  gap: 16px;\n");
        html.push_str("}\n");
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str("<script>\n");
        html.push_str("window.aly = window.aly || {};\n");
        html.push_str("aly.fire = async function(id, event) {\n");
        html.push_str("  const inputs = {};\n");
        html.push_str("  document.querySelectorAll('input, textarea, select').forEach(el => {\n");
        html.push_str("    if (el.type === 'checkbox' || el.type === 'radio') {\n");
        html.push_str("      inputs[el.id] = el.checked ? 'true' : 'false';\n");
        html.push_str("    } else {\n");
        html.push_str("      inputs[el.id] = el.value;\n");
        html.push_str("    }\n");
        html.push_str("  });\n");
        html.push_str("  const response = await fetch('/api/event', {\n");
        html.push_str("    method: 'POST',\n");
        html.push_str("    headers: { 'Content-Type': 'application/json' },\n");
        html.push_str("    body: JSON.stringify({ id, event, inputs })\n");
        html.push_str("  });\n");
        html.push_str("  if (response.ok) {\n");
        html.push_str("    const newHtml = await response.text();\n");
        html.push_str("    const parser = new DOMParser();\n");
        html.push_str("    const doc = parser.parseFromString(newHtml, 'text/html');\n");
        html.push_str("    document.body.innerHTML = doc.body.innerHTML;\n");
        html.push_str("  }\n");
        html.push_str("};\n");
        html.push_str("</script>\n");

        let root_id = DOM_ROOT.with(|r| r.borrow().clone());
        DOM_NODES.with(|n| {
            let nodes = n.borrow();
            if let Some(ref root_id) = root_id {
                if let Some(root) = nodes.get(root_id) {
                    html.push_str(&root.render_html(&nodes, 1));
                }
            }
        });

        html.push_str("</body>\n</html>\n");
        html
    }

    pub fn generate_html_fragment() -> String {
        let mut html = String::new();
        let root_id = DOM_ROOT.with(|r| r.borrow().clone());
        DOM_NODES.with(|n| {
            let nodes = n.borrow();
            if let Some(ref root_id) = root_id {
                if let Some(root) = nodes.get(root_id) {
                    html.push_str(&root.render_html(&nodes, 0));
                }
            }
        });
        html
    }
}

impl GuiBackend for DomBackend {
    fn backend_type(&self) -> BackendType {
        BackendType::Dom
    }

    fn create_window(&mut self, title: &str, _width: i32, _height: i32) -> WidgetId {
        let id = next_widget_id();
        let mut node = DomNode::new(id.clone(), "div", "Window");
        node.set_attr("data-title", title);
        let id_clone = id.clone();
        DOM_ROOT.with(|r| *r.borrow_mut() = Some(id_clone));
        DOM_NODES.with(|n| { n.borrow_mut().insert(id.clone(), node); });
        id
    }

    fn create_button(&mut self, label: &str) -> WidgetId {
        let id = next_widget_id();
        let mut node = DomNode::new(id.clone(), "button", "Button");
        node.text = label.to_owned();
        node.set_attr("class", "aly-btn");
        DOM_NODES.with(|n| { n.borrow_mut().insert(id.clone(), node); });
        id
    }

    fn create_label(&mut self, text: &str) -> WidgetId {
        let id = next_widget_id();
        let mut node = DomNode::new(id.clone(), "span", "Label");
        node.text = text.to_owned();
        DOM_NODES.with(|n| { n.borrow_mut().insert(id.clone(), node); });
        id
    }

    fn create_div(&mut self, direction: &str) -> WidgetId {
        let id = next_widget_id();
        let mut node = DomNode::new(id.clone(), "div", "Div");
        if direction == "horizontal" {
            node.set_attr("style", "flex-direction: row; align-items: center;");
        }
        DOM_NODES.with(|n| { n.borrow_mut().insert(id.clone(), node); });
        id
    }

    fn create_input(&mut self, placeholder: &str) -> WidgetId {
        let id = next_widget_id();
        let mut node = DomNode::new(id.clone(), "input", "Input");
        node.set_attr("placeholder", placeholder);
        DOM_NODES.with(|n| { n.borrow_mut().insert(id.clone(), node); });
        id
    }

    fn create_textarea(&mut self) -> WidgetId {
        let id = next_widget_id();
        let node = DomNode::new(id.clone(), "textarea", "TextArea");
        DOM_NODES.with(|n| { n.borrow_mut().insert(id.clone(), node); });
        id
    }

    fn create_password(&mut self) -> WidgetId {
        let id = next_widget_id();
        let mut node = DomNode::new(id.clone(), "input", "Password");
        node.set_attr("type", "password");
        DOM_NODES.with(|n| { n.borrow_mut().insert(id.clone(), node); });
        id
    }

    fn create_checkbox(&mut self, label: &str) -> WidgetId {
        let id = next_widget_id();
        let mut node = DomNode::new(id.clone(), "label", "Checkbox");
        node.text = label.to_owned();
        DOM_NODES.with(|n| { n.borrow_mut().insert(id.clone(), node); });
        id
    }

    fn create_radio(&mut self, label: &str) -> WidgetId {
        let id = next_widget_id();
        let mut node = DomNode::new(id.clone(), "label", "Radio");
        node.text = label.to_owned();
        DOM_NODES.with(|n| { n.borrow_mut().insert(id.clone(), node); });
        id
    }

    fn create_slider(&mut self, min: f64, max: f64, val: f64) -> WidgetId {
        let id = next_widget_id();
        let mut node = DomNode::new(id.clone(), "input", "Slider");
        node.set_attr("type", "range");
        node.set_attr("min", &min.to_string());
        node.set_attr("max", &max.to_string());
        node.set_attr("value", &val.to_string());
        DOM_NODES.with(|n| { n.borrow_mut().insert(id.clone(), node); });
        id
    }

    fn create_progressbar(&mut self, val: f64) -> WidgetId {
        let id = next_widget_id();
        let mut node = DomNode::new(id.clone(), "progress", "ProgressBar");
        node.set_attr("value", &(val * 100.0).to_string());
        node.set_attr("max", "100");
        DOM_NODES.with(|n| { n.borrow_mut().insert(id.clone(), node); });
        id
    }

    fn create_dropdown(&mut self, items: &[String]) -> WidgetId {
        let id = next_widget_id();
        let mut node = DomNode::new(id.clone(), "select", "Dropdown");
        let items_str = items.join(",");
        node.set_attr("items", &items_str);
        DOM_NODES.with(|n| { n.borrow_mut().insert(id.clone(), node); });
        id
    }

    fn create_separator(&mut self) -> WidgetId {
        let id = next_widget_id();
        let node = DomNode::new(id.clone(), "hr", "Separator");
        DOM_NODES.with(|n| { n.borrow_mut().insert(id.clone(), node); });
        id
    }

    fn create_spinner(&mut self, min: f64, max: f64, step: f64, val: f64) -> WidgetId {
        let id = next_widget_id();
        let mut node = DomNode::new(id.clone(), "input", "Input");
        node.set_attr("type", "number");
        node.set_attr("min", &min.to_string());
        node.set_attr("max", &max.to_string());
        node.set_attr("step", &step.to_string());
        node.set_attr("value", &val.to_string());
        DOM_NODES.with(|n| { n.borrow_mut().insert(id.clone(), node); });
        id
    }

    fn create_container(&mut self, width: i32, height: i32, direction: &str) -> WidgetId {
        let id = next_widget_id();
        let mut node = DomNode::new(id.clone(), "div", "Container");
        let mut style = String::new();
        if width > 0 {
            style.push_str(&format!("width: {}px; ", width));
        }
        if height > 0 {
            style.push_str(&format!("height: {}px; ", height));
        }
        if direction == "horizontal" {
            style.push_str("flex-direction: row; align-items: center; ");
        }
        if !style.is_empty() {
            node.set_attr("style", &style);
        }
        DOM_NODES.with(|n| { n.borrow_mut().insert(id.clone(), node); });
        id
    }

    fn insert_widget(&mut self, parent: &WidgetId, child: &WidgetId) {
        let child_parent = child.clone();
        update_node(parent, |node| {
            node.children.push(child.clone());
        });
        update_node(child, |node| {
            node.parent = Some(child_parent);
        });
    }

    fn set_property(&mut self, id: &WidgetId, prop: &str, value: &str) {
        match prop {
            PROP_LABEL | PROP_TEXT | PROP_VALUE => {
                update_node(id, |node| {
                    node.text = value.to_owned();
                });
            }
            PROP_CHECKED => {
                update_node(id, |node| {
                    if value == "true" || value == "1" {
                        node.set_attr("checked", "true");
                    } else {
                        node.attrs.remove("checked");
                    }
                });
            }
            PROP_ENABLED => {
                update_node(id, |node| {
                    let disabled = value == "false";
                    if disabled {
                        node.set_attr("disabled", "true");
                    } else {
                        node.attrs.remove("disabled");
                    }
                });
            }
            PROP_VISIBLE => {
                update_node(id, |node| {
                    let style = node.attrs.entry("style".to_owned()).or_default();
                    if value == "false" {
                        *style = format!("{}; display:none", style);
                    } else {
                        if let Some(pos) = style.find("display:none") {
                            let before: String = style.chars().take(pos).collect();
                            let after: String = style.chars().skip(pos + 12).collect();
                            *style = format!("{}{}", before, after);
                        }
                    }
                });
            }
            "class" => {
                update_node(id, |node| {
                    node.set_attr("class", value);
                });
            }
            "style" => {
                update_node(id, |node| {
                    let existing = node.attrs.get("style").map(|s| s.as_str()).unwrap_or("");
                    let new_style = if existing.is_empty() {
                        value.to_owned()
                    } else {
                        format!("{}; {}", existing, value)
                    };
                    node.set_attr("style", &new_style);
                });
            }
            _ => {
                update_node(id, |node| {
                    node.set_attr(prop, value);
                });
            }
        }
    }

    fn get_property(&mut self, id: &WidgetId, prop: &str) -> String {
        match find_node(id) {
            Some(node) => match prop {
                PROP_LABEL | PROP_TEXT | PROP_VALUE => node.text.clone(),
                PROP_CHECKED => {
                    if node.attrs.contains_key("checked") { "true" } else { "false" }.to_owned()
                }
                "id" => node.id.clone(),
                _ => node.attrs.get(prop).cloned().unwrap_or_else(|| "None".to_owned()),
            },
            None => "None".to_owned(),
        }
    }

    fn on_event(&mut self, id: &WidgetId, event: &str, callback: String) {
        gui_crate::event::set_callback(id, event, callback);
        update_node(id, |node| {
            node.events.insert(event.to_owned(), String::new());
        });
    }

    fn run(&mut self) {
        println!("============================================================");
        println!("Iniciando servidor HTTP do Aly GUI em http://localhost:3000");
        println!("Abra esse endereço no seu navegador para ver a interface.");
        println!("============================================================");

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async {
            let app = Router::new()
                .route("/", get(handle_index))
                .route("/api/event", post(handle_event));

            let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
            axum::serve(listener, app).await.unwrap();
        });
    }
}
