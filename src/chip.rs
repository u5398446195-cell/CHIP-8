use std::fs;

pub struct Chipe{
memory: [u8; 4096],
v: [u8; 16],
i: u16,
pc: u16,
stack: [u16; 16],
sp: u8,
pub display: [bool; 64 * 32],
delay_timer: u8,
sound_timer: u8,
keys: [bool; 16],
}

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9

    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

impl Chipe {
    pub fn new() -> Self {
        let mut memory = [0; 4096];

        for i in 0..FONT.len() {
            memory[0x050 + i] = FONT[i];
        }

           Chipe {
            memory: memory,
            v: [0; 16],
            i: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            display: [false; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            keys: [false; 16],
        }
    }


pub fn load_rom(&mut self, path: &str) {
    let rom = std::fs::read(path).expect("Impossible de lire la ROM");

    for (i, &byte) in rom.iter().enumerate() {
        self.memory[0x200 + i] = byte;
    }
    
}

pub fn fetch(&mut self) -> u16 {
    let mut opcode: u16 = 0;
    opcode |= (self.memory[self.pc as usize] as u16) << 8;
    opcode |= self.memory[self.pc as usize + 1] as u16;
    self.pc = self.pc + 2;
    opcode
}

pub fn execute(&mut self, opcode:u16){
    let x = ((opcode & 0x0F00) >> 8) as usize;
    let y = ((opcode & 0x00F0) >> 4) as usize;
    let n = (opcode & 0x000F) as u8 ;
    let nn = (opcode & 0x00FF) as u8;
    let nnn = opcode & 0x0FFF;

match opcode >> 12 {
    0x6 => self.v[x] = nn,
    0xA => self.i = nnn,
    0x7 => self.v[x] = self.v[x].wrapping_add(nn),
    0x0 => {if opcode ==  0x00E0 { self.display.fill(false); }}
    0x1 => self.pc = nnn,
    0xD => {
        let vx = self.v[x] as usize;
        let vy = self.v[y] as usize;
         self.v[0xF] = 0;

    for ligne in 0..n as usize {
       let octet =self.memory[self.i as usize + ligne];
      
    for col in 0..8 {
       if (octet >> (7 - col) & 1) == 1 {
        let px = (vx+col) % 64;
        let py = (vy+ligne) % 32;
        let index = py * 64 + px;
        if (self.display[index]) == true{
            self.v[0xF] = 1;
           
        }
         self.display[index] = !self.display[index];
       }
    }
    

        
    
    }
    },

    _ => println!("pas encore fait : {:04X}", opcode),
}

    
}


}

