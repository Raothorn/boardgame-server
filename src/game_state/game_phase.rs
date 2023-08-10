use serde::Serialize;
use serde_json::Value;

use super::{
    challenge::Challenge, event_deck::EventCard, SearchToken, Update, crew::Crew, GameState, action::Action,
};

#[derive(Clone, Serialize, Debug)]
pub enum GamePhase {
    ShipActionPhase(Option<ShipActionSubphase>),
    EventPhase(Option<EventCard>),
    MainActionPhase(Option<MainActionSubphase>, u32),
    ChallengePhase {
        challenge: Challenge,
        added: Option<u32>
    },
    SelectCrewMemberPhase(Option<usize>, String)
}

#[derive(Clone, Serialize, Default, Debug)]
pub enum ShipActionSubphase {
    #[default]
    GalleyAction,
    DeckAction {
        search_tokens_drawn: Vec<SearchToken>,
    },
}

#[derive(Clone, Serialize, Default, Debug)]
pub enum MainActionSubphase {
    #[default]
    Travel,
}
