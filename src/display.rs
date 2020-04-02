use sdl2::{video, rect};

pub struct Display {
    data: [u8; 64 * 32],
    draw: bool,
}

impl Display {
    pub fn new() -> Display {
        Display {
            data: [0; 64 * 32],
            draw: true,
        }
    }

    pub fn clear(&mut self) {
        
    }

    pub fn draw() {

    }
}