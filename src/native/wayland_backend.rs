use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use tiny_skia::{
    self, Color, Paint, Pixmap, Rect, FillRule, Transform, PathBuilder,
};
use ab_glyph::{
    FontArc, Font as AbFont, PxScale, ScaleFont as AbScaleFont,
    Point as AbPoint,
};

use crate::gui::{
    self as gui_crate,
    backend::{GuiBackend, BackendType},
    widget::{
        WidgetId, next_widget_id, PROP_LABEL, PROP_TEXT, PROP_VALUE, PROP_CHECKED,
        PROP_ENABLED, PROP_VISIBLE, PROP_FRACTION, PROP_WIDTH, PROP_HEIGHT,
    },
};

const PADDING: f32 = 8.0;
const WIDGET_SPACING: f32 = 6.0;
const CORNER_RADIUS: f32 = 4.0;
const COLOR_WINDOW_BG: u32 = 0xFFF5F5F5;
const COLOR_PRIMARY: u32 = 0xFF007AFF;
const COLOR_TEXT: u32 = 0xFF1C1C1E;
const COLOR_TEXT_SECONDARY: u32 = 0xFF8E8E93;
const COLOR_BORDER: u32 = 0xFFD1D1D6;
const COLOR_BUTTON_TEXT: u32 = 0xFFFFFFFF;
const COLOR_CHECK_ACTIVE: u32 = 0xFF007AFF;
const COLOR_SLIDER_TRACK: u32 = 0xFFE5E5EA;
const COLOR_SLIDER_FILL: u32 = 0xFF007AFF;
const COLOR_PROGRESS_BG: u32 = 0xFFE5E5EA;
const COLOR_PROGRESS_FILL: u32 = 0xFF34C759;
const COLOR_INPUT_BG: u32 = 0xFFFFFFFF;
const COLOR_DIVIDER: u32 = 0xFFC6C6C8;

fn hex_color(hex: u32) -> Color {
    Color::from_rgba8(
        ((hex >> 16) & 0xFF) as u8,
        ((hex >> 8) & 0xFF) as u8,
        (hex & 0xFF) as u8,
        255,
    )
}

fn rgba_paint(color: &Color) -> Paint {
    let mut p = Paint::default();
    p.set_color_rgba8(
        (color.red() * 255.0) as u8,
        (color.green() * 255.0) as u8,
        (color.blue() * 255.0) as u8,
        (color.alpha() * 255.0) as u8,
    );
    p
}

fn default_font() -> Option<FontArc> {
    let paths = vec![
        "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
        "/usr/share/fonts/TTF/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/ubuntu/Ubuntu-Regular.ttf",
        "/usr/share/fonts/noto/NotoSans-Regular.ttf",
        "/usr/share/fonts/liberation/LiberationSans-Regular.ttf",
        "/usr/share/fonts/dejavu/DejaVuSans.ttf",
    ];
    for path in &paths {
        if let Ok(data) = std::fs::read(path) {
            if let Ok(font) = FontArc::try_from_vec(data) {
                return Some(font);
            }
        }
    }
    None
}

#[derive(Clone, Debug, PartialEq)]
enum WidgetType {
    Window, Button, Label, Div, Input, TextArea, Password,
    Checkbox, Radio, Slider, ProgressBar, Dropdown, Separator, Spinner, Container,
}

#[derive(Clone)]
struct WidgetState {
    type_: WidgetType,
    label: String,
    value: String,
    checked: bool,
    enabled: bool,
    visible: bool,
    fraction: f64,
    min: f64,
    max: f64,
    step: f64,
    items: Vec<String>,
    direction: String,
    width: i32,
    height: i32,
    x: f32,
    y: f32,
    children: Vec<WidgetId>,
    parent: Option<WidgetId>,
}

