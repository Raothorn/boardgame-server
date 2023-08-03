use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};
use serde_json::{json, Value};

pub mod action;
#[allow(dead_code, unreachable_code, unused_variables)]
pub mod event_deck;

use self::event_deck::event_deck;
use event_deck::EventCard;

#[derive(Clone, Serialize)]
enum GamePhase {
    ShipAction(Option<ShipActionSubphase>),
    EventPhase(Option<EventCard>),
    ChallengePhase(Challenge),
}

#[derive(Clone, Serialize)]
enum ShipActionSubphase {
    GalleyAction {
        gain_phase_complete: bool,
    },
    DeckAction {
        search_tokens_drawn: Vec<SearchToken>,
    },
}

#[derive(Clone, Serialize)]
struct Challenge {
    skill: Skill,
    amount: u32,
}

#[derive(Clone, Serialize)]
pub struct GameState {
    #[serde(serialize_with = "gamestate_phase", rename="phase")]
    phase_stack: Vec<GamePhase>,
    players: Vec<Player>,
    crew: Vec<Crew>,
    ability_deck: Deck<AbilityCard>,
    search_token_deck: Deck<SearchToken>,
    event_card_deck: Deck<EventCard>,
    room: ShipRoom,
    resources: Resources,
    pub prompt: Option<Value>,
}

fn gamestate_phase<S>(
    phase_stack: &Vec<GamePhase>,
    ser: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    phase_stack.last().unwrap().serialize(ser)
}

impl GameState {
    pub fn init_state() -> GameState {
        GameState {
            phase_stack: vec![GamePhase::ShipAction(None)],
            players: vec![Player::default()],
            crew: vec![
                Crew {
                    name: String::from("Sofi Odessa"),
                    fatigue: 1,
                    skills: HashMap::from([
                        (Skill::Savvy, 4),
                        (Skill::Craft, 2),
                    ]),
                },
                Crew {
                    name: String::from("Laurant Lapointe"),
                    fatigue: 0,
                    skills: HashMap::from([
                        (Skill::Savvy, 4),
                        (Skill::Craft, 2),
                    ]),
                },
            ],
            room: ShipRoom::None,
            resources: Resources {
                coins: 3,
                grain: 1,
                meat: 0,
            },
            prompt: None,
            ability_deck: Deck::new(vec![
                AbilityCard {
                    name: "card1".to_owned(),
                },
                AbilityCard {
                    name: "card2".to_owned(),
                },
                AbilityCard {
                    name: "card3".to_owned(),
                },
            ]),
            search_token_deck: Deck::new(
                (1..8).into_iter().map(|n| SearchToken(n)).collect(),
            ),
            event_card_deck: Deck::new(event_deck()),
        }
    }

    fn phase(&self) -> GamePhase {
        return self.phase_stack.last().unwrap().clone();
    }

    // TODO doesn't need to be a result
    fn set_phase(&self, new_phase: GamePhase) -> Update {
        let mut gs = self.clone();
        gs.phase_stack.pop();
        gs.phase_stack.push(new_phase);
        Ok(gs)
    }

    fn push_phase(&self, phase: GamePhase) -> GameState {
        let mut gs = self.clone();
        gs.phase_stack.push(phase);
        gs
    }

    fn pop_phase(&self) -> Update {
        let mut gs = self.clone();
        gs.phase_stack.pop();

        Ok(gs)
    }

    fn challenge(&self, challenge: Challenge) -> Update {
        Ok(self.clone())
            .map(|g| {
                g.push_phase(GamePhase::ChallengePhase(challenge))
            })
            .map(|g| g.prompt_str("resolveChallenge"))
    }

    // fn add_player(&self, player: Player) -> Update {
    //     let mut gs = self.clone();
    //     gs.players.push(player);
    //
    //     Ok(gs)
    // }

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

    fn apply_search_tokens(self, token: &SearchToken) -> Update {
        let mut gs = self.clone();
        gs.resources.meat += 1;
        Ok(gs)
    }

    fn draw_cards(self, player_ix: usize, amount: u32) -> Update {
        let mut gs = self.clone();

        if let Some(player) = gs.players.get_mut(player_ix) {
            for _ in 0..amount {
                if let Ok(card) = gs.ability_deck.draw() {
                    player.add_card(card);
                }

                // if let Some(card) = card {
                //     player.add_card(card);
                // }
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
                    gs.ability_deck.add_to_discard(&card);
                    Ok(gs)
                }
                Err(err) => Err(err),
            }
        }
    }

    fn reduce_fatigue(self, crew_ix: usize) -> Update {
        let mut gs = self.clone();
        if let Some(crew) = gs.crew.get_mut(crew_ix) {
            crew.reduce_fatigue()
        };

        Ok(gs)
    }

    fn clear_prompt(self, msg: &str) -> GameState {
        let gs = self.clone();
        match self.prompt {
            Some(Value::String(prompt)) if prompt == msg => {
                gs.prompt(&Value::Null)
            }
            _ => gs,
        }
    }

    fn prompt_str(self, msg: &str) -> GameState {
        let msg_obj = json!({
            "promptType": msg,
            "promptData": {}
        });
        self.prompt(&msg_obj)
    }

    fn prompt(self, msg: &Value) -> GameState {
        let mut gs = self.clone();
        gs.prompt = Some(msg.to_owned());
        gs
    }
}

#[derive(Serialize, Clone)]
struct Crew {
    name: String,
    fatigue: u32,
    skills: HashMap<Skill, u32>,
}

impl Crew {
    fn reduce_fatigue(&mut self) {
        if self.fatigue > 0 {
            self.fatigue -= 1;
        }
    }
}

#[derive(Serialize, Clone)]
struct Deck<T: Clone> {
    items: Vec<T>,
    discard: Vec<T>,
}

impl<T: Clone> Deck<T> {
    fn new(items: Vec<T>) -> Self {
        Deck {
            items,
            discard: Vec::new(),
        }
    }

    fn draw(&mut self) -> Result<T, String> {
        if self.items.len() == 0 {
            self.items.append(&mut self.discard);
            println!("none left");
        }

        self.items
            .pop()
            .ok_or("No items left in the deck".to_string())
    }

    fn add_to_discard(&mut self, item: &T) {
        self.discard.push(item.clone());
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
    meat: u32,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
enum ShipRoom {
    Galley,
    Bridge,
    Deck,
    None,
}

#[derive(Clone, Serialize, Hash, PartialEq, Eq)]
enum Skill {
    Savvy,
    Craft,
    Wits,
}

#[derive(Clone, Serialize, Copy, Default)]
struct SearchToken(u32);

type Update = Result<GameState, String>;
