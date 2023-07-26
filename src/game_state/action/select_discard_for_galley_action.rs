use serde::Deserialize;

use crate::game_state::{GameState, ShipActionPhase, GamePhase};
use super::{Action};


#[derive(Deserialize)]
pub struct SelectDiscardForGalleyAction {
    decline: bool,
    discard_ix: usize,
    crew_ix: usize,
    player_ix: usize
}

impl Action for SelectDiscardForGalleyAction {
    fn execute(&self, state: &mut GameState) -> Option<String> {
        match state.phase {
            GamePhase::ShipAction(
                Some(ShipActionPhase::GalleyAction 
                    { 
                        gain_phase_complete:true,
                    })
            ) => 
            {
                state.phase = GamePhase::ShipActionComplete;
                if self.decline {
                    return None;
                }
                state.crew[self.crew_ix].reduce_fatigue();

                let card = state.players[self.player_ix]
                                .discard_card(self.discard_ix);
                None
            },
            _ => Some(String::from("Wrong phase"))

        }
    }

    fn name(&self) -> &str {
        "Select discard for galley action"
    }
}