impl WidgetState {
    fn new(type_: WidgetType) -> Self {
        Self {
            type_,
            label: String::new(),
            value: String::new(),
            checked: false,
            enabled: true,
            visible: true,
            fraction: 0.0,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            items: Vec::new(),
            direction: "vertical".to_string(),
            width: 200,
            height: 40,
            x: 0.0,
            y: 0.0,
            children: Vec::new(),
            parent: None,
        }
    }
}

type SharedWidget = Rc<RefCell<WidgetState>>;

thread_local! {
    static WIDGETS: RefCell<HashMap<WidgetId, SharedWidget>> = RefCell::new(HashMap::new());
    static ROOTS: RefCell<Vec<WidgetId>> = RefCell::new(Vec::new());
    static FONT: RefCell<Option<FontArc>> = RefCell::new(default_font());
    static FONT_SZ: RefCell<f32> = RefCell::new(14.0);
}

fn find(id: &str) -> Option<SharedWidget> {
    WIDGETS.with(|w| w.borrow().get(id).cloned())
}

fn store(id: &str, ws: WidgetState) -> SharedWidget {
    let shared = Rc::new(RefCell::new(ws));
    WIDGETS.with(|w| { w.borrow_mut().insert(id.to_owned(), shared.clone()); });
    ROOTS.with(|r| {
        if !r.borrow().contains(&id.to_string()) {
            r.borrow_mut().push(id.to_string());
        }
    });
    shared
}

fn fill_rect(pixmap: &mut Pixmap, x: f32, y: f32, w: f32, h: f32, color: Color) {
    if let Some(rect) = Rect::from_xywh(x, y, w, h) {
        let paint = rgba_paint(&color);
        pixmap.fill_rect(rect, &paint, Transform::identity(), None);
    }
}

fn draw_round_rect(pixmap: &mut Pixmap, x: f32, y: f32, w: f32, h: f32, color: Color, radius: f32) {
    if radius <= 0.0 {
        return fill_rect(pixmap, x, y, w, h, color);
    }
    let r = radius.min(w / 2.0).min(h / 2.0);
    let mut pb = PathBuilder::new();
    pb.move_to(x + r, y);
    pb.line_to(x + w - r, y);
    pb.quad_to(x + w, y, x + w, y + r);
    pb.line_to(x + w, y + h - r);
    pb.quad_to(x + w, y + h, x + w - r, y + h);
    pb.line_to(x + r, y + h);
    pb.quad_to(x, y + h, x, y + h - r);
    pb.line_to(x, y + r);
    pb.quad_to(x, y, x + r, y);
    pb.close();
    if let Some(path) = pb.finish() {
        let paint = rgba_paint(&color);
        pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
    }
}

fn draw_border_round(pixmap: &mut Pixmap, x: f32, y: f32, w: f32, h: f32, color: Color, radius: f32, bw: f32) {
    if bw <= 0.0 { return; }
    let r = radius.min(w / 2.0).min(h / 2.0);
    let outer = draw_round_path(x, y, w, h, r);
    let inner_r = (r - bw).max(0.0);
    let inner = draw_round_path(x + bw, y + bw, w - bw * 2.0, h - bw * 2.0, inner_r);
    if let (Some(out), Some(inn)) = (outer, inner) {
        let paint = rgba_paint(&color);
        pixmap.fill_path(&out, &paint, FillRule::Winding, Transform::identity(), None);
        let mut clear = Paint::default();
        clear.set_color_rgba8(0, 0, 0, 0);
        clear.blend_mode = tiny_skia::BlendMode::Clear;
        pixmap.fill_path(&inn, &clear, FillRule::Winding, Transform::identity(), None);
    }
}

