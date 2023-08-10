use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::game_state::{game_phase::GamePhase, GameState, Update};

use super::Action;

#[derive(Deserialize, Serialize)]
pub struct TravelAction {
    to_area: u32,
    player_ix: u32,
}

#[typetag::serde(name = "travelAction")]
impl Action for TravelAction {
    fn execute(&self, state: &GameState) -> Update {
        if let GamePhase::MainActionPhase(_) = state.phase() {
            if state.map.ship_area == self.to_area {
                Err("You can't move to the same area".to_owned())
            } else {
                Ok(state.clone()).and_then(|g| g.move_ship(self.to_area))
            }
        } else {
            Err("wrong phase".to_owned())
        }
    }
}

impl Display for TravelAction {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "Travel Action")
    }
}
