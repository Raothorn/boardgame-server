use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Deck<T: Clone> {
    items: Vec<T>,
    discard: Vec<T>,
}

impl<T: Clone> Deck<T> {
    pub fn new(items: Vec<T>) -> Self {
        Deck {
            items,
            discard: Vec::new(),
        }
    }

    pub fn draw(self) -> Result<(Self, T), String> {
        let mut deck = self.clone();
        if deck.items.is_empty() {
            deck.items.append(&mut deck.discard);
            println!("none left");
        }

        deck.items
            .pop()
            .ok_or("No items left in the deck".to_string())
            .map(|item| (deck, item))
    }

    pub fn add_to_discard(&mut self, item: &T) {
        self.discard.push(item.clone());
    }
}
