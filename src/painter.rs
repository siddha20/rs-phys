use crate::driver::world::World;

use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
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
    pub fn init(window_name: &str, height: u32, width: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();
        let window = video_subsystem.window(window_name, height, width)
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

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }
    
    pub fn paint(&mut self, world: &World) {
        for e in world.get_ents() {
            self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            // println!("x: {}, y: {}", e.pos[0] as i32, e.pos[1] as i32);
            self.canvas.fill_rect(Rect::new(e.pos[0] as i32, 
                                            e.pos[1] as i32, 
                                            e.shape.height, 
                                            e.shape.width))
                       .unwrap();
        }
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn check_quit(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return true;
                },
                _ => {}
            }
        }
        false
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