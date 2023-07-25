#![allow(dead_code)]

mod game_state {
    pub struct GameState {
        pub players: Vec<Player>,
        pub deck: AbilityCardDeck,
        pub room: ShipRoom
    }

    impl GameState {
        pub fn init_state() -> GameState {
            GameState {
                players: Vec::new(),
                deck: AbilityCardDeck { cards: Vec::new() },
                room: ShipRoom::Galley
            }
        }
    }

    pub struct AbilityCardDeck {
        cards: Vec<AbilityCard>
    }

    pub struct AbilityCard {
        name: String 
    }

    pub struct Player {
        command_tokens: u32,
        hand: Vec<AbilityCard>
    }

    struct GameManager {
        state: GameState,
    }

    #[derive(Clone, Copy)]
    pub enum ShipRoom {
        Galley,
        Bridge,
        Deck
    }
}

mod actions {
    use super::game_state;
    use super::game_state::GameState;
    pub trait Action {
        fn execute(&self, state:&mut game_state::GameState);

    }

    pub struct TakeShipAction {
        pub room: game_state::ShipRoom
    }

    impl Action for TakeShipAction{
        fn execute(&self, state:&mut GameState) {
            state.room = self.room;
        }
    }

}

#[cfg(test)]
mod tests {
    use super::game_state::*;
    use super::actions::*;
    use super::actions::Action;

    #[test]
    fn test_take_ship_action() {
        let mut state = GameState::init_state();

        let action = TakeShipAction { room: ShipRoom::Bridge };
        action.execute(&mut state);

        assert!(matches!(state.room, ShipRoom::Bridge))
    }
}

fn main() {
    println!("Hello, world!");
}