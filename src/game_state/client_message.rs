use serde::Serialize;

use super::AbilityCard;

#[derive(Clone, Serialize)]
pub enum ClientMessage {
    GainCommandPoints { amount: u32 },
    DrewAbilityCard { card: AbilityCard }
}
