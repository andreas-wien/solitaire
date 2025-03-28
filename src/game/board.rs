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
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!(" ---\t ---\t\t ---\t ---\t ---\t ---");
        println!("| # |\t|   |\t\t|   |\t|   |\t|   |\t|   |");
        println!("| # |\t|   |\t\t|   |\t|   |\t|   |\t|   |");
        println!(" ---\t ---\t\t ---\t ---\t ---\t ---");
        println!("  0\t  8\t\t  x\t  x\t  x\t  x");

        println!(" ---\t ---\t ---\t ---\t ---\t ---\t ---");
        println!("|   |\t|   |\t|   |\t|   |\t|   |\t|   |\t|   |");
        println!("|   |\t|   |\t|   |\t|   |\t|   |\t|   |\t|   |");
        println!(" ---\t ---\t ---\t ---\t ---\t ---\t ---");
        println!("  1\t  2\t  3\t  4\t  5\t  6\t  7")
    }
}
