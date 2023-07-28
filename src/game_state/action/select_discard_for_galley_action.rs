use std::fmt::Display;

use serde::Deserialize;

use super::Action;
use crate::game_state::{GamePhase, GameState, ShipActionPhase, Update};

#[derive(Deserialize)]
pub struct SelectDiscardForGalleyAction {
    decline: bool,
    discard_ix: usize,
    crew_ix: usize,
    player_ix: usize,
}

fn validate(state: &GameState) -> Update {
    if let GamePhase::ShipAction(
        Some(ShipActionPhase::GalleyAction { gain_phase_complete: true })
    ) = &state.phase {
        Ok(state.clone())
    } else {
        Err("Wrong phase".to_owned())
    }
}

impl Action for SelectDiscardForGalleyAction {
    fn execute(&self, state: &GameState) -> Update {
        let gs = Ok(state.clone())
                    .and_then(|g| validate(&g))
                    .map(|g| {
                        GameState {
                            phase: GamePhase::ShipActionComplete,
                            ..g
                        }
                    });

        if self.decline {
            gs
        } else {
            gs
                .and_then(|g| g.discard_card(self.player_ix, self.discard_ix))
                .and_then(|g| g.reduce_fatigue(self.crew_ix))
        }
    }
}

impl Display for SelectDiscardForGalleyAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "Select Discard For Galley Action\n{}\n{}\n{}",
            self.decline, self.crew_ix, self.player_ix
        )   
    }
}