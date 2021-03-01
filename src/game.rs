use glium::Frame;
use crate::Vec3;
use crate::engine::game::state::Context;
use crate::engine::game::state::GameState;
use crate::Camera;
use crate::engine::game::billboard::Billboard;

pub struct SimpleState {
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
    fn on_init(self, ctx: &Context<'a>) -> Self {
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
    fn on_update(&mut self, _: &Context<'a>) {}
    fn on_draw(&mut self, frame: &mut Frame, _: &Context<'a>) {
        self.billboard.as_ref().unwrap().draw(frame, &self.camera)
    }
}
