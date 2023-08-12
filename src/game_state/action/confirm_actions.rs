use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::game_state::{
    effect::{Effect, resolve_effects}, game_phase::GamePhase, GameState, Update,
};

use super::Action;

#[derive(Deserialize, Serialize)]
pub struct ConfirmHealthDistributionAction {
    crew_damage: Vec<u32>,
    player_ix: usize,
}

#[typetag::serde(name = "confirmHealthDistributionAction")]
impl Action for ConfirmHealthDistributionAction {
    fn execute(&self, state: &GameState) -> Update<GameState> {
        if let GamePhase::ResolveEffectPhase(
            Effect::TakeHealthDamage(damage),
        ) = state.phase()
        {
            let total: u32 = self.crew_damage.iter().sum();
            if total == damage {
                (0..8)
                    .into_iter()
                    .fold(Ok(state.clone()), |g, ix| {
                        let damage =
                            self.crew_damage[ix].try_into().unwrap();
                        g.and_then(|g| {
                            g.update_crew(ix, |c| {
                                c.change_damage(damage)
                            })
                        })
                    })
                    .and_then(|g| g.pop_phase())
            } else {
                Err("You must allocate all damage".to_owned())
            }
        } else {
            Err("Wrong phase".to_owned())
        }
    }
}

impl Display for ConfirmHealthDistributionAction {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "confirm health distribution")
    }
}

// Accept challenge result
#[derive(Deserialize, Serialize)]
pub struct AcceptChallengeResultAction {
    #[allow(dead_code)]
    player_ix: usize,
}

#[typetag::serde(name = "acceptChallengeResultAction")]
impl Action for AcceptChallengeResultAction {
    fn execute(
        &self,
        state: &crate::game_state::GameState,
    ) -> crate::game_state::Update<GameState> {
        if let GamePhase::ChallengePhase {
            challenge,
            skill: Some(skill),
        } = state.phase()
        {
            Ok(state.clone())
                // Resolve challenge effects
                .and_then(|g| g.pop_phase())
                .and_then(|gs| {
                    let effects = if skill >= challenge.amount {
                        challenge.if_succeed
                    } else {
                        challenge.if_fail
                    };
                    resolve_effects(&gs, effects)
                })
        } else {
            Err("Wrong phase".to_owned())
        }
    }
}

impl Display for AcceptChallengeResultAction {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "Accept Challenge Result")
    }
}

// Accept message 
#[derive(Deserialize, Serialize)]
pub struct AcceptMessageAction {
    #[allow(dead_code)]
    player_ix: usize,
}

#[typetag::serde(name="acceptMessageAction")]
impl Action for AcceptMessageAction {
    fn execute(
        &self,
        state: &GameState,
    ) -> Update<GameState> {
        Ok(state.clone()).and_then(|g| g.dequeue_message())
    }
}

impl Display for AcceptMessageAction {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "Accept Message")
    }
}
