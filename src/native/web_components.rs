// Aly Web Components Module - Custom Elements, Shadow DOM, Templates

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::native::types::{Validator, ValueData};
use crate::native::std::{split_args, arg};
use crate::validators::str::put_quoted_str;
use crate::aly::get_runtime;

use serde::{Serialize, Deserialize};

fn ok_str(s: String) -> Box<dyn Validator> {
    Box::new(put_quoted_str(s))
}

fn ok_bool(b: bool) -> Box<dyn Validator> {
    Box::new(ValueData::Bool(b))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebComponent {
    pub tag_name: String,
    pub template: String,
    pub styles: String,
    pub script: String,
    pub observed_attributes: Vec<String>,
    pub properties: HashMap<String, ComponentProperty>,
    pub methods: HashMap<String, ComponentMethod>,
    pub events: Vec<String>,
    pub lifecycle: ComponentLifecycle,
    pub shadow_mode: ShadowMode,
    pub slots: Vec<SlotDefinition>,
    pub defined: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentProperty {
    pub name: String,
    pub r#type: PropertyType,
    pub default: Option<String>,
    pub reflect: bool,
    pub attribute: Option<String>,
    pub readonly: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyType {
    String,
    Number,
    Boolean,
    Object,
    Array,
    Function,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentMethod {
    pub name: String,
    pub params: Vec<String>,
    pub body: String,
    pub is_static: bool,
    pub is_async: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentLifecycle {
    pub constructor: Option<String>,
    pub connected: Option<String>,
    pub disconnected: Option<String>,
    pub attribute_changed: Option<String>,
    pub adopted: Option<String>,
    pub form_associated: Option<String>,
    pub form_disabled: Option<String>,
    pub form_reset: Option<String>,
    pub form_state_restore: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShadowMode {
    Open,
    Closed,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotDefinition {
    pub name: String,
    pub fallback: String,
    pub required: bool,
}

static COMPONENT_REGISTRY: OnceLock<Arc<Mutex<ComponentRegistry>>> = OnceLock::new();

struct ComponentRegistry {
    components: HashMap<String, WebComponent>,
    templates: HashMap<String, String>,
    defined_tags: HashMap<String, bool>,
    bundle_cache: String,
    hydration_scripts: Vec<String>,
}

impl ComponentRegistry {
    fn new() -> Self {
        Self {
            components: HashMap::new(),
            templates: HashMap::new(),
            defined_tags: HashMap::new(),
            bundle_cache: String::new(),
            hydration_scripts: Vec::new(),
        }
    }
}

fn registry() -> &'static Arc<Mutex<ComponentRegistry>> {
    COMPONENT_REGISTRY.get_or_init(|| Arc::new(Mutex::new(ComponentRegistry::new())))
}

pub fn wc_define(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 5);
    let tag_name = arg(&args_list, 0);
    let template = arg(&args_list, 1);
    let styles = arg(&args_list, 2);
    let script = arg(&args_list, 3);
    let observed_attrs_json = arg(&args_list, 4);

    if !tag_name.contains('-') {
        return ok_str(format!("Custom element name '{}' must contain a hyphen", tag_name));
    }

    let observed_attributes: Vec<String> = if observed_attrs_json.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&observed_attrs_json).unwrap_or_default()
    };

    let mut reg = registry().lock().unwrap();
    
    if reg.defined_tags.contains_key(&tag_name) {
        return ok_str(format!("Component '{}' already defined", tag_name));
    }

    let component = WebComponent {
        tag_name: tag_name.clone(),
        template,
        styles,
        script,
        observed_attributes,
        properties: HashMap::new(),
        methods: HashMap::new(),
        events: Vec::new(),
        lifecycle: ComponentLifecycle {
            constructor: None,
            connected: None,
            disconnected: None,
            attribute_changed: None,
            adopted: None,
            form_associated: None,
            form_disabled: None,
            form_reset: None,
            form_state_restore: None,
        },
        shadow_mode: ShadowMode::Open,
        slots: Vec::new(),
        defined: true,
    };

    reg.components.insert(tag_name.clone(), component);
    reg.defined_tags.insert(tag_name.clone(), true);
    
    ok_str("None".to_string())
}

pub fn wc_add_property(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 7);
    let tag_name = arg(&args_list, 0);
    let prop_name = arg(&args_list, 1);
    let prop_type = arg(&args_list, 2);
    let default = arg(&args_list, 3);
    let reflect = arg(&args_list, 4);
    let attribute = arg(&args_list, 5);
    let readonly = arg(&args_list, 6);

    let mut reg = registry().lock().unwrap();
    
    if let Some(component) = reg.components.get_mut(&tag_name) {
        let ptype = match prop_type.as_str() {
            "string" => PropertyType::String,
            "number" => PropertyType::Number,
            "boolean" => PropertyType::Boolean,
            "object" => PropertyType::Object,
            "array" => PropertyType::Array,
            "function" => PropertyType::Function,
            _ => PropertyType::String,
        };

        let prop = ComponentProperty {
            name: prop_name.clone(),
            r#type: ptype,
            default: if default.is_empty() { None } else { Some(default) },
            reflect: reflect == "true",
            attribute: if attribute.is_empty() { None } else { Some(attribute) },
            readonly: readonly == "true",
        };

        component.properties.insert(prop_name, prop);
        ok_str("None".to_string())
    } else {
        ok_str(format!("Component '{}' not found", tag_name))
    }
}

pub fn wc_add_method(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 5);
    let tag_name = arg(&args_list, 0);
    let method_name = arg(&args_list, 1);
    let params_json = arg(&args_list, 2);
    let body = arg(&args_list, 3);
    let is_static = arg(&args_list, 4);

    let mut reg = registry().lock().unwrap();
    
    if let Some(component) = reg.components.get_mut(&tag_name) {
        let params: Vec<String> = if params_json.is_empty() {
            Vec::new()
        } else {
            serde_json::from_str(&params_json).unwrap_or_default()
        };

        let method = ComponentMethod {
            name: method_name.clone(),
            params,
            body,
            is_static: is_static == "true",
            is_async: false,
        };

        component.methods.insert(method_name, method);
        ok_str("None".to_string())
    } else {
        ok_str(format!("Component '{}' not found", tag_name))
    }
}

pub fn wc_add_event(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 2);
    let tag_name = arg(&args_list, 0);
    let event_name = arg(&args_list, 1);

    let mut reg = registry().lock().unwrap();
    
    if let Some(component) = reg.components.get_mut(&tag_name) {
        if !component.events.contains(&event_name) {
            component.events.push(event_name);
        }
        ok_str("None".to_string())
    } else {
        ok_str(format!("Component '{}' not found", tag_name))
    }
}

