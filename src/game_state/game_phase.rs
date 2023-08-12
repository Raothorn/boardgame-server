use serde::{Deserialize, Serialize};
use super::{
    challenge::Challenge, storybook::Story,
    event_deck::EventCard, SearchToken, 
};

#[derive(Clone, Serialize, Debug)]
pub enum GamePhase {
    ShipActionPhase(Option<ShipActionSubphase>),
    EventPhase(Option<EventCard>),
    MainActionPhase(Option<MainActionSubphase>, u32),
    ChallengePhase {
        challenge: Challenge,
        added: Option<u32>,
    },
    SelectCrewMemberPhase(Option<usize>, String),
    ExplorePhase(Story)
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
