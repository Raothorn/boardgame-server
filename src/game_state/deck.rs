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

    pub fn draw(&mut self) -> Result<T, String> {
        if self.items.is_empty() {
            self.items.append(&mut self.discard);
            println!("none left");
        }

        self.items
            .pop()
            .ok_or("No items left in the deck".to_string())
    }

    pub fn add_to_discard(&mut self, item: &T) {
        self.discard.push(item.clone());
    }
}
