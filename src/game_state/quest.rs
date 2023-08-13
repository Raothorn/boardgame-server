use std::collections::HashMap;


pub fn get_quest(index: u32) -> String {
    let mut quests = HashMap::new();
    quests.insert(155, "PINK".to_owned());
    quests[&index].clone()
}
