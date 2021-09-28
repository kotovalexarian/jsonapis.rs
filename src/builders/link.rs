use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinkBuilder {
    href: String,
    meta: Option<MetaOrAttrsBuilder>,
}

impl LinkBuilder {
    pub fn new<H: ToString>(href: H) -> Self {
        Self {
            href: href.to_string(),
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

impl LinkBuilder {
    pub fn meta<M: Into<MetaOrAttrsBuilder>>(self, meta: M) -> Self {
        Self {
            meta: Some(meta.into()),
            ..self
        }
    }
}

impl From<Link> for LinkBuilder {
    fn from(link: Link) -> Self {
        match link {
            Link::String(string) => Self {
                href: string,
                meta: None,
            },
            Link::Object(link_object) => Self {
                href: link_object.href,
                meta: match link_object.meta {
                    None => None,
                    Some(meta) => Some(meta.into()),
                },
            },
        }
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

    #[test]
    fn implicit_from_entity_string() {
        let link = Link::String("http://example.com".into());

        let builder: LinkBuilder = link.clone().into();

        assert_eq!(builder.unwrap(), link);
    }

    #[test]
    fn implicit_from_entity_object() {
        let link = Link::Object(LinkObject {
            href: "http://example.com".into(),
            meta: Some(meta()),
        });

        let builder: LinkBuilder = link.clone().into();

        assert_eq!(builder.unwrap(), link);
    }

    #[test]
    fn with_meta_implicit_from_entity() {
        assert_eq!(
            LinkBuilder::new("http://example.com").meta(meta()).unwrap(),
            Link::Object(LinkObject {
                href: "http://example.com".into(),
                meta: Some(meta()),
            }),
        );
    }
}
