// Aly CSS Module: CSS-in-Aly, CSS Modules, Styled Components, Tailwind-like utilities

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::native::types::{Validator, ValueData};
use crate::native::std::{split_args, arg};
use crate::validators::str::put_quoted_str;

use serde::{Deserialize, Serialize};

fn ok_str(s: String) -> Box<dyn Validator> {
    Box::new(put_quoted_str(s))
}

fn ok_bool(b: bool) -> Box<dyn Validator> {
    Box::new(ValueData::Bool(b))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CSSRule {
    pub selector: String,
    pub properties: HashMap<String, String>,
    pub media_query: Option<String>,
    pub pseudo_class: Option<String>,
    pub pseudo_element: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CSSKeyframes {
    pub name: String,
    pub frames: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CSSModule {
    pub name: String,
    pub rules: Vec<CSSRule>,
    pub keyframes: Vec<CSSKeyframes>,
    pub imports: Vec<String>,
    pub variables: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub colors: HashMap<String, String>,
    pub spacing: HashMap<String, String>,
    pub typography: HashMap<String, String>,
    pub breakpoints: HashMap<String, String>,
    pub shadows: HashMap<String, String>,
    pub border_radius: HashMap<String, String>,
    pub transitions: HashMap<String, String>,
    pub z_indices: HashMap<String, String>,
}

static CSS_REGISTRY: OnceLock<Arc<Mutex<CSSRegistry>>> = OnceLock::new();

struct CSSRegistry {
    modules: HashMap<String, CSSModule>,
    themes: HashMap<String, Theme>,
    current_theme: Option<String>,
    global_styles: String,
    scoped_counter: usize,
}

impl CSSRegistry {
    fn new() -> Self {
        let mut registry = Self {
            modules: HashMap::new(),
            themes: HashMap::new(),
            current_theme: None,
            global_styles: String::new(),
            scoped_counter: 0,
        };
        
        registry.register_default_theme();
        registry
    }

    fn register_default_theme(&mut self) {
        let mut theme = Theme {
            name: "default".to_string(),
            colors: HashMap::new(),
            spacing: HashMap::new(),
            typography: HashMap::new(),
            breakpoints: HashMap::new(),
            shadows: HashMap::new(),
            border_radius: HashMap::new(),
            transitions: HashMap::new(),
            z_indices: HashMap::new(),
        };

        theme.colors.insert("primary".to_string(), "#6366f1".to_string());
        theme.colors.insert("primary-hover".to_string(), "#4f46e5".to_string());
        theme.colors.insert("secondary".to_string(), "#a855f7".to_string());
        theme.colors.insert("success".to_string(), "#22c55e".to_string());
        theme.colors.insert("warning".to_string(), "#f59e0b".to_string());
        theme.colors.insert("danger".to_string(), "#ef4444".to_string());
        theme.colors.insert("background".to_string(), "#0f0f13".to_string());
        theme.colors.insert("surface".to_string(), "#191923".to_string());
        theme.colors.insert("text".to_string(), "#f1f1f7".to_string());
        theme.colors.insert("text-muted".to_string(), "#a1a1aa".to_string());
        theme.colors.insert("border".to_string(), "rgba(255, 255, 255, 0.08)".to_string());

        theme.spacing.insert("xs".to_string(), "4px".to_string());
        theme.spacing.insert("sm".to_string(), "8px".to_string());
        theme.spacing.insert("md".to_string(), "16px".to_string());
        theme.spacing.insert("lg".to_string(), "24px".to_string());
        theme.spacing.insert("xl".to_string(), "32px".to_string());
        theme.spacing.insert("2xl".to_string(), "48px".to_string());

        theme.typography.insert("font-family".to_string(), "'Outfit', -apple-system, sans-serif".to_string());
        theme.typography.insert("font-size-xs".to_string(), "12px".to_string());
        theme.typography.insert("font-size-sm".to_string(), "14px".to_string());
        theme.typography.insert("font-size-base".to_string(), "16px".to_string());
        theme.typography.insert("font-size-lg".to_string(), "18px".to_string());
        theme.typography.insert("font-size-xl".to_string(), "24px".to_string());
        theme.typography.insert("font-size-2xl".to_string(), "32px".to_string());
        theme.typography.insert("font-weight-normal".to_string(), "400".to_string());
        theme.typography.insert("font-weight-medium".to_string(), "500".to_string());
        theme.typography.insert("font-weight-bold".to_string(), "600".to_string());

        theme.breakpoints.insert("sm".to_string(), "640px".to_string());
        theme.breakpoints.insert("md".to_string(), "768px".to_string());
        theme.breakpoints.insert("lg".to_string(), "1024px".to_string());
        theme.breakpoints.insert("xl".to_string(), "1280px".to_string());
        theme.breakpoints.insert("2xl".to_string(), "1536px".to_string());

        theme.shadows.insert("sm".to_string(), "0 1px 2px 0 rgba(0, 0, 0, 0.05)".to_string());
        theme.shadows.insert("md".to_string(), "0 4px 6px -1px rgba(0, 0, 0, 0.1)".to_string());
        theme.shadows.insert("lg".to_string(), "0 10px 15px -3px rgba(0, 0, 0, 0.1)".to_string());
        theme.shadows.insert("xl".to_string(), "0 20px 25px -5px rgba(0, 0, 0, 0.1)".to_string());
        theme.shadows.insert("glow".to_string(), "0 0 20px rgba(99, 102, 241, 0.3)".to_string());

        theme.border_radius.insert("none".to_string(), "0".to_string());
        theme.border_radius.insert("sm".to_string(), "4px".to_string());
        theme.border_radius.insert("md".to_string(), "8px".to_string());
        theme.border_radius.insert("lg".to_string(), "12px".to_string());
        theme.border_radius.insert("xl".to_string(), "16px".to_string());
        theme.border_radius.insert("full".to_string(), "9999px".to_string());

        theme.transitions.insert("fast".to_string(), "150ms ease".to_string());
        theme.transitions.insert("normal".to_string(), "200ms ease".to_string());
        theme.transitions.insert("slow".to_string(), "300ms ease".to_string());

        theme.z_indices.insert("dropdown".to_string(), "100".to_string());
        theme.z_indices.insert("modal".to_string(), "200".to_string());
        theme.z_indices.insert("popover".to_string(), "300".to_string());
        theme.z_indices.insert("tooltip".to_string(), "400".to_string());
        theme.z_indices.insert("toast".to_string(), "500".to_string());

        self.themes.insert("default".to_string(), theme);
        self.current_theme = Some("default".to_string());
    }

    fn get_theme(&self, name: Option<&str>) -> Option<&Theme> {
        let name = name.unwrap_or_else(|| self.current_theme.as_deref().unwrap_or("default"));
        self.themes.get(name)
    }

    fn generate_scoped_class(&mut self, base: &str) -> String {
        self.scoped_counter += 1;
        format!("aly-{}-{}", base, self.scoped_counter)
    }

    fn resolve_token(&self, token: &str, theme: &Theme) -> String {
        if token.starts_with("var(--") {
            return token.to_string();
        }

        if let Some(color) = theme.colors.get(token) {
            return color.clone();
        }
        if let Some(spacing) = theme.spacing.get(token) {
            return spacing.clone();
        }
        if let Some(typo) = theme.typography.get(token) {
            return typo.clone();
        }
        if let Some(shadow) = theme.shadows.get(token) {
            return shadow.clone();
        }
        if let Some(radius) = theme.border_radius.get(token) {
            return radius.clone();
        }
        if let Some(transition) = theme.transitions.get(token) {
            return transition.clone();
        }
        if let Some(z) = theme.z_indices.get(token) {
            return z.clone();
        }

        token.to_string()
    }
}

fn registry() -> &'static Arc<Mutex<CSSRegistry>> {
    CSS_REGISTRY.get_or_init(|| Arc::new(Mutex::new(CSSRegistry::new())))
}

pub fn css_create(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 1);
    let name = arg(&args_list, 0);

    let mut reg = registry().lock().unwrap();
    
    if reg.modules.contains_key(&name) {
        return ok_str(format!("CSS module '{}' already exists", name));
    }

    let module = CSSModule {
        name: name.clone(),
        rules: Vec::new(),
        keyframes: Vec::new(),
        imports: Vec::new(),
        variables: HashMap::new(),
    };

    reg.modules.insert(name.clone(), module);
    ok_str("None".to_string())
}

pub fn css_module_create(args: String) -> Box<dyn Validator> {
    css_create(args)
}

pub fn css_add_rule(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 5);
    let module_name = arg(&args_list, 0);
    let selector = arg(&args_list, 1);
    let properties_json = arg(&args_list, 2);
    let media_query = arg(&args_list, 3);
    let pseudo = arg(&args_list, 4);

    let properties: HashMap<String, String> = if properties_json.is_empty() {
        HashMap::new()
    } else {
        serde_json::from_str(&properties_json).unwrap_or_default()
    };

    let mut reg = registry().lock().unwrap();
    
    if let Some(module) = reg.modules.get_mut(&module_name) {
        let mut pseudo_class = None;
        let mut pseudo_element = None;
        
        if pseudo.starts_with("::") {
            pseudo_element = Some(pseudo);
        } else if pseudo.starts_with(":") {
            pseudo_class = Some(pseudo);
        }

        let rule = CSSRule {
            selector,
            properties,
            media_query: if media_query.is_empty() { None } else { Some(media_query) },
            pseudo_class,
            pseudo_element,
        };

        module.rules.push(rule);
        ok_str("None".to_string())
    } else {
        ok_str(format!("CSS module '{}' not found", module_name))
    }
}

