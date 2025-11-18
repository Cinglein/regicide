use crate::*;
use arrayvec::ArrayVec;
use rand::rngs::ThreadRng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;
use utoipa::ToSchema;

#[derive(Clone, Default, Debug)]
pub enum SharedState {
    #[default]
    Uninit,
    Init {
        phase: Phase,
        deck: Deck,
        turn_order: [UserId; 4],
        damage: u8,
    },
}

impl SharedState {
    pub fn init(&mut self, jesters: usize, turn_order: [UserId; 4], rng: ThreadRng) {
        let deck = Deck::new(jesters, rng);
        let phase = Phase::Play(turn_order[0]);
        let damage = 0;
        *self = Self::Init {
            deck,
            phase,
            turn_order,
            damage,
        };
    }
}

#[derive(Clone, Default, Debug)]
pub enum UserState {
    #[default]
    Uninit,
    Init {
        hand: ArrayVec<Card, 8>,
    },
}

impl UserState {
    pub fn cards<const N: usize>(&mut self, mut cards: ArrayVec<u8, N>) -> Vec<Card> {
        let mut out = Vec::new();
        match self {
            Self::Init { hand } => {
                let mut index = None;
                cards.sort();
                while let Some(i) = cards.pop() {
                    if index != Some(i) && hand.len() > i as usize {
                        out.push(hand.remove(i as usize));
                    }
                    index = Some(i);
                }
            }
            _ => (),
        };
        out
    }
}

#[derive(Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../frontend/src/bindings/")]
pub enum ServerMsg {
    Join {
        #[ts(as = "Vec<String>")]
        #[schema(value_type = Vec<String>)]
        lobbies: Vec<ActorId>,
        #[ts(as = "Option<String>")]
        #[schema(value_type = Option<String>)]
        joined: Option<ActorId>,
    },
    State {
        phase: Phase,
        #[ts(as = "HashMap<String, u8>")]
        #[schema(value_type = HashMap<String, u8>)]
        players: HashMap<UserId, u8>,
        library_size: u8,
        discard_size: u8,
        damage: u8,
        hand: Vec<Card>,
        resolving: Vec<Vec<Card>>,
    },
}
