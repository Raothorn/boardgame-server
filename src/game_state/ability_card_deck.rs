use serde::Serialize;

#[derive(Clone, Serialize, Default)]
pub struct AbilityCard {
    name: String,
    deck_ix: u32,

    modifiers: Vec<Modifier>,
}

impl AbilityCard {
    fn new(name: &str, deck_ix: u32) -> Self {
        AbilityCard {
            name: name.to_owned(),
            deck_ix,
            modifiers: Vec::new(),
        }
    }
}

pub fn ability_card_deck() -> Vec<AbilityCard> {
    let card1 = AbilityCard::new("Triage", 1);
    let card2 = AbilityCard::new("Focused Mind", 2);
    let card3 = AbilityCard::new("Counsel", 3);

    vec![card1, card2, card3]
}

type Modifier = ();
