use crate::vector::Vec2;
use crate::vector::Vec3;

unsafe impl glium::vertex::Attribute for Vec2 {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32F32
    }
}

impl glium::uniforms::AsUniformValue for Vec2 {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Vec2([self.x, self.y])
    }
}

unsafe impl glium::vertex::Attribute for Vec3 {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32F32F32
    }
}

impl glium::uniforms::AsUniformValue for Vec3 {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Vec3([self.x, self.y, self.z])
    }
}
