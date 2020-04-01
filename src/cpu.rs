use display::Display;

struct Cpu {
    opcode: u16,
    memory: [u8; 4096],
    V: [u8; 16],
    I: u16,
    stack: [u16; 16],
    stack_pointer: u8,
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    display: Display,
}

impl Chip8 {
    fn new() -> Chip8 {
        Cpu {
            memory: [0; 4096],
            V: [0; 16],
            I: 0,
            stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200,
            display: Display::new(),
        }
    }

    fn exec() {
        
    }

    fn get_opcode(&mut self) {
        self.opcode = (self.memory[self.pc] << 8) | self.memory[self.pc + 1];
    }
}