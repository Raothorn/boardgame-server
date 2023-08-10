use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::Action;
use crate::game_state::game_phase::ShipActionSubphase;
use crate::game_state::{GamePhase, GameState, SearchToken, Update};

#[derive(Deserialize, Serialize)]
pub struct ChooseTokenForDeckAction {
    token_id: u32,
    #[allow(dead_code)]
    player_ix: usize,
}

#[typetag::serde(name="chooseTokenForDeckAction")]
impl Action for ChooseTokenForDeckAction {
    fn execute(&self, state: &GameState) -> Update {
        let t: Result<(SearchToken, Vec<SearchToken>), String> =
            if let GamePhase::ShipActionPhase(Some(
                ShipActionSubphase::DeckAction {
                    ref search_tokens_drawn,
                },
            )) = state.phase()
            {
                if search_tokens_drawn.is_empty() {
                    Err("You must draw at least 1 token".to_owned())
                } else {
                    let (tokens, discards): (
                        Vec<SearchToken>,
                        Vec<SearchToken>,
                    ) = search_tokens_drawn.clone().iter().partition(
                        |SearchToken(id)| *id == self.token_id,
                    );

                    tokens
                        .first()
                        .ok_or("Couldn't find token id".to_owned())
                        .map(|t| (*t, discards))
                }
            } else {
                Err("wrong phase".to_owned())
            };

        t.and_then(|(token, discards)| {
            let mut gs = state.clone();
            for discard in discards {
                gs.search_token_deck.add_to_discard(&discard);
            }
            gs.search_token_deck.add_to_discard(&token);
            Ok(gs)
                .and_then(|g| {
                    g.set_phase(GamePhase::EventPhase(None))
                })
                .and_then(|g| g.apply_search_tokens(&token))
        })
    }
}

impl Display for ChooseTokenForDeckAction {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "Choose Token For Deck Action")
    }
}
