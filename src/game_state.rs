use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};
use serde_json::{json, Value};

pub mod action;
pub mod challenge;
pub mod crew;
pub mod deck;
pub mod event_deck;
pub mod game_phase;
pub mod player;
pub mod skill;

use self::event_deck::event_deck;
use challenge::Challenge;
use crew::Crew;
use deck::Deck;
use event_deck::EventCard;
use game_phase::GamePhase;
use player::Player;
use skill::Skill;

#[derive(Clone, Serialize)]
pub struct GameState {
    #[serde(serialize_with = "gamestate_phase", rename = "phase")]
    phase_stack: Vec<GamePhase>,
    players: Vec<Player>,
    crew: Vec<Crew>,
    ability_deck: Deck<AbilityCard>,
    search_token_deck: Deck<SearchToken>,
    event_card_deck: Deck<EventCard>,
    room: ShipRoom,
    resources: Resources,
    pub prompt: Option<String>,
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
                Crew::new("Sofi Odessa", 4, 2),
                Crew::new("Laurant Lapointe", 4, 2),
            ],
            room: ShipRoom::None,
            resources: Resources::default(),
            prompt: None,
            ability_deck: Deck::new(
                (1..10)
                    .into_iter()
                    .map(|n| AbilityCard {
                        name: n.to_string(),
                    })
                    .collect(),
            ),
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

    fn apply_search_tokens(self, _token: &SearchToken) -> Update {
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

    fn set_room(self, room: &ShipRoom) -> Update {
        let mut gs = self.clone();
        gs.room = room.clone();
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
        println!("{:?}  {}", self.prompt, msg);
        match self.prompt {
            Some(prompt) if prompt == msg => {
                gs.prompt = None;
            }
            _ => ()
        }

        gs
    }

    fn prompt_str(self, msg: &str) -> GameState {
        let mut gs = self.clone();
        gs.prompt = Some(msg.to_owned());
        gs
    }

    // fn prompt(self, msg: &Value) -> GameState {
    //     let mut gs = self.clone();
    //     gs.prompt = Some(msg.to_owned());
    //     gs
    // }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Default)]
pub struct AbilityCard {
    name: String,
}

#[derive(Clone, Serialize, Default)]
pub struct Resources {
    coins: u32,
    grain: u32,
    meat: u32,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum ShipRoom {
    Galley,
    Bridge,
    Deck,
    None,
}

#[derive(Clone, Serialize, Copy, Default)]
pub struct SearchToken(u32);

type Update = Result<GameState, String>;