fn draw_round_path(x: f32, y: f32, w: f32, h: f32, r: f32) -> Option<tiny_skia::Path> {
    let mut pb = PathBuilder::new();
    pb.move_to(x + r, y);
    pb.line_to(x + w - r, y);
    pb.quad_to(x + w, y, x + w, y + r);
    pb.line_to(x + w, y + h - r);
    pb.quad_to(x + w, y + h, x + w - r, y + h);
    pb.line_to(x + r, y + h);
    pb.quad_to(x, y + h, x, y + h - r);
    pb.line_to(x, y + r);
    pb.quad_to(x, y, x + r, y);
    pb.close();
    pb.finish()
}

#[allow(dead_code)]
fn draw_text(pixmap: &mut Pixmap, text: &str, x: f32, y: f32, color: Color, font_size: f32, max_w: f32) {
    let font = match FONT.with(|f| f.borrow().clone()) {
        Some(f) => f,
        None => return,
    };
    let px = PxScale::from(font_size);
    let sf = font.as_scaled(px);
    let baseline = y + sf.ascent();
    let w = pixmap.width();
    let h = pixmap.height();

    let r = (color.red() * 255.0) as u8;
    let g = (color.green() * 255.0) as u8;
    let b = (color.blue() * 255.0) as u8;

    let mut cx = x;
    let mut cy = baseline;
    for ch in text.chars() {
        if ch == '\n' {
            cx = x;
            cy += sf.height() + sf.line_gap();
            continue;
        }
        let gid = font.glyph_id(ch);
        if gid.0 == 0 && ch != '\0' { continue; }
        let aw = sf.h_advance(gid);
        if cx - x + aw > max_w && max_w > 0.0 { break; }

        let glyph = ab_glyph::Glyph {
            id: gid,
            scale: px,
            position: AbPoint { x: cx, y: cy },
        };
        if let Some(outline) = sf.outline_glyph(glyph) {
            let pixels = pixmap.pixels_mut();
            outline.draw(|px_x, px_y, coverage| {
                if coverage > 0.0 {
                    let draw_x = (cx + px_x as f32) as u32;
                    let draw_y = (cy - px_y as f32) as i32;
                    if draw_y >= 0 && draw_x < w {
                        let dy = draw_y as u32;
                        if dy < h {
                            let idx = (dy * w + draw_x) as usize;
                            if idx < pixels.len() {
                                let a = (coverage * 255.0) as u8;
                                let pc = tiny_skia::ColorU8::from_rgba(r, g, b, a).premultiply();

                                pixels[idx] = pc;
                            }
                        }
                    }
                }
            });
        }
        cx += aw;
    }
}

fn text_width(text: &str, font_size: f32) -> f32 {
    let font = match FONT.with(|f| f.borrow().clone()) {
        Some(f) => f,
        None => return text.len() as f32 * font_size * 0.6,
    };
    let sf = font.as_scaled(PxScale::from(font_size));
    text.chars().map(|c| sf.h_advance(font.glyph_id(c))).sum()
}

fn compute_layout(id: &str, ox: f32, oy: f32) {
    let w = match find(id) { Some(w) => w, None => return };
    let (_ww, _wh, dir, chs, vis) = {
        let b = w.borrow();
        (b.width as f32, b.height as f32, b.direction.clone(), b.children.clone(), b.visible)
    };
    if !vis { return; }
    w.borrow_mut().x = ox;
    w.borrow_mut().y = oy;

    if chs.is_empty() { return; }

    let pad = PADDING;
    let sp = WIDGET_SPACING;
    let mut cy = oy + pad;
    let mut cx = ox + pad;

    for cid in &chs {
        let cv = find(cid).map(|c| c.borrow().visible).unwrap_or(false);
        if !cv { continue; }
        let (cw, ch) = {
            let c = find(cid).unwrap();
            let b = c.borrow();
            (b.width as f32, b.height as f32)
        };
        compute_layout(cid, cx, cy);
        if dir == "horizontal" { cx += cw + sp; } else { cy += ch + sp; }
    }
}

fn compute_all() {
    ROOTS.with(|r| {
        for rid in r.borrow().iter() {
            if let Some(rw) = find(rid) {
                if rw.borrow().visible {
                    compute_layout(rid, 0.0, 0.0);
                }
            }
        }
    });
}

