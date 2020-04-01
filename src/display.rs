pub struct Display {
    data: [u8; 64 * 32],
}

impl Display {
    pub fn new() -> Display {
        Display {
            data: [0; 64 * 32],
        }
    }

    pub fn clear(&mut self) {
        
    }
}