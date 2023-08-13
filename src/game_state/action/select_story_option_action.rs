use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::game_state::effect::resolve_effects;
use crate::game_state::game_phase::GamePhase as Gp;
use crate::game_state::GameState;
use crate::game_state::Update;

use super::Action;

#[derive(Deserialize, Serialize)]
pub struct SelectStoryOptionAction {
    option_ix: usize,
    player_ix: usize,
}

#[typetag::serde(name = "selectStoryOptionAction")]
impl Action for SelectStoryOptionAction {
    fn execute(&self, state: &GameState) -> Update<GameState> {
        if let Gp::ExplorePhase(story) = state.phase() {
            let mut effects = story.effects.clone();

            match story.get_option(self.option_ix) {
                Some(option) => {

                    if option.disabled(state) {
                        return Err("Option is disabled".to_owned());
                    } 
                    let mut option_effects = option.effects.clone();
                    effects.append(&mut option_effects);
                }
                None => (),
            };
            resolve_effects(state, effects)

        } else {
            Err("Wrong phase".to_owned())
        }
    }
}

impl Display for SelectStoryOptionAction {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "Select Story Option")
    }
}
