extern crate sdl2;

mod math;
mod engine;

use crate::engine::Engine;

fn create_window(sdl_context: &sdl2::Sdl) -> sdl2::video::Window {
    let video_subsystem = sdl_context.video().unwrap();
    video_subsystem.window("Game", 900, 700)
        .resizable()
        .build()
        .unwrap()
}

fn main_loop(engine_context: &Engine, sdl_context: &sdl2::Sdl) {
    let _window = create_window(sdl_context); // must live
    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {println!("{:?}", event);}
            }
        }
        engine_context.eval();
        engine_context.render();
    }    
}

fn run() {
    main_loop(&Engine::new(), &sdl2::init().unwrap());
}

fn main() {
    run();
}