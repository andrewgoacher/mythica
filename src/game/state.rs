use crate::core::resource::Resource;

pub struct Context<'a> {
    pub display: glium::Display,
    pub resources: Resource<'a>
}