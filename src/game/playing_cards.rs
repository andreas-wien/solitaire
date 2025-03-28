use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone)]
pub struct Card {
    rank: CardRanks,
    suit: CardSuits,
    value: u8,
    color: Color,
    is_hidden: bool,
}

impl Card {
    fn new(rank: CardRanks, suit: CardSuits) -> Card {
        Card {
            value: match rank {
                CardRanks::Ace => 1,
                CardRanks::Two => 2,
                CardRanks::Three => 3,
                CardRanks::Four => 4,
                CardRanks::Five => 5,
                CardRanks::Six => 6,
                CardRanks::Seven => 7,
                CardRanks::Eight => 8,
                CardRanks::Nine => 9,
                CardRanks::Ten => 10,
                CardRanks::Jack => 11,
                CardRanks::Queen => 12,
                CardRanks::King => 13,
            },
            color: match suit {
                CardSuits::Clubs => Color::Black,
                CardSuits::Diamonds => Color::Red,
                CardSuits::Hearts => Color::Red,
                CardSuits::Spades => Color::Black,
            },
            rank,
            suit,
            is_hidden: true,
        }
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }
    pub fn get_suit(&self) -> CardSuits {
        self.suit
    }
    pub fn get_rank(&self) -> CardRanks {
        self.rank
    }

    pub fn get_is_hidden(&self) -> bool {
        self.is_hidden
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn reveal(&mut self) {
        self.is_hidden = false;
    }
}
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, EnumIter, Copy, Clone)]
pub enum CardRanks {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, EnumIter, Copy, Clone, PartialEq)]
pub enum CardSuits {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();
        for suit in CardSuits::iter() {
            for rank in CardRanks::iter() {
                let card = Card::new(rank, suit);
                cards.push(card);
            }
        }
        Deck { cards }
    }
}
