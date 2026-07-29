// GTK4 Native Module for Aly Language
// Provides GTK4 GUI capabilities and custom screen rendering for Aly apps.

use gtk4::prelude::*;
use gtk4::{
    gio, gdk, glib, gdk_pixbuf,
    Application, ApplicationWindow, Box as GtkBox, Button, CssProvider,
    HeaderBar, Image, Label, Orientation, Stack, StackTransitionType
};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use gtk4::{CheckButton, DropDown, Entry, Notebook, ProgressBar, Scale, TextView};
use crate::native::types::{Validator, ValueData};
use crate::native::std::{arg, split_args};
use crate::validators::str::put_quoted_str;


// ─────────────────────────────────────────────────────────────────────────────
// Custom CSS for Alinix Installer Screens
// ─────────────────────────────────────────────────────────────────────────────

const INSTALLER_CSS: &str = r##"
/* Custom Theme for Alinix GTK Installer Window */
window.installer-window {
    border-radius: 12px;
}

window.window-light {
    background-color: #e6e6e6;
    color: #1a1a1a;
}

window.window-dark {
    background-color: #2b2b2b;
    color: #ffffff;
}

/* Real HeaderBar Styling */
headerbar.installer-header-dark {
    background-color: #242424;
    border-bottom: 1px solid #1e1e1e;
    min-height: 38px;
    padding: 0 12px;
    color: #ffffff;
}

headerbar.installer-header-light {
    background-color: #dcdcdc;
    border-bottom: 1px solid #c0c0c0;
    min-height: 38px;
    padding: 0 12px;
    color: #1a1a1a;
}

.header-title-dark {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    font-size: 14px;
    font-weight: 600;
    color: #e0e0e0;
}

.header-title-light {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    font-size: 14px;
    font-weight: 600;
    color: #333333;
}

/* Buttons */
button.btn-next-blue {
    background-color: #0084ff;
    color: #ffffff;
    font-weight: 600;
    font-size: 13px;
    border-radius: 18px;
    padding: 8px 32px;
    border: none;
    box-shadow: 0 2px 6px rgba(0, 132, 255, 0.4);
}
button.btn-next-blue:hover {
    background-color: #0073e6;
}

button.btn-purple {
    background-color: #9351a6;
    color: #ffffff;
    font-weight: 600;
    font-size: 13px;
    border-radius: 18px;
    padding: 8px 32px;
    border: none;
    box-shadow: 0 2px 6px rgba(147, 81, 166, 0.4);
}
button.btn-purple:hover {
    background-color: #804294;
}

button.btn-mode {
    background-color: #4c5b8a;
    color: #ffffff;
    font-weight: 600;
    font-size: 14px;
    border-radius: 12px;
    padding: 10px 24px;
    border: none;
}
button.btn-mode:hover {
    background-color: #5b6ca4;
}

button.btn-back {
    background: transparent;
    color: #cccccc;
    font-size: 14px;
    border: none;
}
button.btn-back:hover {
    color: #ffffff;
}

/* Sidebar & Language Selector Buttons */
.sidebar-box {
    background-color: #353535;
    border-radius: 12px;
    padding: 8px;
}

button.sidebar-btn {
    background: transparent;
    border: none;
    padding: 10px 16px;
    border-radius: 8px;
    color: #cccccc;
    font-size: 13px;
    font-weight: 600;
}
button.sidebar-btn:hover {
    background-color: rgba(255, 255, 255, 0.08);
    color: #ffffff;
}

button.sidebar-btn-active {
    background-color: #8f52a1;
    color: #ffffff;
    border-radius: 8px;
    padding: 10px 16px;
    font-size: 13px;
    font-weight: bold;
    border: none;
}

/* Theme cards */
button.theme-card-btn {
    background-color: #3a3a3a;
    border-radius: 12px;
    padding: 8px;
    border: 2px solid transparent;
}
button.theme-card-btn:hover {
    background-color: #444444;
}

button.theme-card-btn-selected {
    background-color: #3a3a3a;
    border-radius: 12px;
    padding: 8px;
    border: 2px solid #ffffff;
}

