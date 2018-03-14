#[macro_use] extern crate relm_derive;
#[macro_use] extern crate relm;
pub mod types;
pub mod board;
pub mod gui;
pub mod window;
pub mod player;
pub mod player_example;
pub mod draw;

fn main() {
    window::main();
}
