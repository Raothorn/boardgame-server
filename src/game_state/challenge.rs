use serde::Serialize;

use super::{skill::Skill, Update, GameState};

#[derive(Clone, Serialize)]
pub struct Challenge {
    pub skill: Skill,
    pub amount: u32,

    #[serde(skip_serializing)]
    pub if_fail: fn(&GameState) -> Update,
    #[serde(skip_serializing)]
    pub if_succeed: fn(&GameState) -> Update
}

impl Default for Challenge {
    fn default() -> Self {
        Self {
            skill: Skill::Craft,
            amount: Default::default(),
            if_fail: |gs| Ok(gs.clone()),
            if_succeed: |gs| Ok(gs.clone())
        }
    }
}
