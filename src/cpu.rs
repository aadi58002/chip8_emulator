use std::fmt::Debug;

use crate::bus::Bus;

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    ret_stack: Vec<u16>,
    i: u16,
}

impl Cpu {
    fn debug_draw_sprites(&mut self, x: u8, y: u8, height: u8, bus: &mut Bus) {
        println!("Sprite draw at : {},{}", x, y);
        let mut should_set_vf = false;
        for y in 0..height {
            let byte = bus.ram_read_byte(self.i as usize + y as usize);
            if bus.debug_draw_byte(x, y, byte) {
                should_set_vf = true;
            }
        }
        if should_set_vf {
            self.write_vx_register(0xF, 0);
        } else {
            self.write_vx_register(0xF, 0);
        }
    }

    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            pc: PROGRAM_START,
            ret_stack: vec![],
            i: 0,
        }
    }

    pub fn read_vx_register(&mut self, index: u8) -> u8 {
        self.vx[index as usize]
    }

    pub fn run_instructions(&mut self, bus: &mut Bus) {
        let lo = bus.ram_read_byte(self.pc as usize) as u16;
        let hi: u16 = bus.ram_read_byte((self.pc + 1) as usize) as u16;
        let ins = (lo << 8) | hi;
        let (nnn, kk, n, x, y) = (
            ins & 0x0FFF,
            (ins & 0x00FF) as u8,
            (ins & 0x000F) as u8,
            ((ins & 0x0F00) >> 8) as u8,
            ((ins & 0x00F0) >> 4) as u8,
        );
        match ins {
            0x0000..=0x0FFF => match kk {
                0xE0 => {
                    //Clear Screen
                    bus.clear_screen();
                }
                0xEE => {
                    //return from subroutine
                    let addr = self.ret_stack.pop().unwrap();
                    self.pc = addr - 2;
                }
                _ => {
                    unreachable!();
                }
            },

            0x1000..=0x1FFF => {
                // goto nnn
                self.pc = nnn - 2;
            }

            0x2000..=0x2FFF => {
                // Calling a function
                self.ret_stack.push(self.pc + 2);
                self.pc = nnn - 2;
            }

            0x3000..=0x3FFF => {
                let vx = self.read_vx_register(x);
                if vx == kk {
                    self.pc += 2;
                }
            }

            0x6000..=0x6FFF => {
                //vx = nn
                self.write_vx_register(x, kk);
            }

            0x7000..=0x7FFF => {
                let vx = self.read_vx_register(x);
                self.write_vx_register(x, vx.wrapping_add(kk));
            }

            0x8000..=0x8FFF => {
                let vx = self.read_vx_register(x);
                let vy = self.read_vx_register(y);
                match n {
                    0 => {
                        // Vx=Vy
                        self.write_vx_register(x, vy);
                    }
                    2 => {
                        // Vx=Vx&Vy
                        self.write_vx_register(x, vx & vy);
                    }
                    3 => {
                        // Vx=Vx^Vy
                        self.write_vx_register(x, vx ^ vy);
                    }
                    4 => {
                        //	Vx += Vy
                        let sum = vx as u16 + vy as u16;
                        self.write_vx_register(x, sum as u8);
                        if sum > 0xFF {
                            self.write_vx_register(0xF, 1);
                        }
                    }
                    5 => {
                        let diff = vx as i8 - vy as i8;
                        self.write_vx_register(x, diff as u8);
                        if diff < 0 {
                            self.write_vx_register(0xF, 1);
                        }
                    }
                    6 => {
                        self.write_vx_register(0xF, vy & 0x1);
                        self.write_vx_register(y, vy >> 1);
                        self.write_vx_register(x, vy >> 1);
                    }
                    _ => {
                        panic!("0x8 case failed");
                    }
                }
            }

            0xA000..=0xAFFF => {
                self.i = nnn;
            }

            0xD000..=0xDFFF => {
                self.debug_draw_sprites(x, y, n, bus);
            }

            0xE000..=0xEFFF => match kk {
                0xA1 => {
                    // if the key is not pressed skip the next instruction
                    let key = self.read_vx_register(x);
                    if !bus.key_pressed(key) {
                        self.pc += 2;
                    }
                }
                0x9E => {
                    // if the key is pressed skip the next instruction
                    let key = self.read_vx_register(x);
                    if bus.key_pressed(key) {
                        self.pc += 2;
                    }
                }
                _ => {
                    panic!("{:#X},{:#X},{:#X},{:#X},{:#X},{:#X}", ins, nnn, kk, n, x, y)
                }
            },

            0xF000..=0xFFFF => match kk {
                0x07 => {
                    self.write_vx_register(x, bus.get_delay_timer());
                }
                0x15 => {
                    bus.set_delay_timer(self.read_vx_register(x));
                }
                0x65 => {
                    for index in 0..x + 1 {
                        let value = bus.ram_read_byte(self.i as usize + index as usize);
                        self.write_vx_register(index, value);
                    }
                }
                0x1E => {
                    let vx = self.read_vx_register(x);
                    self.i += vx as u16;
                }
                _ => {
                    panic!("{:#X}", ins)
                }
            },
            _ => panic!("{:?}\ninstruction is: {:#X}", self, ins),
        }
        self.pc += 2;
    }

    pub fn write_vx_register(&mut self, index: u8, value: u8) {
        self.vx[index as usize] = value;
    }
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:#X}", self.pc)?;
        for item in self.vx.iter() {
            write!(f, "{:#X} ", *item)?;
        }
        writeln!(f, "")?;
        writeln!(f, "{:#X}", self.i)
    }
}
