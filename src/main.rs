#![allow(dead_code)]
mod render;

use crate::render::DisplayBuilder;
use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new();

    let display = DisplayBuilder::new()
        .with_title("CHIRP-8")
        .with_size(640, 320)
        .build(&event_loop)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => display.resize(size.width, size.height),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::MainEventsCleared => display.request_redraw(),
            Event::RedrawRequested(_) => {
                display.update().unwrap();
                display.render();
            }
            _ => (),
        }
    });
}
