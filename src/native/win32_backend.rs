#[cfg(target_os = "windows")]
use std::cell::RefCell;
#[cfg(target_os = "windows")]
use std::collections::HashMap;
#[cfg(target_os = "windows")]
use std::rc::Rc;
#[cfg(target_os = "windows")]
use std::sync::mpsc::{self, Sender};
#[cfg(target_os = "windows")]
use std::thread;
#[cfg(target_os = "windows")]
use std::ffi::OsStr;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;

#[cfg(target_os = "windows")]
use windows::{
    core::{w, PCWSTR, PWSTR},
    Win32::Foundation::{
        HWND, LPARAM, LRESULT, RECT, WPARAM, HINSTANCE, HMODULE, HICON, HWND_TOP, SW_SHOW, SW_HIDE,
        WS_OVERLAPPEDWINDOW, WS_CHILD, WS_VISIBLE, WS_TABSTOP, WS_BORDER, WS_VSCROLL, WS_HSCROLL,
        CW_USEDEFAULT, COLOR_WINDOW, WS_EX_CLIENTEDGE, WS_EX_WINDOWEDGE, GWLP_USERDATA,
    },
    Win32::Graphics::Gdi::{
        CreateFontW, CreateSolidBrush, DeleteObject, GetStockObject, SetBkMode, SetTextColor,
        RGB, DEFAULT_GUI_FONT, FW_NORMAL, ANSI_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
        DEFAULT_QUALITY, DEFAULT_PITCH, FF_DONTCARE, HFONT, HBRUSH,
    },
    Win32::System::LibraryLoader::{GetModuleHandleW, LoadImageW, IMAGE_ICON, LR_DEFAULTSIZE},
    Win32::System::Threading::GetCurrentThreadId,
    Win32::UI::WindowsAndMessaging::{
        BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, EndPaint, GetClientRect,
        GetMessageW, LoadCursorW, LoadIconW, PostMessageW, PostQuitMessage, RegisterClassExW,
        SendMessageW, ShowWindow, TranslateMessage, UpdateWindow, WNDCLASSEXW, WM_COMMAND,
        WM_CLOSE, WM_CREATE, WM_DESTROY, WM_PAINT, WM_SETFONT, WM_SETTEXT, WM_GETTEXT,
        WM_GETTEXTLENGTH, EN_CHANGE, BN_CLICKED, BS_PUSHBUTTON, BS_DEFPUSHBUTTON, SS_LEFT,
        SS_CENTER, SS_CENTERIMAGE, ES_AUTOHSCROLL, ES_AUTOVSCROLL, ES_MULTILINE, ES_PASSWORD,
        ES_WANTRETURN, WS_GROUP, WS_EX_TRANSPARENT, GWL_STYLE, GetWindowLongW, SetWindowLongW,
        SetWindowPos, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SWP_FRAMECHANGED, WM_SIZE,
        SIZE_RESTORED, WM_GETMINMAXINFO, MINMAXINFO, WS_CLIPCHILDREN, WS_CLIPSIBLINGS,
        IDI_APPLICATION, IDC_ARROW, CS_HREDRAW, CS_VREDRAW, WS_SYSMENU, WS_MINIMIZEBOX,
        WS_MAXIMIZEBOX, WS_CAPTION, WS_THICKFRAME, WS_OVERLAPPED, GWL_WNDPROC, WNDPROC,
        SetWindowLongPtrW, GetWindowLongPtrW, WM_ERASEBKGND, WM_CTLCOLOREDIT, WM_CTLCOLORSTATIC,
        WM_CTLCOLORBTN, COLOR_BTNFACE,
    },
    Win32::UI::Controls::{
        CreateUpDownControl, UDS_SETBUDDYINT, UDS_ALIGNRIGHT, UDS_AUTOBUDDY, UDS_ARROWKEYS,
    },
    Win32::System::SystemServices::DLL_PROCESS_ATTACH,
};

#[cfg(target_os = "windows")]
use crate::gui::{
    self as gui_crate,
    backend::{GuiBackend, BackendType},
    widget::{WidgetId, next_widget_id, PROP_LABEL, PROP_TEXT, PROP_VALUE, PROP_CHECKED, PROP_ENABLED, PROP_VISIBLE},
};

#[cfg(target_os = "windows")]
thread_local! {
    static WIN32_INSTANCE: RefCell<Option<HINSTANCE>> = RefCell::new(None);
    static WIN32_WIDGETS: RefCell<HashMap<WidgetId, Win32Widget>> = RefCell::new(HashMap::new());
    static WIN32_WINDOW_PROC: RefCell<Option<WNDPROC>> = RefCell::new(None);
    static WIN32_EVENT_SENDER: RefCell<Option<Sender<Win32Event>>> = RefCell::new(None);
    static WIN32_EVENT_RECEIVER: RefCell<Option<std::sync::mpsc::Receiver<Win32Event>>> = RefCell::new(None);
    static COMPONENT_ID_COUNTER: RefCell<u32> = RefCell::new(0);
    static WIDGET_CALLBACKS: RefCell<HashMap<WidgetId, HashMap<String, String>>> = RefCell::new(HashMap::new());
}

#[cfg(target_os = "windows")]
#[derive(Clone, Debug)]
pub enum Win32Widget {
    Window {
        hwnd: HWND,
        class_name: String,
    },
    Button {
        hwnd: HWND,
    },
    Label {
        hwnd: HWND,
    },
    Input {
        hwnd: HWND,
    },
    TextArea {
        hwnd: HWND,
    },
    Password {
        hwnd: HWND,
    },
    Checkbox {
        hwnd: HWND,
    },
    Dropdown {
        hwnd: HWND,
    },
    ProgressBar {
        hwnd: HWND,
    },
    Div {
        hwnd: HWND,
    },
}

#[cfg(target_os = "windows")]
#[derive(Debug, Clone)]
pub enum Win32Event {
    Click(WidgetId),
    Change(WidgetId, String),
    Close(WidgetId),
    Resize(WidgetId, u32, u32),
    Destroy(WidgetId),
}

