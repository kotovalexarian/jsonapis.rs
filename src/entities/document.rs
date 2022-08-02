use super::*;

impl Entity<'_> for Document {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Document {
    pub jsonapi: Option<JsonApi>,
    pub meta: Option<MetaOrAttrs>,
    pub links: Option<Links>,
    pub data: Option<Data>,
    pub errors: Option<Vec<Error>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug() {
        assert_eq!(
            format!(
                "{:?}",
                Document {
                    jsonapi: None,
                    meta: None,
                    links: None,
                    data: None,
                    errors: None,
                },
            ),
            "Document { \
                jsonapi: None, \
                meta: None, \
                links: None, \
                data: None, \
                errors: None \
            }",
        );
    }

    #[test]
    fn equality() {
        assert_eq!(
            Document {
                jsonapi: None,
                meta: None,
                links: None,
                data: None,
                errors: None,
            },
            Document {
                jsonapi: None,
                meta: None,
                links: None,
                data: None,
                errors: None,
            },
        );

        assert_ne!(
            Document {
                jsonapi: None,
                meta: None,
                links: None,
                data: None,
                errors: None,
            },
            Document {
                jsonapi: Some(JsonApi {
                    version: None,
                    meta: None,
                }),
                meta: None,
                links: None,
                data: None,
                errors: None,
            },
        );
    }
}