.card-label {
    font-weight: bold;
    font-size: 14px;
    color: #ffffff;
    margin-top: 8px;
}

.card-subtitle {
    font-size: 12px;
    color: #aaaaaa;
    margin-top: 4px;
}

/* Radio item & Disk Partitioning */
button.radio-row-btn {
    background: transparent;
    border: none;
    padding: 8px 12px;
    border-radius: 10px;
}
button.radio-row-btn:hover {
    background-color: rgba(255, 255, 255, 0.06);
}

.radio-title {
    font-size: 15px;
    font-weight: 600;
    color: #ffffff;
}

.radio-subtitle {
    font-size: 12px;
    color: #aaaaaa;
    margin-top: 2px;
}

.disk-label {
    font-size: 14px;
    font-weight: 600;
    color: #e0e0e0;
}

dropdown {
    background-color: #383838;
    color: #ffffff;
    border-radius: 8px;
    padding: 4px 12px;
    border: 1px solid #555555;
}

button.color-bubble-btn {
    background: transparent;
    border: none;
    padding: 2px;
    border-radius: 50%;
}
button.color-bubble-btn:hover {
    background-color: rgba(255, 255, 255, 0.1);
}
"##;

// Helper to apply CSS
fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(INSTALLER_CSS);
    if let Some(display) = gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SVGs for Icons & Graphics
// ─────────────────────────────────────────────────────────────────────────────

const CHERRY_LOGO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200" width="160" height="160">
  <path d="M110 30 C125 15, 150 15, 165 30 C150 45, 125 40, 110 30 Z" fill="#111111"/>
  <path d="M98 35 Q105 75 70 110" stroke="#111111" stroke-width="7" fill="none" stroke-linecap="round"/>
  <path d="M102 35 Q115 80 135 110" stroke="#111111" stroke-width="7" fill="none" stroke-linecap="round"/>
  <circle cx="65" cy="135" r="38" fill="#111111"/>
  <path d="M135 125 C160 100, 180 130, 165 155 C150 180, 115 170, 120 140 C122 130, 128 125, 135 125 Z" fill="#111111"/>
</svg>"##;

const DISC_ICON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="100" height="100">
  <circle cx="50" cy="50" r="42" stroke="#ffffff" stroke-width="5" fill="none"/>
  <circle cx="50" cy="50" r="14" stroke="#ffffff" stroke-width="5" fill="none"/>
</svg>"##;

const INSTALL_ICON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="100" height="100">
  <rect x="20" y="50" width="60" height="36" rx="4" fill="#ffffff"/>
  <rect x="62" y="68" width="10" height="8" fill="#2d2d2d"/>
  <path d="M50 16 L50 44 M34 32 L50 46 L66 32" stroke="#ffffff" stroke-width="8" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
</svg>"##;

// Theme previews
const LIGHT_PREVIEW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 160 100" width="150" height="95">
  <rect width="160" height="100" rx="8" fill="#2e4a32"/>
  <rect x="25" y="25" width="110" height="60" rx="4" fill="#e6e6e6" stroke="#cccccc" stroke-width="1"/>
  <circle cx="32" cy="31" r="2" fill="#ff5f56"/>
  <circle cx="38" cy="31" r="2" fill="#ffbd2e"/>
  <circle cx="44" cy="31" r="2" fill="#27c93f"/>
</svg>"##;

const DARK_PREVIEW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 160 100" width="150" height="95">
  <rect width="160" height="100" rx="8" fill="#2e4a32"/>
  <rect x="25" y="25" width="110" height="60" rx="4" fill="#2b2b2b" stroke="#444444" stroke-width="1"/>
  <circle cx="32" cy="31" r="2" fill="#ff5f56"/>
  <circle cx="38" cy="31" r="2" fill="#ffbd2e"/>
  <circle cx="44" cy="31" r="2" fill="#27c93f"/>
</svg>"##;