#[allow(dead_code)]
fn hit_test(x: f32, y: f32, id: &str) -> Option<WidgetId> {
    let w = find(id)?;
    let (wx, wy, ww, wh, vis, chs) = {
        let b = w.borrow();
        (b.x, b.y, b.width as f32, b.height as f32, b.visible, b.children.clone())
    };
    if !vis || x < wx || x >= wx + ww || y < wy || y >= wy + wh { return None; }
    for cid in chs.iter().rev() {
        if let Some(h) = hit_test(x, y, cid) { return Some(h); }
    }
    Some(id.to_string())
}

fn draw_widget(pixmap: &mut Pixmap, id: &str) {
    let w = match find(id) { Some(w) => w, None => return };
    let (t, label, value, checked, _enabled, vis, frac, _min, _max, items,
         _dir, ww, wh, x, y, chs) =
    {
        let b = w.borrow();
        (b.type_.clone(), b.label.clone(), b.value.clone(), b.checked, b.enabled,
         b.visible, b.fraction, b.min, b.max, b.items.clone(),
         b.direction.clone(), b.width as f32, b.height as f32, b.x, b.y, b.children.clone())
    };
    if !vis || ww <= 0.0 || wh <= 0.0 { return; }

    let fs = FONT_SZ.with(|f| *f.borrow());

    match t {
        WidgetType::Separator => {
            fill_rect(pixmap, x, y + wh / 2.0, ww, 1.0, hex_color(COLOR_DIVIDER));
        }
        WidgetType::Label => {
            if !label.is_empty() {
                draw_text(pixmap, &label, x + PADDING, y + PADDING, hex_color(COLOR_TEXT), fs, ww - PADDING * 2.0);
            }
        }
        WidgetType::Button => {
            let bg = hex_color(COLOR_PRIMARY);
            draw_round_rect(pixmap, x, y, ww, wh, bg, CORNER_RADIUS);
            if !label.is_empty() {
                let tw = text_width(&label, fs);
                let tx = x + (ww - tw) / 2.0;
                let ty = y + (wh - fs) / 2.0;
                draw_text(pixmap, &label, tx, ty, hex_color(COLOR_BUTTON_TEXT), fs, ww);
            }
        }
        WidgetType::Input | WidgetType::Password | WidgetType::TextArea => {
            fill_rect(pixmap, x, y, ww, wh, hex_color(COLOR_INPUT_BG));
            draw_border_round(pixmap, x, y, ww, wh, hex_color(COLOR_BORDER), CORNER_RADIUS, 1.0);
            let display = match t {
                WidgetType::Password => "*".repeat(value.len()),
                _ => { if value.is_empty() && !label.is_empty() { label.clone() } else { value.clone() } }
            };
            let tc = if value.is_empty() && !label.is_empty() { hex_color(COLOR_TEXT_SECONDARY) } else { hex_color(COLOR_TEXT) };
            draw_text(pixmap, &display, x + 6.0, y + 6.0, tc, fs, ww - 12.0);
        }
        WidgetType::Checkbox => {
            let sz = wh.min(24.0);
            let cbx = x + 4.0;
            let cby = y + (wh - sz) / 2.0;
            draw_border_round(pixmap, cbx, cby, sz, sz, hex_color(COLOR_BORDER), 3.0, 1.5);
            if checked {
                fill_rect(pixmap, cbx + 3.0, cby + 3.0, sz - 6.0, sz - 6.0, hex_color(COLOR_CHECK_ACTIVE));
            }
            if !label.is_empty() {
                draw_text(pixmap, &label, cbx + sz + 8.0, y + (wh - fs) / 2.0, hex_color(COLOR_TEXT), fs, ww - sz - 16.0);
            }
        }
        WidgetType::Radio => {
            let sz = wh.min(24.0);
            let rbx = x + 4.0;
            let rby = y + (wh - sz) / 2.0;
            draw_round_rect(pixmap, rbx, rby, sz, sz, hex_color(COLOR_INPUT_BG), sz / 2.0);
            draw_border_round(pixmap, rbx, rby, sz, sz, hex_color(COLOR_BORDER), sz / 2.0, 1.5);
            if checked {
                let dot = sz - 8.0;
                draw_round_rect(pixmap, rbx + 4.0, rby + 4.0, dot, dot, hex_color(COLOR_CHECK_ACTIVE), dot / 2.0);
            }
            if !label.is_empty() {
                draw_text(pixmap, &label, rbx + sz + 8.0, y + (wh - fs) / 2.0, hex_color(COLOR_TEXT), fs, ww - sz - 16.0);
            }
        }
        WidgetType::Slider => {
            let th = 4.0;
            let ty_ = y + (wh - th) / 2.0;
            fill_rect(pixmap, x + 4.0, ty_, ww - 8.0, th, hex_color(COLOR_SLIDER_TRACK));
            let range = if _max > _min { _max - _min } else { 100.0 };
            let frac = if range > 0.0 { ((value.parse::<f64>().unwrap_or(0.0) - _min) / range).clamp(0.0, 1.0) as f32 } else { 0.0 };
            let fw = (ww - 8.0) * frac;
            if fw > 0.0 { fill_rect(pixmap, x + 4.0, ty_, fw, th, hex_color(COLOR_SLIDER_FILL)); }
        }
        WidgetType::ProgressBar => {
            let bh = wh * 0.6;
            let by = y + (wh - bh) / 2.0;
            draw_round_rect(pixmap, x, by, ww, bh, hex_color(COLOR_PROGRESS_BG), bh / 2.0);
            let fw = (ww * frac as f32).clamp(0.0, ww);
            if fw > 0.0 { draw_round_rect(pixmap, x, by, fw, bh, hex_color(COLOR_PROGRESS_FILL), bh / 2.0); }
            let pct = format!("{:.0}%", frac * 100.0);
            let tw = text_width(&pct, 11.0);
            draw_text(pixmap, &pct, x + (ww - tw) / 2.0, by + 1.0, hex_color(COLOR_BUTTON_TEXT), 11.0, ww);
        }
        WidgetType::Dropdown => {
            fill_rect(pixmap, x, y, ww, wh, hex_color(COLOR_INPUT_BG));
            draw_border_round(pixmap, x, y, ww, wh, hex_color(COLOR_BORDER), CORNER_RADIUS, 1.0);
            let display = if value.is_empty() { items.first().map(|s| s.as_str()).unwrap_or("") } else { &value };
            draw_text(pixmap, display, x + 6.0, y + (wh - fs) / 2.0, hex_color(COLOR_TEXT), fs, ww - 30.0);
            draw_text(pixmap, "\u{25BC}", x + ww - 20.0, y + (wh - 10.0) / 2.0, hex_color(COLOR_TEXT_SECONDARY), 10.0, 20.0);
        }
        WidgetType::Spinner => {
            fill_rect(pixmap, x, y, ww, wh, hex_color(COLOR_INPUT_BG));
            draw_border_round(pixmap, x, y, ww, wh, hex_color(COLOR_BORDER), CORNER_RADIUS, 1.0);
            draw_text(pixmap, &value, x + 6.0, y + (wh - fs) / 2.0, hex_color(COLOR_TEXT), fs, ww - 12.0);
            draw_text(pixmap, "\u{25B2}", x + ww - 18.0, y + 4.0, hex_color(COLOR_TEXT_SECONDARY), 10.0, 16.0);
            draw_text(pixmap, "\u{25BC}", x + ww - 18.0, y + wh / 2.0 + 2.0, hex_color(COLOR_TEXT_SECONDARY), 10.0, 16.0);
        }
        WidgetType::Window => {
            fill_rect(pixmap, x, y, ww, wh, hex_color(COLOR_WINDOW_BG));
        }
        _ => {}
    }

    for cid in &chs {
        draw_widget(pixmap, cid);
    }
}

