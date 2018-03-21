#[macro_use] extern crate relm_derive;
#[macro_use] extern crate relm;
pub mod types;
pub mod board;
pub mod board_view;
pub mod window;
pub mod player;
pub mod player_example;
pub mod draw;
pub mod errors;

fn main() {
    window::main();
}
