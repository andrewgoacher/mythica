#[macro_use]
extern crate glium;

extern crate image;

mod engine;
mod game;

use crate::game::SimpleState;
use crate::engine::core::application::application_builder::ApplicationOptions;
use crate::engine::core::application::Application;
use crate::engine::core::matrix::projection::ProjectionOptions;
use crate::engine::core::matrix::Matrix;
use crate::engine::core::shader::ShaderBuilder;
use crate::engine::core::vector::{Vec2, Vec3};
use crate::engine::game::camera::Camera;
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