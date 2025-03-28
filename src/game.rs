mod board;
mod playing_cards;

use board::Board;

pub fn init() -> Board {
    Board::init()
}