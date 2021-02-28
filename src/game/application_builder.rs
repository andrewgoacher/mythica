pub struct ApplicationOptions {
    pub icon: Option<glutin::window::Icon>,
    pub title: Option<&'static str>,
    pub width: u32,
    pub height: u32
}

impl ApplicationOptions {
    pub fn new(w: u32, h: u32) -> Self {
        ApplicationOptions {
            width: w,
            height: h,
            icon: None,
            title: None,
        }
    }

    pub fn with_icon(self, icon: glutin::window::Icon) -> Self {
        ApplicationOptions {
            icon: Some(icon),
            ..self
        }
    }

    pub fn with_title(self, title: &'static str) -> Self {
        ApplicationOptions {
            title: Some(title),
            ..self
        }
    }
}

impl Default for ApplicationOptions {
    fn default() -> Self {
        ApplicationOptions::new(800, 600)
    }
}
