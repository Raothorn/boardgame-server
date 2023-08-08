use std::fmt::Display;

use serde::Deserialize;

use crate::game_state::{Update, GameState};

use super::Action;

#[derive(Deserialize)]
pub struct AcceptMessageAction {
    #[allow(dead_code)]
    player_ix: usize,
}

impl Action for AcceptMessageAction {
    fn execute(
        &self,
        state: &GameState,
    ) -> Update {
        Ok(state.clone()).and_then(|g| g.dequeue_message())
    }
}

impl Display for AcceptMessageAction {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "Accept Message")
    }
}
