use core::fmt;
use std::fmt::Display;

use super::{GameState, Update};
use serde::Deserialize;
use serde_json::Value::Object;
use serde_json::{from_str, Value};

mod choose_token_for_deck_action;
mod draw_for_deck_action;
mod handle_event_phase_action;
mod resolve_challenge_action;
mod select_discard_for_galley_action;
mod select_event_option_action;
mod take_ship_action;

pub trait Action: fmt::Display {
    fn execute(&self, state: &GameState) -> Update {
        Ok(state.clone())
    }
}

pub fn get_action(action_msg_str: &str) -> Box<dyn Action> {
    let msg: Value = serde_json::from_str(action_msg_str).unwrap();

    if let Object(obj) = msg {
        let (atype, adata) =
            (obj.get("actionType"), obj.get("actionData"));

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
                "selectDiscardForGalleyAction" => Box::new(
                    from_str::<select_discard_for_galley_action::SelectDiscardForGalleyAction>(&adata.to_string())
                        .unwrap(),
                ),
                "drawForDeckAction" => Box::new(
                    from_str::<draw_for_deck_action::DrawForDeckAction>(&adata.to_string())
                        .unwrap(),
                ),
                "chooseTokenForDeckAction" => Box::new(
                    from_str::<choose_token_for_deck_action::ChooseTokenForDeckAction>(&adata.to_string())
                        .unwrap(),
                ),
                "handleEventPhaseAction" => Box::new(
                    from_str::<handle_event_phase_action::HandleEventPhaseAction>(&adata.to_string())
                        .unwrap(),
                ),
                "selectEventOptionAction" => Box::new(
                    from_str::<select_event_option_action::SelectEventOptionAction>(&adata.to_string())
                        .unwrap(),
                ),
                "resolveChallengeAction" => Box::new(
                    from_str::<resolve_challenge_action::ResolveChallengeAction>(&adata.to_string())
                        .unwrap(),
                ),
                _ => Box::new(NoAction)
            };
        }
    }
    Box::new(NoAction)
}

// BASIC ACTIONS

struct NoAction;

impl Action for NoAction {
    fn execute(&self, gs: &GameState) -> Update {
        Ok(gs.to_owned())
    }
}

impl Display for NoAction {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}
