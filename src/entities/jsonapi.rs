use super::*;

impl Entity<'_> for JsonApi {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct JsonApi {
    pub version: Option<Version>,
    pub meta: Option<MetaOrAttrs>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug() {
        assert_eq!(
            format!(
                "{:?}",
                JsonApi {
                    version: Some(Version::default()),
                    meta: None,
                },
            ),
            "JsonApi { \
                version: Some(Version(\"1.0\")), \
                meta: None \
            }",
        );
    }

    #[test]
    fn equality() {
        assert_eq!(
            JsonApi {
                version: None,
                meta: None,
            },
            JsonApi {
                version: None,
                meta: None,
            },
        );

        assert_eq!(
            JsonApi {
                version: Some(Version::new(123)),
                meta: None,
            },
            JsonApi {
                version: Some(Version::new(123)),
                meta: None,
            },
        );

        assert_ne!(
            JsonApi {
                version: Some(Version::new(321)),
                meta: None,
            },
            JsonApi {
                version: Some(Version::new(123)),
                meta: None,
            },
        );
    }
}
