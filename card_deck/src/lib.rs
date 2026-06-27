use rand::Rng;

#[derive(PartialEq, Debug)]
pub enum Suit {
    Heart, 
    Diamond, 
    Spade, 
    Club,
}

#[derive(PartialEq, Debug)]
pub enum Rank {
    Ace, 
    King, 
    Queen, 
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        Self::translate(rand::thread_rng().gen_range(1..=4))
    }

    pub fn translate(value: u8) -> Suit {
        if value == 1 {
            return Suit::Heart;
        } else if value == 2 {
            return Suit::Diamond;
        } else if value == 3 {
            return Suit::Spade;
        }
        Suit::Club
    }
}

impl Rank {
    pub fn random() -> Rank {
        Self::translate(rand::thread_rng().gen_range(1..=13))
    }

    pub fn translate(value: u8) -> Rank {
        if value == 1 {
            return Rank::Ace;
        } else if value == 11 {
            return Rank::Jack;
        } else if value == 12 {
            return Rank::Queen;
        } else if value == 13 {
            return Rank::King;
        }
        Rank::Number(value)
    }
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    if card.suit == Suit::Spade && card.rank == Rank::Ace {
        return true;
    }
    false
}