#![allow(dead_code)]
mod emulator;
mod render;
mod util;

use crate::emulator::Chip;
use crate::render::SurfaceBuilder;
use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use std::time::SystemTime;

const FRAME_RATE: f64 = 60.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new();

    let surface = SurfaceBuilder::new()
        .with_title("CHIRP-8")
        .with_size(640, 320)
        .build(&event_loop)?;

    let mut chip8 = Chip::new();

    let mut last_cycle = SystemTime::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => surface.resize(size.width, size.height),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::MainEventsCleared => {
                if let Ok(elapsed) = last_cycle.elapsed() {
                    if elapsed.as_secs_f64() > 1.0 / FRAME_RATE {
                        chip8.cpu_cycle();

                        last_cycle = SystemTime::now();
                    }
                }
                surface.request_redraw()
            }
            Event::RedrawRequested(_) => {
                surface.update().unwrap();
                surface.render();
            }
            _ => (),
        }
    });
}