fn render_window(id: &str, width: u32, height: u32) -> Option<Pixmap> {
    let mut pixmap = Pixmap::new(width, height)?;
    pixmap.fill(hex_color(COLOR_WINDOW_BG));
    compute_all();
    draw_widget(&mut pixmap, id);
    Some(pixmap)
}

pub struct WaylandBackend;

impl WaylandBackend {
    pub fn new() -> Self { Self }
}

impl Default for WaylandBackend {
    fn default() -> Self { Self::new() }
}

impl GuiBackend for WaylandBackend {
    fn backend_type(&self) -> BackendType {
        BackendType::Wayland
    }

    fn create_window(&mut self, title: &str, width: i32, height: i32) -> WidgetId {
        let id = next_widget_id();
        let mut ws = WidgetState::new(WidgetType::Window);
        ws.label = title.to_string();
        ws.width = width;
        ws.height = height;
        store(&id, ws);
        id
    }

    fn create_button(&mut self, label: &str) -> WidgetId {
        let id = next_widget_id();
        let mut ws = WidgetState::new(WidgetType::Button);
        ws.label = label.to_string();
        ws.width = 120;
        ws.height = 36;
        store(&id, ws);
        id
    }

    fn create_label(&mut self, text: &str) -> WidgetId {
        let id = next_widget_id();
        let mut ws = WidgetState::new(WidgetType::Label);
        ws.label = text.to_string();
        let tw = text_width(text, FONT_SZ.with(|f| *f.borrow())).ceil() as i32 + 16;
        ws.width = tw.max(60);
        ws.height = 24;
        store(&id, ws);
        id
    }

