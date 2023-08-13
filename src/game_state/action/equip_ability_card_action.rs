use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::json;

use super::Action;
use crate::game_state::{game_phase::GamePhase, GameState, Update};

// After crew is chosen
#[derive(Deserialize, Serialize)]
pub struct EquipAbilityCardAction {
    hand_ix: usize,
    player_ix: usize,
}

#[typetag::serde(name = "equipAbilityCardAction")]
impl Action for EquipAbilityCardAction {
    fn execute(&self, state: &GameState) -> Update<GameState> {
        if let GamePhase::SelectCrewMemberPhase{
            crew_ix: Some(crew_ix),title,callback
        } =
            state.phase()
        {
            state.clone().equip_ability_card(self.hand_ix, crew_ix)
        } else {
            Err("wrong phase".to_owned())
        }
    }
}

impl Display for EquipAbilityCardAction {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "Equip Ability Card")
    }
}

// When ability card is selected
#[derive(Clone, Serialize, Deserialize)]
pub struct SelectAbilityCardToEquipAction {
    hand_ix: usize,
    player_ix: usize,
}

#[typetag::serde(name = "selectAbilityCardToEquipAction")]
impl Action for SelectAbilityCardToEquipAction {
    fn execute(&self, state: &GameState) -> Update<GameState> {
        Ok(state.clone()).and_then(|g| {
            let action = EquipAbilityCardAction {
                hand_ix: self.hand_ix,
                player_ix: self.player_ix,
            };
            let action_ser = json! ({
                "actionType": "equipAbilityCardAction",
                "actionData": action,
            })
            .to_string();

            g.push_phase(GamePhase::SelectCrewMemberPhase {
                crew_ix: None,
                title: "Equip ability card:".to_owned(),
                callback: action_ser,
            })
        })
    }
}

impl Display for SelectAbilityCardToEquipAction {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "Select ability card to equip")
    }
}