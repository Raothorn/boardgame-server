use std::ops::Range;

use crate::game_state::{
    challenge::Challenge, crew::Status, skill::Skill, Resource,
};

use super::{
    deck::Deck,
    effect::*,
    modifier::{Modifier, ModifierLifetime, ModifierTrigger}, SerialCard, client_message::ClientMessage,
};
use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
pub struct EventCard {
    pub name: Str,
    pub effects: Vec<Effect>,
    pub options: Vec<EventOption>,
    pub deck_index: u32,
}

#[derive(Clone, Serialize, Debug)]
pub struct EventOption {
    pub text: Str,
    #[serde(skip_serializing)]
    pub effects: Vec<Effect>,
}
//
impl Default for Deck<EventCard> {
    fn default() -> Self {
        let items = vec![card0(), card11(), card1()];
        return Deck::new(items);
    }
}

fn card0() -> EventCard {
    let option1 = EventOption {
        text: "Catch the jellyfish (SAVVY 8)",
        effects: vec![Effect::TryChallenge(Challenge {
            label: "Fail: -3 health. Gain 2 venom. <br> Succeed: Gain 2 meat.",
            skill: Skill::Savvy,
            amount: 8,
            if_fail: vec![
                Effect::TakeHealthDamage(3),
                Effect::TakeStatus(Status::Venom),
                Effect::TakeStatus(Status::Venom),
            ],
            if_succeed: vec![
                Effect::GainResource(Resource::Meat, 2)
            ],
        })],
    };

    let option2 = EventOption {
        text: "Hit the jellyfish away",
        effects: vec![
            Effect::FateConsequence(1..=3, Box::new(Effect::TakeHealthDamage(4)))
        ],
    };

    EventCard {
        deck_index: 0,
        name: "Floating Jellyfish",
        effects: Vec::new(),
        options: vec![option1, option2],
    }
}

fn card1() -> EventCard {
    let deck_index = 1;
    let name = "Possessed Crew";


    let modifier = Modifier {
        effect: Effect::TakeStatus(Status::Madness),
        lifetime: ModifierLifetime::ThisTurn,
        trigger: ModifierTrigger::DrawFate(1..=3),

        trigger_text: ClientMessage::ModifierTriggered {
            text: "You drew a 1-3 on a fate draw. You must gain one madness due to: ".to_owned(),
            card: SerialCard { label: name.to_owned(), deck: "event_deck".to_owned(), index: 1 }
        },

        condition: |_| true,
    };

    let effects = vec![
        Effect::TakeStatus(Status::Madness),
        Effect::GainModifier(Box::new(modifier)),
    ];
    let options = Vec::new();

    EventCard {
        name,
        deck_index,
        effects,
        options,
    }
}

fn card11() -> EventCard {
    EventCard {
        name: "Broken Biplane",
        deck_index: 11,
        effects: Vec::new(),
        options: vec![
            EventOption {
                text: "Help repair the airplane (CRAFT 8)",
                effects: vec![Effect::TryChallenge(Challenge {
                    label: "Fail: Lose 5 health. <br>
                            Succeed: Gain 1 coin and 1 meat.",
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
                text: "Ignore the plane",
                // Todo
                effects: vec![
                    Effect::TakeHealthDamage(1),
                    Effect::TakeStatus(Status::LowMorale),
                ],
            },
        ],
    }
}

type Str = &'static str;
