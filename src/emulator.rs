use crate::ram::Ram;
use crate::cpu::{Cpu, PROGRAM_START};
use std::fs;

pub struct Emulator {
    pub ram: Ram,
    pub cpu: Cpu,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator { ram: Ram::new() ,cpu: Cpu::new()}
    }
    pub fn load_game(name: &str) -> Vec<u8> {
        fs::read(name).unwrap()
    }
    pub fn write_game_to_ram(&mut self, game: Vec<u8>) {
        for (i, byte) in game.iter().enumerate() {
            self.ram.write_byte(PROGRAM_START as usize + i, *byte);
        }
    }
    pub fn run_instructions(&mut self){
        self.cpu.run_instructions(&mut self.ram)
    }
}
