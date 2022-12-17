use crate::keyboard::*;
use crate::ram::*;
use crate::screen::*;

#[derive(Default)]
pub struct Bus {
    pub ram: Ram,
    pub keyboard: Keyboard,
    pub screen: Screen,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: Ram::new(),
            keyboard: Keyboard::new(),
            screen: Screen::new(),
        }
    }
    pub fn ram_read_byte(&self, address: usize) -> u8 {
        self.ram.read_byte(address)
    }
    pub fn ram_write_byte(&mut self, address: usize, value: u8) {
        self.ram.write_byte(address, value);
    }

    pub fn debug_draw_byte(&mut self,x: u8, y: u8, byte: u8) -> bool{
        self.screen.debug_draw_byte(x,y,byte)
    }

    pub fn clear_screen(&mut self){
        self.screen.clear_display();
    }

    pub fn key_pressed(&self, key_code: u8) -> bool {
        self.keyboard.key_pressed(key_code)
    }
}
