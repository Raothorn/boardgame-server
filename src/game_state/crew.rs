use serde::Serialize;
use serde_with::DisplayFromStr;

use super::skill::Skill;
use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(Serialize, Clone)]
pub struct Crew {
    pub name: String,
    pub fatigue: u32,
    #[serde_as(as = "HashMap<DisplayFromStr,_>")]
    pub skills: HashMap<Skill, u32>,
}

impl Crew {
    pub fn new(name: &str, savvy: u32, craft: u32) -> Self {
        let mut skills = HashMap::new();
        skills.insert(Skill::Savvy, savvy);
        skills.insert(Skill::Craft, craft);

        Crew { name: name.to_owned(), fatigue:0, skills }
    }

    pub fn reduce_fatigue(&mut self) {
        if self.fatigue > 0 {
            self.fatigue -= 1;
        }
    }
}
