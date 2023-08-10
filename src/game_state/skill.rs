use serde::Serialize;

#[derive(Clone, Serialize, Hash, PartialEq, Eq, Debug)]
pub enum Skill {
    Savvy,
    Craft,
    Perception,
    Strength,
    Wits
}

impl std::fmt::Display for Skill {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
