use serde::Deserialize;

use crate::game_state::{
    game_phase::ShipActionSubphase, GamePhase, GameState, Update,
};

use super::Action;

#[derive(Deserialize)]
pub struct DrawForDeckAction {
    #[allow(dead_code)]
    player_ix: usize,
}

impl Action for DrawForDeckAction {
    fn execute(&self, state: &GameState) -> Update {
        let mut gs = state.clone();
        match gs.phase() {
            GamePhase::ShipActionPhase(Some(
                ShipActionSubphase::DeckAction {
                    ref search_tokens_drawn,
                },
            )) => {
                if search_tokens_drawn.len() < 3 {
                    match gs.search_token_deck.draw() {
                        Ok(token) => {
                            let mut search_tokens_drawn =
                                search_tokens_drawn.clone();
                            search_tokens_drawn.push(token);
                            let phase = GamePhase::ShipActionPhase(Some(
                                ShipActionSubphase::DeckAction {
                                    search_tokens_drawn,
                                },
                            ));
                            Ok(gs).and_then(|g| g.set_phase(phase))
                        }
                        Err(err) => Err(err),
                    }
                } else {
                    Err("You may only draw 3 tokens".to_owned())
                }
            }
            _ => Err("Wrong phase.".to_owned()),
        }
    }
}

impl std::fmt::Display for DrawForDeckAction {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "Draw for deck action")
    }
}
