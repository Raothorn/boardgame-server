use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::Action;
use crate::game_state::{
    challenge::Challenge, GamePhase, GameState, Update,
};

#[derive(Deserialize, Serialize)]
pub struct ResolveChallengeAction {
    selected_crew: Vec<usize>,
}

impl ResolveChallengeAction {
    fn crew_skill(
        &self,
        state: &GameState,
        challenge: &Challenge,
    ) -> u32 {
        let mut total = 0;
        for crew_ix in self.selected_crew.iter() {
            let crew = &state.crew[*crew_ix];
            total += crew.skills[&challenge.skill];
        }
        total
    }
}

#[typetag::serde(name="resolveChallengeAction")]
impl Action for ResolveChallengeAction {
    fn execute(&self, state: &GameState) -> Update<GameState> {
        let gs = state.clone();

        if let GamePhase::ChallengePhase {
            challenge,
            added: None,
        } = state.phase()
        {
            let added = 4;
            let phase = GamePhase::ChallengePhase {
                challenge: challenge.clone(),
                added: Some(added),
            };

            if self.crew_skill(state, &challenge) + added
                >= challenge.amount
            {
                (challenge.if_succeed)(&gs)
            } else {
                (challenge.if_fail)(&gs)
            }

            .and_then(|g| {
                let mut gs = g.clone();

                let result = self.selected_crew.iter().fold(Ok(gs), |g, crew_ix| {
                    g.and_then(|g| g.update_crew(*crew_ix, |c| c.change_fatigue(1)))
                });
                // for crew_ix in self.selected_crew.iter() {
                //     gs.crew[*crew_ix].change_fatigue(1)
                // }
                // gs
                result
            })

            .and_then(|g| g.set_phase(phase))

        } else {
            Err("Wrong phase".to_owned())
        }
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