pub fn css_add_keyframes(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 3);
    let module_name = arg(&args_list, 0);
    let name = arg(&args_list, 1);
    let frames_json = arg(&args_list, 2);

    let frames: HashMap<String, HashMap<String, String>> = if frames_json.is_empty() {
        HashMap::new()
    } else {
        serde_json::from_str(&frames_json).unwrap_or_default()
    };

    let mut reg = registry().lock().unwrap();
    
    if let Some(module) = reg.modules.get_mut(&module_name) {
        let keyframes = CSSKeyframes { name, frames };
        module.keyframes.push(keyframes);
        ok_str("None".to_string())
    } else {
        ok_str(format!("CSS module '{}' not found", module_name))
    }
}

pub fn css_render(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 2);
    let module_name = arg(&args_list, 0);
    let theme_name = arg(&args_list, 1);

    let reg = registry().lock().unwrap();
    let theme = reg.get_theme(if theme_name.is_empty() { None } else { Some(&theme_name) });

    let mut css = String::new();

    if let Some(module) = reg.modules.get(&module_name) {
        for import in &module.imports {
            if let Some(imported) = reg.modules.get(import) {
                css.push_str(&render_module(imported, theme));
                css.push('\n');
            }
        }
        css.push_str(&render_module(module, theme));
    } else {
        for module in reg.modules.values() {
            css.push_str(&render_module(module, theme));
            css.push('\n');
        }
    }

    css.push_str(&reg.global_styles);

    ok_str(css)
}

