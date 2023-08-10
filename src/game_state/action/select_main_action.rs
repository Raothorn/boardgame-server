use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::game_state::{game_phase::{GamePhase, MainActionSubphase}, GameState, Update};

use super::Action;

#[derive(Serialize, Deserialize)]
pub struct SelectMainAction {
    //TODO action type
    player_ix: u32,
}

#[typetag::serde(name = "selectMainAction")]
impl Action for SelectMainAction {
    fn execute(&self, state: &GameState) -> Update {
        if let GamePhase::MainActionPhase(actions) = state.phase() {
            let mut actions = actions.clone();
            actions.push(MainActionSubphase::Travel);

            let phase = GamePhase::MainActionPhase(actions);

            Ok(state.clone()).and_then(|g| g.set_phase(phase))
        }
        else {
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
