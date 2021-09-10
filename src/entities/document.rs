use super::*;

impl Entity for Document {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Document {
    pub jsonapi: Option<JsonApi>,
    pub meta: Option<MetaOrAttrs>,
    pub links: Option<Links>,
    pub data: Option<Data>,
}
