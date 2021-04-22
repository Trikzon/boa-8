use crate::render::gl;

#[derive(Debug)]
pub struct Buffer {
    len: usize,
    size: usize,

    buffer_id: gl::BufferId,
    buffer_type: gl::BufferType,

    gl: gl::Gl,
}

impl Buffer {
    pub fn new_element_buffer(gl: &gl::Gl, data: &[u32]) -> Self {
        Buffer::from_array(gl, gl::BufferType::ElementArrayBuffer, data, 0)
    }

    pub fn new_array_buffer(gl: &gl::Gl, data: &[f32], size: usize) -> Self {
        Buffer::from_array(gl, gl::BufferType::ArrayBuffer, data, size)
    }

    pub fn from_array<T>(
        gl: &gl::Gl,
        buffer_type: gl::BufferType,
        data: &[T],
        size: usize,
    ) -> Self {
        let buffer = Buffer {
            len: data.len(),
            size,
            buffer_id: gl.create_buffer(),
            buffer_type,
            gl: gl.clone(),
        };

        buffer.bind();
        gl.create_static_buffer_data(buffer.buffer_type, &data);
        buffer.unbind();

        buffer
    }

    pub fn bind(&self) {
        self.gl.bind_buffer(self.buffer_type, &self.buffer_id);
    }

    pub fn unbind(&self) {
        self.gl.unbind_buffer(self.buffer_type);
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn buffer_type(&self) -> gl::BufferType {
        self.buffer_type
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        self.gl.delete_buffer(&self.buffer_id);
    }
}
