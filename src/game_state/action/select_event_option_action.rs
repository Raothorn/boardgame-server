use serde::Deserialize;

use super::Action;
use crate::game_state::{GameState, Update, GamePhase};

#[derive(Deserialize)]
pub struct SelectEventOptionAction {
    option_ix: usize,
    player_ix: usize,
}

impl Action for SelectEventOptionAction {
    fn execute(&self, state: &GameState) -> Update {
        let mut gs = state.clone();

        if let GamePhase::EventPhase(Some(ref card)) = gs.phase() {
            let option = card.options.get(self.option_ix);
            match option {
                Some(option) => {
                    gs.event_card_deck.add_to_discard(card);
                    Ok(gs)
                        .and_then(|g| g.set_phase(GamePhase::ShipAction(None)))
                        .map(|g| g.clear_prompt("selectEventOption"))
                        .and_then(|g| (option.handle_option)(&g))
                }
                None => Err("option index not valid".to_owned()),
            }
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




