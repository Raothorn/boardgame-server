use crate::game_state::{GameState, GamePhase, ShipActionPhase, ShipRoom};
use super::Action;

struct TakeShipAction {
    room: ShipRoom,
    player_ix: usize,
}

impl TakeShipAction {
    fn bridge_action(&self, state: &mut GameState) {
        let player = &mut state.players[self.player_ix];
        player.command_tokens += 3;

        // TODO check for empty deck
        let card = state.deck.draw_card().unwrap();
        player.add_card(card);

        if let GamePhase::ShipAction(None) = state.phase {
            state.phase = GamePhase::ShipAction(
                Some(ShipActionPhase::BridgeAction { complete: true })
            )
        }
    }
}

impl Action for TakeShipAction {
    fn execute(&self, state: &mut GameState) {
        match self.room {
            ShipRoom::Bridge => self.bridge_action(state),
            _ => (),
        }
        state.room = self.room;
    }

    fn invalid(&self, state: &GameState) -> Option<String> {
        match state.phase {
            GamePhase::ShipAction(Some(_)) => {
                if state.room == self.room {
                    Some(String::from("You cannot visit the same room twice"))
                } else {
                    None
                }
            }
            _ => Some(String::from("Wrong phase.")),
        }
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
        assert!(matches!(state.phase,
             GamePhase::ShipAction(
                Some(ShipActionPhase::BridgeAction { complete: true})
            )));
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
        let result = action.invalid(&mut state);

        assert!(matches!(result, Some(_)))
    }
}