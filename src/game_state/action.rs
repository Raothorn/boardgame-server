use super::GameState;
use serde_json::Value;

pub mod take_ship_action;
// pub mod select_discard_for_galley_action;

pub trait Action {
    fn execute(&self, state: &mut GameState) -> Option<String>;
    fn name(&self) -> &str;
}

pub fn get_action(action_msg_str: &str) -> Box<dyn Action> {
    let msg: Value = serde_json::from_str(action_msg_str).unwrap();

    let action_type = msg["actionType"].as_str().unwrap();
    let action_data = msg["actionData"].to_string();

    use crate::match_action;
    match action_type {
        "takeShipAction" => match_action!(take_ship_action::TakeShipAction, action_data),

        _ => Box::new(()),
    }
}

impl Action for () {
    fn execute(&self, _: &mut GameState) -> Option<String> {
        None
    }

    fn name(&self) -> &str {
        "no action"
    }
}

#[macro_export]
macro_rules! match_action {
    ($action_path:path, $action_data:ident) => {
        Box::new(serde_json::from_str::<$action_path>(&$action_data).unwrap())
    };
}
