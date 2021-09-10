use super::*;

impl Entity for LinkObject {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LinkObject {
    pub href: String,
    pub meta: Option<MetaOrAttrs>,
}
