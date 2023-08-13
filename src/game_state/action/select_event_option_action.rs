use serde::{Deserialize, Serialize};

use super::Action;
use crate::game_state::{GamePhase, GameState, Update};

#[derive(Deserialize, Serialize)]
pub struct SelectEventOptionAction {
    option_ix: Option<usize>,
    #[allow(dead_code)]
    player_ix: usize,
}

#[typetag::serde(name = "selectEventOptionAction")]
impl Action for SelectEventOptionAction {
    fn execute(&self, state: &GameState) -> Update<GameState> {
        let mut gs = state.clone();

        if let GamePhase::EventPhase(Some(ref card)) = gs.phase() {
            let effects = match self.option_ix {
                Some(option_ix) => {
                    let option = card.options.get(option_ix).unwrap();
                    option.effects.clone()
                }
                None => card.effects.clone(),
            };
            gs.event_card_deck.add_to_discard(card);
            Ok(gs)
                .and_then(|g| g.set_phase(GamePhase::MainActionPhase(None, 0)))
                .and_then(|g| {
                    effects
                        .iter()
                        .fold(Ok(g), |g, eff| g.and_then(|g| eff.resolve(g)))
                })
        } else {
            Err("wrong phase".to_owned())
        }
    }
}

impl std::fmt::Display for SelectEventOptionAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Select Event Option")
    }
}
