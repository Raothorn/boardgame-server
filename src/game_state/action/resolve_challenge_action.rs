use std::fmt::Display;

use serde::Deserialize;
use serde_json::Value;

use super::Action;
use crate::game_state::{GamePhase, GameState, Update, Challenge};

#[derive(Deserialize)]
pub struct ResolveChallengeAction {
    selected_crew: Vec<usize>,
}

impl ResolveChallengeAction {
    fn sufficient_skill(&self, state: &GameState, challenge: &Challenge) -> bool {
        let mut total = 0;
        for crew_ix in self.selected_crew.iter() {
            let crew = &state.crew[*crew_ix];
            total += crew.skills[&challenge.skill];
        }
        return total >= challenge.amount;
    }
}

impl Action for ResolveChallengeAction {
    fn execute(&self, state: &GameState) -> Update {
        // Validate
        if let GamePhase::ChallengePhase(challenge) = state.phase() {
            if self.sufficient_skill(state, &challenge) {
                Ok(state.clone())
                // TODO fatigue crew
            } else {
                Err("Insufficient skill".to_owned())
            }
        } else {
            Err("Wrong phase".to_owned())
        }
        .and_then(|g| g.pop_phase())
        .map(|g| g.prompt(&Value::Null))
    }
}

impl Display for ResolveChallengeAction {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "Resolve Challenge Action")
    }
}
