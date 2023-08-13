use serde::Serialize;

#[derive(Clone, Serialize, Default, Debug)]
pub struct AbilityCard {
    pub name: String,
    pub deck_ix: u32,

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

    let mut other_deck = vec![card1, card2, card3];
    let mut deck: Vec<_> = (0..100)
        .into_iter()
        .map(|_| AbilityCard::new("SomeCard", 1))
        .collect();

    deck.append(&mut other_deck);
    deck
}

type Modifier = ();
