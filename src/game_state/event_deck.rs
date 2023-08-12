use crate::game_state::{
    challenge::Challenge, skill::Skill, Resource,
};

use super::effect::*;
use serde::Serialize;

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
    pub effects: Vec<Effect>,
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
                    effects: vec![Effect::TryChallenge(Challenge {
                        skill: Skill::Craft,
                        amount: 8,
                        if_fail: vec![Effect::TakeHealthDamage(5)],
                        if_succeed: vec![
                            Effect::GainResource(Resource::Coin, 1),
                            Effect::GainResource(Resource::Meat, 1),
                        ],
                    })],
                },
                EventOption {
                    text: "Ignore the plane".to_owned(),
                    // Todo
                    effects: vec![Effect::TakeHealthDamage(1)],
                },
            ],
        },
        // EventCard {
        //     name: todo!(),
        //     options: todo!(),
        // },
    ]
}
