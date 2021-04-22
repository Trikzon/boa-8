use crate::render::bindings;
use bitflags::bitflags;
use std::ffi::CString;
use std::rc::Rc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GlError {
    #[error("OpenGL failed to create a new shader object")]
    CreateShader,
    #[error("OpenGL failed to compile the given shader's source. Info log: {0}")]
    CompileShader(String),

    #[error("OpenGl failed to create a new program object")]
    CreateProgram,
    #[error("OpenGl failed to link the given program's shaders. Info log: {0}")]
    LinkProgram(String),

    #[error("uniform name is invalid: {0}")]
    InvalidUniformname(String),

    #[error("failed to convert &str into CString because it contains an interior nul byte")]
    NulByteInStr(#[from] std::ffi::NulError),
    #[error("failed to get Utf8 str from OpenGl")]
    Utf8(#[from] std::str::Utf8Error),
}

#[derive(Clone)]
pub struct Gl {
    gl: Rc<bindings::Gl>,
}

impl Gl {
    pub fn load_with<F>(load_fn: F) -> Self
    where
        F: FnMut(&'static str) -> *const std::ffi::c_void,
    {
        Self {
            gl: Rc::new(bindings::Gl::load_with(load_fn)),
        }
    }
}

bitflags! {
    pub struct ClearFlag: u32 {
        const COLOR_BUFFER = bindings::COLOR_BUFFER_BIT;
        const DEPTH_BUFFER = bindings::DEPTH_BUFFER_BIT;
        const STENCIL_BUFFER = bindings::STENCIL_BUFFER_BIT;
    }
}

impl Gl {
    #[inline]
    pub fn set_clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe { self.gl.ClearColor(red, green, blue, alpha) };
    }

    #[inline]
    pub fn clear(&self, clear_flags: &[ClearFlag]) {
        let mut mask = 0;
        clear_flags.iter().for_each(|flag| mask |= flag.bits);
        unsafe { self.gl.Clear(mask) };
    }

    #[inline]
    pub fn set_view_port(&self, x: u32, y: u32, width: u32, height: u32) {
        unsafe {
            self.gl
                .Viewport(x as i32, y as i32, width as i32, height as i32)
        };
    }
}

pub struct ShaderId {
    id: u32,
}

#[derive(Debug)]
#[repr(u32)]
pub enum ShaderType {
    Vertex,
    Fragment,
    Geometry,
    Compute,
    TessControl,
    TessEvaluation,
}

impl ShaderType {
    pub fn value(&self) -> u32 {
        match *self {
            ShaderType::Vertex => bindings::VERTEX_SHADER,
            ShaderType::Fragment => bindings::FRAGMENT_SHADER,
            ShaderType::Geometry => bindings::GEOMETRY_SHADER,
            ShaderType::Compute => bindings::COMPUTE_SHADER,
            ShaderType::TessControl => bindings::TESS_CONTROL_SHADER,
            ShaderType::TessEvaluation => bindings::TESS_EVALUATION_SHADER,
        }
    }
}

#[inline]
fn convert_str_into_c_string(str: &str) -> Result<CString, GlError> {
    CString::new(str.as_bytes()).map_err(|e| GlError::NulByteInStr(e))
}

unsafe fn get_info_log<'a>(gl: &Gl, is_shader: bool, id: u32) -> Result<String, GlError> {
    let mut len: i32 = 0;
    if is_shader {
        gl.gl.GetShaderiv(id, bindings::INFO_LOG_LENGTH, &mut len);
    } else {
        gl.gl.GetProgramiv(id, bindings::INFO_LOG_LENGTH, &mut len);
    }
    let len = len as usize;

    let mut info_log: Vec<u8> = Vec::with_capacity(len + 1);
    info_log.extend([b' '].iter().cycle().take(len));

    if is_shader {
        gl.gl.GetShaderInfoLog(
            id,
            len as i32,
            std::ptr::null_mut(),
            info_log.as_mut_ptr() as *mut bindings::types::GLchar,
        );
    } else {
        gl.gl.GetProgramInfoLog(
            id,
            len as i32,
            std::ptr::null_mut(),
            info_log.as_mut_ptr() as *mut bindings::types::GLchar,
        )
    }

    Ok(std::str::from_utf8(&info_log)?.to_string())
}

impl Gl {
    #[inline]
    pub fn create_shader(&self, shader_type: ShaderType) -> Result<ShaderId, GlError> {
        let id = unsafe { self.gl.CreateShader(shader_type.value()) };
        if id == 0 {
            Err(GlError::CreateShader)
        } else {
            Ok(ShaderId { id })
        }
    }

    #[inline]
    pub fn delete_shader(&self, shader: &ShaderId) {
        unsafe { self.gl.DeleteShader(shader.id) }
    }

    #[inline]
    pub fn set_shader_source(&self, shader: &ShaderId, source: &str) -> Result<(), GlError> {
        let source = convert_str_into_c_string(source)?;
        unsafe {
            self.gl
                .ShaderSource(shader.id, 1, &source.as_ptr(), std::ptr::null())
        };
        Ok(())
    }

    pub fn compile_shader(&self, shader: &ShaderId) -> Result<(), GlError> {
        unsafe { self.gl.CompileShader(shader.id) };

        let mut success = bindings::TRUE as i32;
        unsafe {
            self.gl
                .GetShaderiv(shader.id, bindings::COMPILE_STATUS, &mut success)
        };

        if success == bindings::TRUE as i32 {
            Ok(())
        } else {
            let info_log = unsafe { get_info_log(self, true, shader.id) }?;
            Err(GlError::CompileShader(info_log.to_string()))
        }
    }
}

pub struct ProgramId {
    id: u32,
}

impl Gl {
    #[inline]
    pub fn create_program(&self) -> Result<ProgramId, GlError> {
        let id = unsafe { self.gl.CreateProgram() };
        if id == 0 {
            Err(GlError::CreateProgram)
        } else {
            Ok(ProgramId { id })
        }
    }

    #[inline]
    pub fn delete_program(&self, program: &ProgramId) {
        unsafe { self.gl.DeleteProgram(program.id) };
    }

    #[inline]
    pub fn attach_shader(&self, program: &ProgramId, shader: &ShaderId) {
        unsafe { self.gl.AttachShader(program.id, shader.id) };
    }

    #[inline]
    pub fn detach_shader(&self, program: &ProgramId, shader: &ShaderId) {
        unsafe { self.gl.DetachShader(program.id, shader.id) };
    }

    pub fn link_program(&self, program: &ProgramId) -> Result<(), GlError> {
        unsafe { self.gl.LinkProgram(program.id) };

        let mut success = bindings::TRUE as i32;
        unsafe {
            self.gl
                .GetProgramiv(program.id, bindings::LINK_STATUS, &mut success)
        };

        if success == bindings::TRUE as i32 {
            Ok(())
        } else {
            let info_log = unsafe { get_info_log(self, false, program.id)? };
            Err(GlError::LinkProgram(info_log))
        }
    }

    #[inline]
    pub fn bind_program(&self, program: &ProgramId) {
        unsafe { self.gl.UseProgram(program.id) };
    }

    #[inline]
    pub fn unbind_program(&self) {
        unsafe { self.gl.UseProgram(0) };
    }
}

pub struct UniformLocationId {
    id: i32,
}

impl Gl {
    #[inline]
    pub fn get_uniform_location(
        &self,
        program_id: &ProgramId,
        name: &str,
    ) -> Result<UniformLocationId, GlError> {
        let c_name = convert_str_into_c_string(name)?;
        let location = unsafe { self.gl.GetUniformLocation(program_id.id, c_name.as_ptr()) };
        if location == -1 {
            Err(GlError::InvalidUniformname(name.to_string()))
        } else {
            Ok(UniformLocationId { id: location })
        }
    }

    #[inline]
    pub fn upload_uniform_location_1f(&self, uniform: &UniformLocationId, value: f32) {
        unsafe { self.gl.Uniform1f(uniform.id, value) };
    }

    #[inline]
    pub fn upload_uniform_location_2f(&self, uniform: &UniformLocationId, value: (f32, f32)) {
        unsafe { self.gl.Uniform2f(uniform.id, value.0, value.1) };
    }

    #[inline]
    pub fn upload_uniform_location_3f(&self, uniform: &UniformLocationId, value: (f32, f32, f32)) {
        unsafe { self.gl.Uniform3f(uniform.id, value.0, value.1, value.2) };
    }
}

pub struct VertexArrayId {
    id: u32,
}

impl Gl {
    #[inline]
    pub fn create_vertex_array(&self) -> VertexArrayId {
        let mut id: u32 = 0;
        unsafe { self.gl.GenVertexArrays(1, &mut id) };
        VertexArrayId { id }
    }

    #[inline]
    pub fn delete_vertex_array(&self, vertex_array: &VertexArrayId) {
        unsafe { self.gl.DeleteVertexArrays(1, [vertex_array.id].as_ptr()) };
    }

    #[inline]
    pub fn bind_vertex_array(&self, vertex_array: &VertexArrayId) {
        unsafe { self.gl.BindVertexArray(vertex_array.id) };
    }

    #[inline]
    pub fn unbind_vertex_array(&self) {
        unsafe { self.gl.BindVertexArray(0) };
    }

    #[inline]
    pub fn vertex_attrib_pointer_float(
        &self,
        location: usize,
        size: usize,
        normalized: bool,
        stride: usize,
        offset: usize,
    ) {
        debug_assert!(location < 16);
        debug_assert!(size < 5);
        unsafe {
            self.gl.VertexAttribPointer(
                location as u32,
                size as i32,
                bindings::FLOAT,
                normalized as u8,
                (stride * std::mem::size_of::<f32>()) as i32,
                (offset * std::mem::size_of::<f32>()) as *const bindings::types::GLvoid,
            )
        }
    }

    #[inline]
    pub fn enable_vertex_attrib(&self, location: usize) {
        debug_assert!(location < 16);
        unsafe { self.gl.EnableVertexAttribArray(location as u32) };
    }

    #[inline]
    pub fn disable_vertex_attrib(&self) {
        unsafe { self.gl.EnableVertexAttribArray(0) };
    }
}

pub struct BufferId {
    id: u32,
}

pub enum BufferType {
    ArrayBuffer,
    ElementArrayBuffer,
}

impl BufferType {
    pub fn value(&self) -> u32 {
        match *self {
            BufferType::ArrayBuffer => bindings::ARRAY_BUFFER,
            BufferType::ElementArrayBuffer => bindings::ELEMENT_ARRAY_BUFFER,
        }
    }
}

impl Gl {
    #[inline]
    pub fn create_buffer(&self) -> BufferId {
        let mut id: u32 = 0;
        unsafe { self.gl.GenBuffers(1, &mut id) };
        BufferId { id }
    }

    #[inline]
    pub fn delete_buffer(&self, buffer: &BufferId) {
        unsafe { self.gl.DeleteBuffers(1, [buffer.id].as_ptr()) };
    }

    #[inline]
    pub fn bind_buffer(&self, buffer_type: BufferType, buffer: &BufferId) {
        unsafe { self.gl.BindBuffer(buffer_type.value(), buffer.id) };
    }

    #[inline]
    pub fn unbind_buffer(&self, buffer_type: BufferType) {
        unsafe { self.gl.BindBuffer(buffer_type.value(), 0) }
    }

    #[inline]
    pub fn create_static_buffer_data<T>(&self, buffer_type: BufferType, data: &[T]) {
        unsafe {
            self.gl.BufferData(
                buffer_type.value(),
                (data.len() * std::mem::size_of::<T>()) as isize,
                data.as_ptr() as *const bindings::types::GLvoid,
                bindings::STATIC_DRAW,
            )
        }
    }
}