fn render_module(module: &CSSModule, theme: Option<&Theme>) -> String {
    let mut css = String::new();
    let reg = registry().lock().unwrap();

    for var in &module.variables {
        css.push_str(&format!("  --{}: {};\n", var.0, var.1));
    }

    for rule in &module.rules {
        let mut selector = rule.selector.clone();
        
        if let Some(pseudo) = &rule.pseudo_class {
            selector.push_str(pseudo);
        }
        if let Some(pseudo) = &rule.pseudo_element {
            selector.push_str(pseudo);
        }

        if let Some(media) = &rule.media_query {
            css.push_str(&format!("@media {} {{\n", media));
            css.push_str(&format!("  {} {{\n", selector));
        } else {
            css.push_str(&format!("{} {{\n", selector));
        }

        for (prop, value) in &rule.properties {
            let resolved = if let Some(t) = theme {
                reg.resolve_token(value, t)
            } else {
                value.clone()
            };
            css.push_str(&format!("    {}: {};\n", prop, resolved));
        }

        css.push_str("  }\n");
        
        if rule.media_query.is_some() {
            css.push_str("}\n");
        }
        css.push('\n');
    }

    for kf in &module.keyframes {
        css.push_str(&format!("@keyframes {} {{\n", kf.name));
        for (frame, props) in &kf.frames {
            css.push_str(&format!("  {} {{\n", frame));
            for (prop, value) in props {
                let resolved = if let Some(t) = theme {
                    reg.resolve_token(value, t)
                } else {
                    value.clone()
                };
                css.push_str(&format!("    {}: {};\n", prop, resolved));
            }
            css.push_str("  }\n");
        }
        css.push_str("}\n\n");
    }

    css
}

pub fn css_scope(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 2);
    let module_name = arg(&args_list, 0);
    let base_class = arg(&args_list, 1);

    let mut reg = registry().lock().unwrap();
    let scoped = reg.generate_scoped_class(&base_class);

    if let Some(module) = reg.modules.get_mut(&module_name) {
        for rule in &mut module.rules {
            if !rule.selector.starts_with('.') && !rule.selector.starts_with('#') {
                rule.selector = format!(".{} {}", scoped, rule.selector);
            }
        }
    }

    ok_str(scoped)
}

