mod ram;
mod emulator;
mod cpu;
use emulator::Emulator;

fn main() {
    let mut game = Emulator::new();
    game.write_game_to_ram(Emulator::load_game("../data/INVADERS"));
    loop{
        game.run_instructions();
    }
}
