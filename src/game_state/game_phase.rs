use serde::Serialize;

use super::{
    challenge::Challenge, event_deck::EventCard, SearchToken,
};

#[derive(Clone, Serialize)]
pub enum GamePhase {
    ShipActionPhase(Option<ShipActionSubphase>),
    EventPhase(Option<EventCard>),
    MainActionPhase(Option<MainActionSubphase>, u32),
    ChallengePhase {
        challenge: Challenge,
        added: Option<u32>
    },
}

#[derive(Clone, Serialize, Default)]
pub enum ShipActionSubphase {
    #[default]
    GalleyAction,
    DeckAction {
        search_tokens_drawn: Vec<SearchToken>,
    },
}

#[derive(Clone, Serialize, Default)]
pub enum MainActionSubphase {
    #[default]
    Travel,
}
