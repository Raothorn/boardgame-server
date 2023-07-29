use serde::Deserialize;

use crate::game_state::{
    GamePhase, GameState, SearchToken, ShipActionSubphase, Update,
};

use super::Action;

#[derive(Deserialize)]
pub struct DrawForDeckAction {
    player_ix: usize,
}

impl Action for DrawForDeckAction {
    fn execute(&self, state: &GameState) -> Update {
        let mut gs = state.clone();
        match gs.phase {
            GamePhase::ShipAction(Some(
                ShipActionSubphase::DeckAction {
                    ref mut search_tokens_drawn,
                },
            )) => {
                if search_tokens_drawn.len() < 3 {
                    match gs.search_token_deck.draw() {
                        Ok((token, deck)) => {
                            search_tokens_drawn.push(token);
                            gs.search_token_deck = deck;
                            Ok(gs)
                        },
                        Err(err) => Err(err)
                    }
                } else {
                    Err("You may only draw 3 tokens".to_owned())
                }
            },
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
