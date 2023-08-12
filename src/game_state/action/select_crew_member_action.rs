use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::game_state::game_phase::GamePhase as Gp;
use crate::game_state::GameState;
use crate::game_state::Update;

use super::Action;

#[derive(Deserialize, Serialize)]
pub struct SelectCrewMemberAction {
    crew_ix: Option<usize>,
    player_ix: usize,
}

#[typetag::serde(name = "selectCrewMemberAction")]
impl Action for SelectCrewMemberAction {
    fn execute(&self, state: &GameState) -> Update<GameState> {
        if let Gp::SelectCrewMemberPhase{crew_ix: None, title, callback} =
            state.phase()
        {
            match self.crew_ix {
                // Crew member selected
                Some(crew_ix) => {
                    let action = super::get_action(&callback);

                    Ok(state.clone())
                        .and_then(|g| {
                            g.set_phase(Gp::SelectCrewMemberPhase {
                                crew_ix: Some(crew_ix),
                                title,
                                callback: "".to_owned(),
                            })
                        })
                        .and_then(|g| action.execute(&g))
                        .and_then(|g| g.pop_phase())
                }

                // Selection cancelled
                None => Ok(state.clone()).and_then(|g| g.pop_phase()),
            }
        } else {
            Err("wrong phase".to_owned())
        }
    }
}

impl Display for SelectCrewMemberAction {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "select crew member")
    }
}