    fn create_div(&mut self, direction: &str) -> WidgetId {
        let id = next_widget_id();
        let mut ws = WidgetState::new(WidgetType::Div);
        ws.direction = if direction.is_empty() { "vertical".to_string() } else { direction.to_string() };
        ws.width = 300;
        ws.height = 200;
        store(&id, ws);
        id
    }

    fn create_input(&mut self, placeholder: &str) -> WidgetId {
        let id = next_widget_id();
        let mut ws = WidgetState::new(WidgetType::Input);
        ws.label = placeholder.to_string();
        ws.width = 200;
        ws.height = 34;
        store(&id, ws);
        id
    }

    fn create_textarea(&mut self) -> WidgetId {
        let id = next_widget_id();
        let mut ws = WidgetState::new(WidgetType::TextArea);
        ws.width = 250;
        ws.height = 80;
        store(&id, ws);
        id
    }

    fn create_password(&mut self) -> WidgetId {
        let id = next_widget_id();
        let mut ws = WidgetState::new(WidgetType::Password);
        ws.width = 200;
        ws.height = 34;
        store(&id, ws);
        id
    }

    fn create_checkbox(&mut self, label: &str) -> WidgetId {
        let id = next_widget_id();
        let mut ws = WidgetState::new(WidgetType::Checkbox);
        ws.label = label.to_string();
        let tw = text_width(label, FONT_SZ.with(|f| *f.borrow())).ceil() as i32 + 40;
        ws.width = tw.max(80);
        ws.height = 28;
        store(&id, ws);
        id
    }

    fn create_radio(&mut self, label: &str) -> WidgetId {
        let id = next_widget_id();
        let mut ws = WidgetState::new(WidgetType::Radio);
        ws.label = label.to_string();
        let tw = text_width(label, FONT_SZ.with(|f| *f.borrow())).ceil() as i32 + 40;
        ws.width = tw.max(80);
        ws.height = 28;
        store(&id, ws);
        id
    }

