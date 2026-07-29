use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::gui::{
    self as gui_crate,
    backend::{GuiBackend, BackendType},
    widget::{WidgetId, next_widget_id, PROP_LABEL, PROP_TEXT, PROP_VALUE, PROP_CHECKED, PROP_ENABLED, PROP_VISIBLE},
};

pub type ComponentId = WidgetId;

#[derive(Debug, Clone)]
pub enum GuiEvent {
    Click(ComponentId),
    InputChanged(ComponentId, String),
    WindowClosed,
    Resize(u32, u32),
}

#[cfg(target_os = "macos")]
use objc2::rc::Retained;
#[cfg(target_os = "macos")]
use objc2_foundation::{NSString, NSRect, NSPoint, NSSize};
#[cfg(target_os = "macos")]
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSWindow, NSView, NSButton, NSTextField,
    NSWindowStyleMask, NSBackingStoreType,
};

#[derive(Clone)]
pub enum CocoaWidget {
    #[cfg(target_os = "macos")]
    Window(Retained<NSWindow>, Retained<NSView>),
    #[cfg(target_os = "macos")]
    Button(Retained<NSButton>),
    #[cfg(target_os = "macos")]
    Label(Retained<NSTextField>),
    #[cfg(target_os = "macos")]
    Input(Retained<NSTextField>),
    #[cfg(target_os = "macos")]
    Div(Retained<NSView>),
    #[cfg(target_os = "macos")]
    Generic(Retained<NSView>),

    #[cfg(not(target_os = "macos"))]
    Dummy,
}

thread_local! {
    static COCOA_WIDGETS: RefCell<HashMap<WidgetId, CocoaWidget>> = RefCell::new(HashMap::new());
}

pub struct CocoaBackend;

impl CocoaBackend {
    pub fn init() {
        #[cfg(target_os = "macos")]
        unsafe {
            let app = NSApplication::sharedApplication();
            app.setActivationPolicy(NSApplicationActivationPolicy::Regular);
        }
    }
}

#[cfg(target_os = "macos")]
fn to_nsstring(s: &str) -> Retained<NSString> {
    NSString::from_str(s)
}

impl GuiBackend for CocoaBackend {
    fn backend_type(&self) -> BackendType {
        BackendType::Cocoa
    }

    fn create_window(&mut self, title: &str, width: i32, height: i32) -> WidgetId {
        let id = next_widget_id();
        #[cfg(target_os = "macos")]
        unsafe {
            let rect = NSRect::new(
                NSPoint::new(100.0, 100.0),
                NSSize::new(width as f64, height as f64),
            );
            let style = NSWindowStyleMask::Titled
                | NSWindowStyleMask::Closable
                | NSWindowStyleMask::Miniaturizable
                | NSWindowStyleMask::Resizable;
            let window = NSWindow::initWithContentRect_styleMask_backing_defer(
                NSWindow::alloc(),
                rect,
                style,
                NSBackingStoreType::Buffered,
                false,
            );
            window.setTitle(&to_nsstring(title));
            window.center();

            let content_view = window.contentView().expect("Window should have a content view");
            COCOA_WIDGETS.with(|w| {
                w.borrow_mut().insert(id.clone(), CocoaWidget::Window(window, content_view));
            });
        }
        id
    }

    fn create_button(&mut self, label: &str) -> WidgetId {
        let id = next_widget_id();
        #[cfg(target_os = "macos")]
        unsafe {
            let rect = NSRect::new(NSPoint::new(20.0, 20.0), NSSize::new(120.0, 30.0));
            let button = NSButton::buttonWithTitle_target_action(
                &to_nsstring(label),
                None,
                None,
            );
            button.setFrame(rect);
            COCOA_WIDGETS.with(|w| {
                w.borrow_mut().insert(id.clone(), CocoaWidget::Button(button));
            });
        }
        id
    }

    fn create_label(&mut self, text: &str) -> WidgetId {
        let id = next_widget_id();
        #[cfg(target_os = "macos")]
        unsafe {
            let label = NSTextField::labelWithString(&to_nsstring(text));
            label.setEditable(false);
            label.setBezeled(false);
            label.setDrawsBackground(false);
            COCOA_WIDGETS.with(|w| {
                w.borrow_mut().insert(id.clone(), CocoaWidget::Label(label));
            });
        }
        id
    }

