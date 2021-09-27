use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinkBuilder {
    href: String,
    meta: Option<MetaOrAttrsBuilder>,
}

impl LinkBuilder {
    pub fn new(href: &str) -> Self {
        Self {
            href: href.into(),
            meta: None,
        }
    }
}

impl Builder for LinkBuilder {
    type Entity = Link;

    fn finish(self) -> Result<Self::Entity, ()> {
        Ok(match self.meta {
            None => Link::String(self.href),
            Some(meta) => Link::Object(LinkObject {
                href: self.href,
                meta: Some(meta.finish()?),
            }),
        })
    }
}

impl<S: ToString> From<S> for LinkBuilder {
    fn from(s: S) -> Self {
        Self {
            href: s.to_string(),
            meta: None,
        }
    }
}

impl LinkBuilder {
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
            LinkBuilder::new("http://example.com").unwrap(),
            Link::String("http://example.com".into()),
        );
    }

    #[test]
    fn full() {
        assert_eq!(
            LinkBuilder::new("http://example.com")
                .meta(
                    MetaOrAttrsBuilder::default()
                        .item("foo", 123)
                        .item("bar", "qwe"),
                )
                .unwrap(),
            Link::Object(LinkObject {
                href: "http://example.com".into(),
                meta: Some(meta()),
            }),
        );
    }
}