pub fn wc_set_lifecycle(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 10);
    let tag_name = arg(&args_list, 0);
    let constructor = arg(&args_list, 1);
    let connected = arg(&args_list, 2);
    let disconnected = arg(&args_list, 3);
    let attribute_changed = arg(&args_list, 4);
    let adopted = arg(&args_list, 5);
    let form_associated = arg(&args_list, 6);
    let form_disabled = arg(&args_list, 7);
    let form_reset = arg(&args_list, 8);
    let form_state_restore = arg(&args_list, 9);

    let mut reg = registry().lock().unwrap();
    
    if let Some(component) = reg.components.get_mut(&tag_name) {
        component.lifecycle = ComponentLifecycle {
            constructor: if constructor.is_empty() { None } else { Some(constructor) },
            connected: if connected.is_empty() { None } else { Some(connected) },
            disconnected: if disconnected.is_empty() { None } else { Some(disconnected) },
            attribute_changed: if attribute_changed.is_empty() { None } else { Some(attribute_changed) },
            adopted: if adopted.is_empty() { None } else { Some(adopted) },
            form_associated: if form_associated.is_empty() { None } else { Some(form_associated) },
            form_disabled: if form_disabled.is_empty() { None } else { Some(form_disabled) },
            form_reset: if form_reset.is_empty() { None } else { Some(form_reset) },
            form_state_restore: if form_state_restore.is_empty() { None } else { Some(form_state_restore) },
        };
        ok_str("None".to_string())
    } else {
        ok_str(format!("Component '{}' not found", tag_name))
    }
}

