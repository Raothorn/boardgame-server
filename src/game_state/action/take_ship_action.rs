use serde::Deserialize;

use super::Action;
use crate::game_state::{GamePhase, GameState, ShipActionPhase, ShipRoom};

#[derive(Deserialize)]
pub struct TakeShipAction {
    room: ShipRoom,
    player_ix: usize,
}

impl TakeShipAction {
    fn bridge_action(&self, state: &mut GameState) {                   
        state.give_command_tokens(self.player_ix, 3);
        state.draw_cards(self.player_ix, 1);

        state.phase = GamePhase::ShipActionComplete;
        state.room = ShipRoom::Bridge;
    }

    fn galley_action(&self, state: &mut GameState) {
        state.give_command_tokens(self.player_ix, 3);
        state.draw_cards(self.player_ix, 2);
        state.room = ShipRoom::Galley;
        state.phase =
            GamePhase::ShipAction(Some(ShipActionPhase::GalleyAction {
                gain_phase_complete: true,
            }));
        state.prompt = Some(String::from("selectDiscardForGalleyAction"));
    }
}

impl Action for TakeShipAction {
    fn execute(&self, state: &mut GameState) -> Option<String> {
        if let GamePhase::ShipAction(None) = &state.phase {
            if state.room == self.room {
                Some(String::from("You cannot visit the same room twice"))
            } else {
                match self.room {
                    ShipRoom::Bridge => {
                        self.bridge_action(state);
                        None
                    }
                    ShipRoom::Galley => {
                        self.galley_action(state);
                        None
                    }
                    _ => Some(String::from("Not implemented")),
                }
            }
        } else {
            Some(String::from("Wrong phase."))
        }
    }

    fn name(&self) -> &str {
        "take ship action"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::{AbilityCard, Player, ShipRoom};

    #[test]
    fn test_take_ship_action_bridge() {
        // Setup
        let mut state = GameState::init_state();
        state.add_player(Player::default());
        let card = AbilityCard {
            name: String::from("Card 1"),
        };
        state.deck.cards.push(card.clone());
        state.phase = GamePhase::ShipAction(None);

        let action = TakeShipAction {
            room: ShipRoom::Bridge,
            player_ix: 0,
        };
        action.execute(&mut state);

        // The room has changed
        assert!(matches!(state.room, ShipRoom::Bridge));

        // The player has drawn a card
        let player_card = state.players[0].hand.first().unwrap();
        assert_eq!(*player_card, card);

        // The phase is complete
        assert!(matches!(
            state.phase,
            GamePhase::ShipAction(Some(ShipActionPhase::BridgeAction))
        ));
    }

    #[test]
    fn test_cannot_use_same_ship_action() {
        let mut state = GameState::init_state();
        state.add_player(Player::default());

        let card = AbilityCard {
            name: String::from("Card 1"),
        };
        state.deck.cards.push(card.clone());
        state.room = ShipRoom::Bridge;

        let action = TakeShipAction {
            room: ShipRoom::Bridge,
            player_ix: 0,
        };
        let result = action.execute(&mut state);

        assert!(matches!(result, Some(_)))
    }
}
