use crate::{Card, Deck, JsCard, Phase};
use actor::{ActorId, UserId};
use arrayvec::ArrayVec;
use rand::rngs::ThreadRng;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

#[allow(clippy::large_enum_variant)]
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
        if let Self::Init { hand } = self {
            let mut index = None;
            cards.sort();
            while let Some(i) = cards.pop() {
                if index != Some(i) && hand.len() > i as usize {
                    out.push(hand.remove(i as usize));
                }
                index = Some(i);
            }
        }
        out
    }
}

#[derive(Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../frontend/src/bindings/")]
pub enum ServerMsg {
    Join {
        #[schema(value_type = String)]
        joined: ActorId,
    },
    Game {
        phase: Phase,
        #[ts(as = "Vec<(String, u8)>")]
        #[schema(value_type = Vec<(String, u8)>)]
        players: Vec<(UserId, u8)>,
        library_size: u8,
        discard_size: u8,
        damage: u8,
        enemy: JsCard,
        hand: Vec<JsCard>,
        resolving: Vec<Vec<JsCard>>,
    },
    Victory,
    Defeat,
}