const AUTO_PREVIEW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 160 100" width="150" height="95">
  <rect width="160" height="100" rx="8" fill="#2e4a32"/>
  <path d="M25 25 L135 25 L135 85 Z" fill="#e6e6e6"/>
  <path d="M25 25 L135 85 L25 85 Z" fill="#2b2b2b"/>
  <rect x="25" y="25" width="110" height="60" rx="4" fill="none" stroke="#666666" stroke-width="1"/>
  <circle cx="32" cy="31" r="2" fill="#ff5f56"/>
  <circle cx="38" cy="31" r="2" fill="#ffbd2e"/>
  <circle cx="44" cy="31" r="2" fill="#27c93f"/>
</svg>"##;

const COLOR_WHEEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 40 40" width="32" height="32">
  <defs>
    <linearGradient id="g" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#ff0000"/>
      <stop offset="20%" stop-color="#ffff00"/>
      <stop offset="40%" stop-color="#00ff00"/>
      <stop offset="60%" stop-color="#00ffff"/>
      <stop offset="80%" stop-color="#0000ff"/>
      <stop offset="100%" stop-color="#ff00ff"/>
    </linearGradient>
  </defs>
  <circle cx="20" cy="20" r="16" fill="url(#g)"/>
</svg>"##;

fn create_image_from_svg(svg: &str) -> Image {
    let bytes = glib::Bytes::from(svg.as_bytes());
    let stream = gio::MemoryInputStream::from_bytes(&bytes);
    if let Ok(pixbuf) = gdk_pixbuf::Pixbuf::from_stream(&stream, gio::Cancellable::NONE) {
        let texture = gdk::Texture::for_pixbuf(&pixbuf);
        Image::from_paintable(Some(&texture))
    } else {
        Image::new()
    }
}

