use crate::display::Display;
use rand::Rng;

pub struct Cpu {
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    stack: [u16; 16],
    stack_pointer: usize,
    delay_timer: u8,
    sound_timer: u8,
    pc: usize,
    pub display: Display,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200,
            display: Display::new(),
        }
    }

    pub fn cycle(&mut self) {
        let opcode: u16 = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16); // Fetch opcode

        // Decode and execute opcode
        match opcode {
            0x0000..=0x0FFF => {
                match opcode & 0x000F {
                    0x0000 => {
                        self.display.clear();
                    }, // 00E0 Clear screen
                    0x000E => { // 00EE Returns from subroutine
                        self.stack_pointer -= 1;
                        self.pc = self.stack[self.stack_pointer] as usize;
                    },
                    _ => (),
                }
                self.pc += 2;
            },
            0x1000..=0x1FFF => self.pc = (opcode & 0x0FFF) as usize, // 1NNN jumps to address NNN
            0x2000..=0x2FFF => { // 2NNN calls subroutine at NNN
                self.stack[self.stack_pointer] = self.pc as u16;
                self.stack_pointer += 1;
                self.pc = (opcode & 0x0FFF) as usize;
            },
            0x3000..=0x3FFF => { // 3XNN skip next instruction if VX = NN
                if self.v[(opcode & 0x0F00) as usize] == (opcode & 0x00FF) as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x4000..=0x4FFF => { // 4XNN skip next instruction if VX != NN
                if self.v[(opcode & 0x0F00) as usize] != (opcode & 0x00FF) as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x5000..=0x5FF0 => { // 5XY0 skip next instruction if VX == VY
                if self.v[(opcode & 0x0F00) as usize] == self.v[(opcode & 0x00F0) as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x6000..=0x6FFF => { // 6XNN set VX to NN
                self.v[(opcode & 0x0F00) as usize] = (opcode & 0x00FF) as u8;
                self.pc += 2;
            },
            0x7000..=0x7FFF => { // 7XNN adds NN to VX
                self.v[(opcode & 0x0F00) as usize] += (opcode & 0x00FF) as u8;
                self.pc += 2;
            },
            0x8000..=0x8FFF => {
                match opcode & 0x000F {
                    0x0 => self.v[(opcode & 0x0F00) as usize]  = self.v[(opcode & 0x00F0) as usize], // 8XY0 set VX to VY
                    0x1 => self.v[(opcode & 0x0F00) as usize] |= self.v[(opcode & 0x00F0) as usize], // 8XY1 set VX to VX or VY
                    0x2 => self.v[(opcode & 0x0F00) as usize] &= self.v[(opcode & 0x00F0) as usize], // 8XY2 set VX to VX and VY
                    0x3 => self.v[(opcode & 0x0F00) as usize] ^= self.v[(opcode & 0x00F0) as usize], // 8XY3 set VX to VX xor VY
                    0x4 => { // 8XY4 add VY to VX. VF = 1 if carry
                        self.v[(opcode & 0x0F00) as usize] += self.v[(opcode & 0x00F0) as usize];
                        self.v[15] = if self.v[(opcode & 0x0F00) as usize] < self.v[(opcode & 0x00F0) as usize] {1} else {0};
                    },
                    0x5 => { // 8XY5 VY subtracted from VX. VF = 0 if borrow
                        self.v[15] = if self.v[(opcode & 0x00F0) as usize] > self.v[(opcode & 0x0F00) as usize] {0} else {1};
                        self.v[(opcode & 0x0F00) as usize] -= self.v[(opcode & 0x00F0) as usize];
                    },
                    0x6 => { // 8XY6 Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
                        self.v[15] = self.v[(opcode & 0x0F00) as usize] & 0x1;
                        self.v[(opcode & 0x0F00) as usize] >>= 1;
                    },
                    0x7 => { // 8XY7 set VX to VY - VX, VF = 0 if borrow
                        self.v[15] = if self.v[(opcode & 0x0F00) as usize] > self.v[(opcode & 0x00F0) as usize] {0} else {1};
                        self.v[(opcode & 0x0F00) as usize] = self.v[(opcode & 0x00F0) as usize] - self.v[(opcode & 0x0F00) as usize];
                    },
                    0xE => { // 8XYE Stores the most significant bit of VX in VF then shifts VX to the left by 1.
                        self.v[15] = self.v[(opcode & 0x0F00) as usize] >> 7;
                        self.v[(opcode & 0x0F00) as usize] <<= 1;
                    }
                    _ => (),
                }
                self.pc += 2;
            },
            0x9000..=0x9FFF => { // 9XY0 Skip next instruction if VX != VY
                if self.v[(opcode & 0x0F00) as usize] != self.v[(opcode & 0x00F0) as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0xA000..=0xAFFF => { // ANNN set I to address NNN
                self.i = opcode & 0x0FFF;
                self.pc += 2;
            },
            0xB000..=0xBFFF => self.pc = (((opcode & 0x0FFF) as u8) + self.v[0]) as usize, // BNNN jumps to address NNN plus V0
            0xC000..=0xCFFF => { // CXNN Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
                let mut rng = rand::thread_rng();
                self.v[(opcode & 0x0F00) as usize] = rng.gen::<u8>() & (opcode & 0x00FF) as u8;
                self.pc += 2;
            },
            // Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels.
            // Each row of 8 pixels is read as bit-coded starting from memory location I; I value doesn’t change after the execution of this instruction.
            // As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that doesn’t happen 
            0xD000..=0xDFFF => { // DXYN

            },
            0xE000..=0xEFFF => { 
                match opcode & 0x000F {
                    0xE => { // EX9E Skips the next instruction if the key stored in VX is pressed. (Usually the next instruction is a jump to skip a code block) 

                    },
                    0x1 => { // EXA1 Skips the next instruction if the key stored in VX isn't pressed. (Usually the next instruction is a jump to skip a code block) 

                    },
                    _ => (),
                }
            },


            _ => (),
        }
    }
}