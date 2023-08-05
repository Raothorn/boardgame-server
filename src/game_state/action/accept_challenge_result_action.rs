
use std::fmt::Display;

use serde::Deserialize;

use crate::game_state::game_phase::GamePhase;

use super::Action;

#[derive(Deserialize)]
pub struct AcceptChallengeResultAction {
    #[allow(dead_code)]
    player_ix: usize
}

impl Action for AcceptChallengeResultAction {
    fn execute(&self, state: &crate::game_state::GameState) -> crate::game_state::Update {
        
        if let GamePhase::ChallengePhase {
            challenge: _,
            added: Some(_),
        } = state.phase() {
            Ok(state.clone())
                .and_then(|g| g.pop_phase())
        } else {
            Err("Wrong phase".to_owned())
        }
    }
}

impl Display for AcceptChallengeResultAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Accept Challenge Result")
    }
}