pub fn css_theme_create(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 9);
    let name = arg(&args_list, 0);
    let colors_json = arg(&args_list, 1);
    let spacing_json = arg(&args_list, 2);
    let typography_json = arg(&args_list, 3);
    let breakpoints_json = arg(&args_list, 4);
    let shadows_json = arg(&args_list, 5);
    let border_radius_json = arg(&args_list, 6);
    let transitions_json = arg(&args_list, 7);
    let z_indices_json = arg(&args_list, 8);

    let colors: HashMap<String, String> = if colors_json.is_empty() { HashMap::new() } else { serde_json::from_str(&colors_json).unwrap_or_default() };
    let spacing: HashMap<String, String> = if spacing_json.is_empty() { HashMap::new() } else { serde_json::from_str(&spacing_json).unwrap_or_default() };
    let typography: HashMap<String, String> = if typography_json.is_empty() { HashMap::new() } else { serde_json::from_str(&typography_json).unwrap_or_default() };
    let breakpoints: HashMap<String, String> = if breakpoints_json.is_empty() { HashMap::new() } else { serde_json::from_str(&breakpoints_json).unwrap_or_default() };
    let shadows: HashMap<String, String> = if shadows_json.is_empty() { HashMap::new() } else { serde_json::from_str(&shadows_json).unwrap_or_default() };
    let border_radius: HashMap<String, String> = if border_radius_json.is_empty() { HashMap::new() } else { serde_json::from_str(&border_radius_json).unwrap_or_default() };
    let transitions: HashMap<String, String> = if transitions_json.is_empty() { HashMap::new() } else { serde_json::from_str(&transitions_json).unwrap_or_default() };
    let z_indices: HashMap<String, String> = if z_indices_json.is_empty() { HashMap::new() } else { serde_json::from_str(&z_indices_json).unwrap_or_default() };

    let mut reg = registry().lock().unwrap();
    
    let theme = Theme {
        name: name.clone(),
        colors,
        spacing,
        typography,
        breakpoints,
        shadows,
        border_radius,
        transitions,
        z_indices,
    };

    reg.themes.insert(name.clone(), theme);
    ok_str("None".to_string())
}

pub fn css_theme_set(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 1);
    let name = arg(&args_list, 0);

    let mut reg = registry().lock().unwrap();
    
    if reg.themes.contains_key(&name) {
        reg.current_theme = Some(name);
        ok_str("None".to_string())
    } else {
        ok_str(format!("Theme '{}' not found", name))
    }
}

pub fn css_token(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 2);
    let token = arg(&args_list, 0);
    let theme_name = arg(&args_list, 1);

    let reg = registry().lock().unwrap();
    let theme = reg.get_theme(if theme_name.is_empty() { None } else { Some(&theme_name) });

    if let Some(t) = theme {
        let resolved = reg.resolve_token(&token, t);
        ok_str(resolved)
    } else {
        ok_str(token)
    }
}

pub fn css_module_import(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 2);
    let module_name = arg(&args_list, 0);
    let import_name = arg(&args_list, 1);

    let mut reg = registry().lock().unwrap();
    
    if let Some(module) = reg.modules.get_mut(&module_name) {
        if reg.modules.contains_key(&import_name) && !module.imports.contains(&import_name) {
            module.imports.push(import_name);
            ok_str("None".to_string())
        } else {
            ok_str(format!("Module '{}' not found", import_name))
        }
    } else {
        ok_str(format!("CSS module '{}' not found", module_name))
    }
}

pub fn css_global_add(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 1);
    let css = arg(&args_list, 0);

    let mut reg = registry().lock().unwrap();
    reg.global_styles.push_str(&css);
    reg.global_styles.push('\n');
    
    ok_str("None".to_string())
}

pub fn css_variable_set(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 3);
    let module_name = arg(&args_list, 0);
    let name = arg(&args_list, 1);
    let value = arg(&args_list, 2);

    let mut reg = registry().lock().unwrap();
    
    if let Some(module) = reg.modules.get_mut(&module_name) {
        module.variables.insert(name, value);
        ok_str("None".to_string())
    } else {
        ok_str(format!("CSS module '{}' not found", module_name))
    }
}

pub fn css_reset(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 1);
    let module_name = arg(&args_list, 0);

    let mut reg = registry().lock().unwrap();
    
    if !module_name.is_empty() {
        reg.modules.remove(&module_name);
    } else {
        reg.modules.clear();
        reg.themes.clear();
        reg.global_styles.clear();
        reg.scoped_counter = 0;
        reg.register_default_theme();
    }
    
    ok_str("None".to_string())
}

pub fn css_list_modules(args: String) -> Box<dyn Validator> {
    let _ = args;
    let reg = registry().lock().unwrap();
    let modules: Vec<String> = reg.modules.keys().cloned().collect();
    ok_str(modules.join(", "))
}

pub fn css_list_themes(args: String) -> Box<dyn Validator> {
    let _ = args;
    let reg = registry().lock().unwrap();
    let themes: Vec<String> = reg.themes.keys().cloned().collect();
    ok_str(themes.join(", "))
}