pub fn wc_set_shadow_mode(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 2);
    let tag_name = arg(&args_list, 0);
    let mode = arg(&args_list, 1);

    let mut reg = registry().lock().unwrap();
    
    if let Some(component) = reg.components.get_mut(&tag_name) {
        component.shadow_mode = match mode.as_str() {
            "open" => ShadowMode::Open,
            "closed" => ShadowMode::Closed,
            "none" => ShadowMode::None,
            _ => ShadowMode::Open,
        };
        ok_str("None".to_string())
    } else {
        ok_str(format!("Component '{}' not found", tag_name))
    }
}

pub fn wc_add_slot(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 4);
    let tag_name = arg(&args_list, 0);
    let slot_name = arg(&args_list, 1);
    let fallback = arg(&args_list, 2);
    let required = arg(&args_list, 3);

    let mut reg = registry().lock().unwrap();
    
    if let Some(component) = reg.components.get_mut(&tag_name) {
        let slot = SlotDefinition {
            name: slot_name.clone(),
            fallback,
            required: required == "true",
        };
        component.slots.push(slot);
        ok_str("None".to_string())
    } else {
        ok_str(format!("Component '{}' not found", tag_name))
    }
}

pub fn wc_render(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 2);
    let tag_name = arg(&args_list, 0);
    let props_json = arg(&args_list, 1);

    let props: HashMap<String, serde_json::Value> = if props_json.is_empty() {
        HashMap::new()
    } else {
        serde_json::from_str(&props_json).unwrap_or_default()
    };

    let reg = registry().lock().unwrap();
    
    if let Some(component) = reg.components.get(&tag_name) {
        let mut html = String::new();
        
        match component.shadow_mode {
            ShadowMode::Open | ShadowMode::Closed => {
                html.push_str(&format!("<{}>", tag_name));
                html.push_str("#shadow-root");
                if component.shadow_mode == ShadowMode::Open {
                    html.push_str("(open)");
                }
                html.push_str("\n");
            }
            ShadowMode::None => {
                html.push_str(&format!("<{}", tag_name));
            }
        }

        for (key, value) in &props {
            if let Some(prop) = component.properties.get(key) {
                if prop.reflect {
                    let attr = prop.attribute.as_deref().unwrap_or(key);
                    html.push_str(&format!(" {}=\"{}\"", attr, value));
                }
            } else {
                html.push_str(&format!(" {}=\"{}\"", key, value));
            }
        }

        if component.shadow_mode != ShadowMode::None {
            html.push_str(">\n");
        } else {
            html.push_str(">\n");
        }

        if !component.styles.is_empty() {
            html.push_str("<style>\n");
            html.push_str(&component.styles);
            html.push_str("\n</style>\n");
        }

        let mut template = component.template.clone();
        for (key, value) in &props {
            let placeholder = format!("{{{{{}}}}}", key);
            let replacement = match value {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                _ => value.to_string(),
            };
            template = template.replace(&placeholder, &replacement);
        }

        for slot in &component.slots {
            if slot.name.is_empty() {
                template = template.replace("<slot></slot>", &slot.fallback);
            } else {
                let slot_pattern = format!("<slot name=\"{}\"></slot>", slot.name);
                template = template.replace(&slot_pattern, &slot.fallback);
            }
        }

        html.push_str(&template);

        if component.shadow_mode != ShadowMode::None {
            html.push_str("\n#shadow-root-end\n");
            html.push_str(&format!("</{}>\n", tag_name));
        } else {
            html.push_str(&format!("</{}>\n", tag_name));
        }

        ok_str(html)
    } else {
        ok_str(format!("Component '{}' not found", tag_name))
    }
}

