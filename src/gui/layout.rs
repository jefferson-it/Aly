#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayoutDirection {
    Vertical,
    Horizontal,
}

impl Default for LayoutDirection {
    fn default() -> Self {
        Self::Vertical
    }
}

impl LayoutDirection {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "horizontal" | "row" => Self::Horizontal,
            _ => Self::Vertical,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Alignment {
    Start,
    Center,
    End,
    Fill,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::Start
    }
}
