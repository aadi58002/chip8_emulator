const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Screen([[u8; WIDTH]; HEIGHT]);

impl Screen {
    pub fn new() -> Self {
        Screen([[0; WIDTH]; HEIGHT])
    }
    pub fn debug_draw_byte(&mut self, x: u8, y: u8, byte: u8) -> bool {
        let mut flipped = false;
        let bits: Vec<_> = format!("{:08b}", byte).chars().collect();
        for (index, val) in bits.iter().enumerate() {
            let xcor = x as usize + index;
            let ycor = y as usize;
            match *val {
                '0' => {
                    if self.0[ycor][xcor] == 1 {
                        flipped = true;
                    }
                    self.0[ycor][xcor] = 0;
                }
                '1' => self.0[ycor][xcor] = 1,
                _ => unreachable!("Unreachable state"),
            }
        }
        self.print_display();
        flipped
    }

    pub fn clear_display(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.0[y][x] = 0;
            }
        }
    }

    pub fn print_display(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.0[y][x] == 1 {
                    print!("#");
                } else {
                    print!("_");
                }
            }
            println!("");
        }
    }
}

impl Default for Screen {
    fn default() -> Self {
        Screen([[0; WIDTH]; HEIGHT])
    }
}