pub fn wc_get_template(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 1);
    let tag_name = arg(&args_list, 0);

    let reg = registry().lock().unwrap();
    
    if let Some(component) = reg.components.get(&tag_name) {
        let mut result = serde_json::Map::new();
        result.insert("tagName".to_string(), serde_json::Value::String(component.tag_name.clone()));
        result.insert("template".to_string(), serde_json::Value::String(component.template.clone()));
        result.insert("styles".to_string(), serde_json::Value::String(component.styles.clone()));
        result.insert("script".to_string(), serde_json::Value::String(component.script.clone()));
        result.insert("observedAttributes".to_string(), serde_json::to_value(&component.observed_attributes).unwrap());
        result.insert("properties".to_string(), serde_json::to_value(&component.properties).unwrap());
        result.insert("methods".to_string(), serde_json::to_value(&component.methods).unwrap());
        result.insert("events".to_string(), serde_json::to_value(&component.events).unwrap());
        result.insert("slots".to_string(), serde_json::to_value(&component.slots).unwrap());
        result.insert("shadowMode".to_string(), serde_json::Value::String(format!("{:?}", component.shadow_mode).to_lowercase()));
        
        ok_str(serde_json::to_string(&result).unwrap_or_default())
    } else {
        ok_str("{}".to_string())
    }
}

pub fn wc_register(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 1);
    let tag_name = arg(&args_list, 0);

    let reg = registry().lock().unwrap();
    
    if let Some(component) = reg.components.get(&tag_name) {
        let js = generate_js_class(component);
        ok_str(js)
    } else {
        ok_str(format!("Component '{}' not found", tag_name))
    }
}

pub fn wc_list(args: String) -> Box<dyn Validator> {
    let _ = args;
    let reg = registry().lock().unwrap();
    let tags: Vec<String> = reg.components.keys().cloned().collect();
    ok_str(tags.join(", "))
}

pub fn wc_remove(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 1);
    let tag_name = arg(&args_list, 0);

    let mut reg = registry().lock().unwrap();
    reg.components.remove(&tag_name);
    reg.defined_tags.remove(&tag_name);
    ok_str("None".to_string())
}

pub fn wc_create_element(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 2);
    let tag_name = arg(&args_list, 0);
    let props_json = arg(&args_list, 1);

    wc_render(args)
}

pub fn wc_attach_shadow(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 2);
    let tag_name = arg(&args_list, 0);
    let mode = arg(&args_list, 1);

    let mut reg = registry().lock().unwrap();
    
    if let Some(component) = reg.components.get_mut(&tag_name) {
        component.shadow_mode = match mode.as_str() {
            "open" => ShadowMode::Open,
            "closed" => ShadowMode::Closed,
            _ => ShadowMode::Open,
        };
        ok_str("None".to_string())
    } else {
        ok_str(format!("Component '{}' not found", tag_name))
    }
}

pub fn wc_slot(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 3);
    let tag_name = arg(&args_list, 0);
    let slot_name = arg(&args_list, 1);
    let content = arg(&args_list, 2);

    let mut reg = registry().lock().unwrap();
    
    if let Some(component) = reg.components.get_mut(&tag_name) {
        if let Some(slot) = component.slots.iter_mut().find(|s| s.name == slot_name) {
            slot.fallback = content;
        } else {
            component.slots.push(SlotDefinition {
                name: slot_name,
                fallback: content,
                required: false,
            });
        }
        ok_str("None".to_string())
    } else {
        ok_str(format!("Component '{}' not found", tag_name))
    }
}

pub fn wc_hydrate(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 1);
    let html = arg(&args_list, 0);

    let hydrate_script = r#"
