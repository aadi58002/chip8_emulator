#[derive(Default)]
pub struct Keyboard {}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {}
    }
    pub fn key_pressed(&self, key_code: u8) -> bool {
        true
    }
}
