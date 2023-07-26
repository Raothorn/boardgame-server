use serde::Deserialize;

use crate::game_state::GamePhase;

use super::Action;

#[derive(Deserialize)]
pub struct EndTurnAction {
    player_ix: usize
}

impl Action for EndTurnAction {
    fn execute(&self, state: &mut crate::game_state::GameState) -> Option<String> {
        
        if let GamePhase::ShipActionComplete = state.phase {
            state.phase = GamePhase::ShipAction(None);
            return None;
        }
        Some(String::from("You can't end the turn yet"))
    }

    fn name(&self) -> &str {
        "End Turn"
    }
}