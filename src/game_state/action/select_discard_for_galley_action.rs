use std::fmt::Display;

use serde::Deserialize;
use serde_json::Value;

use super::Action;
use crate::game_state::{
    game_phase::ShipActionSubphase, GamePhase, GameState, Update,
};

#[derive(Deserialize)]
pub struct SelectDiscardForGalleyAction {
    decline: bool,
    discard_ix: usize,
    crew_ix: usize,
    player_ix: usize,
}

fn validate(state: &GameState) -> Update {
    if let GamePhase::ShipAction(Some(
        ShipActionSubphase::GalleyAction,
    )) = &state.phase()
    {
        Ok(state.clone())
    } else {
        Err("Wrong phase".to_owned())
    }
}

impl Action for SelectDiscardForGalleyAction {
    fn execute(&self, state: &GameState) -> Update {
        let gs = Ok(state.clone())
            .and_then(|g| validate(&g))
            .and_then(|g| g.set_phase(GamePhase::EventPhase(None)))
            .map(|g| g.prompt(&Value::Null));

        if self.decline {
            gs
        } else {
            gs.and_then(|g| {
                g.discard_card(self.player_ix, self.discard_ix)
            })
            .and_then(|g| g.reduce_fatigue(self.crew_ix))
        }
    }
}

impl Display for SelectDiscardForGalleyAction {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "Select Discard For Galley Action\n{}\n{}\n{}",
            self.decline, self.crew_ix, self.player_ix
        )
    }
}


#[cfg(test)]
mod test {
    use crate::game_state::AbilityCard;

    use super::*;

    use GamePhase::ShipAction as Sa;
    use ShipActionSubphase as Sas;

    #[test]
    fn test_action() {
        let mut gs = GameState::init_state()
            .set_phase(Sa(Some(Sas::GalleyAction)))
            .unwrap();

        gs.players[0].add_card(AbilityCard::default());
        gs.crew[0].fatigue = 1;
        let action = SelectDiscardForGalleyAction {
            decline: false,
            discard_ix: 0,
            crew_ix: 0,
            player_ix: 0, 
        };

        let result = action.execute(&gs);
        assert!(result.is_ok());

        insta::with_settings!({sort_maps => true}, {
            insta::assert_json_snapshot!(result.unwrap());
        });
    }

    #[test]
    fn test_err_if_index_out_of_range()  {
        let gs = GameState::init_state()
            .set_phase(Sa(Some(Sas::GalleyAction)))
            .unwrap();
        
        assert!(gs.players[0].hand.len() < 1000);
        let action = SelectDiscardForGalleyAction {
            decline: false,
            discard_ix: 1000,
            crew_ix: 0,
            player_ix: 0, 
        };

        let result = action.execute(&gs);
        assert!(result.is_err());
    }
}
