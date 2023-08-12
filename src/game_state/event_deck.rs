use serde::Serialize;

use crate::game_state::{challenge::Challenge, Skill};

use super::{GameState, Update};

#[derive(Clone, Serialize, Debug)]
pub struct EventCard {
    pub name: String,
    pub options: Vec<EventOption>,
    // TODO: deck index
    pub deck_index: u32,
}

#[derive(Clone, Serialize, Debug)]
pub struct EventOption {
    pub text: String,
    #[serde(skip_serializing)]
    pub handle_option: fn(&GameState) -> Update<GameState>,
}

pub fn event_deck() -> Vec<EventCard> {
    vec![
        EventCard {
            name: "Broken Biplane".to_owned(),
            deck_index: 11,
            options: vec![
                EventOption {
                    text: "Help repair the airplane (CRAFT 8)"
                        .to_owned(),
                    handle_option: (|g| {
                        g.challenge(Challenge {
                            skill: Skill::Craft,
                            amount: 8,
                            if_fail: |gs| {
                                Ok(gs.clone())
                                    .and_then(|g| take_damage(g, 5))
                            },
                            if_succeed: |gs| {
                                Ok(gs.clone())
                                    .and_then(gain_coin)
                                    .and_then(gain_meat)
                            },
                        })
                    }),
                },
                EventOption {
                    text: "Ignore the plane".to_owned(),
                    handle_option: |g| {
                        Ok(g.clone()).and_then(|g| take_damage(g, 1))
                    },
                },
            ],
        },
        // EventCard {
        //     name: todo!(),
        //     options: todo!(),
        // },
    ]
}

fn gain_coin(state: GameState) -> Update<GameState> {
    let mut gs = state.clone();
    gs.resources.coins += 1;

    Ok(gs)
}

fn gain_meat(state: GameState) -> Update<GameState> {
    let mut gs = state.clone();
    gs.resources.meat += 1;

    Ok(gs)
}

fn take_damage(state: GameState, damage: u8) -> Update<GameState> {
    // TODO: allow user to distribute damage
    let mut gs = state.clone();
    gs.crew[0].damage += damage;
    Ok(gs)
}
