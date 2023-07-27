use serde::Deserialize;

use super::Action;
use crate::game_state::{GamePhase, GameState, ShipActionPhase};

#[derive(Deserialize)]
pub struct SelectDiscardForGalleyAction {
    decline: bool,
    discard_ix: usize,
    crew_ix: usize,
    player_ix: usize,
}

impl Action for SelectDiscardForGalleyAction {
    fn execute(&self, state: &mut GameState) -> Option<String> {
        // VALIDATE
        match state.phase {
            GamePhase::ShipAction(Some(
                ShipActionPhase::GalleyAction {
                    gain_phase_complete: true,
                },
            )) => {
                let card = state.players[self.player_ix]
                    .discard_card(self.discard_ix);

                //ACT
                state.phase = GamePhase::ShipActionComplete;         
                if self.decline {
                    None
                } else {
                    state.crew[self.crew_ix].reduce_fatigue();           
                    None
                }
            }
            _ => Some(String::from("Wrong phase")),
        }
    }

    fn name(&self) -> &str {
        "Select discard for galley action"
    }
}
