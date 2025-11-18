use macros::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

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

#[derive(Clone, Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../frontend/src/bindings/")]
pub struct JsCard {
    pub suit: Option<Suit>,
    pub rank: String,
    pub value: u8,
}

impl From<Card> for JsCard {
    fn from(card: Card) -> Self {
        Self {
            suit: card.suit(),
            rank: card.to_string(),
            value: card.value(),
        }
    }
}

impl From<&Card> for JsCard {
    fn from(card: &Card) -> Self {
        Self {
            suit: card.suit(),
            rank: card.to_string(),
            value: card.value(),
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
                vec![
                    Card::HeartTwo,
                    Card::SpadeTwo,
                    Card::DiamondTwo,
                    Card::ClubTwo,
                ]
            }
        }
    }

    pub fn js_cards(self) -> Vec<JsCard> {
        match self {
            Self::Jester => vec![Card::Joker.into()],
            Self::Single(card) => vec![card.into()],
            Self::Companion { card, companion } => vec![card.into(), companion.into()],
            Self::Pair { one, two } => vec![one.into(), two.into()],
            Self::Triple { one, two, three } => vec![one.into(), two.into(), three.into()],
            Self::Quad => {
                vec![
                    Card::HeartTwo.into(),
                    Card::SpadeTwo.into(),
                    Card::DiamondTwo.into(),
                    Card::ClubTwo.into(),
                ]
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
                (v, 1) if v != 0 => Some(Combo::Companion {
                    card: one,
                    companion: two,
                }),
                (1, v) if v != 0 => Some(Combo::Companion {
                    card: two,
                    companion: one,
                }),
                (a, b) if a == b && a > 1 && a <= 5 => Some(Combo::Pair { one, two }),
                _ => None,
            }
        } else if let Ok([one, two, three]) = <[Card; 3]>::try_from(cards.as_slice()) {
            (one.value() == two.value()
                && one.value() == three.value()
                && one.value() <= 3
                && one.value() > 1)
                .then_some(Combo::Triple { one, two, three })
        } else if let Ok([one, two, three, four]) = <[Card; 4]>::try_from(cards) {
            (one.value() == 2 && two.value() == 2 && three.value() == 2 && four.value() == 2)
                .then_some(Combo::Quad)
        } else {
            None
        }
    }
}
