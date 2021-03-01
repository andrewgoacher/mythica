use crate::matrix::Matrix;

impl glium::uniforms::AsUniformValue for Matrix {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Mat4(self.to_arr())
    }
}
