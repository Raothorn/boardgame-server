use super::GameState;

pub mod take_ship_action;
pub mod select_discard_for_galley_action;

trait Action {
    fn execute(&self, state: &mut GameState);
    fn invalid(&self, state: &GameState) -> Option<String>;
}