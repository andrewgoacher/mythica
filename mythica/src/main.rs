

extern crate image;
mod game;

use mythica_engine::application::Application;
use mythica_engine::application::ApplicationOptions;
use crate::game::SimpleState;
use std::io::Cursor;

fn main() {
    let options = Some(
        ApplicationOptions::new(1024, 768)
            .with_title("Demo")
            .with_icon(load_icon().unwrap()),
    );

    let state = SimpleState::new();

    Application::new(options).run(state);
}

fn load_icon() -> Option<glutin::window::Icon> {
    match image::load(
        Cursor::new(&include_bytes!("../../assets/icon.png")[..]),
        image::ImageFormat::Png,
    ) {
        Ok(img) => {
            let bytes = img.to_rgba8().to_vec();
            match glutin::window::Icon::from_rgba(bytes, 43, 40) {
                Ok(icon) => Some(icon),
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}