use std::fmt::Display;

use serde::Deserialize;

use super::Action;
use crate::game_state::{
    GamePhase, GameState, ShipActionPhase, ShipRoom, Update,
};

#[derive(Deserialize)]
pub struct TakeShipAction {
    room: ShipRoom,
    player_ix: usize,
}

impl TakeShipAction {
    fn bridge_action(&self, state: &GameState) -> Update {
        Ok(state.clone())
            .and_then(|g| g.give_command_tokens(self.player_ix, 3))
            .and_then(|g| g.draw_cards(self.player_ix, 1))
            .map(|g| GameState {
                room: ShipRoom::Bridge,
                phase: GamePhase::ShipActionComplete,
                ..g
            })
    }

    fn deck_action(&self, state: &GameState) -> Update {
        // Ok(state.clone())
        //     .map(|g| g.prompt_str())
        todo!()
    }

    fn galley_action(&self, state: &GameState) -> Update {
        let phase = GamePhase::ShipAction(Some(
            ShipActionPhase::GalleyAction {
                gain_phase_complete: true,
            },
        ));


        Ok(state.clone())
            .and_then(|g| g.give_command_tokens(self.player_ix, 3))
            .and_then(|g| g.draw_cards(self.player_ix, 2))
            .map(|g| GameState {
                room: ShipRoom::Galley,
                phase: phase,
                ..g
            })
            .map(|g| g.prompt_str("selectDiscardForGalleyAction"))
    }
}

impl Action for TakeShipAction {
    fn execute(&self, state: &GameState) -> Update {
        if let GamePhase::ShipAction(None) = &state.phase {
            if state.room == self.room {
                Err("You cannot visit the same room twice".to_owned())
            } else {
                match self.room {
                    ShipRoom::Bridge => self.bridge_action(state),
                    ShipRoom::Galley => self.galley_action(state),
                    _ => Err(String::from("Not implemented")),
                }
            }
        } else {
            Err("Wrong phase.".to_owned())
        }
    }
}

impl Display for TakeShipAction {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "Take Ship Action:\n Room: {:?}\n PlayerIx: {}",
            self.room, self.player_ix
        )
    }
}
