use crate::ram::Ram;

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
        }
    }

    pub fn run_instructions(&mut self,ram: &mut Ram){
        let lo = ram.read_byte(self.pc as usize) as u16;
        let hi = ram.read_byte((self.pc+1) as usize) as u16;
        let ins = (lo << 8) | hi;
        println!("ins: {:X},lo: {:X},hi: {:X}",ins,lo,hi);
        if lo == 0 && hi == 0{
            panic!();
        }
        self.pc +=2;
    }
}
