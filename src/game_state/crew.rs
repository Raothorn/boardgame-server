use serde::Serialize;
use serde_with::DisplayFromStr;

use super::skill::Skill;
use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(Serialize, Clone)]
pub struct Crew {
    pub name: String,
    pub fatigue: u8,
    pub damage: u8,
    #[serde_as(as = "HashMap<DisplayFromStr,_>")]
    pub skills: HashMap<Skill, u32>,
}

impl Crew {
    pub fn new(
        name: &str,
        savvy: u32,
        craft: u32,
        strength: u32,
        wits: u32,
        perception: u32,
    ) -> Self {
        let mut skills = HashMap::new();

        skills.insert(Skill::Craft, craft);
        skills.insert(Skill::Savvy, savvy);
        skills.insert(Skill::Strength, strength);
        skills.insert(Skill::Perception, perception);
        skills.insert(Skill::Wits, wits);

        Crew {
            name: name.to_owned(),
            fatigue: 0,
            damage: 0,
            skills,
        }
    }

    pub fn change_fatigue(&mut self, amount: i32) {
        let mut fatigue = i32::from(self.fatigue) + amount;
        // Clamp fatigue to [0, 2]
        if fatigue > 2 {
            fatigue = 2;
        } else if fatigue < 0 {
            fatigue = 0;
        }

        self.fatigue = u8::try_from(fatigue).unwrap_or(0);
    }
}
