use serde::Serialize;

use crate::game_state::{challenge::Challenge, Skill};

use super::{GameState, Update};

#[derive(Clone, Serialize)]
pub struct EventCard {
    pub name: String,
    pub options: Vec<EventOption>,
    // TODO: deck index
}

#[derive(Clone, Serialize)]
pub struct EventOption {
    pub text: String,
    #[serde(skip_serializing)]
    pub handle_option: fn(&GameState) -> Update,
}

pub fn event_deck() -> Vec<EventCard> {
    vec![
        EventCard {
            name: "Biting Starfish".to_owned(),
            options: vec![
                EventOption {
                    text: "Gain 1 Coin".to_owned(),
                    handle_option: gain_coin,
                },
                EventOption {
                    text: "Gain 1 Meat".to_owned(),
                    handle_option: gain_meat,
                },
            ],
        },
        EventCard {
            name: "Broken Biplane".to_owned(),
            options: vec![EventOption {
                text: "Help repair the airplane: CRAFT 8".to_owned(),
                handle_option: (|g| {
                    g.challenge(Challenge {
                        skill: Skill::Craft,
                        amount: 3,
                    })
                }),
            }],
        },
    ]
}

fn gain_coin(state: &GameState) -> Update {
    let mut gs = state.clone();
    gs.resources.coins += 1;

    Ok(gs)
}

fn gain_meat(state: &GameState) -> Update {
    let mut gs = state.clone();
    gs.resources.meat += 1;

    Ok(gs)
}
