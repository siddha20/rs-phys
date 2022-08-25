use crate::driver::world::World;


use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use sdl2::VideoSubsystem;
use sdl2::EventPump;

pub struct Painter {
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    event_pump: EventPump,
    canvas: Canvas<Window>
}

impl Painter {
    pub fn init(window_name: &str) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();
        let window = video_subsystem.window("engine demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        Painter {
            sdl_context:  sdl_context,
            video_subsystem: video_subsystem,
            event_pump: event_pump,
            canvas: canvas
        }
    }

    pub fn test(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();
        let mut i = 0;
        'running: loop {
            i = (i + 1) % 255;
            self.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            self.canvas.clear();
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
            // The rest of the game loop goes here...
            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}