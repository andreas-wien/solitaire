use std::io;

mod game;
mod utility;

pub fn start_game() {
    let board = game::init();
    // loop {
        let playable_cards = board.get_playable_cards();
        // println!("{:#?}", playable_cards);
        board.draw_command_line_gui();
        println!("\nPlease select a card: ");
        // let mut card_selection = String::new();
        // io::stdin().read_line(&mut card_selection);
    // }
}
