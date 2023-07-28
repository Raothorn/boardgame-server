use serde::{Deserialize, Serialize};

pub mod action;
#[derive(Clone, Serialize)]
enum GamePhase {
    ShipAction(Option<ShipActionPhase>),
    ShipActionComplete,
}

#[derive(Clone, Serialize)]
enum ShipActionPhase {
    GalleyAction { gain_phase_complete: bool },
}

#[derive(Clone, Serialize)]
pub struct GameState {
    phase: GamePhase,
    players: Vec<Player>,
    crew: Vec<Crew>,
    deck: AbilityCardDeck,
    room: ShipRoom,
    resources: Resources,
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
                    fatigue: 1,
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

    fn add_player(&self, player: Player) -> Update {
        let mut gs = self.clone();
        gs.players.push(player);

        Ok(gs)
    }

    fn give_command_tokens(
        self,
        player_ix: usize,
        amount: u32,
    ) -> Update {
        let mut gs = self.clone();
        if let Some(player) = gs.players.get_mut(player_ix) {
            player.command_tokens += amount;
            Ok(gs)
        } else {
            Err("Player does not exist".to_owned())
        }
    }

    fn draw_cards(self, player_ix: usize, amount: u32) -> Update {
        let mut gs = self.clone();

        if let Some(player) = gs.players.get_mut(player_ix) {
            for _ in 0..amount {
                let card = gs.deck.draw_card();

                if let Some(card) = card {
                    player.add_card(card);
                }
            }
        }

        Ok(gs)
    }

    fn discard_card(
        self,
        player_ix: usize,
        card_ix: usize,
    ) -> Update {

        if player_ix >= self.players.len() {
            Err("".to_owned())
        } else {
            let mut gs = self.clone();
            let player = &gs.players[player_ix];

            match player.discard_card(card_ix) {
                Ok((player, card)) => {
                    gs.players[player_ix] = player;
                    gs.deck.cards.push(card);
                    Ok(gs)
                }
                Err(err) => Err(err)
            }
        }
    }

    fn reduce_fatigue(self, crew_ix:usize) -> Update {
        let mut gs = self.clone();
        if let Some(crew) = gs.crew.get_mut(crew_ix) {
            crew.reduce_fatigue()
        };

        Ok(gs) 
    }

    fn prompt(self, msg: &str) -> Update {
        let mut gs = self.clone();
        gs.prompt = Some(msg.to_owned());
        Ok(gs)
    }

}

#[derive(Serialize, Clone)]
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

#[derive(Serialize, Clone)]
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

#[derive(Default, Serialize, Clone)]
struct Player {
    command_tokens: u32,
    hand: Vec<AbilityCard>,
}

impl Player {
    fn add_card(&mut self, card: AbilityCard) {
        self.hand.push(card);
    }

    fn discard_card(
        &self,
        card_ix: usize,
    ) -> Result<(Player, AbilityCard), String> {
        let mut player = self.clone();
        if player.hand.len() <= card_ix {
            Err("this card does not exist in the players hand"
                .to_owned())
        } else {
            let card = player.hand.remove(card_ix);
            Ok((player, card))
        }
    }
}

#[derive(Clone, Serialize)]
struct Resources {
    coins: u32,
    grain: u32,
    meat: u32
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
enum ShipRoom {
    Galley,
    Bridge,
    Deck,
    None,
}

type Update = Result<GameState, String>;
