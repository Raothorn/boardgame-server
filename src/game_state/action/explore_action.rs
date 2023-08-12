use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::game_state::game_phase::GamePhase as Gp;
use crate::game_state::game_phase::MainActionSubphase as Mas;
use crate::game_state::GameState;
use crate::game_state::Update;

use super::Action;

#[derive(Deserialize, Serialize)]
pub struct ExploreAction {
    port: u32,
    player_ix: usize,
}

#[typetag::serde(name = "exploreAction")]
impl Action for ExploreAction {
    fn execute(&self, state: &GameState) -> Update<GameState> {
        if let Gp::MainActionPhase(Some(Mas::Explore), _) =
            state.phase()
        {
            let story = state.map.storybook.port_story(self.port);
            match story {
                Ok(story) => Ok(state.clone()).and_then(|g| {
                    g.push_phase(Gp::ExplorePhase(story.clone()))
                }),
                Err(err) => Err(err),
            }
        } else {
            Err("wrong phase".to_owned())
        }
    }
}

impl Display for ExploreAction {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "Expore")
    }
}
