use super::*;

impl Entity for JsonApi {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct JsonApi {
    pub meta: Option<MetaOrAttrs>,
    pub version: Option<Version>,
}
