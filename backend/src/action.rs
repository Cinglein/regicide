use crate::*;
use arrayvec::ArrayVec;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const MAX_PLAYERS: usize = 4;
const HAND_SIZE: usize = 9;
const JESTERS: usize = 2;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RegicideAction {
    Init,
    Play { cards: ArrayVec<u8, 4> },
    Discard { cards: ArrayVec<u8, 8> },
    Jester { player: UserId },
}

impl Action for RegicideAction {
    type Shared = SharedState;
    type User = UserState;
    fn can_join(shared: &Self::Shared, user: &HashMap<UserId, Self::User>) -> bool {
        matches!(shared, SharedState::Uninit) && user.len() <= MAX_PLAYERS
    }
    fn update(
        self,
        shared: &mut Self::Shared,
        user: &mut HashMap<UserId, Self::User>,
        user_id: UserId,
    ) {
        match (self, shared) {
            (Self::Init, shared) if matches!(shared, SharedState::Uninit) => {
                let mut rng = rand::rng();
                let hand_size = HAND_SIZE - user.len();
                let jesters = user.len().saturating_sub(JESTERS);
                let mut turn_order: Vec<UserId> = user.keys().copied().collect();
                turn_order.shuffle(&mut rng);
                if let Ok(turn_order) = turn_order.try_into() {
                    shared.init(jesters, turn_order, rng);
                    if let SharedState::Init { deck, .. } = shared {
                        user.values_mut().for_each(|u| {
                            let mut hand = ArrayVec::new();
                            for _ in 0..hand_size {
                                hand.push(deck.draw());
                            }
                            *u = UserState::Init { hand };
                        });
                    }
                }
            }
            (
                Self::Play { cards },
                SharedState::Init {
                    phase,
                    deck,
                    turn_order,
                    damage,
                },
            ) if matches!(*phase, Phase::Play(id) if id == user_id) => {
                let Some(player) = user.get_mut(&user_id) else {
                    return;
                };
                let Some(&enemy) = deck.battling() else {
                    return;
                };
                let cards = player.cards(cards);
                let Some(combo) = Combo::from_cards(cards) else {
                    return;
                };
                deck.play_cards(combo);
                if matches!(combo, Combo::Jester) {
                    *phase = Phase::Jester(user_id);
                } else {
                    deck.heal(combo.suit_value(Suit::Heart) as usize);

                    let mut draw = combo.suit_value(Suit::Diamond);
                    let mut players = turn_order.iter().cycle().peekable();
                    while players.peek() != Some(&&user_id) {
                        players.next();
                    }
                    let max_hand_size = HAND_SIZE - user.len();
                    while draw > 0 {
                        let player = players.next().unwrap();
                        if let Some(UserState::Init { hand }) = user.get_mut(player) {
                            if hand.len() < max_hand_size {
                                hand.push(deck.draw());
                            }
                        }
                        draw -= 1;
                    }

                    *damage += combo.strength();
                    if *damage >= enemy.value() * 2 {
                        deck.next_battle(*damage == enemy.value() * 2);
                        if deck.battling().is_none() {
                            *phase = Phase::Victory;
                        } else {
                            *phase = Phase::Defend(user_id);
                        }
                    }
                }
            }
            (Self::Jester { player }, SharedState::Init { phase, .. }) if matches!(*phase, Phase::Jester(id) if id == user_id) => {
                if user.contains_key(&player) {
                    *phase = Phase::Play(player);
                }
            }
            (
                Self::Discard { cards },
                SharedState::Init {
                    phase,
                    deck,
                    turn_order,
                    ..
                },
            ) if matches!(*phase, Phase::Defend(id) if id == user_id) => {
                let Some(player) = user.get_mut(&user_id) else {
                    return;
                };
                let Some(&enemy) = deck.battling() else {
                    return;
                };
                let mut cards = player.cards(cards);
                let spades = if deck.jester() || enemy.suit() != Some(Suit::Spade) {
                    deck.defense_value()
                } else {
                    0
                };
                let defense = spades + cards.iter().fold(0, |sum, c| sum + c.strength());
                if defense >= enemy.value() {
                    deck.discard(&mut cards);
                    let mut players = turn_order.iter().cycle();
                    players.find(|p| p == &&user_id);
                    *phase = Phase::Play(*players.next().unwrap());
                } else {
                    *phase = Phase::Defeat;
                }
            }
            _ => (),
        };
    }
}
