use std::fmt;

use serde::Deserialize;

use crate::game_state::GamePhase;
use crate::game_state::Update;

use super::Action;
use super::GameState;

#[derive(Deserialize)]
pub struct HandleEventPhaseAction {
}

impl Action for HandleEventPhaseAction {
    fn execute(&self, state: &GameState) -> Update {
        if let GamePhase::EventPhase(None) = state.phase {
            let mut gs = state.clone();
            
            match gs.event_card_deck.draw() {
                Ok((event_card, deck)) => {
                    gs.event_card_deck = deck;
                    gs.phase = GamePhase::EventPhase(Some(event_card));
                    Ok(gs).map(|g| g.prompt_str("selectEventOption"))
                }
                Err(e) => Err(e),
            }
        } else {
            Err("Wrong phase".to_owned())
        }
    }
}

impl fmt::Display for HandleEventPhaseAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Handle Event Phase")
    }
}