    fn create_div(&mut self, _direction: &str) -> WidgetId {
        let id = next_widget_id();
        #[cfg(target_os = "macos")]
        unsafe {
            let view = NSView::alloc();
            COCOA_WIDGETS.with(|w| {
                w.borrow_mut().insert(id.clone(), CocoaWidget::Div(view));
            });
        }
        id
    }

    fn create_input(&mut self, placeholder: &str) -> WidgetId {
        let id = next_widget_id();
        #[cfg(target_os = "macos")]
        unsafe {
            let rect = NSRect::new(NSPoint::new(20.0, 60.0), NSSize::new(200.0, 24.0));
            let input = NSTextField::initWithFrame(NSTextField::alloc(), rect);
            input.setPlaceholderString(Some(&to_nsstring(placeholder)));
            COCOA_WIDGETS.with(|w| {
                w.borrow_mut().insert(id.clone(), CocoaWidget::Input(input));
            });
        }
        id
    }

    fn create_textarea(&mut self) -> WidgetId {
        self.create_input("TextArea")
    }

    fn create_password(&mut self) -> WidgetId {
        self.create_input("Password")
    }

    fn create_checkbox(&mut self, label: &str) -> WidgetId {
        self.create_button(label)
    }

    fn create_radio(&mut self, label: &str) -> WidgetId {
        self.create_button(label)
    }

    fn create_slider(&mut self, _min: f64, _max: f64, _val: f64) -> WidgetId {
        self.create_div("horizontal")
    }

    fn create_progressbar(&mut self, _val: f64) -> WidgetId {
        self.create_div("horizontal")
    }

    fn create_dropdown(&mut self, _items: &[String]) -> WidgetId {
        self.create_div("horizontal")
    }

    fn create_separator(&mut self) -> WidgetId {
        self.create_div("horizontal")
    }

    fn create_spinner(&mut self, _min: f64, _max: f64, _step: f64, _val: f64) -> WidgetId {
        self.create_div("horizontal")
    }

    fn create_container(&mut self, _width: i32, _height: i32, _direction: &str) -> WidgetId {
        self.create_div("vertical")
    }

    fn insert_widget(&mut self, parent: &WidgetId, child: &WidgetId) {
        #[cfg(target_os = "macos")]
        unsafe {
            let parent_widget = COCOA_WIDGETS.with(|w| w.borrow().get(parent).cloned());
            let child_widget = COCOA_WIDGETS.with(|w| w.borrow().get(child).cloned());

            if let (Some(CocoaWidget::Window(_, content_view)), Some(child_w)) = (parent_widget.clone(), child_widget.clone()) {
                match child_w {
                    CocoaWidget::Button(b) => content_view.addSubview(&b),
                    CocoaWidget::Label(l) => content_view.addSubview(&l),
                    CocoaWidget::Input(i) => content_view.addSubview(&i),
                    CocoaWidget::Div(v) => content_view.addSubview(&v),
                    _ => {}
                }
            } else if let (Some(CocoaWidget::Div(p_view)), Some(child_w)) = (parent_widget, child_widget) {
                match child_w {
                    CocoaWidget::Button(b) => p_view.addSubview(&b),
                    CocoaWidget::Label(l) => p_view.addSubview(&l),
                    CocoaWidget::Input(i) => p_view.addSubview(&i),
                    CocoaWidget::Div(v) => p_view.addSubview(&v),
                    _ => {}
                }
            }
        }
    }

