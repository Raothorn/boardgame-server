use serde::Serialize;

use super::{
    challenge::Challenge, event_deck::EventCard, SearchToken,
};

#[derive(Clone, Serialize)]
pub enum GamePhase {
    ShipAction(Option<ShipActionSubphase>),
    EventPhase(Option<EventCard>),
    ChallengePhase(Challenge),
}

#[derive(Clone, Serialize, Default)]
pub enum ShipActionSubphase {
    #[default]
    GalleyAction,
    DeckAction {
        search_tokens_drawn: Vec<SearchToken>,
    },
}
