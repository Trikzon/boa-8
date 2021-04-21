mod opengl;

use glutin::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("CHIRP-8")
        .with_inner_size(LogicalSize::new(640, 480));

    let context = ContextBuilder::new().build_windowed(window_builder, &event_loop)?;
    let context = unsafe { context.make_current().unwrap() };

    println!(
        "Pixel format of the window's GL context: {:?}",
        context.get_pixel_format()
    );

    let gl =
        opengl::bindings::Gl::load_with(|ptr| context.context().get_proc_address(ptr) as *const _);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::MainEventsCleared => {
                context.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                unsafe {
                    gl.ClearColor(0.0, 0.0, 0.0, 1.0);
                    gl.Clear(opengl::bindings::COLOR_BUFFER_BIT);
                }
                context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