    fn set_property(&mut self, id: &WidgetId, prop: &str, value: &str) {
        #[cfg(target_os = "macos")]
        unsafe {
            let widget_opt = COCOA_WIDGETS.with(|w| w.borrow().get(id).cloned());
            if let Some(widget) = widget_opt {
                let ns_val = to_nsstring(value);
                match prop {
                    PROP_LABEL | PROP_TEXT | PROP_VALUE => {
                        match widget {
                            CocoaWidget::Button(b) => b.setTitle(&ns_val),
                            CocoaWidget::Label(l) => l.setStringValue(&ns_val),
                            CocoaWidget::Input(i) => i.setStringValue(&ns_val),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn get_property(&mut self, id: &WidgetId, prop: &str) -> String {
        #[cfg(target_os = "macos")]
        unsafe {
            let widget_opt = COCOA_WIDGETS.with(|w| w.borrow().get(id).cloned());
            if let Some(widget) = widget_opt {
                match prop {
                    PROP_LABEL | PROP_TEXT | PROP_VALUE => {
                        match widget {
                            CocoaWidget::Button(b) => return b.title().to_string(),
                            CocoaWidget::Label(l) => return l.stringValue().to_string(),
                            CocoaWidget::Input(i) => return i.stringValue().to_string(),
                            _ => {}
                        }
                    }
                    "id" => return id.clone(),
                    _ => {}
                }
            }
        }
        "None".to_owned()
    }

    fn on_event(&mut self, id: &WidgetId, event: &str, callback: String) {
        gui_crate::event::set_callback(id, event, callback);
    }

    // System tray
    fn set_tray_icon(&mut self, _window_id: &WidgetId, _icon_path: &str, _tooltip: &str) -> bool {
        #[cfg(target_os = "macos")]
        {
            // macOS uses NSStatusItem for system tray
            unsafe {
                use objc2_app_kit::NSStatusBar;
                use objc2_foundation::NSString;
                
                let status_bar = NSStatusBar::systemStatusBar();
                let status_item = status_bar.statusItemWithLength(-1.0); // NSVariableStatusItemLength
                if let Some(button) = status_item.button() {
                    if let Some(icon) = objc2_app_kit::NSImage::imageNamed(&NSString::from_str("NSApplicationIcon")) {
                        button.setImage(Some(&icon));
                    }
                    button.setToolTip(Some(&NSString::from_str("Aly App")));
                    true
                } else {
                    false
                }
            }
        }
        #[cfg(not(target_os = "macos"))]
        { false }
    }

    fn show_notification(&mut self, title: &str, body: &str, _icon: Option<&str>) {
        #[cfg(target_os = "macos")]
        {
            use objc2_app_kit::NSUserNotification;
            use objc2_foundation::NSString;
            
            unsafe {
                let notification = NSUserNotification::alloc().init();
                notification.setTitle(&NSString::from_str(title));
                notification.setInformativeText(&NSString::from_str(body));
                notification.setSoundName(None);
                
                let center = objc2_app_kit::NSUserNotificationCenter::defaultUserNotificationCenter();
                center.deliverNotification(&notification);
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            eprintln!("Notifications only supported on macOS with Cocoa backend");
        }
    }

    fn clipboard_set(&mut self, text: &str) {
        #[cfg(target_os = "macos")]
        {
            use objc2_app_kit::NSPasteboard;
            use objc2_foundation::NSString;
            
            unsafe {
                let pasteboard = NSPasteboard::generalPasteboard();
                pasteboard.clearContents();
                let str = NSString::from_str(text);
                pasteboard.writeObjects(&[&str]);
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            eprintln!("Clipboard only supported on macOS with Cocoa backend");
        }
    }

    fn clipboard_get(&mut self) -> String {
        #[cfg(target_os = "macos")]
        {
            use objc2_app_kit::NSPasteboard;
            
            unsafe {
                let pasteboard = NSPasteboard::generalPasteboard();
                if let Some(str) = pasteboard.stringForType(&objc2_foundation::NSPasteboardTypeString) {
                    return str.to_string();
                }
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            eprintln!("Clipboard only supported on macOS with Cocoa backend");
        }
        String::new()
    }

    fn drag_drop_init(&mut self, _widget_id: &WidgetId, _data: &str) {
        // macOS drag and drop is handled differently
    }

    fn drag_drop_accept(&mut self, _widget_id: &WidgetId, _types: &[&str]) {
        // macOS drag and drop is handled differently
    }

    // OpenGL
    fn gl_context_create(&mut self, _window_id: &WidgetId) -> bool {
        #[cfg(target_os = "macos")]
        {
            // macOS uses NSOpenGLContext or Metal
            false // Simplified
        }
        #[cfg(not(target_os = "macos"))]
        { false }
    }

    fn gl_make_current(&mut self, _window_id: &WidgetId) -> bool {
        false
    }

    fn gl_swap_buffers(&mut self, _window_id: &WidgetId) {}

    fn gl_get_proc_address(&mut self, _proc_name: &str) -> *const std::ffi::c_void {
        std::ptr::null()
    }

    fn run(&mut self) {
        #[cfg(target_os = "macos")]
        unsafe {
            let app = NSApplication::sharedApplication();
            
            COCOA_WIDGETS.with(|w| {
                for widget in w.borrow().values() {
                    if let CocoaWidget::Window(window, _) = widget {
                        window.makeKeyAndOrderFront(None);
                    }
                }
            });

            app.run();
        }
    }
}
