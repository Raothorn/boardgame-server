#![allow(dead_code)]

mod game_state {
    struct GameState {
        players: Vec<Player>,
        crew: Vec<Crew>,
        deck: AbilityCardDeck,
        room: ShipRoom,
    }

    impl GameState {
        fn init_state() -> GameState {
            GameState {
                players: Vec::new(),
                crew: vec![
                    Crew { name: String::from("Sofi Odessa"), fatigue: 0},
                    Crew { name: String::from("Laurant Lapointe"), fatigue: 0}
                ],
                deck: AbilityCardDeck { cards: Vec::new() },
                room: ShipRoom::None
            }
        }

        fn add_player(&mut self, player: Player) {
            self.players.push(player);
        }
    }

    struct Crew {
        name: String,
        fatigue: usize,
    }

    struct AbilityCardDeck {
        cards: Vec<AbilityCard>,
    }

    impl AbilityCardDeck {
        fn draw_card(&mut self) -> Option<AbilityCard> {
            self.cards.pop()
        }
    }

    #[derive (PartialEq, Eq, Debug, Clone)]
    struct AbilityCard {
        name: String,
    }

    #[derive(Default)]
    struct Player {
        command_tokens: u32,
        hand: Vec<AbilityCard>,
    }

    impl Player {
        fn add_card(&mut self, card: AbilityCard) {
            self.hand.push(card);
        }
    }

    struct GameManager {
        state: GameState,
    }

    #[derive(Clone, Copy, PartialEq)]
    enum ShipRoom {
        Galley,
        Bridge,
        Deck,
        None
    }

    mod actions {
        use super::{GameState, Player, ShipRoom};

        trait Action {
            fn execute(&self, state: &mut GameState);
            fn invalid(&self, state: &GameState) -> Option<String>;
        }

        struct TakeShipAction {
            room: super::ShipRoom,
            player_ix: usize
        }

        impl TakeShipAction {
            fn bridge_action(&self, state: &mut GameState) {
                let player = &mut state.players[self.player_ix];
                player.command_tokens += 3;

                // TODO check for empty deck
                let card = state.deck.draw_card().unwrap();
                player.add_card(card);
            }
        }

        impl Action for TakeShipAction {
            fn execute(&self, state: &mut GameState) {
                match self.room {
                    ShipRoom::Bridge => self.bridge_action(state),
                    _ => ()
                }
                state.room = self.room;
            }

            fn invalid(&self, state: &GameState) -> Option<String> {
                if state.room == self.room {
                    Some(String::from("You cannot visit the same room twice"))
                }
                else {
                    None
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::game_state::{ShipRoom, AbilityCard};

            #[test]
            fn test_take_ship_action_bridge() {
                let mut state = GameState::init_state();
                state.add_player(Player::default());

                let card = AbilityCard {name: String::from("Card 1")};
                state.deck.cards.push(card.clone());

                let action = TakeShipAction {
                    room: ShipRoom::Bridge,
                    player_ix: 0
                };
                action.execute(&mut state);

                
                // The room has changed
                assert!(matches!(state.room, ShipRoom::Bridge));

                // The player has drawn a card
                let player_card = state.players[0].hand.first().unwrap();
                assert_eq!(*player_card, card);
            }

            #[test]
            fn test_cannot_use_same_ship_action() {
                let mut state = GameState::init_state();
                state.add_player(Player::default());

                let card = AbilityCard {name: String::from("Card 1")};
                state.deck.cards.push(card.clone());
                state.room = ShipRoom::Bridge;

                let action = TakeShipAction {
                    room: ShipRoom::Bridge,
                    player_ix: 0
                };
                let result = action.invalid(&mut state);

                assert!(matches!(result, Some(_)))
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
