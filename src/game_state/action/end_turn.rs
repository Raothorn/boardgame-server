use std::fmt::Display;

use serde::Deserialize;

use crate::game_state::{GamePhase, GameState, Update};

use super::Action;

#[derive(Deserialize)]
pub struct EndTurnAction {
    player_ix: usize
}

impl Action for EndTurnAction {
    fn execute(&self, state: &GameState) -> Update {
        if let GamePhase::EventPhase(Some(_)) = state.phase() {
            let mut gs = state.clone();
            gs.set_phase(GamePhase::ShipAction(None));
            Ok(gs)
        } else {
            Err("You can't end the turn yet".to_owned())
        }
    }
}

impl Display for EndTurnAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "End Turn Action")
    }
}
