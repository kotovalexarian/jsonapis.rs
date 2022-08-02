use super::*;

impl Entity<'_> for Resource {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Resource {
    #[serde(rename = "type")]
    pub type_: String,
    pub id: Option<String>,
    pub meta: Option<MetaOrAttrs>,
    pub links: Option<Links>,
    pub attributes: Option<MetaOrAttrs>,
    pub relationships: Option<Relationships>,
}
