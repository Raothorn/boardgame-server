use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::game_state::{GamePhase, GameState, Update, modifier::{ModifierLifetime, Modifier}};

use super::Action;

#[derive(Deserialize, Serialize, Clone)]
pub struct EndTurnAction {
    player_ix: usize,
}

#[typetag::serde(name = "endTurnAction")]
impl Action for EndTurnAction {
    fn execute(&self, state: &GameState) -> Update<GameState> {
        if let GamePhase::MainActionPhase(_, action_ct) =
            state.phase()
        {
            // TODO might as well do an actual end turn phase at some point
            if action_ct == 2 {
                state
                    .set_phase(GamePhase::ShipActionPhase(None))
                    .and_then(|g| {
                        g.filter_out_modifiers(|m| {
                            match m.lifetime {
                                ModifierLifetime::ThisTurn => true,
                                _ => false
                            }
                        })
                    })
            } else {
                Err("Haven't done 2 actions yet".to_owned())
            }
        } else {
            Err("You can't end the turn yet".to_owned())
        }
    }
}

impl Display for EndTurnAction {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "End Turn Action")
    }
}
