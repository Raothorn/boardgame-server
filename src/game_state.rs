use serde::{Deserialize, Serialize, Serializer};
use std::{collections::HashMap, ops::RangeInclusive};

mod ability_card_deck;
pub mod action;
mod challenge;
pub mod client_message;
mod crew;
mod deck;
pub mod effect;
mod event_deck;
mod game_phase;
mod map;
mod market;
mod player;
mod quest;
mod skill;
mod storybook;

use crate::game_state::modifier::ModifierTrigger;

use self::{
    ability_card_deck::ability_card_deck,
    client_message::ClientMessage,
    effect::{resolve_effects, Effect},
    map::SerialMap,
    map::{GameMap, MapData},
    market::MarketCard,
    modifier::Modifier,
    storybook::Storybook,
};
use ability_card_deck::AbilityCard;
use challenge::Challenge;
use crew::Crew;
use deck::Deck;
use event_deck::EventCard;
use game_phase::GamePhase;
use player::Player;

#[derive(Clone, Serialize)]
pub struct GameState {
    #[serde(serialize_with = "serialize_gamestate_phase", rename = "phase")]
    phase_stack: Vec<GamePhase>,

    players: Vec<Player>,
    crew: Vec<Crew>,
    #[serde(serialize_with = "serialize_map")]
    map: GameMap,

    room: ShipRoom,
    resources: HashMap<Resource, u32>,
    message_queue: Vec<ClientMessage>,

    #[serde(serialize_with = "serialize_quests", rename = "keywords")]
    quests: HashMap<u32, String>,

    // TODO probably refactor decks into it's own struct
    #[serde(skip_serializing)]
    ability_deck: Deck<AbilityCard>,
    #[serde(skip_serializing)]
    search_token_deck: Deck<SearchToken>,
    #[serde(skip_serializing)]
    event_card_deck: Deck<EventCard>,
    #[serde(skip_serializing)]
    market_deck: Deck<MarketCard>,

    modifiers: Vec<Modifier>,
}

// Serializers
fn serialize_gamestate_phase<S>(phase_stack: &[GamePhase], ser: S) -> Result<S::Ok, S::Error>
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