    fn create_slider(&mut self, min: f64, max: f64, val: f64) -> WidgetId {
        let id = next_widget_id();
        let mut ws = WidgetState::new(WidgetType::Slider);
        ws.min = min;
        ws.max = max;
        ws.value = val.to_string();
        ws.width = 200;
        ws.height = 32;
        store(&id, ws);
        id
    }

    fn create_progressbar(&mut self, val: f64) -> WidgetId {
        let id = next_widget_id();
        let mut ws = WidgetState::new(WidgetType::ProgressBar);
        ws.fraction = val;
        ws.width = 200;
        ws.height = 24;
        store(&id, ws);
        id
    }

    fn create_dropdown(&mut self, items: &[String]) -> WidgetId {
        let id = next_widget_id();
        let mut ws = WidgetState::new(WidgetType::Dropdown);
        ws.items = items.to_vec();
        if !items.is_empty() { ws.value = items[0].clone(); }
        ws.width = 200;
        ws.height = 34;
        store(&id, ws);
        id
    }

    fn create_separator(&mut self) -> WidgetId {
        let id = next_widget_id();
        let mut ws = WidgetState::new(WidgetType::Separator);
        ws.width = 200;
        ws.height = 2;
        store(&id, ws);
        id
    }

    fn create_spinner(&mut self, min: f64, max: f64, step: f64, val: f64) -> WidgetId {
        let id = next_widget_id();
        let mut ws = WidgetState::new(WidgetType::Spinner);
        ws.min = min;
        ws.max = max;
        ws.step = step;
        ws.value = val.to_string();
        ws.width = 100;
        ws.height = 34;
        store(&id, ws);
        id
    }

    fn create_container(&mut self, width: i32, height: i32, direction: &str) -> WidgetId {
        let id = next_widget_id();
        let mut ws = WidgetState::new(WidgetType::Container);
        ws.width = width;
        ws.height = height;
        ws.direction = if direction.is_empty() { "vertical".to_string() } else { direction.to_string() };
        store(&id, ws);
        id
    }

    fn insert_widget(&mut self, parent: &WidgetId, child: &WidgetId) {
        if let (Some(pw), Some(cw)) = (find(parent), find(child)) {
            cw.borrow_mut().parent = Some(parent.clone());
            let mut p = pw.borrow_mut();
            if !p.children.contains(&child.to_string()) {
                p.children.push(child.to_string());
            }
        }
    }

    fn set_property(&mut self, id: &WidgetId, prop: &str, value: &str) {
        let w = match find(id) { Some(w) => w, None => return };
        let mut b = w.borrow_mut();
        match prop {
            PROP_LABEL => b.label = value.to_string(),
            PROP_TEXT | PROP_VALUE => {
                match b.type_ {
                    WidgetType::Button | WidgetType::Label => b.label = value.to_string(),
                    WidgetType::ProgressBar => {
                        if let Ok(v) = value.parse::<f64>() { b.fraction = v; }
                    }
                    _ => b.value = value.to_string(),
                }
            }
            PROP_CHECKED => b.checked = value == "true" || value == "1",
            PROP_ENABLED => b.enabled = value != "false",
            PROP_VISIBLE => b.visible = value != "false",
            PROP_FRACTION => { if let Ok(v) = value.parse::<f64>() { b.fraction = v; } }
            PROP_WIDTH => { if let Ok(v) = value.parse::<i32>() { b.width = v; } }
            PROP_HEIGHT => { if let Ok(v) = value.parse::<i32>() { b.height = v; } }
            _ => {}
        }
    }

