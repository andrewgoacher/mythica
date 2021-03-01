use crate::engine::core::resource::Resource;

pub struct ShaderBuilder {
    vertex: Option<String>,
    fragment: Option<String>,
}

impl ShaderBuilder {
    pub fn new() -> Self {
        Self {
            vertex: None,
            fragment: None,
        }
    }

    pub fn from_file(resource: &Resource, file: &str) -> Self {
        let vertex_source = resource
            .read_string(&[file, ".vertex"].join("")[..])
            .expect("Vertex shader missing");
        let fragment_source = resource
            .read_string(&[file, ".fragment"].join("")[..])
            .expect("Fragment shader missing");

        Self::new()
            .with_vertex_shader(&vertex_source[..])
            .with_fragment_shader(&fragment_source[..])
    }

    pub fn with_vertex_shader(self, vertex_src: &str) -> Self {
        Self {
            vertex: Some(String::from(vertex_src)),
            ..self
        }
    }

    pub fn with_fragment_shader(self, fragment_src: &str) -> Self {
        Self {
            fragment: Some(String::from(fragment_src)),
            ..self
        }
    }

    pub fn build<T: ?Sized>(self, display: &T) -> glium::Program
    where
        T: glium::backend::Facade,
    {
        let v = self.vertex.unwrap();
        let f = self.fragment.unwrap();

        glium::Program::from_source(display, &v[..], &f[..], None).unwrap()
    }
}
