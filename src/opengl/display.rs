use glutin::{
    dpi::{LogicalSize, PhysicalSize},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
    ContextBuilder, ContextWrapper, PossiblyCurrent,
};
use thiserror::Error;

use super::bindings;
use super::bindings::Gl;

#[derive(Debug, Error)]
pub enum DisplayError {
    #[error("failed to create the glutin window")]
    WindowCreation,
    #[error("failed to make the glutin window current")]
    ContextCurrent,
    #[error("failed to swap glutin window's buffers")]
    SwapBuffers,
}

pub struct Display {
    context: ContextWrapper<PossiblyCurrent, Window>,
    clear_color: (f32, f32, f32),
    gl: Gl,
}

impl Display {
    pub fn new<T>(
        builder: DisplayBuilder,
        event_loop: &EventLoop<T>,
    ) -> Result<Self, DisplayError> {
        let title = builder.title.unwrap_or("CHIRP-8".to_string());
        let size = builder.size.unwrap_or((640, 480));

        let context = ContextBuilder::new()
            .build_windowed(
                WindowBuilder::new()
                    .with_title(title)
                    .with_inner_size(LogicalSize::new(size.0, size.1)),
                event_loop,
            )
            .map_err(|_| DisplayError::WindowCreation)?;
        let context = unsafe {
            context
                .make_current()
                .map_err(|_| DisplayError::ContextCurrent)?
        };

        let gl = Gl::load_with(|ptr| context.get_proc_address(ptr) as *const _);

        Ok(Self {
            context,
            clear_color: (0.0, 0.0, 0.0),
            gl,
        })
    }

    pub fn resize(&self, width: u32, height: u32) {
        self.context.resize(PhysicalSize::new(width, height));
    }

    pub fn request_redraw(&self) {
        self.context.window().request_redraw();
    }

    pub fn update(&self) -> Result<(), DisplayError> {
        unsafe {
            self.gl.ClearColor(
                self.clear_color.0,
                self.clear_color.1,
                self.clear_color.2,
                1.0,
            );
            self.gl.Clear(bindings::COLOR_BUFFER_BIT);
        }
        self.context
            .swap_buffers()
            .map_err(|_| DisplayError::SwapBuffers)?;

        Ok(())
    }
}

pub struct DisplayBuilder {
    title: Option<String>,
    size: Option<(u32, u32)>,
}

impl DisplayBuilder {
    pub fn new() -> Self {
        Self {
            title: None,
            size: None,
        }
    }

    pub fn with_title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.size = Some((width, height));
        self
    }

    pub fn build<T>(self, event_loop: &EventLoop<T>) -> Result<Display, DisplayError> {
        Display::new(self, event_loop)
    }
}
