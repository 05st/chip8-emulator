use cpu::Cpu;
use sdl2;

mod cpu;

pub struct Display {
    data: [u8; 64 * 32],
    draw: bool,
    window: sdl2::video::Window,
}

impl Display {
    pub fn new(_sdl: sdl2::Sdl) -> Display {
        
        let video_subsystem = _sdl.video().unwrap();

        Display {
            data: [0; 64 * 32],
            draw: true,
            window: video_subsystem.window("CHIP-8 Emulator", 1280, 720).build().unwrap(),
        }
    }

    pub fn clear(&mut self) {
        
    }

    pub fn draw() {

    }
}

fn main() {
    let _sdl = sdl2::init().unwrap();
    let mut event_pump = _sdl.event_pump().unwrap();

    let chip8: Cpu = Cpu::new();
    let display: Display = Display::new(_sdl);

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }
    }
}
