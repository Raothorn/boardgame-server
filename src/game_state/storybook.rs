use std::collections::HashMap;

use serde::Serialize;

#[derive(Clone)]
pub struct Storybook {
    stories: HashMap<String, Story>,
}

impl Storybook {
    pub fn port_story(&self, port: u32) -> Result<&Story, String> {
        self
            .stories
            .get(&port.to_string())
            .ok_or("Story not implemented yet.".to_owned())
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Story {
    main_text: Str,
    keyword_links: HashMap<Keyword, StoryIx>,
    options: Vec<StoryOption>,
    effects: Vec<StoryEffect>,
}

#[derive(Clone, Debug, Serialize)]
pub struct StoryOption {
    text: Str,
    effects: Vec<StoryEffect>,
}

#[derive(Clone, Debug, Serialize)]
pub enum StoryEffect {
    TurnToStory(StoryIx),
    ReturnToShip,
}

type StoryIx = String;
type Keyword = String;

impl Default for Storybook {
    fn default() -> Self {
        let mut stories = HashMap::new();
        stories.insert("2".to_owned(), story2());

        Self { stories }
    }
}

fn story2() -> Story {
    let main_text =
                "
                Seafoam winds through a maze of mossy rocks on the cave floor.
                They brush the heel of a smiling skeleton which sits reclined
                against an old iron door, blood red with rust. <br>
                <b>You find a skeleton against an iron door at the back of
                a cave.</b>
                ";

    let a = StoryOption {
        text: "Search the skeleton.",
        effects: vec![StoryEffect::TurnToStory("133".to_owned())],
    };
    // TODO option b
    let c = StoryOption {
        text: "Leave",
        effects: vec![StoryEffect::ReturnToShip],
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
    with hair like polished Platinum<br> <b>You find a camp of Pann traders.</b>

    <i>Draw 7 market cards. You may buy any of them. You may use material as 
    coins when you are purchasing these cards. Return to the ship.";

    Story {
        main_text,
        keyword_links: links.into_iter().collect(),
        options: Vec::new(),
        //TODO market effect
        effects: vec![StoryEffect::ReturnToShip],
    }
}

type Str = &'static str;
