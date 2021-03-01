
use crate::core::Resource;
use glium::Frame;

pub struct Context<'a> {
    pub display: glium::Display,
    pub resources: Resource<'a>,
    pub dimensions: (u32, u32),
}

pub trait GameState<'a> {
    fn on_init(self, context: &Context<'a>) -> Self;
    fn on_update(&mut self, context: &Context<'a>);
    fn on_draw(&mut self, frame: &mut Frame, context: &Context<'a>);
}
