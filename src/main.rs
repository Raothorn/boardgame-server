#![allow(dead_code)]

struct GameState {
    players: Vec<Player>,
    deck: AbilityCardDeck,
    room: ShipRoom
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

struct AbilityCardDeck {
    cards: Vec<AbilityCard>
}

struct AbilityCard {
    name: String 
}

struct Player {
    command_tokens: u32,
    hand: Vec<AbilityCard>
}

struct GameManager {
    state: GameState,
}

impl GameManager {
    fn execute_action(&mut self, action: &impl Action) {
        todo!();
    }
}

trait Action {
    fn execute(&self, state:&mut GameState);

}

struct TakeShipAction {
    room: ShipRoom
}

impl Action for TakeShipAction{
    fn execute(&self, state:&mut GameState) {
        state.room = self.room;
    }
}

#[derive(Clone, Copy)]
enum ShipRoom {
    Galley,
    Bridge,
    Deck
}

mod tests {
    use super::*;

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