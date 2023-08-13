use serde::{Deserialize, Serialize};

use super::deck::Deck;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MarketCard {
    card: AdventureCard,
    cost: u32,
    deck_ix: usize,
}

pub type AdventureCard = ();

impl Default for Deck<MarketCard> {
    fn default() -> Self {
        let items = vec![
            MarketCard { card: (), cost: 6, deck_ix: 0 },
            MarketCard { card: (), cost: 1, deck_ix: 1 },
            MarketCard { card: (), cost: 11, deck_ix: 2 },
            MarketCard { card: (), cost: 1, deck_ix: 3 },
            MarketCard { card: (), cost: 5, deck_ix: 4 },
            MarketCard { card: (), cost: 3, deck_ix: 5 },
        ];

        Deck::new(items)
    }
}
