use std::cell::RefCell;

pub type WidgetId = String;

thread_local! {
    static WIDGET_COUNTER: RefCell<usize> = RefCell::new(0);
}

pub fn next_widget_id() -> WidgetId {
    let mut counter = 0;
    WIDGET_COUNTER.with(|c| {
        let mut b = c.borrow_mut();
        *b += 1;
        counter = *b;
    });
    format!("widget_{}", counter)
}

pub const PROP_LABEL: &str = "label";
pub const PROP_TEXT: &str = "innerText";
pub const PROP_VALUE: &str = "value";
pub const PROP_CHECKED: &str = "checked";
pub const PROP_ENABLED: &str = "enabled";
pub const PROP_VISIBLE: &str = "visible";
pub const PROP_FRACTION: &str = "fraction";
pub const PROP_WIDTH: &str = "width";
pub const PROP_HEIGHT: &str = "height";
pub const PROP_CLASS: &str = "class";
pub const PROP_STYLE: &str = "style";
pub const PROP_ID: &str = "id";

pub const EVENT_CLICK: &str = "onClick";
pub const EVENT_CHANGE: &str = "onChange";
pub const EVENT_MOUSEOVER: &str = "onMouseOver";
pub const EVENT_MOUSEOUT: &str = "onMouseOut";
pub const EVENT_FOCUS: &str = "onFocus";
pub const EVENT_BLUR: &str = "onBlur";
pub const EVENT_KEYPRESS: &str = "onKeyPress";
pub const EVENT_MOUSEWHEEL: &str = "onMouseWheel";
pub const EVENT_RESIZE: &str = "onResize";
