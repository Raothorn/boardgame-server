use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::game_state::game_phase::GamePhase as Gp;
use crate::game_state::game_phase::MainActionSubphase as Mas;
use crate::game_state::GameState;
use crate::game_state::Update;

use super::Action;

#[derive(Deserialize, Serialize)]
pub struct TravelAction {
    to_area: u32,
    player_ix: u32,
}

#[typetag::serde(name = "travelAction")]
impl Action for TravelAction {
    fn execute(&self, state: &GameState) -> Update {
        if let Gp::MainActionPhase(Some(Mas::Travel), action_ct) =
            state.phase()
        {
            if state.map.ship_area == self.to_area {
                Err("You can't move to the same area".to_owned())
            } else {
                let phase = Gp::MainActionPhase(None, action_ct);
                Ok(state.clone())
                    .and_then(|g| g.move_ship(self.to_area))
                    .and_then(|g| g.set_phase(phase))
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
