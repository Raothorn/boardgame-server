use serde::{Deserialize, Serialize, Serializer};
pub mod ability_card_deck;
pub mod action;
pub mod challenge;
pub mod client_message;
pub mod crew;
pub mod deck;
pub mod event_deck;
pub mod game_phase;
pub mod map;
pub mod player;
pub mod skill;

use self::{
    ability_card_deck::ability_card_deck,
    event_deck::event_deck,
    map::SerialMap,
    map::{GameMap, MapData},
};
use ability_card_deck::AbilityCard;
use challenge::Challenge;
use client_message::ClientMessage;
use crew::Crew;
use deck::Deck;
use event_deck::EventCard;
use game_phase::GamePhase;
use player::Player;
use skill::Skill;

#[derive(Clone, Serialize)]
pub struct GameState {
    #[serde(
        serialize_with = "serialize_gamestate_phase",
        rename = "phase"
    )]
    phase_stack: Vec<GamePhase>,

    players: Vec<Player>,
    crew: Vec<Crew>,
    #[serde(serialize_with = "serialize_map")]
    map: GameMap,

    room: ShipRoom,
    resources: Resources,
    message_queue: Vec<ClientMessage>,

    #[serde(skip_serializing)]
    ability_deck: Deck<AbilityCard>,
    #[serde(skip_serializing)]
    search_token_deck: Deck<SearchToken>,
    #[serde(skip_serializing)]
    event_card_deck: Deck<EventCard>,
}

// Serializers
fn serialize_gamestate_phase<S>(
    phase_stack: &[GamePhase],
    ser: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    phase_stack.last().unwrap().serialize(ser)
}

fn serialize_map<S>(map: &GameMap, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    SerialMap::from(map.clone()).serialize(ser)
}

// Impl
impl GameState {
    pub fn init_state() -> GameState {
        GameState {
            phase_stack: vec![GamePhase::ShipActionPhase(None)],
            players: vec![Player::default()],
            crew: vec![
                Crew::new("Rafael Vieira", 0, 1, 1, 0, 0),
                Crew::new("Audrie Williams", 0, 1, 0, 0, 1),
                Crew::new("Katsumi Aoshima", 1, 0, 0, 1, 1),
                Crew::new("Kannan Sharma", 1, 0, 1, 1, 0),
                Crew::new("Sofi Odessa", 1, 1, 1, 1, 1),
                Crew::new("Gregory Little", 1, 0, 1, 0, 0),
                Crew::new("Laurant Lapointe", 1, 1, 1, 0, 1),
                Crew::new("Marco Reyes", 0, 0, 1, 1, 0),
            ],
            map: GameMap {
                ship_area: 1,
                map_data: MapData::default(),
            },
            room: ShipRoom::None,
            resources: Resources::default(),
            ability_deck: Deck::new(ability_card_deck()),
            search_token_deck: Deck::new(
                (1..8).map(SearchToken).collect(),
            ),
            event_card_deck: Deck::new(event_deck()),
            message_queue: Vec::new(),
        }
    }

    fn phase(&self) -> GamePhase {
        return self.phase_stack.last().unwrap().clone();
    }

    // TODO doesn't need to be a result
    fn set_phase(&self, new_phase: GamePhase) -> Update<GameState> {
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

    fn pop_phase(&self) -> Update<GameState> {
        let mut gs = self.clone();
        gs.phase_stack.pop();

        Ok(gs)
    }

    fn challenge(&self, challenge: Challenge) -> Update<GameState> {
        Ok(self.clone()).map(|g| {
            g.push_phase(GamePhase::ChallengePhase {
                challenge,
                added: None,
            })
        })
    }

    fn give_command_tokens(
        self,
        player_ix: usize,
        amount: u32,
    ) -> Update<GameState> {
        let mut gs = self.clone();
        if let Some(player) = gs.players.get_mut(player_ix) {
            player.command_tokens += amount;
            Ok(gs).and_then(|g| {
                g.queue_message(ClientMessage::GainCommandPoints {
                    amount,
                })
            })
        } else {
            Err("Player does not exist".to_owned())
        }
    }

    fn apply_search_tokens(
        self,
        _token: &SearchToken,
    ) -> Update<GameState> {
        let mut gs = self.clone();
        gs.resources.meat += 1;
        Ok(gs)
    }

    fn draw_cards(
        self,
        player_ix: usize,
        amount: u32,
    ) -> Update<GameState> {
        let gamestate = (0..amount).into_iter().fold(
            Ok(self.clone()),
            |gs, _| {
                gs.and_then(|g| match g.ability_deck.clone().draw() {
                    Ok((deck, card)) => g
                        .update_player(player_ix, |p| {
                            p.add_card(card.clone())
                        })
                        .map(|g| GameState {
                            ability_deck: deck,
                            ..g
                        })
                        .and_then(|g| {
                            g.append_message(
                                ClientMessage::DrewAbilityCard {
                                    card: card.clone(),
                                },
                            )
                        }),
                    Err(err) => Err(err),
                })
            },
        );
        gamestate
    }

    fn append_message(self, msg: ClientMessage) -> Update<GameState> {
        let mut gs = self.clone();
        gs.message_queue.push(msg);
        Ok(gs)
    }

    fn set_room(self, room: &ShipRoom) -> Update<GameState> {
        let mut gs = self.clone();
        gs.room = room.clone();
        Ok(gs)
    }

    fn discard_card(
        self,
        player_ix: usize,
        card_ix: usize,
    ) -> Update<GameState> {
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

    fn queue_message(self, msg: ClientMessage) -> Update<GameState> {
        let mut gs = self.clone();
        gs.message_queue.push(msg);
        Ok(gs)
    }

    fn dequeue_message(self) -> Update<GameState> {
        let mut gs = self.clone();
        gs.message_queue.pop();

        // We don't want it to fail even if there is no message
        Ok(gs)
    }

    fn update_player(
        self,
        player_ix: usize,
        player_update: impl Fn(Player) -> Update<Player>,
    ) -> Update<GameState> {
        if player_ix > self.players.len() {
            Err("invalid player ix".to_owned())
        } else {
            player_update(self.players[player_ix].clone()).map(
                |player_| {
                    let mut players = self.players.clone();
                    players[player_ix] = player_;
                    GameState { players, ..self }
                },
            )
        }
    }

    fn update_crew(
        self,
        crew_ix: usize,
        crew_update: impl Fn(Crew) -> Update<Crew>,
    ) -> Update<GameState> {
        if crew_ix > self.crew.len() {
            Err("invalid crew ix".to_owned())
        } else {
            crew_update(self.crew[crew_ix].clone()).map(|crew| {
                let mut all_crew = self.crew.clone();
                all_crew[crew_ix] = crew;
                GameState {
                    crew: all_crew,
                    ..self
                }
            })
        }
    }

    fn move_ship(self, to_area: u32) -> Update<GameState> {
        let mut gs = self.clone();
        gs.map.ship_area = to_area;
        Ok(gs)
    }

    fn equip_ability_card(
        self,
        hand_ix: usize,
        crew_ix: usize,
    ) -> Update<GameState> {
        let mut gs = self.clone();

        // TODO validate
        let card = gs.players[0].hand.remove(hand_ix).clone();

        Ok(gs).and_then(|g| {
            g.update_crew(crew_ix, |c| {
                c.equip_ability_card(card.clone())
            })
        })
    }
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
    Quarters,
    Sickbay,
    None,
}

#[derive(Clone, Serialize, Copy, Default, Debug)]
pub struct SearchToken(u32);

type Update<T> = Result<T, String>;
