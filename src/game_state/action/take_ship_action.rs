use std::fmt::Display;

use serde::Deserialize;

use super::Action;
use crate::game_state::{
    GamePhase, GameState, ShipActionSubphase, ShipRoom, Update,
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
                ..g
            })
            .and_then(|g| g.set_phase(GamePhase::EventPhase(None)))
    }

    fn deck_action(&self, state: &GameState) -> Update {
        let phase = GamePhase::ShipAction(Some(
            ShipActionSubphase::DeckAction {
                search_tokens_drawn: Vec::new(),
            },
        ));
        Ok(state.clone())
            .map(|g| GameState {
                room: ShipRoom::Deck,
                ..g
            })
            .and_then(|g| g.set_phase(phase))
            .map(|g| g.prompt_str("drawForDeckAction"))
    }

    fn galley_action(&self, state: &GameState) -> Update {
        let phase = GamePhase::ShipAction(Some(
            ShipActionSubphase::GalleyAction {
                gain_phase_complete: true,
            },
        ));

        Ok(state.clone())
            .and_then(|g| g.give_command_tokens(self.player_ix, 3))
            .and_then(|g| g.draw_cards(self.player_ix, 2))
            .map(|g| GameState {
                room: ShipRoom::Galley,
                ..g
            })
            .and_then(|g| g.set_phase(GamePhase::EventPhase(None)))
            .map(|g| g.prompt_str("selectDiscardForGalleyAction"))
    }
}

impl Action for TakeShipAction {
    fn execute(&self, state: &GameState) -> Update {
        if let GamePhase::ShipAction(None) = &state.phase() {
            if state.room == self.room {
                Err("You cannot visit the same room twice".to_owned())
            } else {
                match self.room {
                    ShipRoom::Bridge => self.bridge_action(state),
                    ShipRoom::Galley => self.galley_action(state),
                    ShipRoom::Deck => self.deck_action(state),
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

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    use ShipActionSubphase as Sas;
    use GamePhase::ShipAction as Sa;
    use GamePhase::EventPhase as Ep;
    use GamePhase::ChallengePhase as Cp;

    #[test]
    fn test_bridge_action() {
        let gs = GameState::init_state();
        let action = TakeShipAction {
            room: ShipRoom::Bridge,
            player_ix: 0,
        };

        let result = action.execute(&gs);
        assert!(result.is_ok());

        insta::with_settings!({sort_maps => true}, {
            // the tests here will force maps to sort
            insta::assert_json_snapshot!(result.unwrap());
        });
    }

    // #[test_case(Sa(Some(Sas::GalleyAction{gain_phase_complete: true})); 
    //             "Current in ship action")]
    #[test_case(Ep(None); "In event phase")]
    // #[test_case(Cp(); "In event phase")]
    // #[test_case(GamePhase)]
    fn test_bridge_action_err_if_wrong_phase(phase: GamePhase) {
        let gs = GameState::init_state();
        let action = TakeShipAction {
            room: ShipRoom::Bridge,
            player_ix: 0,
        };

        let result = action.execute(&gs);
        assert!(result.is_err());
    }
}
