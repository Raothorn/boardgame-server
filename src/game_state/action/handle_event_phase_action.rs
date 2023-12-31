use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::game_state::GamePhase;
use crate::game_state::Update;

use super::Action;
use super::GameState;

#[derive(Deserialize, Serialize)]
pub struct HandleEventPhaseAction {
    player_ix: u32
}

#[typetag::serde(name="handleEventPhaseAction")]
impl Action for HandleEventPhaseAction {
    fn execute(&self, state: &GameState) -> Update {
        if let GamePhase::EventPhase(None) = state.phase() {
            let mut gs = state.clone();

            match gs.event_card_deck.draw() {
                Ok(event_card) => Ok(gs)
                    .and_then(|g| {
                        g.set_phase(GamePhase::EventPhase(Some(
                            event_card,
                        )))
                    }),
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
