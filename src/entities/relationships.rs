use super::*;

impl Entity<'_> for Relationships {}

pub type Relationships = HashMap<String, Relationship>;
