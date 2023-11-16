
extern crate sdl3;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;

fn main() -> Result<(), String> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Puzzle Game", 640, 480).position_centered().build().map_err(|e|e.to_string())?;
    let mut canvas = window.into_canvas().software().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(sdl3::pixels::Color::RGBA(0, 0, 0, 255));

    let mut event_pump = sdl_context.event_pump()?;

    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { 
                    keycode: Some(Keycode::Escape),
                    ..
                 } => {
                    running = false;
                 }
                 _ => {}
            }
        }

        canvas.clear();

        canvas.present();
    }

    Ok(())
}
