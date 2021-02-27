pub struct ShaderBuilder {
    vertex: &'static str,
    fragment: &'static str,
}

impl ShaderBuilder {
    pub fn new() -> ShaderBuilder {
        Self {
            vertex: "",
            fragment: "",
        }
    }

    pub fn with_vertex_shader(self, vertex_src: &'static str) -> ShaderBuilder {
        Self {
            vertex: vertex_src,
            ..self
        }
    }

    pub fn with_fragment_shader(self, fragment_src: &'static str) -> ShaderBuilder {
        Self {
            fragment: fragment_src,
            ..self
        }
    }

    pub fn build<T: ?Sized>(self, display: &T) -> glium::Program
    where
        T: glium::backend::Facade,
    {
        glium::Program::from_source(display, self.vertex, self.fragment, None).unwrap()
    }
}
