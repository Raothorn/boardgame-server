use super::GameState;
use serde_json::Value::Object;
use serde_json::{from_str, Value};

pub mod end_turn;
pub mod take_ship_action;
pub mod select_discard_for_galley_action;

pub trait Action {
    fn execute(&self, state: &mut GameState) -> Option<String>;
    fn name(&self) -> &str;
}

pub fn get_action(action_msg_str: &str) -> Box<dyn Action> {
    let msg: Value = serde_json::from_str(action_msg_str).unwrap();

    if let Object(obj) = msg {
        let (atype, adata) = (obj.get("actionType"), obj.get("actionData"));

        if let (Some(serde_json::Value::String(atype)), Some(adata)) =
            (atype, adata)
        {
            return match atype.as_str() {
                "takeShipAction" => Box::new(
                    from_str::<take_ship_action::TakeShipAction>(
                        &adata.to_string(),
                    )
                    .unwrap(),
                ),
                "endTurnAction" => Box::new(
                    from_str::<end_turn::EndTurnAction>(&adata.to_string())
                        .unwrap(),
                ),
                "selectDiscardForGalleyAction" => Box::new(
                    from_str::<select_discard_for_galley_action::SelectDiscardForGalleyAction>(&adata.to_string())
                        .unwrap(),
                ),
                _ => Box::new(()),
            };
        }
    }
    Box::new(())
}

impl Action for () {
    fn execute(&self, _: &mut GameState) -> Option<String> {
        None
    }

    fn name(&self) -> &str {
        "no action"
    }
}

// #[macro_export]
// macro_rules! match_action {
//     ($action_path:path, $action_data:ident) => {
//         let action = from_str::<$action_path>(&$action_data)
//                         .ok();
//         None
//     };
// }
