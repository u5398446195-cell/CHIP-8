pub struct Chipe{
memory: [u8; 4096],
v: [u8; 16],
i: u16,
pc: u16,
stack: [u16; 16],
sp: usize,
pub display: [bool; 64 * 32],
delay_timer: u8,
sound_timer: u8,
pub keys: [bool; 16],
seed: u32,
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
            seed: 0x1234_5678,
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

pub fn tick_timers(&mut self) {
    if self.delay_timer > 0 { self.delay_timer -= 1; }
    if self.sound_timer > 0 { self.sound_timer -= 1; }
}

fn random(&mut self) -> u8 {
    self.seed ^= self.seed << 13;
    self.seed ^= self.seed >> 17;
    self.seed ^= self.seed << 5;
    (self.seed >> 24) as u8
}

pub fn execute(&mut self, opcode:u16){
    let x = ((opcode & 0x0F00) >> 8) as usize;
    let y = ((opcode & 0x00F0) >> 4) as usize;
    let n = (opcode & 0x000F) as u8 ;
    let nn = (opcode & 0x00FF) as u8;
    let nnn = opcode & 0x0FFF;

match opcode >> 12 {
    0x0 => match opcode {
        0x00E0 => self.display.fill(false),
        0x00EE => {
            self.sp -= 1;
            self.pc = self.stack[self.sp];
        }
        _ => {}
    },
    0x1 => self.pc = nnn,
    0x2 => {
        self.stack[self.sp] = self.pc;
        self.sp += 1;
        self.pc = nnn;
    }
    0x3 => if self.v[x] == nn { self.pc += 2 },
    0x4 => if self.v[x] != nn { self.pc += 2 },
    0x5 => if self.v[x] == self.v[y] { self.pc += 2 },
    0x9 => if self.v[x] != self.v[y] { self.pc += 2 },

    0x6 => self.v[x] = nn,
    0x7 => self.v[x] = self.v[x].wrapping_add(nn),

    0x8 => match n {
        0x0 => self.v[x] = self.v[y],
        0x1 => self.v[x] |= self.v[y],
        0x2 => self.v[x] &= self.v[y],
        0x3 => self.v[x] ^= self.v[y],
        0x4 => {
            let (resultat, retenue) = self.v[x].overflowing_add(self.v[y]);
            self.v[x] = resultat;
            self.v[0xF] = retenue as u8;
        }
        0x5 => {
            let (resultat, emprunt) = self.v[x].overflowing_sub(self.v[y]);
            self.v[x] = resultat;
            self.v[0xF] = !emprunt as u8;
        }
        0x6 => {
            let bit = self.v[x] & 1;
            self.v[x] >>= 1;
            self.v[0xF] = bit;
        }
        0x7 => {
            let (resultat, emprunt) = self.v[y].overflowing_sub(self.v[x]);
            self.v[x] = resultat;
            self.v[0xF] = !emprunt as u8;
        }
        0xE => {
            let bit = self.v[x] >> 7;
            self.v[x] <<= 1;
            self.v[0xF] = bit;
        }
        _ => println!("pas encore fait : {:04X}", opcode),
    },

    0xA => self.i = nnn,
    0xB => self.pc = nnn + self.v[0] as u16,
    0xC => self.v[x] = self.random() & nn,

    0xD => {
        let vx = self.v[x] as usize % 64;
        let vy = self.v[y] as usize % 32;
        self.v[0xF] = 0;

        for ligne in 0..n as usize {
            let py = vy + ligne;
            if py >= 32 { break; }
            let octet = self.memory[(self.i as usize + ligne) % 4096];

            for col in 0..8 {
                let px = vx + col;
                if px >= 64 { break; }
                if (octet >> (7 - col)) & 1 == 1 {
                    let index = py * 64 + px;
                    if self.display[index] {
                        self.v[0xF] = 1;
                    }
                    self.display[index] = !self.display[index];
                }
            }
        }
    }

    0xE => match nn {
        0x9E => if self.keys[(self.v[x] & 0xF) as usize] { self.pc += 2 },
        0xA1 => if !self.keys[(self.v[x] & 0xF) as usize] { self.pc += 2 },
        _ => println!("pas encore fait : {:04X}", opcode),
    },

    0xF => match nn {
        0x07 => self.v[x] = self.delay_timer,
        0x15 => self.delay_timer = self.v[x],
        0x18 => self.sound_timer = self.v[x],
        0x0A => match self.keys.iter().position(|&appuyee| appuyee) {
            Some(touche) => self.v[x] = touche as u8,
            None => self.pc -= 2,
        },
        0x1E => self.i = self.i.wrapping_add(self.v[x] as u16),
        0x29 => self.i = 0x050 + (self.v[x] & 0xF) as u16 * 5,
        0x33 => {
            let i = self.i as usize;
            self.memory[i] = self.v[x] / 100;
            self.memory[i + 1] = self.v[x] / 10 % 10;
            self.memory[i + 2] = self.v[x] % 10;
        }
        0x55 => {
            for r in 0..=x { self.memory[self.i as usize + r] = self.v[r]; }
        }
        0x65 => {
            for r in 0..=x { self.v[r] = self.memory[self.i as usize + r]; }
        }
        _ => println!("pas encore fait : {:04X}", opcode),
    },

    _ => println!("pas encore fait : {:04X}", opcode),
}
}


}
