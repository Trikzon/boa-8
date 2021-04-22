use crate::render::gl;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShaderError {
    #[error("combo shader is missing a type header")]
    MissingTypeHeader,
    #[error("combo shader type header: {0} is invalid")]
    InvalidTypeHeader(String),
    #[error("shader builder is missing shader type: {0}")]
    MissingShaderType(u32),
    #[error("encountered a gl error")]
    GlError(#[from] gl::GlError),
}

pub struct ShaderProgram {
    program_id: gl::ProgramId,
    uniform_locations: HashMap<String, gl::UniformLocationId>,
    gl: gl::Gl,
}

impl ShaderProgram {
    pub fn new(builder: ProgramBuilder, gl: &gl::Gl) -> Result<Self, ShaderError> {
        let vertex_source = builder.vertex.ok_or(ShaderError::MissingShaderType(
            gl::ShaderType::Vertex.value(),
        ))?;
        let fragment_source = builder.fragment.ok_or(ShaderError::MissingShaderType(
            gl::ShaderType::Fragment.value(),
        ))?;

        let vertex = Self::compile_shader(gl, gl::ShaderType::Vertex, vertex_source)?;
        let fragment = Self::compile_shader(gl, gl::ShaderType::Fragment, fragment_source)?;

        let program = Self::link_shaders(gl, &[vertex, fragment])?;

        Ok(Self {
            program_id: program,
            uniform_locations: HashMap::new(),
            gl: gl.clone(),
        })
    }

    pub fn compile_shader(
        gl: &gl::Gl,
        shader_type: gl::ShaderType,
        source: String,
    ) -> Result<gl::ShaderId, ShaderError> {
        let id = gl.create_shader(shader_type)?;

        gl.set_shader_source(&id, &source)?;
        gl.compile_shader(&id)?;

        Ok(id)
    }

    pub fn link_shaders(
        gl: &gl::Gl,
        shaders: &[gl::ShaderId],
    ) -> Result<gl::ProgramId, ShaderError> {
        let program = gl.create_program()?;

        for shader in shaders {
            gl.attach_shader(&program, shader);
        }

        gl.link_program(&program)?;

        for shader in shaders {
            gl.detach_shader(&program, shader);
            gl.delete_shader(shader);
        }

        Ok(program)
    }

    pub fn bind(&self) {
        self.gl.bind_program(&self.program_id);
    }

    pub fn unbind(&self) {
        self.gl.unbind_program();
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        self.unbind();
        self.gl.delete_program(&self.program_id)
    }
}

pub struct ProgramBuilder {
    vertex: Option<String>,
    fragment: Option<String>,
}

impl ProgramBuilder {
    fn new() -> Self {
        Self {
            vertex: None,
            fragment: None,
        }
    }

    pub fn with_vertex<S: Into<String>>(mut self, source: S) -> Self {
        self.vertex = Some(source.into());
        self
    }

    pub fn with_fragment<S: Into<String>>(mut self, source: S) -> Self {
        self.fragment = Some(source.into());
        self
    }

    /// Source contains multiple shaders marked with their `#type` tags.
    pub fn with_combo<S: Into<String>>(mut self, source: S) -> Result<Self, ShaderError> {
        for shader in source.into().split("#type ") {
            if shader.trim().is_empty() {
                continue;
            }

            let end_of_type_header = match shader.find(|s| s == ' ' || s == '\n') {
                Some(index) => index,
                None => return Err(ShaderError::MissingTypeHeader),
            };
            let type_header = shader[0..end_of_type_header].trim();
            let source = &shader[end_of_type_header + 1..];

            match type_header {
                "vertex" => self.vertex = Some(source.to_string()),
                "fragment" => self.fragment = Some(source.to_string()),
                _ => return Err(ShaderError::InvalidTypeHeader(type_header.to_string())),
            }
        }
        Ok(self)
    }

    pub fn build(self, gl: &gl::Gl) -> Result<ShaderProgram, ShaderError> {
        ShaderProgram::new(self, gl)
    }
}
