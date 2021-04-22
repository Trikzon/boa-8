use crate::render::gl;
use crate::render::Buffer;
use std::rc::Rc;

pub struct VertexArray {
    element_buffer: Option<Rc<Buffer>>,
    array_buffers: [Option<Rc<Buffer>>; 16],

    vertex_array_id: gl::VertexArrayId,
    gl: gl::Gl,
}

impl VertexArray {
    pub fn new(gl: &gl::Gl) -> Self {
        let mut vec = Vec::<Option<Rc<Buffer>>>::with_capacity(16);
        for _ in 0..16 {
            vec.push(None);
        }

        use std::convert::TryInto;
        let empty_array_buffers = vec.try_into().unwrap();

        Self {
            element_buffer: None,
            array_buffers: empty_array_buffers,
            vertex_array_id: gl.create_vertex_array(),
            gl: gl.clone(),
        }
    }

    pub fn put_element_buffer(&mut self, buffer: Buffer) {
        self.put_element_buffer_ref(&Rc::new(buffer));
    }

    pub fn put_element_buffer_ref(&mut self, buffer: &Rc<Buffer>) {
        debug_assert_eq!(buffer.buffer_type(), gl::BufferType::ElementArrayBuffer);

        self.bind();
        buffer.bind();
        self.unbind();
        buffer.unbind();

        self.element_buffer = Some(buffer.clone());
    }

    pub fn put_array_buffer(&mut self, location: usize, buffer: Buffer) {
        self.put_array_buffer_ref(location, &Rc::new(buffer));
    }

    pub fn put_array_buffer_ref(&mut self, location: usize, buffer: &Rc<Buffer>) {
        debug_assert_eq!(buffer.buffer_type(), gl::BufferType::ArrayBuffer);
        debug_assert!(location < 16);

        self.array_buffers[location] = None;

        self.bind();
        buffer.bind();
        self.gl.enable_vertex_attrib(location);
        self.gl
            .vertex_attrib_pointer_f(location, buffer.size(), false, 0, 0);
        self.gl.disable_vertex_attrib(location);
        self.unbind();
        buffer.unbind();

        self.array_buffers[location] = Some(buffer.clone());
    }

    pub fn bind(&self) {
        self.gl.bind_vertex_array(&self.vertex_array_id);
    }

    pub fn unbind(&self) {
        self.gl.unbind_vertex_array();
    }

    pub fn enable_attrib_arrays(&self) {
        for (location, buffer) in self.array_buffers.iter().enumerate() {
            if buffer.is_some() {
                self.gl.enable_vertex_attrib(location);
            }
        }
    }

    pub fn disable_attrib_arrays(&self) {
        for (location, buffer) in self.array_buffers.iter().enumerate() {
            if buffer.is_some() {
                self.gl.disable_vertex_attrib(location);
            }
        }
    }
}
