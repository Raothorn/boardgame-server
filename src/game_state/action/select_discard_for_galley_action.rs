use crate::game_state::{GameState, ShipActionPhase, GamePhase};

use super::{Action};

struct SelectDiscardForGalleyAction {
    discard_ix: u32,
    crew_ix: u32
}

impl Action for SelectDiscardForGalleyAction {
    fn execute(&self, state: &mut GameState) {

    }

    fn invalid(&self, state: &GameState) -> Option<String> {
        match state.phase {
            GamePhase::ShipAction(
                Some(ShipActionPhase::GalleyAction 
                    { 
                        gain_phase_complete:true,
                        discard_chosen: false,
                        ..
                    })
            ) => {
                None
            }

            _ => Some (String::from("Wrong phase"))
        }
    }
}