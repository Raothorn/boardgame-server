use serde::Serialize;
use std::collections::HashMap;

use super::{game_phase::GamePhase, GameState, Update, effect::Effect, Resource, crew::Status};

#[derive(Clone)]
pub struct Storybook {
    stories: HashMap<String, Story>,
}

impl Storybook {
    pub fn port_story(&self, port: u32) -> Result<&Story, String> {
        self.stories
            .get(&port.to_string())
            .ok_or("Story not implemented yet.".to_owned())
    }

    pub fn story(&self, story_ix: &str) -> Result<&Story, String> {
        self.stories
            .get(story_ix)
            .ok_or("Story not implemented yet.".to_owned())
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Story {
    main_text: Str,
    keyword_links: HashMap<Keyword, StoryIx>,

    options: Vec<StoryOption>,

    #[serde(skip_serializing)]
    pub effects: Vec<Effect>,
}

impl Story {
    pub fn get_option(&self, ix: usize) -> Option<&StoryOption> {
        self.options.get(ix)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct StoryOption {
    text: Str,

    #[serde(skip_serializing)]
    pub effects: Vec<Effect>,
}

type StoryIx = String;
type Keyword = String;

impl Default for Storybook {
    fn default() -> Self {
        let mut stories = HashMap::new();
        stories.insert("2".to_owned(), story2());
        stories.insert("133".to_owned(), story133());

        Self { stories }
    }
}

fn story2() -> Story {
    let main_text =
                " Seafoam winds through a maze of mossy rocks on the cave floor.
                They brush the heel of a smiling skeleton which sits reclined
                against an old iron door, blood red with rust. <br>
                <span class='storybook_bold'>You find a skeleton against an iron door at the back of
                a cave.</span> ";

    let a = StoryOption {
        text: "Search the skeleton.",
        effects: vec![Effect::TurnToStory("133".to_owned())],
    };
    // TODO option b
    let c = StoryOption {
        text: "Leave",
        effects: vec![Effect::ReturnToShip],
    };

    Story {
        main_text,
        keyword_links: HashMap::new(),
        options: vec![a, c],
        effects: Vec::new(),
    }
}

fn story3() -> Story {
    let links = vec![
        ("SHELL".to_owned(), "3.11".to_owned()),
        ("MIGHT".to_owned(), "3.12".to_owned()),
    ];

    let main_text = "The quiet shore is lined with fir trees, their border
    thick but penetrable. Within the forest lies an encampment of Pann traders
    with hair like polished Platinum<br> <span class='storybook_bold'>You find a
    camp of Pann traders.</span>

    <i>Draw 7 market cards. You may buy any of them. You may use material as 
    coins when you are purchasing these cards. Return to the ship.";

    Story {
        main_text,
        keyword_links: links.into_iter().collect(),
        options: Vec::new(),
        //TODO market effect
        effects: vec![Effect::ReturnToShip],
    }
}

fn story133() -> Story {
    let main_text = 
        "A bag falls slack from the skeleton's fingers. You expect to collect
        it easily, but a snake with a milk-pink mouth hisses out and fastens
        to your hand. You whip it by the tail against the wet wall of the cave.

        <span class='storybook_bold'>A snake attacks you as you take
        the skeleton's bag.</span>

        <i> -1 health. Gain 4 coins and 1 venom. Gain quest 155. Return to the
        ship </i> ";

    // TODO should we always put these in reverse order?
    let effects = vec![
        Effect::ReturnToShip,
        Effect::TakeHealthDamage(1),
        Effect::TakeStatus(Status::Venom),
        Effect::GainResource(Resource::Coin, 4),
        // TODO venom and quest
    ];

    Story {
        main_text,
        keyword_links: HashMap::new(),
        options: Vec::new(),
        effects
    }
}

type Str = &'static str;
