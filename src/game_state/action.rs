use core::fmt;
use std::fmt::Display;

use super::{GameState, Update};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

mod accept_challenge_result_action;
mod accept_message_action;
mod choose_token_for_deck_action;
mod draw_for_deck_action;
mod handle_event_phase_action;
mod resolve_challenge_action;
mod select_discard_for_galley_action;
mod select_event_option_action;
mod take_ship_action;
mod select_main_action;
mod travel_action;
mod equip_ability_card_action;
mod select_crew_member_action;

#[typetag::serde(tag = "actionType", content = "actionData")]
pub trait Action: fmt::Display {
    fn execute(&self, state: &GameState) -> Update<GameState> {
        Ok(state.clone())
    }
}

pub fn get_action(action_msg_str: &str) -> Box<dyn Action>{
    let msg: Value = serde_json::from_str(action_msg_str).unwrap();

    let result = serde_json::from_value::<Box<dyn Action>>(msg);
    match result {
        Ok(action) => action,
        
        // We want to panic if an action isn't being parsed
        Err(err) => panic!("{}", err),
    }
}


// BASIC ACTIONS

#[derive(Deserialize, Serialize, Clone)]
struct NoAction;

#[typetag::serde(name = "noAction")]
impl Action for NoAction {
    fn execute(&self, gs: &GameState) -> Update<GameState> {
        Ok(gs.to_owned())
    }
}

impl Display for NoAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No action")
    }
}
