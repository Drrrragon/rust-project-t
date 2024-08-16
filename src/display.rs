pub mod board;
pub mod cursor;
pub mod common;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub(crate) fn render(layer: &Vec<Vec<char>>) {
    board::clear_board();
    disable_raw_mode().expect("Failed to disable raw mode");
    for row in layer.iter() {
        for col in row.iter() {
            print!("{}", col);
        }
        println!()
    }
    enable_raw_mode().expect("Failed to enable raw mode");
}