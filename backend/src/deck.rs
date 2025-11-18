use crate::*;
use arrayvec::ArrayVec;
use macros::list_cards;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::collections::VecDeque;

macro_rules! list_deck {
    ($n:literal) => {
        list_cards!(
            suits: [Heart, Spade, Diamond, Club],
            ranks: [Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten],
            other: [Joker = $n]
        )
    };
}

#[derive(Clone, Debug)]
pub struct Deck {
    rng: ThreadRng,
    library: VecDeque<Card>,
    discard: Vec<Card>,
    court: ArrayVec<Card, 12>,
    battling: Option<Card>,
    resolving: ArrayVec<Combo, 17>,
}

impl Deck {
    pub fn new(jesters: usize, mut rng: ThreadRng) -> Self {
        let library = Self::library(&mut rng, jesters);
        let discard = Vec::new();
        let court = Self::court(&mut rng);
        let battling = None;
        let resolving = ArrayVec::new();
        Self {
            rng,
            library: library.into(),
            discard,
            court,
            battling,
            resolving,
        }
    }
    pub fn battling(&self) -> Option<&Card> {
        self.battling.as_ref()
    }
    pub fn library_count(&self) -> usize {
        self.library.len()
    }
    pub fn discard_count(&self) -> usize {
        self.discard.len()
    }
    pub fn jester(&self) -> bool {
        self.resolving
            .iter()
            .any(|c| matches!(c, Combo::Single(Card::Joker)))
    }
    pub fn defense_value(&self) -> u8 {
        self.resolving
            .iter()
            .fold(0, |def, c| def + c.suit_value(Suit::Spade))
    }
    pub fn draw(&mut self) -> Card {
        self.library.pop_front().unwrap_or_else(|| {
            self.discard.shuffle(&mut self.rng);
            self.library = std::mem::take(&mut self.discard).into();
            self.library
                .pop_front()
                .expect("Should never happen: tried to draw from empty library and discard")
        })
    }
    pub fn discard(&mut self, cards: &mut Vec<Card>) {
        self.discard.append(cards);
    }
    pub fn next_battle(&mut self, exact: bool) {
        let prev = std::mem::take(&mut self.battling);
        if let Some(prev) = prev {
            if exact {
                self.library.push_front(prev);
            } else {
                self.library.push_back(prev);
            }
            self.discard.extend(
                std::mem::take(&mut self.resolving)
                    .into_iter()
                    .map(|combo| combo.cards())
                    .flatten(),
            );
        }
        self.battling = self.court.pop();
    }
    pub fn play_cards(&mut self, combo: Combo) {
        self.resolving.push(combo);
    }
    pub fn heal(&mut self, heal: usize) {
        let i = self.discard.len() - std::cmp::min(heal, self.discard.len());
        self.discard.shuffle(&mut self.rng);
        self.library.append(&mut self.discard.split_off(i).into());
    }
    pub fn court(rng: &mut ThreadRng) -> ArrayVec<Card, 12> {
        let mut court = ArrayVec::new();
        let mut kings = list_cards!(suits: [Heart, Spade, Diamond, Club], ranks: [King], other: []);
        kings.shuffle(rng);
        court.try_extend_from_slice(&mut kings).expect("kings");
        let mut queens =
            list_cards!(suits: [Heart, Spade, Diamond, Club], ranks: [Queen], other: []);
        queens.shuffle(rng);
        court.try_extend_from_slice(&mut queens).expect("queens");
        let mut jacks = list_cards!(suits: [Heart, Spade, Diamond, Club], ranks: [Jack], other: []);
        jacks.shuffle(rng);
        court.try_extend_from_slice(&mut jacks).expect("jacks");
        court
    }
    pub fn library(rng: &mut ThreadRng, jesters: usize) -> Vec<Card> {
        let mut library = match jesters {
            2 => list_deck!(2),
            1 => list_deck!(1),
            _ => list_deck!(0),
        };
        library.shuffle(rng);
        library
    }
}
