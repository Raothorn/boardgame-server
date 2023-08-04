use serde::Serialize;

use super::skill::Skill;

#[derive(Clone, Serialize)]
pub struct Challenge {
    pub skill: Skill,
    pub amount: u32,
}

impl Default for Challenge {
    fn default() -> Self {
        Self {
            skill: Skill::Craft,
            amount: Default::default(),
        }
    }
}
