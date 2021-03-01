#[macro_use]
extern crate glium;

extern crate image;

mod engine;

use crate::engine::core::application::application_builder::ApplicationOptions;
use crate::engine::core::application::Application;
use crate::engine::core::matrix::projection::ProjectionOptions;
use crate::engine::core::matrix::Matrix;
use crate::engine::core::shader::ShaderBuilder;
use crate::engine::core::vector::{Vec2, Vec3};
use crate::engine::game::billboard::Billboard;
use crate::engine::game::camera::Camera;
use crate::engine::game::state::GameState;
use glium::Frame;

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

    let state = SimpleState::new();

    Application::new(options).run(state);
}

struct SimpleState {
    billboard: Option<Billboard>,
    camera: Camera,
}

impl SimpleState {
    pub fn new() -> Self {
        Self {
            billboard: None,
            camera: Camera::new(),
        }
    }
}

impl<'a> GameState<'a> for SimpleState {
    fn on_init(self, ctx: &engine::game::state::Context<'a>) -> Self {
        let pos = Vec3::new_with(0.5f32, 0.2f32, -3f32);
        let direction = Vec3::new_with(-0.5f32, -0.2f32, 3f32);
        let (w, h) = ctx.dimensions;

        Self {
            billboard: Some(Billboard::new(&ctx.display, &ctx.resources)),
            camera: Camera::new()
                .with_position(pos)
                .with_direction(direction)
                .with_projection(w, h),
            ..self
        }
    }
    fn on_update(&mut self, _: &engine::game::state::Context<'a>) {}
    fn on_draw(&mut self, frame: &mut Frame, _: &engine::game::state::Context<'a>) {
        self.billboard.as_ref().unwrap().draw(frame, &self.camera)
    }
}
