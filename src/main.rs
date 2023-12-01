#![allow(dead_code)]
mod emulator;
mod render;

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

    let mut surface = SurfaceBuilder::new()
        .with_title("CHIRP-8 Emulator")
        .with_size(640, 320)
        .build(&event_loop)?;

    let mut chip8 = Chip::new();
    // chip8.load_rom_from_path(std::path::Path::new("./roms/BC_test.ch8")).unwrap();
    // chip8.load_rom_from_path(std::path::Path::new("./roms/test_opcode.ch8")).unwrap();
    // chip8.load_rom_from_path(std::path::Path::new("./roms/IBM_Logo.ch8")).unwrap();
    // chip8.load_rom_from_path(std::path::Path::new("./roms/Fishie_[Hap,_2005].ch8")).unwrap();
    // chip8.load_rom_from_path(std::path::Path::new("./roms/SQRT_Test_[Sergey_Naydenov,_2010].ch8")).unwrap();
    // chip8.load_rom_from_path(std::path::Path::new("./roms/Trip8_Demo_(2008)_[Revival_Studios].ch8")).unwrap();
    // chip8.load_rom_from_path(std::path::Path::new("./roms/Tetris [Fran Dachille, 1991].ch8")).unwrap();
    // chip8.load_rom_from_path(std::path::Path::new("./roms/Pong (1 player).ch8")).unwrap();

    // Test Suite from https://github.com/Timendus/chip8-test-suite.
    // chip8.load_rom_from_path(std::path::Path::new("./roms/tests/1-chip8-logo.ch8")).unwrap();
    // chip8.load_rom_from_path(std::path::Path::new("./roms/tests/2-ibm-logo.ch8")).unwrap();
    // chip8.load_rom_from_path(std::path::Path::new("./roms/tests/3-corax+.ch8")).unwrap();
    // chip8.load_rom_from_path(std::path::Path::new("./roms/tests/4-flags.ch8")).unwrap();
    chip8.load_rom_from_path(std::path::Path::new("./roms/tests/5-quirks.ch8")).unwrap();
    // chip8.load_rom_from_path(std::path::Path::new("./roms/tests/6-keypad.ch8")).unwrap();
    // chip8.load_rom_from_path(std::path::Path::new("./roms/tests/7-beep.ch8")).unwrap();

    let mut last_cycle = SystemTime::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => surface.resize(size.width, size.height),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => chip8.process_input(input),
                _ => (),
            },
            Event::MainEventsCleared => {
                if let Ok(elapsed) = last_cycle.elapsed() {
                    if elapsed.as_secs_f64() > 1.0 / FRAME_RATE {
                        chip8.cpu_cycle();

                        last_cycle = SystemTime::now();
                    }
                }
                surface.update_with_display(chip8.display());
                surface.request_redraw();
            }
            Event::RedrawRequested(_) => {
                surface.update().unwrap();
                surface.render();
            }
            _ => (),
        }
    });
}
