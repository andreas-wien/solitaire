use std::io;

mod game;
mod utility;

pub fn start_game() {
    let mut board = game::init();
    // loop {
    let playable_cards = board.get_playable_cards();
    // println!("{:#?}", playable_cards);
    board.draw_command_line_gui();
    'outer: loop {
        println!("Please select a card: ");

        let mut card_selection = String::new();

        io::stdin()
            .read_line(&mut card_selection)
            .expect("Failed to read line");

        let card_selection: u8 = match card_selection.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error");
                continue;
            }
        };

        if card_selection > 0 && card_selection < 8 {
            // Card in play area selected

            loop {
                println!("Please select target: ");

                let mut area_selection = String::new();

                io::stdin()
                    .read_line(&mut area_selection)
                    .expect("Failed to read line");

                let area_selection: u8 = match area_selection.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let stack = board
                    .get_stack(area_selection)
                    .expect("Valid area should be supplied");
                if area_selection > 8 && area_selection < 13 {
                    // Goal area
                } else if area_selection > 0 && area_selection < 8 {
                    // Play area
                    match board.add_card_to_play_area(
                        area_selection as usize - 1,
                        *board.get_first_card_on_stack(stack),
                    ) {
                        Ok(msg) => println!("{msg}"),
                        Err(msg) => {
                            println!("{msg}");
                            continue 'outer;
                        }
                    }
                }
            }
        }
    }

    // }
}
