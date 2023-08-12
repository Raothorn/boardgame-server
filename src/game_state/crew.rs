use serde::Serialize;
use serde_with::DisplayFromStr;

use super::{ability_card_deck::AbilityCard, skill::Skill};
use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(Serialize, Clone)]
pub struct Crew {
    pub name: String,
    pub fatigue: u8,
    pub damage: u8,
    #[serde_as(as = "HashMap<DisplayFromStr,_>")]
    pub skills: HashMap<Skill, u32>,

    pub equipped_ability_cards: Vec<AbilityCard>,
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
            equipped_ability_cards: Vec::new(),
        }
    }

    pub fn change_fatigue(self, amount: i32) -> Result<Self, String> {
        let mut crew = self.clone();
        let mut fatigue = i32::from(self.fatigue) + amount;
        // Clamp fatigue to [0, 2]
        if fatigue > 2 {
            fatigue = 2;
        } else if fatigue < 0 {
            fatigue = 0;
        }

        crew.fatigue = u8::try_from(fatigue).unwrap_or(0);
        Ok(crew)
    }

    pub fn equip_ability_card(
        self,
        card: AbilityCard,
    ) -> Result<Self, String> {
        let mut crew = self.clone();
        if crew.equipped_ability_cards.len() >= 2 {
            Err("Ability card limit".to_owned())
        } else {
            crew.equipped_ability_cards.push(card);
            Ok(crew)
        }
    }
}
