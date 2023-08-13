use serde::Serialize;

use super::SerialCard;

#[derive(Clone, Serialize, Debug)]
pub enum ClientMessage {
    GainCommandPoints { amount: u32 },
    DrewAbilityCard { card: SerialCard },
    DrewFate { result: u32 },
    ModifierTriggered { text: String, card: SerialCard }
    // TODO recieved quest message
}
