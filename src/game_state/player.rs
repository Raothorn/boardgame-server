use serde::Serialize;

use super::{AbilityCard, Update};

#[derive(Default, Serialize, Clone)]
pub struct Player {
    pub command_tokens: u32,
    pub hand: Vec<AbilityCard>,
}

impl Player {
    pub fn add_card(&self, card: AbilityCard) -> Update<Self> {
        let mut player = self.clone();
        player.hand.push(card);
        Ok(player)
    }

    pub fn discard_card(
        &self,
        card_ix: usize,
    ) -> Result<(Player, AbilityCard), String> {
        let mut player = self.clone();
        if player.hand.len() <= card_ix {
            Err("this card does not exist in the players hand"
                .to_owned())
        } else {
            let card = player.hand.remove(card_ix);
            Ok((player, card))
        }
    }
}