fn serialize_quests<S>(quests: &HashMap<u32, String>, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    quests
        .clone()
        .into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<_>>()
        .serialize(ser)
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
                storybook: Storybook::default(),
            },
            room: ShipRoom::None,
            resources: [(Resource::Coin, 0), (Resource::Meat, 0)]
                .into_iter()
                .collect(),
            quests: HashMap::new(),
            ability_deck: Deck::new(ability_card_deck()),
            search_token_deck: Deck::new((1..8).map(SearchToken).collect()),
            event_card_deck: Deck::default(),
            market_deck: Deck::default(),
            modifiers: Vec::new(),
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

    fn push_phase(&self, phase: GamePhase) -> Update<GameState> {
        let mut gs = self.clone();
        gs.phase_stack.push(phase);
        Ok(gs)
    }

    fn pop_phase(&self) -> Update<GameState> {
        let mut gs = self.clone();

        match gs.phase_stack.pop() {
            Some(_) => Ok(gs),
            None => Err("No phase to pop".to_owned()),
        }
    }

    fn challenge(&self, challenge: Challenge) -> Update<GameState> {
        Ok(self.clone()).and_then(|g| {
            g.push_phase(GamePhase::ChallengePhase {
                challenge,
                skill: None,
            })
        })
    }

    fn gain_resource(self, resource: Resource, amount: i32) -> Update<Self> {
        let mut gs = self.clone();
        let prev_amount: i32 = gs.resources[&resource].try_into().unwrap();
        if prev_amount + amount >= 0 {
            gs.resources
                .insert(resource, (prev_amount + amount).try_into().unwrap());
        } else {
            gs.resources.insert(resource, 0);
        }

        Ok(gs)
    }

    fn add_quest(self, index: u32) -> Update<Self> {
        let mut gs = self.clone();
        gs.quests.insert(index, quest::get_quest(index));
        Ok(gs)
    }

    fn give_command_tokens(self, player_ix: usize, amount: u32) -> Update<GameState> {
        let mut gs = self.clone();
        if let Some(player) = gs.players.get_mut(player_ix) {
            player.command_tokens += amount;
            Ok(gs).and_then(|g| g.queue_message(&ClientMessage::GainCommandPoints { amount }))
        } else {
            Err("Player does not exist".to_owned())
        }
    }

    fn keywords(&self) -> Vec<String> {
        self.quests.clone().into_iter().map(|(_, v)| v).collect()
    }

    fn apply_search_tokens(self, _token: &SearchToken) -> Update<GameState> {
        println!("Gaining meat!");
        self.gain_resource(Resource::Meat, 1)
    }

    fn draw_cards(self, player_ix: usize, amount: u32) -> Update<GameState> {
        let gamestate = (0..amount).into_iter().fold(Ok(self.clone()), |gs, _| {
            gs.and_then(|g| match g.ability_deck.clone().draw() {
                Ok((deck, card)) => {
                    let serial_card = SerialCard {
                        label: card.name.clone(),
                        deck: "ability_card_deck".to_owned(),
                        index: card.deck_ix,
                    };
                    let message = ClientMessage::DrewAbilityCard { card: serial_card };

                    g.update_player(player_ix, |p| p.add_card(card.clone()))
                        .map(|g| GameState {
                            ability_deck: deck,
                            ..g
                        })
                        .and_then(|g| g.queue_message(&message))
                }
                Err(err) => Err(err),
            })
        });
        gamestate
    }

    // Filt is the ones we want to *remove*
    fn filter_out_modifiers(self, filt: fn(&Modifier) -> bool) -> Update<Self> {
        let mut gs = self.clone();

        gs.modifiers = gs.modifiers.into_iter().filter(|m| !filt(m)).collect();
        Ok(gs)
    }

    fn add_modifier(self, modifier: &Modifier) -> Update<Self> {
        let mut gs = self.clone();
        gs.modifiers.push(modifier.clone());
        Ok(gs)
    }

    fn queue_message(self, msg: &ClientMessage) -> Update<GameState> {
        let mut gs = self.clone();
        gs.message_queue.push(msg.clone());
        Ok(gs)
    }

    fn set_room(self, room: &ShipRoom) -> Update<GameState> {
        let mut gs = self.clone();
        gs.room = room.clone();
        Ok(gs)
    }

    fn draw_fate(self) -> Update<(Self, u32)> {
        // TODO random numbers...
        let result = 2;

        let gs = self.clone();
        let modifiers = gs.modifiers.iter().filter(|m| match &m.trigger {
            ModifierTrigger::DrawFate(range) => range.contains(&result),
            _ => false,
        });

        let modifier_effects = modifiers.clone().map(|m| m.effect.clone()).collect();

        let modifier_messages = modifiers.clone().map(|m| m.trigger_text.clone());

        println!("{:?}", modifier_messages);

        (Ok(gs.clone()))
            .and_then(|g| {
                modifier_messages.fold(Ok(g), |g, m| g.and_then(|g| g.queue_message(&m)))
            })?
            .queue_message(&ClientMessage::DrewFate { result })
            .and_then(|g| resolve_effects(&g, modifier_effects))
            .map(|g| (g, result))
    }

    fn discard_card(self, player_ix: usize, card_ix: usize) -> Update<GameState> {
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
            player_update(self.players[player_ix].clone()).map(|player_| {
                let mut players = self.players.clone();
                players[player_ix] = player_;
                GameState { players, ..self }
            })
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

    fn equip_ability_card(self, hand_ix: usize, crew_ix: usize) -> Update<GameState> {
        let mut gs = self.clone();

        // TODO validate
        let card = gs.players[0].hand.remove(hand_ix).clone();

        Ok(gs).and_then(|g| g.update_crew(crew_ix, |c| c.equip_ability_card(card.clone())))
    }
}

#[derive(Clone, Copy, Serialize, Debug, PartialEq, Eq, Hash)]
pub enum Resource {
    Coin,
    Meat,
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

mod modifier {
    // TODO refactor to own file
    use super::*;

    #[derive(Clone, Debug, Serialize)]
    pub struct Modifier {
        pub lifetime: ModifierLifetime,
        pub trigger: ModifierTrigger,
        pub effect: Effect,

        pub trigger_text: ClientMessage,

        #[serde(skip_serializing)]
        pub condition: fn(&GameState) -> bool,
    }

    #[derive(Clone, Debug, Serialize)]
    pub enum ModifierTrigger {
        DrawFate(RangeInclusive<u32>),
    }

    #[derive(Clone, Debug, Serialize)]
    pub enum ModifierLifetime {
        ThisTurn,
    }

    #[derive(Clone, Serialize, Debug)]
    pub enum ModifierScope {}
}

#[derive(Serialize, Clone, Debug)]
pub struct SerialCard {
    label: String,
    deck: String,
    index: u32,
}

type Update<T> = Result<T, String>;
