extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use draw::to_coord_u32;

const BACK_COLOR: Color = [0.0, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (20, 20);

    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        [to_coord_u32(width), to_coord_u32(height)],
    ).exit_on_esc (true)
        .build()
        .unwrap();

    let mut game = Game::new(width, height);
    

}
