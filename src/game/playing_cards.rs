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

    pub fn get_unicode_symbol(&self) -> char {
        match self.get_suit() {
            CardSuits::Spades => match self.get_rank() {
                CardRanks::Ace => '\u{1F0A1}',   // 🂡
                CardRanks::Two => '\u{1F0A2}',   // 🂢
                CardRanks::Three => '\u{1F0A3}', // 🂣
                CardRanks::Four => '\u{1F0A4}',  // 🂤
                CardRanks::Five => '\u{1F0A5}',  // 🂥
                CardRanks::Six => '\u{1F0A6}',   // 🂦
                CardRanks::Seven => '\u{1F0A7}', // 🂧
                CardRanks::Eight => '\u{1F0A8}', // 🂨
                CardRanks::Nine => '\u{1F0A9}',  // 🂩
                CardRanks::Ten => '\u{1F0AA}',   // 🂪
                CardRanks::Jack => '\u{1F0AB}',  // 🂫
                CardRanks::Queen => '\u{1F0AD}', // 🂭
                CardRanks::King => '\u{1F0AE}',  // 🂮
            },
            CardSuits::Hearts => match self.get_rank() {
                CardRanks::Ace => '\u{1F0B1}',   // 🂱
                CardRanks::Two => '\u{1F0B2}',   // 🂲
                CardRanks::Three => '\u{1F0B3}', // 🂳
                CardRanks::Four => '\u{1F0B4}',  // 🂴
                CardRanks::Five => '\u{1F0B5}',  // 🂵
                CardRanks::Six => '\u{1F0B6}',   // 🂶
                CardRanks::Seven => '\u{1F0B7}', // 🂷
                CardRanks::Eight => '\u{1F0B8}', // 🂸
                CardRanks::Nine => '\u{1F0B9}',  // 🂹
                CardRanks::Ten => '\u{1F0BA}',   // 🂺
                CardRanks::Jack => '\u{1F0BB}',  // 🂻
                CardRanks::Queen => '\u{1F0BD}', // 🂽
                CardRanks::King => '\u{1F0BE}',  // 🂾
            },
            CardSuits::Diamonds => match self.get_rank() {
                CardRanks::Ace => '\u{1F0C1}',   // 🃁
                CardRanks::Two => '\u{1F0C2}',   // 🃂
                CardRanks::Three => '\u{1F0C3}', // 🃃
                CardRanks::Four => '\u{1F0C4}',  // 🃄
                CardRanks::Five => '\u{1F0C5}',  // 🃅
                CardRanks::Six => '\u{1F0C6}',   // 🃆
                CardRanks::Seven => '\u{1F0C7}', // 🃇
                CardRanks::Eight => '\u{1F0C8}', // 🃈
                CardRanks::Nine => '\u{1F0C9}',  // 🃉
                CardRanks::Ten => '\u{1F0CA}',   // 🃊
                CardRanks::Jack => '\u{1F0CB}',  // 🃋
                CardRanks::Queen => '\u{1F0CD}', // 🃍
                CardRanks::King => '\u{1F0CE}',  // 🃎
            },
            CardSuits::Clubs => match self.get_rank() {
                CardRanks::Ace => '\u{1F0D1}',   // 🃑
                CardRanks::Two => '\u{1F0D2}',   // 🃒
                CardRanks::Three => '\u{1F0D3}', // 🃓
                CardRanks::Four => '\u{1F0D4}',  // 🃔
                CardRanks::Five => '\u{1F0D5}',  // 🃕
                CardRanks::Six => '\u{1F0D6}',   // 🃖
                CardRanks::Seven => '\u{1F0D7}', // 🃗
                CardRanks::Eight => '\u{1F0D8}', // 🃘
                CardRanks::Nine => '\u{1F0D9}',  // 🃙
                CardRanks::Ten => '\u{1F0DA}',   // 🃚
                CardRanks::Jack => '\u{1F0DB}',  // 🃛
                CardRanks::Queen => '\u{1F0DD}', // 🃝
                CardRanks::King => '\u{1F0DE}',  // 🃞
            },
        }
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
