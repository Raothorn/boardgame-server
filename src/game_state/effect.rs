use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::json;

use super::{
    action::Action,
    challenge::Challenge,
    crew::Status,
    game_phase::{GamePhase, MainActionSubphase},
    GameState, Resource, Update,
};

#[derive(Clone, Serialize, Debug)]
pub enum Effect {
    TryChallenge(Challenge),
    GainResource(Resource, i32),
    TakeHealthDamage(u32),
    TakeStatus(Status),
    TurnToStory(String),
    ReturnToShip,
}

impl Effect {
    pub fn resolve(&self, state: GameState) -> Update<GameState> {
        match self {
            Effect::TurnToStory(story_ix) => {
                state.map.storybook.story(story_ix).clone().and_then(
                    |story| {
                        state.set_phase(GamePhase::ExplorePhase(
                            story.clone(),
                        ))
                    },
                )
            }

            Effect::ReturnToShip => state.pop_phase().and_then(|g| {
                if let GamePhase::MainActionPhase(
                    Some(MainActionSubphase::Explore),
                    action_ct,
                ) = g.phase()
                {
                    let phase =
                        GamePhase::MainActionPhase(None, action_ct);

                    g.set_phase(phase)
                } else {
                    Err("Wrong phase".to_owned())
                }
            }),

            Effect::GainResource(resource, amount) => {
                state.gain_resource(*resource, *amount)
            }

            Effect::TryChallenge(challenge) => {
                state.challenge(challenge.clone())
            }

            Effect::TakeStatus(status) => {
                let action = TakeStatusAction { status: *status };
                let action_ser = json! ({
                    "actionType": "takeStatusAction",
                    "actionData": action
                })
                .to_string();
                let display = format!("Give {:?} to", status);

                state.push_phase(GamePhase::SelectCrewMemberPhase {
                    crew_ix: None,
                    title: display,
                    callback: action_ser,
                })
            }

            _ => state.push_phase(GamePhase::ResolveEffectPhase(
                self.clone(),
            )),
        }
    }
}

pub fn resolve_effects(
    state: &GameState,
    effects: Vec<Effect>,
) -> Update<GameState> {
    effects.into_iter().fold(Ok(state.clone()), |g, eff| {
        g.and_then(|g| eff.resolve(g))
    })
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TakeStatusAction {
    status: Status,
}

#[typetag::serde(name = "takeStatusAction")]
impl Action for TakeStatusAction {
    fn execute(&self, state: &GameState) -> Update<GameState> {
        // TODO add string to phase
        if let GamePhase::SelectCrewMemberPhase{crew_ix: Some(crew_ix), title: _, callback: _} =
            state.phase()
        {
            state
                .clone()
                .update_crew(crew_ix, |c| c.add_status(self.status))
        } else {
            Err("wrong phase".to_owned())
        }
    }
}

impl Display for TakeStatusAction {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "Take Status")
    }
}
