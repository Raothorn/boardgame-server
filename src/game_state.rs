pub mod action;

#[derive(Clone)]
enum GamePhase {
    Start,
    ShipAction(Option<ShipActionPhase>),
    ShipActionComplete
}

#[derive(Clone)]
enum ShipActionPhase {
    BridgeAction {
        complete: bool,
    },
    GalleyAction {
        gain_phase_complete: bool,
        discard_chosen: bool,
        discard_choice: u32,
        crew_choice: u32,
    },
}

struct GameState {
    phase: GamePhase,
    players: Vec<Player>,
    crew: Vec<Crew>,
    deck: AbilityCardDeck,
    room: ShipRoom,
}

impl GameState {
    fn init_state() -> GameState {
        GameState {
            phase: GamePhase::Start,
            players: Vec::new(),
            crew: vec![
                Crew {
                    name: String::from("Sofi Odessa"),
                    fatigue: 0,
                },
                Crew {
                    name: String::from("Laurant Lapointe"),
                    fatigue: 0,
                },
            ],
            deck: AbilityCardDeck { cards: Vec::new() },
            room: ShipRoom::None,
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

#[derive(PartialEq, Eq, Debug, Clone)]
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
    None,
}