fn create_color_circle_svg(color: &str, selected: bool) -> String {
    if selected {
        format!(
            r##"<svg width="32" height="32" viewBox="0 0 32 32"><circle cx="16" cy="16" r="14" fill="none" stroke="{}" stroke-width="2.5"/><circle cx="16" cy="16" r="9" fill="{}"/></svg>"##,
            color, color
        )
    } else {
        format!(
            r##"<svg width="32" height="32" viewBox="0 0 32 32"><circle cx="16" cy="16" r="12" fill="{}"/></svg>"##,
            color
        )
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Builder for Screen 1: Welcome Screen
// ─────────────────────────────────────────────────────────────────────────────

fn build_welcome_screen(stack: &Stack) -> GtkBox {
    let page = GtkBox::new(Orientation::Vertical, 0);
    page.add_css_class("window-light");
    page.set_vexpand(true);

    // Center Content Box
    let center_box = GtkBox::new(Orientation::Vertical, 12);
    center_box.set_valign(gtk4::Align::Center);
    center_box.set_halign(gtk4::Align::Center);
    center_box.set_vexpand(true);

    let logo = create_image_from_svg(CHERRY_LOGO_SVG);
    logo.set_margin_bottom(10);

    let title = Label::new(None);
    title.set_markup("<span font='36' weight='bold' foreground='#000000'>ALINIX</span>");

    let subtitle = Label::new(None);
    subtitle.set_markup("<span font='20' weight='bold' foreground='#000000'>WELCOME</span>");

    center_box.append(&logo);
    center_box.append(&title);
    center_box.append(&subtitle);
    page.append(&center_box);

    // Bottom Bar
    let bottom_bar = GtkBox::new(Orientation::Horizontal, 0);
    bottom_bar.set_margin_bottom(24);
    bottom_bar.set_margin_end(28);

    let next_btn = Button::with_label("NEXT");
    next_btn.add_css_class("btn-next-blue");
    next_btn.set_halign(gtk4::Align::End);
    next_btn.set_hexpand(true);

    let stack_clone = stack.clone();
    next_btn.connect_clicked(move |_| {
        stack_clone.set_visible_child_name("screen_language");
    });

    bottom_bar.append(&next_btn);
    page.append(&bottom_bar);

    page
}

// ─────────────────────────────────────────────────────────────────────────────
// Builder for Screen 2: Language & Mode Selection
// ─────────────────────────────────────────────────────────────────────────────

fn build_language_screen(stack: &Stack) -> GtkBox {
    let page = GtkBox::new(Orientation::Vertical, 0);
    page.add_css_class("window-dark");
    page.set_vexpand(true);

    // Main Layout (Sidebar + Center area)
    let main_h = GtkBox::new(Orientation::Horizontal, 32);
    main_h.set_margin_top(30);
    main_h.set_margin_bottom(30);
    main_h.set_margin_start(36);
    main_h.set_margin_end(36);
    main_h.set_vexpand(true);

    // Sidebar (Language Selector)
    let sidebar = GtkBox::new(Orientation::Vertical, 6);
    sidebar.add_css_class("sidebar-box");
    sidebar.set_size_request(210, -1);
    sidebar.set_valign(gtk4::Align::Start);

    let languages = ["ENGLISH (US)", "PORTUGUÊS (BRASIL)", "PORTUGUÊS (PORTUGAL)", "ESPANHOL", "FRANÇAIS"];
    let lang_buttons: Rc<RefCell<Vec<Button>>> = Rc::new(RefCell::new(Vec::new()));

    for (i, lang) in languages.iter().enumerate() {
        let btn = Button::with_label(lang);
        if i == 0 {
            btn.add_css_class("sidebar-btn-active");
        } else {
            btn.add_css_class("sidebar-btn");
        }
        btn.set_halign(gtk4::Align::Fill);

        let lang_btns_clone = lang_buttons.clone();
        let btn_clone = btn.clone();
        btn.connect_clicked(move |_| {
            for b in lang_btns_clone.borrow().iter() {
                b.remove_css_class("sidebar-btn-active");
                b.add_css_class("sidebar-btn");
            }
            btn_clone.remove_css_class("sidebar-btn");
            btn_clone.add_css_class("sidebar-btn-active");
        });

        lang_buttons.borrow_mut().push(btn.clone());
        sidebar.append(&btn);
    }
    main_h.append(&sidebar);

    // Mode Option Cards
    let cards_h = GtkBox::new(Orientation::Horizontal, 48);
    cards_h.set_halign(gtk4::Align::Center);
    cards_h.set_valign(gtk4::Align::Center);
    cards_h.set_hexpand(true);

    // Card 1: Live Mode
    let card1 = GtkBox::new(Orientation::Vertical, 8);
    card1.set_halign(gtk4::Align::Center);
    let disc_img = create_image_from_svg(DISC_ICON_SVG);
    disc_img.set_margin_bottom(12);

    let desc1 = Label::new(Some("Experimentar o Alinix sem instalar."));
    desc1.add_css_class("card-subtitle");

    let btn_live = Button::with_label("Live Mode");
    btn_live.add_css_class("btn-mode");

    let stack_clone1 = stack.clone();
    btn_live.connect_clicked(move |_| {
        stack_clone1.set_visible_child_name("screen_customize");
    });

    card1.append(&disc_img);
    card1.append(&desc1);
    card1.append(&btn_live);

    // Card 2: Install Alinix
    let card2 = GtkBox::new(Orientation::Vertical, 8);
    card2.set_halign(gtk4::Align::Center);
    let install_img = create_image_from_svg(INSTALL_ICON_SVG);
    install_img.set_margin_bottom(12);

    let desc2 = Label::new(Some("Instalar permanentemente no computador."));
    desc2.add_css_class("card-subtitle");

    let btn_install = Button::with_label("Install Alinix");
    btn_install.add_css_class("btn-mode");

    let stack_clone2 = stack.clone();
    btn_install.connect_clicked(move |_| {
        stack_clone2.set_visible_child_name("screen_customize");
    });

    card2.append(&install_img);
    card2.append(&desc2);
    card2.append(&btn_install);

    cards_h.append(&card1);
    cards_h.append(&card2);

    main_h.append(&cards_h);
    page.append(&main_h);

    page
}

// ─────────────────────────────────────────────────────────────────────────────
// Builder for Screen 3: Theme & Accent Color Customization
// ─────────────────────────────────────────────────────────────────────────────

fn build_customize_screen(stack: &Stack) -> GtkBox {
    let page = GtkBox::new(Orientation::Vertical, 0);
    page.add_css_class("window-dark");
    page.set_vexpand(true);

    // Main Box
    let center_box = GtkBox::new(Orientation::Vertical, 24);
    center_box.set_valign(gtk4::Align::Center);
    center_box.set_halign(gtk4::Align::Center);
    center_box.set_vexpand(true);

    // Theme Previews (Light, DARK, AUTO)
    let themes_h = GtkBox::new(Orientation::Horizontal, 24);
    let theme_btns: Rc<RefCell<Vec<Button>>> = Rc::new(RefCell::new(Vec::new()));

    let theme_data = [
        ("Ligth", LIGHT_PREVIEW_SVG, false),
        ("DARK", DARK_PREVIEW_SVG, true),
        ("AUTO", AUTO_PREVIEW_SVG, false),
    ];

    for (name, svg_src, is_selected) in theme_data.iter() {
        let box_v = GtkBox::new(Orientation::Vertical, 6);
        let img = create_image_from_svg(svg_src);
        let lbl = Label::new(Some(*name));
        lbl.add_css_class("card-label");
        box_v.append(&img);
        box_v.append(&lbl);

        let theme_btn = Button::new();
        theme_btn.set_child(Some(&box_v));
        if *is_selected {
            theme_btn.add_css_class("theme-card-btn-selected");
        } else {
            theme_btn.add_css_class("theme-card-btn");
        }

        let theme_btns_clone = theme_btns.clone();
        let btn_clone = theme_btn.clone();
        theme_btn.connect_clicked(move |_| {
            for b in theme_btns_clone.borrow().iter() {
                b.remove_css_class("theme-card-btn-selected");
                b.add_css_class("theme-card-btn");
            }
            btn_clone.remove_css_class("theme-card-btn");
            btn_clone.add_css_class("theme-card-btn-selected");
        });

        theme_btns.borrow_mut().push(theme_btn.clone());
        themes_h.append(&theme_btn);
    }
    center_box.append(&themes_h);

    // Color Swatches Row
    let colors_h = GtkBox::new(Orientation::Horizontal, 14);
    colors_h.set_margin_top(10);
    colors_h.set_halign(gtk4::Align::Center);

    let colors = [
        "#007aff", "#00a896", "#4caf50", "#f2c94c", "#f2994a",
        "#eb5757", "#e056fd", "#8f52a1", "#607d8b"
    ];

    let color_btns: Rc<RefCell<Vec<(Button, String)>>> = Rc::new(RefCell::new(Vec::new()));

    for (i, col) in colors.iter().enumerate() {
        let is_selected = i == 7; // Purple selected
        let svg = create_color_circle_svg(col, is_selected);
        let img = create_image_from_svg(&svg);

        let col_btn = Button::new();
        col_btn.set_child(Some(&img));
        col_btn.add_css_class("color-bubble-btn");

        let color_btns_clone = color_btns.clone();
        let color_hex = col.to_string();
        let col_btn_clone = col_btn.clone();

        col_btn.connect_clicked(move |_| {
            for (b, hex) in color_btns_clone.borrow().iter() {
                let unselected_svg = create_color_circle_svg(hex, false);
                b.set_child(Some(&create_image_from_svg(&unselected_svg)));
            }
            let selected_svg = create_color_circle_svg(&color_hex, true);
            col_btn_clone.set_child(Some(&create_image_from_svg(&selected_svg)));
        });

        color_btns.borrow_mut().push((col_btn.clone(), col.to_string()));
        colors_h.append(&col_btn);
    }

    // Add Rainbow Color Wheel
    let wheel_img = create_image_from_svg(COLOR_WHEEL_SVG);
    let wheel_btn = Button::new();
    wheel_btn.set_child(Some(&wheel_img));
    wheel_btn.add_css_class("color-bubble-btn");
    colors_h.append(&wheel_btn);

    center_box.append(&colors_h);
    page.append(&center_box);

    // Bottom Bar
    let bottom_bar = GtkBox::new(Orientation::Horizontal, 0);
    bottom_bar.set_margin_bottom(24);
    bottom_bar.set_margin_end(28);

    let next_btn = Button::with_label("NEXT");
    next_btn.add_css_class("btn-purple");
    next_btn.set_halign(gtk4::Align::End);
    next_btn.set_hexpand(true);

    let stack_clone = stack.clone();
    next_btn.connect_clicked(move |_| {
        stack_clone.set_visible_child_name("screen_partition");
    });

    bottom_bar.append(&next_btn);
    page.append(&bottom_bar);

    page
}

// ─────────────────────────────────────────────────────────────────────────────
// Builder for Screen 4: Disk Partitioning
// ─────────────────────────────────────────────────────────────────────────────

fn build_partition_screen(stack: &Stack) -> GtkBox {
    let page = GtkBox::new(Orientation::Vertical, 0);
    page.add_css_class("window-dark");
    page.set_vexpand(true);

    let main_box = GtkBox::new(Orientation::Vertical, 20);
    main_box.set_valign(gtk4::Align::Center);
    main_box.set_halign(gtk4::Align::Start);
    main_box.set_margin_start(120);
    main_box.set_margin_end(120);
    main_box.set_vexpand(true);

    // Disk Selector Row
    let disk_row = GtkBox::new(Orientation::Horizontal, 12);
    disk_row.set_valign(gtk4::Align::Center);
    disk_row.set_margin_bottom(12);

    let disk_lbl = Label::new(Some("💾 Disco de Destino:"));
    disk_lbl.add_css_class("disk-label");

    let dropdown = DropDown::from_strings(&[
        "NVMe SSD 512 GB (/dev/nvme0n1)",
        "SATA SSD 1 TB (/dev/sda)",
    ]);

    disk_row.append(&disk_lbl);
    disk_row.append(&dropdown);
    main_box.append(&disk_row);

    // Radio Options Box
    let options = [
        (
            "Usar Disco inteiro",
            "Apaga todo o disco selecionado e instala o Alinix de forma automática.",
            false
        ),
        (
            "Usar Disco inteiro + /home separada",
            "Preserva seus arquivos em reinstalações e cria partição de dados dedicada.",
            true
        ),
        (
            "Partição manual",
            "Para usuários avançados criarem, redimensionarem ou editarem partições.",
            false
        ),
    ];

    let radio_rows: Rc<RefCell<Vec<(Button, Image, usize)>>> = Rc::new(RefCell::new(Vec::new()));

    for (idx, (title_text, subtitle_text, is_selected)) in options.iter().enumerate() {
        let row_content = GtkBox::new(Orientation::Horizontal, 16);
        row_content.set_valign(gtk4::Align::Center);

        let svg_selected = r##"<svg width="24" height="24" viewBox="0 0 24 24"><circle cx="12" cy="12" r="10" fill="#8f52a1"/><circle cx="12" cy="12" r="4" fill="#ffffff"/></svg>"##;
        let svg_unselected = r##"<svg width="24" height="24" viewBox="0 0 24 24"><circle cx="12" cy="12" r="10" fill="none" stroke="#ffffff" stroke-width="2"/></svg>"##;

        let current_svg = if *is_selected { svg_selected } else { svg_unselected };
        let radio_img = create_image_from_svg(current_svg);

        let text_vbox = GtkBox::new(Orientation::Vertical, 2);
        let title_lbl = Label::new(Some(*title_text));
        title_lbl.add_css_class("radio-title");
        title_lbl.set_halign(gtk4::Align::Start);

        let sub_lbl = Label::new(Some(*subtitle_text));
        sub_lbl.add_css_class("radio-subtitle");
        sub_lbl.set_halign(gtk4::Align::Start);

        text_vbox.append(&title_lbl);
        text_vbox.append(&sub_lbl);

        row_content.append(&radio_img);
        row_content.append(&text_vbox);

        let row_btn = Button::new();
        row_btn.set_child(Some(&row_content));
        row_btn.add_css_class("radio-row-btn");
        row_btn.set_halign(gtk4::Align::Fill);

        let radio_rows_clone = radio_rows.clone();
        let target_idx = idx;

        row_btn.connect_clicked(move |_| {
            for (_btn_item, img_item, item_idx) in radio_rows_clone.borrow().iter() {
                let svg = if *item_idx == target_idx {
                    r##"<svg width="24" height="24" viewBox="0 0 24 24"><circle cx="12" cy="12" r="10" fill="#8f52a1"/><circle cx="12" cy="12" r="4" fill="#ffffff"/></svg>"##
                } else {
                    r##"<svg width="24" height="24" viewBox="0 0 24 24"><circle cx="12" cy="12" r="10" fill="none" stroke="#ffffff" stroke-width="2"/></svg>"##
                };
                img_item.set_paintable(create_image_from_svg(svg).paintable().as_ref());
            }
        });

        radio_rows.borrow_mut().push((row_btn.clone(), radio_img, idx));
        main_box.append(&row_btn);
    }
    page.append(&main_box);

    // Bottom Navigation Bar
    let bottom_bar = GtkBox::new(Orientation::Horizontal, 0);
    bottom_bar.set_margin_bottom(24);
    bottom_bar.set_margin_start(120);
    bottom_bar.set_margin_end(28);

    let back_btn = Button::with_label("Voltar");
    back_btn.add_css_class("btn-back");
    let stack_clone_back = stack.clone();
    back_btn.connect_clicked(move |_| {
        stack_clone_back.set_visible_child_name("screen_customize");
    });

    let proceed_btn = Button::with_label("Prosseguir");
    proceed_btn.add_css_class("btn-purple");
    proceed_btn.set_halign(gtk4::Align::End);
    proceed_btn.set_hexpand(true);

    let stack_clone_proceed = stack.clone();
    proceed_btn.connect_clicked(move |_| {
        stack_clone_proceed.set_visible_child_name("screen_welcome");
    });

    bottom_bar.append(&back_btn);
    bottom_bar.append(&proceed_btn);
    page.append(&bottom_bar);

    page
}

// ─────────────────────────────────────────────────────────────────────────────
// Native GTK Module Functions Exposing GTK to Aly Language
// ─────────────────────────────────────────────────────────────────────────────

pub fn gtk_installer_app(_x: String) -> Box<dyn Validator> {
    let app = Application::builder()
        .application_id("com.alinix.installer")
        .build();

    app.connect_activate(|app| {
        load_css();

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Alinix Installer")
            .default_width(780)
            .default_height(480)
            .resizable(false)
            .build();

        // Native GTK HeaderBar with OS Title Buttons (real traffic lights)
        let header_bar = HeaderBar::new();
        header_bar.set_show_title_buttons(true);
        header_bar.add_css_class("installer-header-light");

        let title_label = Label::new(Some("Alinix Installer"));
        title_label.add_css_class("header-title-light");
        header_bar.set_title_widget(Some(&title_label));

        window.set_titlebar(Some(&header_bar));
        window.add_css_class("installer-window");
        window.add_css_class("window-light");

        let stack = Stack::new();
        stack.set_transition_type(StackTransitionType::SlideLeftRight);
        stack.set_transition_duration(300);

        let welcome = build_welcome_screen(&stack);
        let language = build_language_screen(&stack);
        let customize = build_customize_screen(&stack);
        let partition = build_partition_screen(&stack);

        stack.add_named(&welcome, Some("screen_welcome"));
        stack.add_named(&language, Some("screen_language"));
        stack.add_named(&customize, Some("screen_customize"));
        stack.add_named(&partition, Some("screen_partition"));

        // Synchronize HeaderBar & Window Theme dynamically when page changes
        let win_clone = window.clone();
        let header_clone = header_bar.clone();
        let title_clone = title_label.clone();

        stack.connect_visible_child_name_notify(move |stack| {
            if let Some(child_name) = stack.visible_child_name() {
                if child_name.as_str() == "screen_welcome" {
                    win_clone.remove_css_class("window-dark");
                    win_clone.add_css_class("window-light");

                    header_clone.remove_css_class("installer-header-dark");
                    header_clone.add_css_class("installer-header-light");

                    title_clone.remove_css_class("header-title-dark");
                    title_clone.add_css_class("header-title-light");
                } else {
                    win_clone.remove_css_class("window-light");
                    win_clone.add_css_class("window-dark");

                    header_clone.remove_css_class("installer-header-light");
                    header_clone.add_css_class("installer-header-dark");

                    title_clone.remove_css_class("header-title-light");
                    title_clone.add_css_class("header-title-dark");
                }
            }
        });

        window.set_child(Some(&stack));
        window.present();
    });

    let empty_args: Vec<String> = vec![];
    app.run_with_args(&empty_args);

    Box::new(put_quoted_str("OK".to_string()))
}
// ─────────────────────────────────────────────────────────────────────────────
// Generic GTK Widget System
// ─────────────────────────────────────────────────────────────────────────────


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
    Separator(gtk4::Separator),
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
        provider.load_from_data(&css_content);
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
    let already_ran = GTK_RUN.with(|r| *r.borrow());
    if already_ran {
        return Box::new(put_quoted_str("OK".to_string()));
    }
    GTK_RUN.with(|r| *r.borrow_mut() = true);

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

    GTK_RUN.with(|r| *r.borrow_mut() = false);

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
    let id = next_gtk_id();
    let shared = Rc::new(RefCell::new(GtkWidget::Radio(radio)));
    GTK_WIDGETS.with(|w| w.borrow_mut().insert(id.clone(), shared));
    ValueData::String(id)
}

pub fn gtk_new_slider(args: Vec<ValueData>) -> ValueData {
    let min = args.get(0).and_then(|v| match v { ValueData::Int(i) => Some(*i), _ => None }).unwrap_or(0);
    let max = args.get(1).and_then(|v| match v { ValueData::Int(i) => Some(*i), _ => None }).unwrap_or(100);
    let val = args.get(2).and_then(|v| match v { ValueData::Int(i) => Some(*i), _ => None }).unwrap_or(50);

    let adj = gtk4::Adjustment::new(val as f64, min as f64, max as f64, 1.0, 10.0, 0.0);
    let scale = Scale::new(Orientation::Horizontal, Some(&adj));
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
    let dd = DropDown::from_strings(&item_refs);
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

pub fn gtk_new_separator(_args: Vec<ValueData>) -> ValueData {
    let sep = gtk4::Separator::new(Orientation::Horizontal);
    let id = next_gtk_id();
    let shared = Rc::new(RefCell::new(GtkWidget::Separator(sep)));
    GTK_WIDGETS.with(|w| w.borrow_mut().insert(id.clone(), shared));
    ValueData::String(id)
}
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
    Box::new(gtk_new_progressbar(vec![ValueData::Float(val as f32)]))
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
            GtkWidget::Input(e) => e.set_text(&value),
            GtkWidget::TextArea(tv) => {
                let buf = tv.buffer();
                buf.set_text(&value);
            }
            GtkWidget::PasswordField(e) => e.set_text(&value),
            GtkWidget::Checkbox(c) => c.set_label(Some(&value)),
            GtkWidget::Radio(r) => r.set_label(Some(&value)),
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
            GtkWidget::Button(b) => Box::new(put_quoted_str(b.label().map(|s| s.to_string()).unwrap_or_default())),
            GtkWidget::Label(l) => Box::new(put_quoted_str(l.label().to_string())),
            GtkWidget::Input(e) => Box::new(put_quoted_str(e.text().to_string())),
            GtkWidget::TextArea(tv) => {
                let buf = tv.buffer();
                let start = buf.start_iter();
                let end = buf.end_iter();
                let text = buf.text(&start, &end, true);
                Box::new(put_quoted_str(text.to_string()))
            }
            GtkWidget::PasswordField(e) => Box::new(put_quoted_str(e.text().to_string())),
            GtkWidget::Checkbox(c) => Box::new(put_quoted_str(c.label().map(|s| s.to_string()).unwrap_or_default())),
            GtkWidget::Radio(r) => Box::new(put_quoted_str(r.label().map(|s| s.to_string()).unwrap_or_default())),
            _ => Box::new(put_quoted_str("None".to_owned())),
        },
        "checked" => match &*borrow {
            GtkWidget::Checkbox(c) => Box::new(ValueData::Bool(c.is_active())),
            GtkWidget::Radio(r) => Box::new(ValueData::Bool(r.is_active())),
            _ => Box::new(put_quoted_str("None".to_owned())),
        },
        "selected" => match &*borrow {
            GtkWidget::Dropdown(d) => {
                let idx = d.selected();
                Box::new(put_quoted_str(format!("{}", idx)))
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
