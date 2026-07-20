// GTK4 Native Module for Aly Language
// Provides GTK4 GUI capabilities and custom screen rendering for Aly apps.

use gtk4::prelude::*;
use gtk4::{
    gio, gdk, glib, gdk_pixbuf,
    Application, ApplicationWindow, Box as GtkBox, Button, CssProvider,
    HeaderBar, Image, Label, Orientation, Stack, StackTransitionType
};

use std::cell::RefCell;\nuse std::collections::HashMap;\nuse std::rc::Rc;\nuse gtk4::{CheckButton, DropDown, Entry, Notebook, Orientation, ProgressBar, Scale, TextView};\nuse crate::native::types::{Validator, ValueData};\nuse crate::native::std::{arg, split_args};\nuse crate::validators::str::put_quoted_str;

use crate::native::types::Validator;
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
    background-color: #2d2d2d;
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
    background-color: #9b51e0;
    color: #ffffff;
    font-weight: 600;
    font-size: 13px;
    border-radius: 18px;
    padding: 8px 32px;
    border: none;
    box-shadow: 0 2px 6px rgba(155, 81, 224, 0.4);
}
button.btn-purple:hover {
    background-color: #8842ca;
}

button.btn-mode {
    background-color: #4c5678;
    color: #ffffff;
    font-weight: 600;
    font-size: 14px;
    border-radius: 10px;
    padding: 10px 24px;
    border: none;
}
button.btn-mode:hover {
    background-color: #5d6992;
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

/* Sidebar */
.sidebar-box {
    background-color: #353535;
    border-radius: 12px;
    padding: 8px;
}

.sidebar-item {
    padding: 10px 16px;
    border-radius: 8px;
    color: #cccccc;
    font-size: 13px;
    font-weight: 600;
}

.sidebar-item-active {
    background-color: #9b51e0;
    color: #ffffff;
    border-radius: 8px;
    padding: 10px 16px;
    font-size: 13px;
    font-weight: bold;
}

/* Theme cards */
.theme-card {
    background-color: #3a3a3a;
    border-radius: 12px;
    padding: 6px;
    border: 2px solid transparent;
}

.theme-card-selected {
    border: 2px solid #ffffff;
}

.card-label {
    font-weight: bold;
    font-size: 14px;
    color: #ffffff;
    margin-top: 8px;
}

/* Radio item */
.radio-text {
    font-size: 16px;
    color: #ffffff;
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
    subtitle.set_markup("<span font='22' weight='bold' foreground='#000000'>WELCOME</span>");

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
    let main_h = GtkBox::new(Orientation::Horizontal, 24);
    main_h.set_margin_top(30);
    main_h.set_margin_bottom(30);
    main_h.set_margin_start(30);
    main_h.set_margin_end(30);
    main_h.set_vexpand(true);

    // Sidebar
    let sidebar = GtkBox::new(Orientation::Vertical, 6);
    sidebar.add_css_class("sidebar-box");
    sidebar.set_size_request(200, -1);
    sidebar.set_valign(gtk4::Align::Start);

    let languages = ["ENGLISH (US)", "PORTUGUÊS (BRASIL)", "PORTUGUÊS (PORTUGAL)", "ESPANHOL"];
    for (i, lang) in languages.iter().enumerate() {
        let lbl = Label::new(Some(lang));
        if i == 0 {
            lbl.add_css_class("sidebar-item-active");
        } else {
            lbl.add_css_class("sidebar-item");
        }
        lbl.set_halign(gtk4::Align::Start);
        sidebar.append(&lbl);
    }
    main_h.append(&sidebar);

    // Mode Option Cards
    let cards_h = GtkBox::new(Orientation::Horizontal, 40);
    cards_h.set_halign(gtk4::Align::Center);
    cards_h.set_valign(gtk4::Align::Center);
    cards_h.set_hexpand(true);

    // Card 1: Live Mode
    let card1 = GtkBox::new(Orientation::Vertical, 16);
    card1.set_halign(gtk4::Align::Center);
    let disc_img = create_image_from_svg(DISC_ICON_SVG);
    disc_img.set_margin_bottom(10);
    let btn_live = Button::with_label("Live Mode");
    btn_live.add_css_class("btn-mode");

    let stack_clone1 = stack.clone();
    btn_live.connect_clicked(move |_| {
        stack_clone1.set_visible_child_name("screen_customize");
    });

    card1.append(&disc_img);
    card1.append(&btn_live);

    // Card 2: Install Alinix
    let card2 = GtkBox::new(Orientation::Vertical, 16);
    card2.set_halign(gtk4::Align::Center);
    let install_img = create_image_from_svg(INSTALL_ICON_SVG);
    install_img.set_margin_bottom(10);
    let btn_install = Button::with_label("Install Alinix");
    btn_install.add_css_class("btn-mode");

    let stack_clone2 = stack.clone();
    btn_install.connect_clicked(move |_| {
        stack_clone2.set_visible_child_name("screen_customize");
    });

    card2.append(&install_img);
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

    // 1. Light
    let box1 = GtkBox::new(Orientation::Vertical, 6);
    box1.add_css_class("theme-card");
    let img1 = create_image_from_svg(LIGHT_PREVIEW_SVG);
    let lbl1 = Label::new(Some("Ligth"));
    lbl1.add_css_class("card-label");
    box1.append(&img1);
    box1.append(&lbl1);

    // 2. DARK (Selected with white border)
    let box2 = GtkBox::new(Orientation::Vertical, 6);
    box2.add_css_class("theme-card");
    box2.add_css_class("theme-card-selected");
    let img2 = create_image_from_svg(DARK_PREVIEW_SVG);
    let lbl2 = Label::new(Some("DARK"));
    lbl2.add_css_class("card-label");
    box2.append(&img2);
    box2.append(&lbl2);

    // 3. AUTO
    let box3 = GtkBox::new(Orientation::Vertical, 6);
    box3.add_css_class("theme-card");
    let img3 = create_image_from_svg(AUTO_PREVIEW_SVG);
    let lbl3 = Label::new(Some("AUTO"));
    lbl3.add_css_class("card-label");
    box3.append(&img3);
    box3.append(&lbl3);

    themes_h.append(&box1);
    themes_h.append(&box2);
    themes_h.append(&box3);
    center_box.append(&themes_h);

    // Color Swatches Row
    let colors_h = GtkBox::new(Orientation::Horizontal, 14);
    colors_h.set_margin_top(10);
    colors_h.set_halign(gtk4::Align::Center);

    let colors = [
        "#007aff", "#00a896", "#4caf50", "#f2c94c", "#f2994a",
        "#eb5757", "#e056fd", "#9b51e0", "#607d8b"
    ];

    for (i, col) in colors.iter().enumerate() {
        let is_selected = i == 7; // Purple selected
        let svg = create_color_circle_svg(col, is_selected);
        let img = create_image_from_svg(&svg);
        colors_h.append(&img);
    }
    // Add Rainbow Color Wheel
    let wheel_img = create_image_from_svg(COLOR_WHEEL_SVG);
    colors_h.append(&wheel_img);

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

    // Radio Options Box
    let center_box = GtkBox::new(Orientation::Vertical, 28);
    center_box.set_valign(gtk4::Align::Center);
    center_box.set_halign(gtk4::Align::Start);
    center_box.set_margin_start(160);
    center_box.set_vexpand(true);

    let options = [
        ("Usar Disco inteiro", false),
        ("Usar Disco inteiro + /home separada", true),
        ("Partição manual", false),
    ];

    for (text, is_selected) in options.iter() {
        let row = GtkBox::new(Orientation::Horizontal, 16);
        row.set_valign(gtk4::Align::Center);

        let svg = if *is_selected {
            r##"<svg width="24" height="24" viewBox="0 0 24 24"><circle cx="12" cy="12" r="10" fill="#ffffff"/></svg>"##
        } else {
            r##"<svg width="24" height="24" viewBox="0 0 24 24"><circle cx="12" cy="12" r="10" fill="none" stroke="#ffffff" stroke-width="2"/></svg>"##
        };
        let radio_img = create_image_from_svg(svg);
        let lbl = Label::new(Some(text));
        lbl.add_css_class("radio-text");

        row.append(&radio_img);
        row.append(&lbl);
        center_box.append(&row);
    }
    page.append(&center_box);

    // Bottom Navigation Bar
    let bottom_bar = GtkBox::new(Orientation::Horizontal, 0);
    bottom_bar.set_margin_bottom(24);
    bottom_bar.set_margin_start(160);
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
