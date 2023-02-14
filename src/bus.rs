use crate::keyboard::*;
use crate::ram::*;
use crate::screen::*;
use core::fmt;

#[derive(Default)]
pub struct Bus {
    pub ram: Ram,
    pub keyboard: Keyboard,
    pub screen: Screen,
    pub delay_timer: u8,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: Ram::new(),
            keyboard: Keyboard::new(),
            screen: Screen::new(),
            delay_timer: 0,
        }
    }
    pub fn ram_read_byte(&self, address: usize) -> u8 {
        self.ram.read_byte(address)
    }
    pub fn ram_write_byte(&mut self, address: usize, value: u8) {
        self.ram.write_byte(address, value);
    }

    pub fn debug_draw_byte(&mut self, x: u8, y: u8, byte: u8) -> bool {
        self.screen.debug_draw_byte(x, y, byte)
    }

    pub fn clear_screen(&mut self) {
        self.screen.clear_display();
    }

    pub fn key_pressed(&self, key_code: u8) -> bool {
        self.keyboard.key_pressed(key_code)
    }

    pub fn tick(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }

    pub fn set_delay_timer(&mut self, value: u8) {
        self.delay_timer = value;
    }

    pub fn get_delay_timer(&self) -> u8 {
        self.delay_timer
    }
}

impl fmt::Debug for Bus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Delay time: {}", self.delay_timer)
    }
}
