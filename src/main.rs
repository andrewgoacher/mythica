#[macro_use]
extern crate glium;

extern crate image;

mod core;
mod game;

use crate::core::application::application_builder::ApplicationOptions;
use crate::core::application::Application;
use crate::core::matrix::projection::ProjectionOptions;
use crate::core::matrix::Matrix;
use crate::core::shader::ShaderBuilder;
use crate::core::vector::{Vec2, Vec3};

use std::io::Cursor;

fn load_icon() -> Option<glutin::window::Icon> {
    match image::load(
        Cursor::new(&include_bytes!("../assets/icon.png")[..]),
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

fn main() {
    let options = Some(
        ApplicationOptions::new(1024, 768)
            .with_title("Demo")
            .with_icon(load_icon().unwrap()),
    );

    let mut application = Application::new();
    application.run(options);
}