(function() {
  'use strict';
  
  const components = document.querySelectorAll('[data-aly-component]');
  
  components.forEach(el => {
    const tagName = el.getAttribute('data-aly-component');
    const props = JSON.parse(el.getAttribute('data-aly-props') || '{}');
    
    if (window.AlyComponents && window.AlyComponents[tagName]) {
      const Component = window.AlyComponents[tagName];
      const instance = new Component();
      
      Object.keys(props).forEach(key => {
        instance[key] = props[key];
      });
      
      if (el.shadowRoot) {
        el.shadowRoot.innerHTML = instance.render();
      } else {
        el.innerHTML = instance.render();
      }
    }
  });
})();
"#;

    let mut reg = registry().lock().unwrap();
    reg.hydration_scripts.push(hydrate_script.to_string());
    
    ok_str(hydrate_script.to_string())
}

pub fn wc_bundle(args: String) -> Box<dyn Validator> {
    let _ = args;
    let reg = registry().lock().unwrap();
    
    let mut bundle = String::new();
    bundle.push_str("(function() {\n");
    bundle.push_str("  'use strict';\n\n");
    bundle.push_str("  window.AlyComponents = window.AlyComponents || {};\n\n");
    
    for component in reg.components.values() {
        bundle.push_str(&generate_js_class(component));
        bundle.push_str("\n");
    }
    
    for script in &reg.hydration_scripts {
        bundle.push_str(script);
        bundle.push_str("\n");
    }
    
    bundle.push_str("})();\n");
    
    ok_str(bundle)
}

fn generate_js_class(component: &WebComponent) -> String {
    let mut js = String::new();
    
    js.push_str(&format!("  class {} extends HTMLElement {{\n", pascal_case(&component.tag_name)));
    
    if let Some(constructor) = &component.lifecycle.constructor {
        js.push_str("    constructor() {\n");
        js.push_str("      super();\n");
        js.push_str(&format!("      {}\n", constructor));
        js.push_str("    }\n\n");
    } else {
        js.push_str("    constructor() {\n");
        js.push_str("      super();\n");
        js.push_str("    }\n\n");
    }
    
    if !component.observed_attributes.is_empty() {
        js.push_str(&format!("    static get observedAttributes() {{\n"));
        js.push_str(&format!("      return {};\n", serde_json::to_string(&component.observed_attributes).unwrap()));
        js.push_str("    }\n\n");
    }
    
    if let Some(connected) = &component.lifecycle.connected {
        js.push_str("    connectedCallback() {\n");
        js.push_str(&format!("      {}\n", connected));
        js.push_str("    }\n\n");
    }
    
    if let Some(disconnected) = &component.lifecycle.disconnected {
        js.push_str("    disconnectedCallback() {\n");
        js.push_str(&format!("      {}\n", disconnected));
        js.push_str("    }\n\n");
    }
    
    if let Some(attr_changed) = &component.lifecycle.attribute_changed {
        js.push_str("    attributeChangedCallback(name, oldValue, newValue) {\n");
        js.push_str(&format!("      {}\n", attr_changed));
        js.push_str("    }\n\n");
    }
    
    for (name, method) in &component.methods {
        if method.is_static {
            js.push_str(&format!("    static {}(", name));
        } else {
            js.push_str(&format!("    {}(", name));
        }
        js.push_str(&method.params.join(", "));
        js.push_str(") {\n");
        if method.is_async {
            js.push_str("      return (async () => {\n");
            js.push_str(&format!("        {}\n", method.body));
            js.push_str("      })();\n");
        } else {
            js.push_str(&format!("      {}\n", method.body));
        }
        js.push_str("    }\n\n");
    }
    
    js.push_str("    render() {\n");
    js.push_str("      const template = document.createElement('template');\n");
    
    let mut template_html = component.template.clone();
    for (key, prop) in &component.properties {
        let placeholder = format!("{{{{{}}}}}", key);
        let default = prop.default.as_deref().unwrap_or("");
        template_html = template_html.replace(&placeholder, default);
    }
    
    js.push_str(&format!("      template.innerHTML = `{}`;\n", template_html.replace('`', "\\`")));
    js.push_str("      return template.content.cloneNode(true);\n");
    js.push_str("    }\n\n");
    
    js.push_str("    connectedCallback() {\n");
    js.push_str("      if (!this.shadowRoot) {\n");
    match component.shadow_mode {
        ShadowMode::Open => js.push_str("        this.attachShadow({ mode: 'open' });\n"),
        ShadowMode::Closed => js.push_str("        this.attachShadow({ mode: 'closed' });\n"),
        ShadowMode::None => {}
    }
    js.push_str("      }\n");
    js.push_str("      if (this.shadowRoot) {\n");
    js.push_str("        this.shadowRoot.innerHTML = '';\n");
    js.push_str("        this.shadowRoot.appendChild(this.render());\n");
    if !component.styles.is_empty() {
        js.push_str(&format!("        const style = document.createElement('style');\n"));
        js.push_str(&format!("        style.textContent = `{}`;\n", component.styles.replace('`', "\\`")));
        js.push_str("        this.shadowRoot.appendChild(style);\n");
    }
    js.push_str("      } else {\n");
    js.push_str("        this.innerHTML = '';\n");
    js.push_str("        this.appendChild(this.render());\n");
    js.push_str("      }\n");
    js.push_str("    }\n");
    
    js.push_str("  }\n\n");
    
    js.push_str(&format!("  customElements.define('{}', {});\n", component.tag_name, pascal_case(&component.tag_name)));
    js.push_str(&format!("  window.AlyComponents['{}'] = {};\n", component.tag_name, pascal_case(&component.tag_name)));
    
    js
}

