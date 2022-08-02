use super::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct JsonApiBuilder {
    version: Option<Version>,
    meta: Option<MetaOrAttrsBuilder>,
}

impl Builder<'_> for JsonApiBuilder {
    type Entity = JsonApi;

    fn finish(self) -> Result<Self::Entity, BuildErrors> {
        Ok(Self::Entity {
            version: self.version,
            meta: match self.meta {
                None => None,
                Some(meta) => Some(meta.finish()?),
            },
        })
    }
}

impl JsonApiBuilder {
    pub fn version(self, version: Version) -> Self {
        Self {
            version: Some(version),
            ..self
        }
    }

    pub fn meta<M: Into<MetaOrAttrsBuilder>>(self, meta: M) -> Self {
        Self {
            meta: Some(meta.into()),
            ..self
        }
    }
}

impl From<JsonApi> for JsonApiBuilder {
    fn from(jsonapi: JsonApi) -> Self {
        Self {
            version: jsonapi.version,
            meta: jsonapi.meta.map(|meta| meta.into()),
        }
    }
}

impl From<Version> for JsonApiBuilder {
    fn from(version: Version) -> Self {
        Self {
            version: Some(version),
            meta: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn meta() -> MetaOrAttrs {
        let mut meta = MetaOrAttrs::new();
        meta.insert("foo".into(), 123.into());
        meta.insert("bar".into(), "qwe".into());
        meta
    }

    #[test]
    fn empty() {
        assert_eq!(
            JsonApiBuilder::default().unwrap(),
            JsonApi {
                version: None,
                meta: None,
            },
        );
    }

    #[test]
    fn full() {
        assert_eq!(
            JsonApiBuilder::default()
                .version(Version::new(456))
                .meta(
                    MetaOrAttrsBuilder::default()
                        .item("foo", 123)
                        .item("bar", "qwe"),
                )
                .unwrap(),
            JsonApi {
                version: Some(Version::new(456)),
                meta: Some(meta()),
            },
        );
    }

    #[test]
    fn with_version() {
        assert_eq!(
            JsonApiBuilder::default()
                .version(Version::new(456))
                .unwrap(),
            JsonApi {
                version: Some(Version::new(456)),
                meta: None,
            },
        );
    }

    #[test]
    fn with_meta() {
        assert_eq!(
            JsonApiBuilder::default()
                .meta(
                    MetaOrAttrsBuilder::default()
                        .item("foo", 123)
                        .item("bar", "qwe"),
                )
                .unwrap(),
            JsonApi {
                version: None,
                meta: Some(meta()),
            },
        );
    }

    #[test]
    fn implicit_from_entity() {
        let jsonapi = JsonApi {
            version: Some(Version::new(456)),
            meta: Some(meta()),
        };

        let builder: JsonApiBuilder = jsonapi.clone().into();

        assert_eq!(builder.unwrap(), jsonapi);
    }

    #[test]
    fn with_meta_implicit_from_entity() {
        assert_eq!(
            JsonApiBuilder::default().meta(meta()).unwrap(),
            JsonApi {
                version: None,
                meta: Some(meta()),
            },
        );
    }
}
