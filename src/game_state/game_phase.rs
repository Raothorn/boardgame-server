use super::{
    challenge::Challenge, effect::Effect, event_deck::EventCard,
    storybook::Story, SearchToken,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Debug)]
pub enum GamePhase {
    ShipActionPhase(Option<ShipActionSubphase>),
    EventPhase(Option<EventCard>),
    MainActionPhase(Option<MainActionSubphase>, u32),
    ChallengePhase {
        challenge: Challenge,
        skill: Option<u32>,
    },
    SelectCrewMemberPhase {
        crew_ix: Option<usize>,
        title: String,
        #[serde(skip_serializing)]
        callback: String,
    },
    ExplorePhase(Story),
    ResolveEffectPhase(Effect),
}

#[derive(Clone, Serialize, Default, Debug)]
pub enum ShipActionSubphase {
    #[default]
    GalleyAction,
    DeckAction {
        search_tokens_drawn: Vec<SearchToken>,
    },
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, Copy)]
pub enum MainActionSubphase {
    #[default]
    Travel,
    Explore,
}