    fn get_property(&mut self, id: &WidgetId, prop: &str) -> String {
        let w = match find(id) { Some(w) => w, None => return "None".to_owned() };
        let b = w.borrow();
        match prop {
            PROP_LABEL => b.label.clone(),
            PROP_TEXT | PROP_VALUE => match b.type_ {
                WidgetType::Checkbox | WidgetType::Radio => {
                    if b.checked { "true".to_owned() } else { "false".to_owned() }
                }
                WidgetType::ProgressBar => b.fraction.to_string(),
                _ => b.value.clone(),
            },
            PROP_CHECKED => { if b.checked { "true".to_owned() } else { "false".to_owned() } }
            PROP_ENABLED => { if b.enabled { "true".to_owned() } else { "false".to_owned() } }
            PROP_VISIBLE => { if b.visible { "true".to_owned() } else { "false".to_owned() } }
            "selected" => {
                if b.type_ == WidgetType::Dropdown { b.value.clone() } else { "None".to_owned() }
            }
            _ => "None".to_owned(),
        }
    }

    fn on_event(&mut self, id: &WidgetId, event: &str, callback: String) {
        gui_crate::event::set_callback(id, event, callback);
    }

    // System tray - not natively supported in Wayland without compositor support
    fn set_tray_icon(&mut self, _window_id: &WidgetId, _icon_path: &str, _tooltip: &str) -> bool {
        false
    }

    fn show_notification(&mut self, title: &str, body: &str, _icon: Option<&str>) {
        // Wayland notifications typically go through compositor or portal
        #[cfg(target_os = "linux")]
        {
            let _ = std::process::Command::new("notify-send")
                .arg(title)
                .arg(body)
                .spawn();
        }
    }

    fn clipboard_set(&mut self, text: &str) {
        // Wayland clipboard requires compositor support (wlr-data-control or ext-data-control)
        eprintln!("Clipboard not fully implemented for Wayland backend");
    }

    fn clipboard_get(&mut self) -> String {
        String::new()
    }

    fn drag_drop_init(&mut self, _widget_id: &WidgetId, _data: &str) {
        // Wayland drag and drop uses wlr-data-control or ext-data-control
    }

    fn drag_drop_accept(&mut self, _widget_id: &WidgetId, _types: &[&str]) {}

    // OpenGL - Wayland uses EGL
    fn gl_context_create(&mut self, _window_id: &WidgetId) -> bool {
        false // Requires EGL setup
    }

    fn gl_make_current(&mut self, _window_id: &WidgetId) -> bool {
        false
    }

    fn gl_swap_buffers(&mut self, _window_id: &WidgetId) {}

    fn gl_get_proc_address(&mut self, _proc_name: &str) -> *const std::ffi::c_void {
        std::ptr::null()
    }

    fn run(&mut self) {
        match smithay_client_toolkit::reexports::client::Connection::connect_to_env() {
            Ok(conn) => {
                let mut event_queue = conn.new_event_queue();
                let _qh = event_queue.handle();

                compute_all();

                let windows: Vec<WidgetId> = ROOTS.with(|r| {
                    r.borrow().iter()
                        .filter_map(|id| {
                            find(id).and_then(|w| {
                                if w.borrow().type_ == WidgetType::Window { Some(id.clone()) } else { None }
                            })
                        })
                        .collect()
                });

                if windows.is_empty() {
                    eprintln!("RuntimeError [gui-wayland]: No windows to display.");
                    return;
                }

                let wid = &windows[0];
                let (ww, wh) = {
                    let w = find(wid).unwrap();
                    let b = w.borrow();
                    (b.width as u32, b.height as u32)
                };

                let rendered = render_window(wid, ww, wh);
                if rendered.is_none() {
                    eprintln!("RuntimeError [gui-wayland]: Failed to render window.");
                    return;
                }

                loop {
                    if event_queue.dispatch_pending(&mut ()).is_err() {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(16));
                }
            }
            Err(e) => {
                eprintln!("RuntimeError [gui-wayland]: Could not connect to Wayland compositor: {}", e);
                eprintln!("RuntimeError [gui-wayland]: Is a Wayland compositor running? Try running under a Wayland session.");
            }
        }
    }
}
