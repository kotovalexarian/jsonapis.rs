use super::*;

impl Entity<'_> for LinkObject {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LinkObject {
    pub href: String,
    pub meta: Option<MetaOrAttrs>,
}
