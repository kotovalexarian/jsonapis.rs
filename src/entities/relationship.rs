use super::*;

impl Entity<'_> for Relationship {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Relationship {
    pub meta: Option<MetaOrAttrs>,
    pub links: Option<Links>,
    pub data: Option<Data>,
}
