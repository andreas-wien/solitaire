use super::super::utility;
use super::playing_cards::{Card, Deck};

#[derive(Debug)]
pub struct Board {
    deck: Deck,
    play_area: Vec<Vec<Card>>,
    goal_area: Vec<Vec<Card>>,
}

impl Board {
    pub fn init() -> Board {
        let mut board = Board {
            deck: Deck::new(),
            play_area: Vec::new(),
            goal_area: Vec::new(),
        };

        for _ in 0..7 {
            board.play_area.push(Vec::new());
        }

        for _ in 0..4 {
            board.goal_area.push(Vec::new());
        }

        utility::shuffle(&mut board.deck.cards);

        for i in 0..7 {
            for j in i..7 {
                let mut card = match board.deck.cards.pop() {
                    Some(card) => card,
                    None => panic!("Not enough cards to initialize the game!"),
                };
                if i == j {
                    card.reveal();
                }
                board.play_area[j].push(card);
            }
        }

        board
    }

    pub fn get_stack(&self, area_selector: u8) -> Result<&Vec<Card>, String> {
        if area_selector > 0 && area_selector < 8 {
            Ok(&self.play_area[area_selector as usize - 1])
        } else if area_selector > 7 && area_selector < 13 {
            Ok(&self.goal_area[area_selector as usize - 1])
        } else {
            Err(String::from("No area with that index"))
        }
    }

    pub fn get_first_card_on_stack<'a>(&self, stack: &'a Vec<Card>) -> &'a Card {
        stack.last().unwrap()
    }

    pub fn add_card_to_play_area(
        &mut self,
        play_area_index: usize,
        card: Card,
    ) -> Result<String, String> {
        match self.play_area[play_area_index].last() {
            Some(last_card) => {
                if last_card.get_color() != card.get_color()
                    && last_card.get_value() - card.get_value() == 1
                {
                    self.play_area[play_area_index].push(card);
                    Ok(format!("Card was added to stack {}", play_area_index))
                } else {
                    Err(format!("Card was not added, because of an illegal move"))
                }
            }
            _ => {
                self.play_area[play_area_index].push(card);
                Ok(format!("Card was added to stack {}", play_area_index))
            }
        }
    }
    pub fn add_card_to_goal_area(
        &mut self,
        goal_area_index: usize,
        card: Card,
    ) -> Result<String, String> {
        match self.goal_area[goal_area_index].last() {
            Some(last_card) => {
                if card.get_suit() == last_card.get_suit()
                    && card.get_value() - last_card.get_value() == 1
                {
                    self.goal_area[goal_area_index].push(card);
                    Ok(format!("Card was added to goal stack {}", goal_area_index))
                } else {
                    Err(format!("Card was not added, because of an illegal move"))
                }
            }
            _ => {
                if card.get_value() == 1 {
                    self.goal_area[goal_area_index].push(card);
                    Ok(format!("Card was added to goal stack {}", goal_area_index))
                } else {
                    Err(format!("Card was not added, because of an illegal move"))
                }
            }
        }
    }
    fn reveal_cards_in_play_area(&mut self) {
        for area in &mut self.play_area {
            if let Some(card) = area.last_mut() {
                card.reveal();
            }
        }
    }

    pub fn get_playable_cards(&self) -> Vec<&Card> {
        let mut playable_cards: Vec<&Card> = Vec::new();
        for area in &self.play_area {
            area.iter()
                .filter(|card| card.get_is_hidden() == false)
                .for_each(|card| playable_cards.push(card));
        }

        playable_cards
    }
    pub fn draw_command_line_gui(&self) {
        let playing_card_back = '\u{1F0A0}';
        let no_card = ' ';
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        // Print deck
        if self.deck.cards.len() > 0 {
            print!("{playing_card_back} ")
        } else {
            print!("{no_card}");
        }
        print!(" ");

        // Print zone for newly drawn cards from deck
        // TODO: logic for drawing cards from deck
        print!("    ");

        // Print goal zones
        for goal_area in &self.goal_area {
            let sym = match goal_area.last() {
                Some(card) => card.get_unicode_symbol(),
                None => no_card,
            };
            print!("{} ", sym);
        }
        print!("\n");

        // Print selectors for areas
        println!("0   8       9   10  11  12\n");

        let mut longest_play_area = 0;
        for play_area in &self.play_area {
            if play_area.len() > longest_play_area {
                longest_play_area = play_area.len();
            }
        }

        // Print play area
        let mut n = 0;
        while n != longest_play_area {
            let mut i = 0;
            for play_area in &self.play_area {
                if i != 0 {
                    print!("   ");
                }
                if play_area.len() == 0 || n > play_area.len() - 1 {
                    print!("{no_card}");
                } else {
                    let sym = if play_area[n].get_is_hidden() == false {
                        play_area[n].get_unicode_symbol()
                    } else {
                        playing_card_back
                    };
                    print!("{sym}");
                }
                i += 1;
            }
            print!("\n");
            n += 1;
        }

        // Print selectors for areas
        println!("1   2   3   4   5   6   7\n");
    }
}
