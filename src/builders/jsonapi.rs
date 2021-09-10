use super::*;

#[derive(Clone)]
pub struct JsonApiBuilder {
    version: Option<Version>,
    meta: Option<MetaOrAttrsBuilder>,
}

impl Default for JsonApiBuilder {
    fn default() -> Self {
        Self {
            version: None,
            meta: None,
        }
    }
}

impl Builder for JsonApiBuilder {
    type Entity = JsonApi;

    fn finish(self) -> Result<Self::Entity, ()> {
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

    pub fn meta(self, meta: MetaOrAttrsBuilder) -> Self {
        Self {
            meta: Some(meta),
            ..self
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
}
