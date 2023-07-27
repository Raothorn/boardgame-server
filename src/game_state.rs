use serde::{Deserialize, Serialize};

pub mod action;

#[derive(Clone, Serialize)]
enum GamePhase {
    ShipAction(Option<ShipActionPhase>),
    ShipActionComplete,
}

#[derive(Clone, Serialize)]
enum ShipActionPhase {
    BridgeAction,
    GalleyAction { gain_phase_complete: bool },
}

#[derive(Serialize)]
pub struct GameState {
    phase: GamePhase,
    players: Vec<Player>,
    crew: Vec<Crew>,
    deck: AbilityCardDeck,
    room: ShipRoom,
    #[serde(skip)]
    pub prompt: Option<String>,
}

impl GameState {
    pub fn init_state() -> GameState {
        GameState {
            phase: GamePhase::ShipAction(None),
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
            deck: AbilityCardDeck {
                cards: vec![
                    AbilityCard {
                        name: String::from("card 1"),
                    },
                    AbilityCard {
                        name: String::from("card 2"),
                    },
                ],
            },
            room: ShipRoom::None,
            prompt: None,
        }
    }

    fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    fn give_command_tokens(&mut self, player_ix: usize, amount: u32) {
        if let Some(player) = self.players.get_mut(player_ix) {
            player.command_tokens += amount;
        }
    }

    fn draw_cards(&mut self, player_ix: usize, amount: u32) {
        if let Some(player) = self.players.get_mut(player_ix) {
            for _ in 0..amount {
                let card = self.deck.draw_card();

                if let Some(card) = card {
                    player.add_card(card);
                }
            }
        }
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

    fn discard_card(
        &mut self,
        card_ix: usize,
    ) -> Option<AbilityCard> {

        if self.hand.len() <= card_ix {
            None
        } else {
            Some(self.hand.remove(card_ix))
        }
    }
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
enum ShipRoom {
    Galley,
    Bridge,
    Deck,
    None,
}