#[cfg(target_os = "windows")]
fn next_component_id() -> u32 {
    COMPONENT_ID_COUNTER.with(|c| {
        let mut counter = c.borrow_mut();
        *counter += 1;
        *counter
    })
}

#[cfg(target_os = "windows")]
fn to_wide_string(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

#[cfg(target_os = "windows")]
fn wide_to_string(wide: &[u16]) -> String {
    String::from_utf16_lossy(wide)
}

#[cfg(target_os = "windows")]
fn get_window_text(hwnd: HWND) -> String {
    unsafe {
        let len = SendMessageW(hwnd, WM_GETTEXTLENGTH, WPARAM(0), LPARAM(0));
        if len == 0 {
            return String::new();
        }
        let mut buffer: Vec<u16> = vec![0; (len as usize) + 1];
        SendMessageW(hwnd, WM_GETTEXT, WPARAM(buffer.len()), LPARAM(buffer.as_mut_ptr() as isize));
        wide_to_string(&buffer)
    }
}

#[cfg(target_os = "windows")]
fn set_window_text(hwnd: HWND, text: &str) {
    unsafe {
        let wide = to_wide_string(text);
        SendMessageW(hwnd, WM_SETTEXT, WPARAM(0), LPARAM(wide.as_ptr() as isize));
    }
}

#[cfg(target_os = "windows")]
fn get_default_font() -> HFONT {
    unsafe { GetStockObject(DEFAULT_GUI_FONT).into() }
}

#[cfg(target_os = "windows")]
fn set_widget_font(hwnd: HWND) {
    unsafe {
        let font = get_default_font();
        SendMessageW(hwnd, WM_SETFONT, WPARAM(font.0 as usize), LPARAM(1));
    }
}

#[cfg(target_os = "windows")]
fn register_window_class(class_name: &str) -> bool {
    unsafe {
        let instance = GetModuleHandleW(None).unwrap_or_default();
        WIN32_INSTANCE.with(|i| *i.borrow_mut() = Some(instance));

        let class_name_wide = to_wide_string(class_name);
        let wnd_class = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: instance,
            hIcon: LoadIconW(None, IDI_APPLICATION).unwrap_or_default(),
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap_or_default(),
            hbrBackground: (COLOR_WINDOW.0 + 1) as _,
            lpszMenuName: PWSTR::null(),
            lpszClassName: PCWSTR(class_name_wide.as_ptr()),
            hIconSm: LoadIconW(None, IDI_APPLICATION).unwrap_or_default(),
        };

        RegisterClassExW(&wnd_class) != 0
    }
}

