use serde::{Serialize, Deserialize};
use serde_with::DisplayFromStr;

use crate::utils;

use super::{ability_card_deck::AbilityCard, skill::Skill, Update};
use std::{collections::{HashMap, HashSet}, fmt::Display};

#[serde_with::serde_as]
#[derive(Serialize, Clone)]
pub struct Crew {
    pub name: String,
    pub fatigue: u32,
    pub damage: u32,
    #[serde_as(as = "HashMap<DisplayFromStr,_>")]
    pub skills: HashMap<Skill, u32>,
    pub status: HashSet<Status>,
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
            status: HashSet::new(),
            equipped_ability_cards: Vec::new(),
        }
    }

    pub fn change_fatigue(self, amount: i32) -> Update<Self> {
        let mut crew = self.clone();

        crew.fatigue =
            utils::change_and_clamp(self.fatigue, amount, (0, 2));
        Ok(crew)
    }

    pub fn change_damage(self, amount: i32) -> Update<Self> {
        let mut crew = self.clone();

        crew.damage = utils::change_and_clamp(
            self.damage,
            amount,
            // TODO change 8 to the max damage of the character
            (0, 8),
        );
        Ok(crew)
    }

    pub fn equip_ability_card(
        self,
        card: AbilityCard,
    ) -> Update<Self> {
        let mut crew = self.clone();
        if crew.equipped_ability_cards.len() >= 2 {
            Err("Ability card limit".to_owned())
        } else {
            crew.equipped_ability_cards.push(card);
            Ok(crew)
        }
    }

    pub fn add_status(self, status: Status) -> Update<Self>{
        if self.status.contains(&status) {
            Err("You can't have more than one of that status".to_owned())
        } else {
            let mut crew = self.clone();
            crew.status.insert(status);
            Ok(crew)
        }
    }
} 

#[derive(Clone, Serialize, PartialEq, Eq, Hash, Debug, Deserialize, Copy)]
pub enum Status {
    Venom, Frightened, Weakend, Madness, LowMorale 
}


