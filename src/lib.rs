use std::io;

mod game;
mod utility;

pub fn start_game() {
    let board = game::init();
    // loop {
    let playable_cards = board.get_playable_cards();
    // println!("{:#?}", playable_cards);
    board.draw_command_line_gui();
    loop {
        println!("\nPlease select a card: ");
        let mut card_selection = String::new();
        io::stdin().read_line(&mut card_selection);
        let card_selection: i8 = match card_selection.parse::<i8>() {
            Ok(selection) => selection,
            Err(_) => {
                println!("That's not a valid move");
                -1
            }
        };

        

        if card_selection != -1 {
            break;
        }
    }
    // }
}
