use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::game_state::game_phase::GamePhase as Gp;
use crate::game_state::game_phase::MainActionSubphase as Mas;
use crate::game_state::{GameState, Update};

use super::Action;

#[derive(Serialize, Deserialize)]
pub struct SelectMainAction {
    //TODO action type
    player_ix: u32,
}

#[typetag::serde(name = "selectMainAction")]
impl Action for SelectMainAction {
    fn execute(&self, state: &GameState) -> Update {
        // TODO  increase count here, not in travel_action
        if let Gp::MainActionPhase(None, action_ct) = state.phase() {
            let phase = if action_ct < 2 {
                Gp::MainActionPhase(Some(Mas::Travel), action_ct + 1)
            } else {
                Gp::ShipActionPhase(None)
            };

            Ok(state.clone()).and_then(|g| g.set_phase(phase))
        } else {
            Err("wrong phase".to_owned())
        }
    }
}

impl Display for SelectMainAction {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "Select Main Action")
    }
}
