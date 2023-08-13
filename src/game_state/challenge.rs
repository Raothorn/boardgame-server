use serde::Serialize;

use super::{skill::Skill, effect::Effect};

#[derive(Clone, Serialize, Debug)]
pub struct Challenge {
    pub skill: Skill,
    pub amount: u32,

    pub label: &'static str,

    #[serde(skip_serializing)]
    pub if_fail: Vec<Effect>,
    #[serde(skip_serializing)]
    pub if_succeed: Vec<Effect>
}

impl Default for Challenge {
    fn default() -> Self {
        Self {
            skill: Skill::Craft,
            amount: Default::default(),
            label: "",
            //TODO
            if_fail: vec![],
            if_succeed: vec![]
        }
    }
}
