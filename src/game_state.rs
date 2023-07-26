use serde::{Serialize, Deserialize};

pub mod action;

#[derive(Clone, Serialize)]
enum GamePhase {
    Start,
    ShipAction(Option<ShipActionPhase>),
    ShipActionComplete
}

#[derive(Clone, Serialize)]
enum ShipActionPhase {
    BridgeAction,
    GalleyAction {
        gain_phase_complete: bool,
    },
}

#[derive(Serialize)]
pub struct GameState {
    #[serde(skip)]
    phase: GamePhase,
    players: Vec<Player>,
    crew: Vec<Crew>,
    deck: AbilityCardDeck,
    room: ShipRoom,
    #[serde(skip)]
    prompt: Option<String>
}

impl GameState {
    pub fn init_state() -> GameState {
        GameState {
            phase: GamePhase::Start,
            players: vec![Player::default()],
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
            prompt: None
        }
    }

    fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }
}

#[derive(Serialize)]
struct Crew {
    name: String,
    fatigue: u32,
}

impl Crew {
    fn reduce_fatigue(&mut self) {
        if self.fatigue > 0 {
            self.fatigue -= 1;
        }
    }
}

#[derive(Serialize)]
struct AbilityCardDeck {
    cards: Vec<AbilityCard>,
}

impl AbilityCardDeck {
    fn draw_card(&mut self) -> Option<AbilityCard> {
        self.cards.pop()
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize)]
struct AbilityCard {
    name: String,
}

#[derive(Default, Serialize)]
struct Player {
    command_tokens: u32,
    hand: Vec<AbilityCard>,
}

impl Player {
    fn add_card(&mut self, card: AbilityCard) {
        self.hand.push(card);
    }

    fn discard_card(&mut self, card_ix: usize) -> AbilityCard {
        self.hand.remove(card_ix)
    }
}

struct GameManager {
    state: GameState,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
enum ShipRoom {
    Galley,
    Bridge,
    Deck,
    None,
}
