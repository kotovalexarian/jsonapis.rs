use super::*;

impl Entity<'_> for ErrorObject {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ErrorObject {
    pub id: Option<String>,
    pub links: Option<Links>,
    pub status: Option<HttpStatus>,
    pub code: Option<String>,
    pub title: Option<String>,
    pub detail: Option<String>,
    pub source: Option<ErrorSource>,
    pub meta: Option<MetaOrAttrs>,
}