#[cfg(target_os = "windows")]
extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match msg {
            WM_CREATE => {
                let create_struct = lparam.0 as *const windows::Win32::UI::WindowsAndMessaging::CREATESTRUCTW;
                if !create_struct.is_null() {
                    let user_data = (*create_struct).lpCreateParams;
                    SetWindowLongPtrW(hwnd, GWLP_USERDATA, user_data as isize);
                }
                LRESULT(0)
            }
            WM_COMMAND => {
                let notification_code = (wparam.0 >> 16) as u16;
                let control_id = (wparam.0 & 0xFFFF) as u16;
                let hwnd_control = HWND(lparam.0);

                if let Some(sender) = WIN32_EVENT_SENDER.with(|s| s.borrow().clone()) {
                    WIN32_WIDGETS.with(|widgets| {
                        let widgets_borrow = widgets.borrow();
                        for (widget_id, widget) in widgets_borrow.iter() {
                            if let Some(widget_hwnd) = widget.get_hwnd() {
                                if widget_hwnd == hwnd_control {
                                    match notification_code {
                                        BN_CLICKED => {
                                            let _ = sender.send(Win32Event::Click(widget_id.clone()));
                                        }
                                        EN_CHANGE => {
                                            let text = get_window_text(hwnd_control);
                                            let _ = sender.send(Win32Event::Change(widget_id.clone(), text));
                                        }
                                        _ => {}
                                    }
                                    break;
                                }
                            }
                        }
                    });
                }
                LRESULT(0)
            }
            WM_CLOSE => {
                if let Some(sender) = WIN32_EVENT_SENDER.with(|s| s.borrow().clone()) {
                    WIN32_WIDGETS.with(|widgets| {
                        let widgets_borrow = widgets.borrow();
                        for (widget_id, widget) in widgets_borrow.iter() {
                            if let Some(widget_hwnd) = widget.get_hwnd() {
                                if widget_hwnd == hwnd {
                                    let _ = sender.send(Win32Event::Close(widget_id.clone()));
                                    break;
                                }
                            }
                        }
                    });
                }
                LRESULT(0)
            }
            WM_DESTROY => {
                if let Some(sender) = WIN32_EVENT_SENDER.with(|s| s.borrow().clone()) {
                    WIN32_WIDGETS.with(|widgets| {
                        let widgets_borrow = widgets.borrow();
                        for (widget_id, widget) in widgets_borrow.iter() {
                            if let Some(widget_hwnd) = widget.get_hwnd() {
                                if widget_hwnd == hwnd {
                                    let _ = sender.send(Win32Event::Destroy(widget_id.clone()));
                                    break;
                                }
                            }
                        }
                    });
                }
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_SIZE => {
                if let Some(sender) = WIN32_EVENT_SENDER.with(|s| s.borrow().clone()) {
                    WIN32_WIDGETS.with(|widgets| {
                        let widgets_borrow = widgets.borrow();
                        for (widget_id, widget) in widgets_borrow.iter() {
                            if let Some(widget_hwnd) = widget.get_hwnd() {
                                if widget_hwnd == hwnd {
                                    let width = (lparam.0 & 0xFFFF) as u32;
                                    let height = ((lparam.0 >> 16) & 0xFFFF) as u32;
                                    let _ = sender.send(Win32Event::Resize(widget_id.clone(), width, height));
                                    break;
                                }
                            }
                        }
                    });
                }
                LRESULT(0)
            }
            WM_CTLCOLORSTATIC | WM_CTLCOLOREDIT | WM_CTLCOLORBTN => {
                let hdc = wparam.0 as *mut std::ffi::c_void;
                let hwnd_ctl = HWND(lparam.0);
                let mut found = false;
                WIN32_WIDGETS.with(|widgets| {
                    let widgets_borrow = widgets.borrow();
                    for widget in widgets_borrow.values() {
                        if let Some(widget_hwnd) = widget.get_hwnd() {
                            if widget_hwnd == hwnd_ctl {
                                found = true;
                            }
                        }
                    }
                });
                if found {
                    unsafe {
                        SetBkMode(hdc as _, 1);
                        SetTextColor(hdc as _, RGB(0, 0, 0));
                    }
                }
                let brush = GetStockObject(COLOR_BTNFACE).0 as isize;
                LRESULT(brush)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

#[cfg(target_os = "windows")]
impl Win32Widget {
    fn get_hwnd(&self) -> Option<HWND> {
        match self {
            Win32Widget::Window { hwnd, .. } => Some(*hwnd),
            Win32Widget::Button { hwnd } => Some(*hwnd),
            Win32Widget::Label { hwnd } => Some(*hwnd),
            Win32Widget::Input { hwnd } => Some(*hwnd),
            Win32Widget::TextArea { hwnd } => Some(*hwnd),
            Win32Widget::Password { hwnd } => Some(*hwnd),
            Win32Widget::Checkbox { hwnd } => Some(*hwnd),
            Win32Widget::Dropdown { hwnd } => Some(*hwnd),
            Win32Widget::ProgressBar { hwnd } => Some(*hwnd),
            Win32Widget::Div { hwnd } => Some(*hwnd),
        }
    }

    fn set_text(&self, text: &str) {
        if let Some(hwnd) = self.get_hwnd() {
            set_window_text(hwnd, text);
        }
    }

    fn get_text(&self) -> String {
        if let Some(hwnd) = self.get_hwnd() {
            get_window_text(hwnd)
        } else {
            String::new()
        }
    }
}

#[cfg(target_os = "windows")]
fn store_widget(id: &WidgetId, widget: Win32Widget) {
    WIN32_WIDGETS.with(|w| {
        w.borrow_mut().insert(id.clone(), widget);
    });
}

#[cfg(target_os = "windows")]
fn find_widget(id: &WidgetId) -> Option<Win32Widget> {
    WIN32_WIDGETS.with(|w| w.borrow().get(id).cloned())
}

#[cfg(target_os = "windows")]
fn insert_widget(parent_id: &WidgetId, child_id: &WidgetId) {
    let parent_opt = find_widget(parent_id);
    let child_opt = find_widget(child_id);

    if let (Some(parent), Some(child)) = (parent_opt, child_opt) {
        if let (Some(parent_hwnd), Some(child_hwnd)) = (parent.get_hwnd(), child.get_hwnd()) {
            unsafe {
                let parent_style = GetWindowLongW(parent_hwnd, GWL_STYLE);
                SetWindowLongW(child_hwnd, GWL_STYLE, parent_style | WS_CHILD | WS_VISIBLE);
                SetWindowPos(
                    child_hwnd,
                    HWND_TOP,
                    0, 0, 0, 0,
                    SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED,
                );
            }
        }
    }
}

#[cfg(target_os = "windows")]
pub struct Win32Backend {
    initialized: bool,
    class_name_counter: u32,
}

#[cfg(target_os = "windows")]
impl Win32Backend {
    pub fn new() -> Self {
        Self {
            initialized: false,
            class_name_counter: 0,
        }
    }

    fn next_class_name(&mut self) -> String {
        self.class_name_counter += 1;
        format!("AlyWin32Class{}", self.class_name_counter)
    }
}

#[cfg(target_os = "windows")]
impl Default for Win32Backend {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_os = "windows")]
impl GuiBackend for Win32Backend {
    fn backend_type(&self) -> BackendType {
        BackendType::Win32
    }

    fn create_window(&mut self, title: &str, width: i32, height: i32) -> WidgetId {
        let id = next_widget_id();
        let class_name = self.next_class_name();
        
        if !register_window_class(&class_name) {
            eprintln!("Failed to register window class: {}", class_name);
        }

        unsafe {
            let instance = GetModuleHandleW(None).unwrap_or_default();
            let title_wide = to_wide_string(title);
            let class_wide = to_wide_string(&class_name);

            let hwnd = CreateWindowExW(
                WS_EX_CLIENTEDGE | WS_EX_WINDOWEDGE,
                PCWSTR(class_wide.as_ptr()),
                PCWSTR(title_wide.as_ptr()),
                WS_OVERLAPPEDWINDOW | WS_CLIPCHILDREN | WS_CLIPSIBLINGS,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                width,
                height,
                HWND::default(),
                None,
                instance,
                None,
            );

            if hwnd.0.is_null() {
                eprintln!("Failed to create window: {:?}", windows::core::Error::from_win32());
            } else {
                ShowWindow(hwnd, SW_SHOW);
                UpdateWindow(hwnd);
            }

            let widget = Win32Widget::Window { hwnd, class_name };
            store_widget(&id, widget);
        }

        id
    }

    fn create_button(&mut self, label: &str) -> WidgetId {
        let id = next_widget_id();
        
        WIN32_WIDGETS.with(|widgets| {
            let mut widgets_borrow = widgets.borrow();
            if let Some((_, parent_widget)) = widgets_borrow.iter().find(|(_, w)| matches!(w, Win32Widget::Window { .. })) {
                if let Some(parent_hwnd) = parent_widget.get_hwnd() {
                    unsafe {
                        let instance = GetModuleHandleW(None).unwrap_or_default();
                        let label_wide = to_wide_string(label);
                        
                        let hwnd = CreateWindowExW(
                            WS_EX_CLIENTEDGE,
                            w!("BUTTON"),
                            PCWSTR(label_wide.as_ptr()),
                            WS_CHILD | WS_VISIBLE | WS_TABSTOP | BS_PUSHBUTTON,
                            10, 10, 200, 35,
                            parent_hwnd,
                            None,
                            instance,
                            None,
                        );

                        if hwnd.0.is_null() {
                            eprintln!("Failed to create button: {:?}", windows::core::Error::from_win32());
                        } else {
                            set_widget_font(hwnd);
                            let widget = Win32Widget::Button { hwnd };
                            drop(widgets_borrow);
                            store_widget(&id, widget);
                        }
                    }
                }
            }
        });

        id
    }

    fn create_label(&mut self, text: &str) -> WidgetId {
        let id = next_widget_id();
        
        WIN32_WIDGETS.with(|widgets| {
            let mut widgets_borrow = widgets.borrow();
            if let Some((_, parent_widget)) = widgets_borrow.iter().find(|(_, w)| matches!(w, Win32Widget::Window { .. })) {
                if let Some(parent_hwnd) = parent_widget.get_hwnd() {
                    unsafe {
                        let instance = GetModuleHandleW(None).unwrap_or_default();
                        let text_wide = to_wide_string(text);
                        
                        let hwnd = CreateWindowExW(
                            0,
                            w!("STATIC"),
                            PCWSTR(text_wide.as_ptr()),
                            WS_CHILD | WS_VISIBLE | SS_LEFT,
                            10, 10, 300, 25,
                            parent_hwnd,
                            None,
                            instance,
                            None,
                        );

                        if hwnd.0.is_null() {
                            eprintln!("Failed to create label: {:?}", windows::core::Error::from_win32());
                        } else {
                            set_widget_font(hwnd);
                            let widget = Win32Widget::Label { hwnd };
                            drop(widgets_borrow);
                            store_widget(&id, widget);
                        }
                    }
                }
            }
        });

        id
    }

    fn create_div(&mut self, direction: &str) -> WidgetId {
        let id = next_widget_id();
        
        WIN32_WIDGETS.with(|widgets| {
            let mut widgets_borrow = widgets.borrow();
            if let Some((_, parent_widget)) = widgets_borrow.iter().find(|(_, w)| matches!(w, Win32Widget::Window { .. })) {
                if let Some(parent_hwnd) = parent_widget.get_hwnd() {
                    unsafe {
                        let instance = GetModuleHandleW(None).unwrap_or_default();
                        
                        let hwnd = CreateWindowExW(
                            0,
                            w!("STATIC"),
                            PCWSTR::null(),
                            WS_CHILD | WS_VISIBLE | WS_CLIPCHILDREN | WS_CLIPSIBLINGS,
                            0, 0, 0, 0,
                            parent_hwnd,
                            None,
                            instance,
                            None,
                        );

                        if hwnd.0.is_null() {
                            eprintln!("Failed to create div: {:?}", windows::core::Error::from_win32());
                        } else {
                            let widget = Win32Widget::Div { hwnd };
                            drop(widgets_borrow);
                            store_widget(&id, widget);
                        }
                    }
                }
            }
        });

        id
    }

    fn create_input(&mut self, placeholder: &str) -> WidgetId {
        let id = next_widget_id();
        
        WIN32_WIDGETS.with(|widgets| {
            let mut widgets_borrow = widgets.borrow();
            if let Some((_, parent_widget)) = widgets_borrow.iter().find(|(_, w)| matches!(w, Win32Widget::Window { .. })) {
                if let Some(parent_hwnd) = parent_widget.get_hwnd() {
                    unsafe {
                        let instance = GetModuleHandleW(None).unwrap_or_default();
                        let placeholder_wide = to_wide_string(placeholder);
                        
                        let hwnd = CreateWindowExW(
                            WS_EX_CLIENTEDGE,
                            w!("EDIT"),
                            PCWSTR(placeholder_wide.as_ptr()),
                            WS_CHILD | WS_VISIBLE | WS_TABSTOP | WS_BORDER | ES_AUTOHSCROLL,
                            10, 10, 300, 25,
                            parent_hwnd,
                            None,
                            instance,
                            None,
                        );

                        if hwnd.0.is_null() {
                            eprintln!("Failed to create input: {:?}", windows::core::Error::from_win32());
                        } else {
                            set_widget_font(hwnd);
                            let widget = Win32Widget::Input { hwnd };
                            drop(widgets_borrow);
                            store_widget(&id, widget);
                        }
                    }
                }
            }
        });

        id
    }

    fn create_textarea(&mut self) -> WidgetId {
        let id = next_widget_id();
        
        WIN32_WIDGETS.with(|widgets| {
            let mut widgets_borrow = widgets.borrow();
            if let Some((_, parent_widget)) = widgets_borrow.iter().find(|(_, w)| matches!(w, Win32Widget::Window { .. })) {
                if let Some(parent_hwnd) = parent_widget.get_hwnd() {
                    unsafe {
                        let instance = GetModuleHandleW(None).unwrap_or_default();
                        
                        let hwnd = CreateWindowExW(
                            WS_EX_CLIENTEDGE,
                            w!("EDIT"),
                            PCWSTR::null(),
                            WS_CHILD | WS_VISIBLE | WS_TABSTOP | WS_BORDER | WS_VSCROLL | WS_HSCROLL | ES_MULTILINE | ES_AUTOVSCROLL | ES_AUTOHSCROLL | ES_WANTRETURN,
                            10, 10, 300, 100,
                            parent_hwnd,
                            None,
                            instance,
                            None,
                        );

                        if hwnd.0.is_null() {
                            eprintln!("Failed to create textarea: {:?}", windows::core::Error::from_win32());
                        } else {
                            set_widget_font(hwnd);
                            let widget = Win32Widget::TextArea { hwnd };
                            drop(widgets_borrow);
                            store_widget(&id, widget);
                        }
                    }
                }
            }
        });

        id
    }

    fn create_password(&mut self) -> WidgetId {
        let id = next_widget_id();
        
        WIN32_WIDGETS.with(|widgets| {
            let mut widgets_borrow = widgets.borrow();
            if let Some((_, parent_widget)) = widgets_borrow.iter().find(|(_, w)| matches!(w, Win32Widget::Window { .. })) {
                if let Some(parent_hwnd) = parent_widget.get_hwnd() {
                    unsafe {
                        let instance = GetModuleHandleW(None).unwrap_or_default();
                        
                        let hwnd = CreateWindowExW(
                            WS_EX_CLIENTEDGE,
                            w!("EDIT"),
                            PCWSTR::null(),
                            WS_CHILD | WS_VISIBLE | WS_TABSTOP | WS_BORDER | ES_AUTOHSCROLL | ES_PASSWORD,
                            10, 10, 300, 25,
                            parent_hwnd,
                            None,
                            instance,
                            None,
                        );

                        if hwnd.0.is_null() {
                            eprintln!("Failed to create password field: {:?}", windows::core::Error::from_win32());
                        } else {
                            set_widget_font(hwnd);
                            let widget = Win32Widget::Password { hwnd };
                            drop(widgets_borrow);
                            store_widget(&id, widget);
                        }
                    }
                }
            }
        });

        id
    }

    fn create_checkbox(&mut self, label: &str) -> WidgetId {
        let id = next_widget_id();
        
        WIN32_WIDGETS.with(|widgets| {
            let mut widgets_borrow = widgets.borrow();
            if let Some((_, parent_widget)) = widgets_borrow.iter().find(|(_, w)| matches!(w, Win32Widget::Window { .. })) {
                if let Some(parent_hwnd) = parent_widget.get_hwnd() {
                    unsafe {
                        let instance = GetModuleHandleW(None).unwrap_or_default();
                        let label_wide = to_wide_string(label);
                        
                        let hwnd = CreateWindowExW(
                            0,
                            w!("BUTTON"),
                            PCWSTR(label_wide.as_ptr()),
                            WS_CHILD | WS_VISIBLE | WS_TABSTOP | WS_GROUP | 0x0003,
                            10, 10, 200, 25,
                            parent_hwnd,
                            None,
                            instance,
                            None,
                        );

                        if hwnd.0.is_null() {
                            eprintln!("Failed to create checkbox: {:?}", windows::core::Error::from_win32());
                        } else {
                            set_widget_font(hwnd);
                            let widget = Win32Widget::Checkbox { hwnd };
                            drop(widgets_borrow);
                            store_widget(&id, widget);
                        }
                    }
                }
            }
        });

        id
    }

    fn create_radio(&mut self, label: &str) -> WidgetId {
        let id = next_widget_id();
        
        WIN32_WIDGETS.with(|widgets| {
            let mut widgets_borrow = widgets.borrow();
            if let Some((_, parent_widget)) = widgets_borrow.iter().find(|(_, w)| matches!(w, Win32Widget::Window { .. })) {
                if let Some(parent_hwnd) = parent_widget.get_hwnd() {
                    unsafe {
                        let instance = GetModuleHandleW(None).unwrap_or_default();
                        let label_wide = to_wide_string(label);
                        
                        let hwnd = CreateWindowExW(
                            0,
                            w!("BUTTON"),
                            PCWSTR(label_wide.as_ptr()),
                            WS_CHILD | WS_VISIBLE | WS_TABSTOP | WS_GROUP | 0x0004,
                            10, 10, 200, 25,
                            parent_hwnd,
                            None,
                            instance,
                            None,
                        );

                        if hwnd.0.is_null() {
                            eprintln!("Failed to create radio: {:?}", windows::core::Error::from_win32());
                        } else {
                            set_widget_font(hwnd);
                            let widget = Win32Widget::Checkbox { hwnd };
                            drop(widgets_borrow);
                            store_widget(&id, widget);
                        }
                    }
                }
            }
        });

        id
    }

    fn create_slider(&mut self, min: f64, max: f64, val: f64) -> WidgetId {
        let id = next_widget_id();
        
        WIN32_WIDGETS.with(|widgets| {
            let mut widgets_borrow = widgets.borrow();
            if let Some((_, parent_widget)) = widgets_borrow.iter().find(|(_, w)| matches!(w, Win32Widget::Window { .. })) {
                if let Some(parent_hwnd) = parent_widget.get_hwnd() {
                    unsafe {
                        let instance = GetModuleHandleW(None).unwrap_or_default();
                        
                        let hwnd = CreateWindowExW(
                            0,
                            w!("msctls_trackbar32"),
                            PCWSTR::null(),
                            WS_CHILD | WS_VISIBLE | WS_TABSTOP | 0x0004,
                            10, 10, 300, 30,
                            parent_hwnd,
                            None,
                            instance,
                            None,
                        );

                        if hwnd.0.is_null() {
                            eprintln!("Failed to create slider: {:?}", windows::core::Error::from_win32());
                        } else {
                            SendMessageW(hwnd, 0x0400 + 4, WPARAM(1), LPARAM((max as i32) << 16 | min as i32));
                            SendMessageW(hwnd, 0x0400 + 5, WPARAM(0), LPARAM(val as i32));
                            let widget = Win32Widget::Div { hwnd };
                            drop(widgets_borrow);
                            store_widget(&id, widget);
                        }
                    }
                }
            }
        });

        id
    }

    fn create_progressbar(&mut self, val: f64) -> WidgetId {
        let id = next_widget_id();
        
        WIN32_WIDGETS.with(|widgets| {
            let mut widgets_borrow = widgets.borrow();
            if let Some((_, parent_widget)) = widgets_borrow.iter().find(|(_, w)| matches!(w, Win32Widget::Window { .. })) {
                if let Some(parent_hwnd) = parent_widget.get_hwnd() {
                    unsafe {
                        let instance = GetModuleHandleW(None).unwrap_or_default();
                        
                        let hwnd = CreateWindowExW(
                            0,
                            w!("msctls_progress32"),
                            PCWSTR::null(),
                            WS_CHILD | WS_VISIBLE,
                            10, 10, 300, 25,
                            parent_hwnd,
                            None,
                            instance,
                            None,
                        );

                        if hwnd.0.is_null() {
                            eprintln!("Failed to create progressbar: {:?}", windows::core::Error::from_win32());
                        } else {
                            SendMessageW(hwnd, 0x0400 + 1, WPARAM(0), LPARAM(100));
                            SendMessageW(hwnd, 0x0400 + 2, WPARAM(val as i32), LPARAM(0));
                            let widget = Win32Widget::ProgressBar { hwnd };
                            drop(widgets_borrow);
                            store_widget(&id, widget);
                        }
                    }
                }
            }
        });

        id
    }

    fn create_dropdown(&mut self, items: &[String]) -> WidgetId {
        let id = next_widget_id();
        
        WIN32_WIDGETS.with(|widgets| {
            let mut widgets_borrow = widgets.borrow();
            if let Some((_, parent_widget)) = widgets_borrow.iter().find(|(_, w)| matches!(w, Win32Widget::Window { .. })) {
                if let Some(parent_hwnd) = parent_widget.get_hwnd() {
                    unsafe {
                        let instance = GetModuleHandleW(None).unwrap_or_default();
                        
                        let hwnd = CreateWindowExW(
                            0,
                            w!("COMBOBOX"),
                            PCWSTR::null(),
                            WS_CHILD | WS_VISIBLE | WS_TABSTOP | WS_VSCROLL | 0x0003 | 0x0002,
                            10, 10, 200, 200,
                            parent_hwnd,
                            None,
                            instance,
                            None,
                        );

                        if hwnd.0.is_null() {
                            eprintln!("Failed to create dropdown: {:?}", windows::core::Error::from_win32());
                        } else {
                            for item in items {
                                let item_wide = to_wide_string(item);
                                SendMessageW(hwnd, 0x0143, WPARAM(0), LPARAM(item_wide.as_ptr() as isize));
                            }
                            set_widget_font(hwnd);
                            let widget = Win32Widget::Dropdown { hwnd };
                            drop(widgets_borrow);
                            store_widget(&id, widget);
                        }
                    }
                }
            }
        });

        id
    }

    fn create_separator(&mut self) -> WidgetId {
        let id = next_widget_id();
        
        WIN32_WIDGETS.with(|widgets| {
            let mut widgets_borrow = widgets.borrow();
            if let Some((_, parent_widget)) = widgets_borrow.iter().find(|(_, w)| matches!(w, Win32Widget::Window { .. })) {
                if let Some(parent_hwnd) = parent_widget.get_hwnd() {
                    unsafe {
                        let instance = GetModuleHandleW(None).unwrap_or_default();
                        
                        let hwnd = CreateWindowExW(
                            0,
                            w!("STATIC"),
                            PCWSTR::null(),
                            WS_CHILD | WS_VISIBLE | 0x000A,
                            10, 10, 300, 2,
                            parent_hwnd,
                            None,
                            instance,
                            None,
                        );

                        if hwnd.0.is_null() {
                            eprintln!("Failed to create separator: {:?}", windows::core::Error::from_win32());
                        } else {
                            let widget = Win32Widget::Label { hwnd };
                            drop(widgets_borrow);
                            store_widget(&id, widget);
                        }
                    }
                }
            }
        });

        id
    }

    fn create_spinner(&mut self, min: f64, max: f64, step: f64, val: f64) -> WidgetId {
        let id = next_widget_id();
        
        WIN32_WIDGETS.with(|widgets| {
            let mut widgets_borrow = widgets.borrow();
            if let Some((_, parent_widget)) = widgets_borrow.iter().find(|(_, w)| matches!(w, Win32Widget::Window { .. })) {
                if let Some(parent_hwnd) = parent_widget.get_hwnd() {
                    unsafe {
                        let instance = GetModuleHandleW(None).unwrap_or_default();
                        
                        let hwnd = CreateWindowExW(
                            0,
                            w!("msctls_updown32"),
                            PCWSTR::null(),
                            WS_CHILD | WS_VISIBLE | UDS_SETBUDDYINT | UDS_ALIGNRIGHT | UDS_AUTOBUDDY | UDS_ARROWKEYS,
                            0, 0, 0, 0,
                            parent_hwnd,
                            None,
                            instance,
                            None,
                        );

                        if hwnd.0.is_null() {
                            eprintln!("Failed to create spinner: {:?}", windows::core::Error::from_win32());
                        } else {
                            let widget = Win32Widget::Div { hwnd };
                            drop(widgets_borrow);
                            store_widget(&id, widget);
                        }
                    }
                }
            }
        });

        id
    }

    fn create_container(&mut self, width: i32, height: i32, direction: &str) -> WidgetId {
        self.create_div(direction)
    }

    fn insert_widget(&mut self, parent: &WidgetId, child: &WidgetId) {
        insert_widget(parent, child);
    }

    fn set_property(&mut self, id: &WidgetId, prop: &str, value: &str) {
        if let Some(widget) = find_widget(id) {
            match prop {
                PROP_LABEL | PROP_TEXT | PROP_VALUE => {
                    widget.set_text(value);
                }
                PROP_CHECKED => {
                    if let Some(hwnd) = widget.get_hwnd() {
                        let checked = value == "true" || value == "1";
                        unsafe {
                            SendMessageW(hwnd, 0x00F1, WPARAM(checked as usize), LPARAM(0));
                        }
                    }
                }
                PROP_ENABLED => {
                    if let Some(hwnd) = widget.get_hwnd() {
                        let enabled = value != "false";
                        unsafe {
                            windows::Win32::UI::WindowsAndMessaging::EnableWindow(hwnd, enabled);
                        }
                    }
                }
                PROP_VISIBLE => {
                    if let Some(hwnd) = widget.get_hwnd() {
                        let visible = value != "false";
                        unsafe {
                            ShowWindow(hwnd, if visible { SW_SHOW } else { SW_HIDE });
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn get_property(&mut self, id: &WidgetId, prop: &str) -> String {
        if let Some(widget) = find_widget(id) {
            match prop {
                PROP_LABEL | PROP_TEXT | PROP_VALUE => widget.get_text(),
                PROP_CHECKED => {
                    if let Some(hwnd) = widget.get_hwnd() {
                        unsafe {
                            let checked = SendMessageW(hwnd, 0x00F0, WPARAM(0), LPARAM(0));
                            if checked != 0 { "true".to_string() } else { "false".to_string() }
                        }
                    } else {
                        "false".to_string()
                    }
                }
                "id" => id.clone(),
                _ => "None".to_string(),
            }
        } else {
            "None".to_string()
        }
    }

    fn on_event(&mut self, id: &WidgetId, event: &str, callback: String) {
        gui_crate::event::set_callback(id, event, callback);

        WIDGET_CALLBACKS.with(|callbacks| {
            let mut callbacks_borrow = callbacks.borrow_mut();
            let widget_callbacks = callbacks_borrow.entry(id.clone()).or_insert_with(HashMap::new);
            widget_callbacks.insert(event.to_string(), callback);
        });
    }

    fn run(&mut self) {
        if self.initialized {
            return;
        }
        self.initialized = true;

        let (sender, receiver) = mpsc::channel();
        WIN32_EVENT_SENDER.with(|s| *s.borrow_mut() = Some(sender));
        WIN32_EVENT_RECEIVER.with(|r| *r.borrow_mut() = Some(receiver));

        unsafe {
            let mut msg = windows::Win32::UI::WindowsAndMessaging::MSG::default();
            while GetMessageW(&mut msg, HWND::default(), 0, 0).into() {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);

                WIN32_EVENT_RECEIVER.with(|r| {
                    if let Some(receiver) = r.borrow().as_ref() {
                        while let Ok(event) = receiver.try_recv() {
                            match event {
                                Win32Event::Click(widget_id) => {
                                    gui_crate::event::fire_callback(&widget_id, "onClick");
                                }
                                Win32Event::Change(widget_id, text) => {
                                    gui_crate::event::fire_callback(&widget_id, "onChange");
                                }
                                Win32Event::Close(widget_id) => {
                                    gui_crate::event::fire_callback(&widget_id, "onClose");
                                }
                                Win32Event::Resize(widget_id, width, height) => {
                                    gui_crate::event::fire_callback(&widget_id, "onResize");
                                }
                                Win32Event::Destroy(widget_id) => {
                                    WIN32_WIDGETS.with(|w| w.borrow_mut().remove(&widget_id));
                                }
                            }
                        }
                    }
                });
            }
        }
    }

    // System tray
    fn set_tray_icon(&mut self, window_id: &WidgetId, icon_path: &str, tooltip: &str) -> bool {
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::UI::Shell::{
                NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_MODIFY, NOTIFYICONDATAW,
            };
            use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
            use windows::Win32::UI::WindowsAndMessaging::LoadImageW;
            use windows::Win32::UI::WindowsAndMessaging::IMAGE_ICON;
            use windows::Win32::UI::WindowsAndMessaging::LR_DEFAULTSIZE;
            
            if let Some(widget) = find_widget(window_id) {
                if let Some(hwnd) = widget.get_hwnd() {
                    unsafe {
                        let icon_wide = to_wide_string(icon_path);
                        let hicon = LoadImageW(
                            None,
                            PCWSTR(icon_wide.as_ptr()),
                            IMAGE_ICON,
                            0,
                            0,
                            LR_DEFAULTSIZE,
                        );
                        
                        let mut nid = NOTIFYICONDATAW::default();
                        nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
                        nid.hWnd = hwnd;
                        nid.uID = 1;
                        nid.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
                        nid.uCallbackMessage = 0x8000; // WM_USER
                        nid.hIcon = hicon;
                        let tooltip_wide = to_wide_string(tooltip);
                        nid.szTip[..tooltip_wide.len().min(127)].copy_from_slice(&tooltip_wide[..tooltip_wide.len().min(127)]);
                        
                        let result = windows::Win32::UI::Shell::Shell_NotifyIconW(NIM_ADD, &nid);
                        result.as_bool()
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        }
        #[cfg(not(target_os = "windows"))]
        { false }
    }

    fn show_notification(&mut self, title: &str, body: &str, icon: Option<&str>) {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            let ps_cmd = format!(
                "Add-Type -AssemblyName System.Windows.Forms; \
                 $notify = New-Object System.Windows.Forms.NotifyIcon; \
                 $notify.Icon = [System.Drawing.SystemIcons]::Information; \
                 $notify.Visible = \$true; \
                 $notify.ShowBalloonTip(5000, '{}', '{}', 'Info')",
                title.replace("'", "''"), body.replace("'", "''")
            );
            let _ = Command::new("powershell").arg("-Command").arg(&ps_cmd).spawn();
        }
    }

    fn clipboard_set(&mut self, text: &str) {
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::UI::WindowsAndMessaging::{OpenClipboard, EmptyClipboard, SetClipboardData, CloseClipboard, CF_UNICODETEXT};
            use windows::Win32::Foundation::HWND;
            use windows::Win32::System::Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE};
            
            unsafe {
                if OpenClipboard(HWND::default()).as_bool() {
                    EmptyClipboard();
                    let wide = to_wide_string(text);
                    let size = wide.len() * 2;
                    if let Ok(hmem) = GlobalAlloc(GMEM_MOVEABLE, size) {
                        if let Ok(ptr) = GlobalLock(hmem) {
                            std::ptr::copy_nonoverlapping(wide.as_ptr(), ptr as *mut u16, wide.len());
                            GlobalUnlock(hmem);
                            SetClipboardData(CF_UNICODETEXT, hmem);
                        }
                    }
                    CloseClipboard();
                }
            }
        }
    }

    fn clipboard_get(&mut self) -> String {
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::UI::WindowsAndMessaging::{OpenClipboard, GetClipboardData, CloseClipboard, CF_UNICODETEXT};
            use windows::Win32::Foundation::HWND;
            
            unsafe {
                if OpenClipboard(HWND::default()).as_bool() {
                    if let Ok(hmem) = GetClipboardData(CF_UNICODETEXT) {
                        let ptr = hmem.0 as *const u16;
                        let mut len = 0;
                        while *ptr.add(len) != 0 {
                            len += 1;
                        }
                        let slice = std::slice::from_raw_parts(ptr, len);
                        let result = String::from_utf16_lossy(slice);
                        CloseClipboard();
                        return result;
                    }
                    CloseClipboard();
                }
            }
        }
        String::new()
    }

    fn drag_drop_init(&mut self, widget_id: &WidgetId, _data: &str) {
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::UI::WindowsAndMessaging::{RegisterDragDrop, RevokeDragDrop};
            if let Some(widget) = find_widget(widget_id) {
                if let Some(hwnd) = widget.get_hwnd() {
                    unsafe {
                        RevokeDragDrop(hwnd);
                        RegisterDragDrop(hwnd, std::ptr::null_mut());
                    }
                }
            }
        }
    }

    fn drag_drop_accept(&mut self, widget_id: &WidgetId, _types: &[&str]) {
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::UI::WindowsAndMessaging::{RegisterDragDrop, RevokeDragDrop};
            if let Some(widget) = find_widget(widget_id) {
                if let Some(hwnd) = widget.get_hwnd() {
                    unsafe {
                        RevokeDragDrop(hwnd);
                        RegisterDragDrop(hwnd, std::ptr::null_mut());
                    }
                }
            }
        }
    }

    // OpenGL
    fn gl_context_create(&mut self, window_id: &WidgetId) -> bool {
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::Graphics::OpenGL::{wglCreateContext, wglMakeCurrent};
            use windows::Win32::Graphics::Gdi::GetDC;
            use windows::Win32::Foundation::HWND;
            
            if let Some(widget) = find_widget(window_id) {
                if let Some(hwnd) = widget.get_hwnd() {
                    unsafe {
                        let hdc = GetDC(hwnd);
                        let hglrc = wglCreateContext(hdc);
                        if !hglrc.is_null() {
                            wglMakeCurrent(hdc, hglrc);
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn gl_make_current(&mut self, window_id: &WidgetId) -> bool {
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::Graphics::OpenGL::wglMakeCurrent;
            use windows::Win32::Graphics::Gdi::GetDC;
            use windows::Win32::Foundation::HWND;
            
            if let Some(widget) = find_widget(window_id) {
                if let Some(hwnd) = widget.get_hwnd() {
                    unsafe {
                        let hdc = GetDC(hwnd);
                        wglMakeCurrent(hdc, std::ptr::null_mut());
                        return true;
                    }
                }
            }
        }
        false
    }

    fn gl_swap_buffers(&mut self, window_id: &WidgetId) {
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::Graphics::Gdi::SwapBuffers;
            use windows::Win32::Graphics::Gdi::GetDC;
            use windows::Win32::Foundation::HWND;
            
            if let Some(widget) = find_widget(window_id) {
                if let Some(hwnd) = widget.get_hwnd() {
                    unsafe {
                        let hdc = GetDC(hwnd);
                        SwapBuffers(hdc);
                    }
                }
            }
        }
    }

    fn gl_get_proc_address(&mut self, proc_name: &str) -> *const std::ffi::c_void {
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::Graphics::OpenGL::wglGetProcAddress;
            use std::ffi::CString;
            
            let cname = CString::new(proc_name).unwrap();
            unsafe { wglGetProcAddress(PCSTR(cname.as_ptr() as *const u8)) as *const std::ffi::c_void }
        }
        #[cfg(not(target_os = "windows"))]
        { std::ptr::null() }
    }
}

#[cfg(not(target_os = "windows"))]
pub struct Win32Backend;

#[cfg(not(target_os = "windows"))]
impl GuiBackend for Win32Backend {
    fn backend_type(&self) -> BackendType {
        BackendType::Win32
    }

    fn create_window(&mut self, _title: &str, _width: i32, _height: i32) -> WidgetId {
        unimplemented!("Win32 backend only available on Windows")
    }

    fn create_button(&mut self, _label: &str) -> WidgetId {
        unimplemented!("Win32 backend only available on Windows")
    }

    fn create_label(&mut self, _text: &str) -> WidgetId {
        unimplemented!("Win32 backend only available on Windows")
    }

    fn create_div(&mut self, _direction: &str) -> WidgetId {
        unimplemented!("Win32 backend only available on Windows")
    }

    fn create_input(&mut self, _placeholder: &str) -> WidgetId {
        unimplemented!("Win32 backend only available on Windows")
    }

    fn create_textarea(&mut self) -> WidgetId {
        unimplemented!("Win32 backend only available on Windows")
    }

    fn create_password(&mut self) -> WidgetId {
        unimplemented!("Win32 backend only available on Windows")
    }

    fn create_checkbox(&mut self, _label: &str) -> WidgetId {
        unimplemented!("Win32 backend only available on Windows")
    }

    fn create_radio(&mut self, _label: &str) -> WidgetId {
        unimplemented!("Win32 backend only available on Windows")
    }

    fn create_slider(&mut self, _min: f64, _max: f64, _val: f64) -> WidgetId {
        unimplemented!("Win32 backend only available on Windows")
    }

    fn create_progressbar(&mut self, _val: f64) -> WidgetId {
        unimplemented!("Win32 backend only available on Windows")
    }

    fn create_dropdown(&mut self, _items: &[String]) -> WidgetId {
        unimplemented!("Win32 backend only available on Windows")
    }

    fn create_separator(&mut self) -> WidgetId {
        unimplemented!("Win32 backend only available on Windows")
    }

    fn create_spinner(&mut self, _min: f64, _max: f64, _step: f64, _val: f64) -> WidgetId {
        unimplemented!("Win32 backend only available on Windows")
    }

    fn create_container(&mut self, _width: i32, _height: i32, _direction: &str) -> WidgetId {
        unimplemented!("Win32 backend only available on Windows")
    }

    fn insert_widget(&mut self, _parent: &WidgetId, _child: &WidgetId) {}

    fn set_property(&mut self, _id: &WidgetId, _prop: &str, _value: &str) {}

    fn get_property(&mut self, _id: &WidgetId, _prop: &str) -> String {
        "None".to_string()
    }

    fn on_event(&mut self, _id: &WidgetId, _event: &str, _callback: String) {}

    fn run(&mut self) {
        unimplemented!("Win32 backend only available on Windows")
    }
}