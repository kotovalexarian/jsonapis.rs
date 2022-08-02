use super::*;

impl Entity<'_> for ErrorSource {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ErrorSource {
    // TODO: Add entity with validation
    pub pointer: Option<String>,
    pub parameter: Option<String>,
}
