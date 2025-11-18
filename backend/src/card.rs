use macros::*;

cards! {
    suits: [Heart, Spade, Diamond, Club],
    ranks: [
        Ace = 1,
        Two = 2,
        Three = 3,
        Four = 4,
        Five = 5,
        Six = 6,
        Seven = 7,
        Eight = 8,
        Nine = 9,
        Ten = 10,
        Jack = 10,
        Queen = 15,
        King = 20
    ],
    other: [Joker = 0],
}

impl Card {
    pub fn strength(&self) -> u8 {
        self.value()
            * if self.suit() == Some(Suit::Club) {
                2
            } else {
                1
            }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Combo {
    Jester,
    Single(Card),
    Companion { card: Card, companion: Card },
    Pair { one: Card, two: Card },
    Triple { one: Card, two: Card, three: Card },
    Quad,
}

impl Combo {
    pub fn strength(&self) -> u8 {
        match self {
            Self::Jester => 0,
            Self::Single(card) => card.strength(),
            Self::Companion { card, companion } => {
                let value = card.value() + 1;
                if card.suit() == Some(Suit::Club) || companion.suit() == Some(Suit::Club) {
                    value * 2
                } else {
                    value
                }
            }
            Self::Pair { one, two } => one.strength() + two.strength(),
            Self::Triple { one, two, three } => one.strength() + two.strength() + three.strength(),
            Self::Quad => 10,
        }
    }
    pub fn suit_value(&self, suit: Suit) -> u8 {
        match self {
            Self::Jester => 0,
            Self::Single(card) => {
                if card.suit() == Some(suit) {
                    card.value()
                } else {
                    0
                }
            }
            Self::Companion { card, companion } => {
                let value = card.value() + 1;
                if card.suit() == Some(suit) || companion.suit() == Some(suit) {
                    value
                } else {
                    0
                }
            }
            Self::Pair { one, two } => {
                if one.suit() == Some(suit) || two.suit() == Some(suit) {
                    one.value()
                } else {
                    0
                }
            }
            Self::Triple { one, two, three } => {
                if one.suit() == Some(suit)
                    || two.suit() == Some(suit)
                    || three.suit() == Some(suit)
                {
                    one.value()
                } else {
                    0
                }
            }
            Self::Quad => 2,
        }
    }
    pub fn cards(self) -> Vec<Card> {
        match self {
            Self::Jester => vec![Card::Joker],
            Self::Single(card) => vec![card],
            Self::Companion { card, companion } => vec![card, companion],
            Self::Pair { one, two } => vec![one, two],
            Self::Triple { one, two, three } => vec![one, two, three],
            Self::Quad => {
                list_cards!(suits: [Heart, Spade, Diamond, Club], ranks: [Two], other: [])
            }
        }
    }
    pub fn from_cards(cards: Vec<Card>) -> Option<Self> {
        if let Ok([one]) = <[Card; 1]>::try_from(cards.as_slice()) {
            Some(if one == Card::Joker {
                Combo::Jester
            } else {
                Combo::Single(one)
            })
        } else if let Ok([one, two]) = <[Card; 2]>::try_from(cards.as_slice()) {
            match (one.value(), two.value()) {
                (_, 1) => Some(Combo::Companion {
                    card: one,
                    companion: two,
                }),
                (1, _) => Some(Combo::Companion {
                    card: two,
                    companion: one,
                }),
                (a, b) if a == b && a <= 5 => Some(Combo::Pair { one, two }),
                _ => None,
            }
        } else if let Ok([one, two, three]) = <[Card; 3]>::try_from(cards.as_slice()) {
            (one.value() == two.value() && one.value() == three.value() && one.value() <= 3)
                .then_some(Combo::Triple { one, two, three })
        } else if let Ok([one, two, three, four]) = <[Card; 4]>::try_from(cards) {
            (one.value() == 2 && two.value() == 2 && three.value() == 2 && four.value() == 2)
                .then_some(Combo::Quad)
        } else {
            None
        }
    }
}