fn pascal_case(s: &str) -> String {
    s.split('-')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                None => String::new(),
            }
        })
        .collect()
}

pub fn wc_define_from_file(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 1);
    let file_path = arg(&args_list, 0);

    let content = std::fs::read_to_string(&file_path).unwrap_or_default();
    
    #[derive(Deserialize)]
    struct ComponentFile {
        tag_name: String,
        template: String,
        styles: String,
        script: String,
        observed_attributes: Option<Vec<String>>,
        properties: Option<HashMap<String, ComponentProperty>>,
        methods: Option<HashMap<String, ComponentMethod>>,
        events: Option<Vec<String>>,
        lifecycle: Option<ComponentLifecycle>,
        shadow_mode: Option<String>,
        slots: Option<Vec<SlotDefinition>>,
    }
    
    if let Ok(comp_file) = serde_json::from_str::<ComponentFile>(&content) {
        let mut reg = registry().lock().unwrap();
        
        let component = WebComponent {
            tag_name: comp_file.tag_name.clone(),
            template: comp_file.template,
            styles: comp_file.styles,
            script: comp_file.script,
            observed_attributes: comp_file.observed_attributes.unwrap_or_default(),
            properties: comp_file.properties.unwrap_or_default(),
            methods: comp_file.methods.unwrap_or_default(),
            events: comp_file.events.unwrap_or_default(),
            lifecycle: comp_file.lifecycle.unwrap_or(ComponentLifecycle {
                constructor: None,
                connected: None,
                disconnected: None,
                attribute_changed: None,
                adopted: None,
                form_associated: None,
                form_disabled: None,
                form_reset: None,
                form_state_restore: None,
            }),
            shadow_mode: match comp_file.shadow_mode.as_deref() {
                Some("open") => ShadowMode::Open,
                Some("closed") => ShadowMode::Closed,
                Some("none") => ShadowMode::None,
                _ => ShadowMode::Open,
            },
            slots: comp_file.slots.unwrap_or_default(),
            defined: true,
        };
        
        reg.components.insert(comp_file.tag_name.clone(), component);
        reg.defined_tags.insert(comp_file.tag_name, true);
        ok_str("None".to_string())
    } else {
        ok_str("Invalid component file format")
    }
}