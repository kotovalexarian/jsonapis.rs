use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationshipBuilder {
    meta: Option<MetaOrAttrsBuilder>,
    links: Option<LinksBuilder>,
    data: Option<DataBuilder>,
}

impl Default for RelationshipBuilder {
    fn default() -> Self {
        Self {
            meta: None,
            links: None,
            data: None,
        }
    }
}

impl Builder for RelationshipBuilder {
    type Entity = Relationship;

    fn finish(self) -> Result<Self::Entity, ()> {
        Ok(Self::Entity {
            meta: match self.meta {
                None => None,
                Some(meta) => Some(meta.finish()?),
            },
            links: match self.links {
                None => None,
                Some(links) => Some(links.finish()?),
            },
            data: match self.data {
                None => None,
                Some(data) => Some(data.finish()?),
            },
        })
    }
}

impl RelationshipBuilder {
    pub fn meta(self, meta: MetaOrAttrsBuilder) -> Self {
        Self {
            meta: Some(meta),
            ..self
        }
    }

    pub fn links(self, links: LinksBuilder) -> Self {
        Self {
            links: Some(links),
            ..self
        }
    }

    pub fn data<D: Into<DataBuilder>>(self, data: D) -> Self {
        Self {
            data: Some(data.into()),
            ..self
        }
    }

    pub fn data_single(self, resource: ResourceBuilder) -> Self {
        self.data(resource)
    }

    pub fn meta1<V: Into<Value>>(self, name: &str, meta1: V) -> Self {
        let meta = self
            .meta
            .unwrap_or(MetaOrAttrsBuilder::default())
            .item(name, meta1);

        Self {
            meta: Some(meta),
            ..self
        }
    }

    pub fn link<L: Into<LinkBuilder>>(self, name: &str, link: L) -> Self {
        let links = self
            .links
            .unwrap_or(LinksBuilder::default())
            .link(name, link);

        Self {
            links: Some(links),
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
            RelationshipBuilder::default().unwrap(),
            Relationship {
                meta: None,
                links: None,
                data: None,
            },
        );
    }

    #[test]
    fn full() {
        assert_eq!(
            RelationshipBuilder::default()
                .meta(
                    MetaOrAttrsBuilder::default()
                        .item("foo", 123)
                        .item("bar", "qwe"),
                )
                .links(
                    LinksBuilder::default()
                        .self_(LinkBuilder::new("http://self.com"))
                        .prev(
                            LinkBuilder::new("http://prev.com").meta(
                                MetaOrAttrsBuilder::default()
                                    .item("foo", 123)
                                    .item("bar", "qwe"),
                            ),
                        ),
                )
                .data(DataBuilder::Single(ResourceBuilder::new("qwerties")))
                .unwrap(),
            Relationship {
                meta: Some(meta()),
                links: Some(Links {
                    other: HashMap::new(),
                    self_: Some(Link::String("http://self.com".into())),
                    related: None,
                    first: None,
                    last: None,
                    prev: Some(Link::Object(LinkObject {
                        href: "http://prev.com".into(),
                        meta: Some(meta()),
                    })),
                    next: None,
                }),
                data: Some(Data::Single(Resource {
                    type_: "qwerties".into(),
                    id: None,
                    meta: None,
                    links: None,
                    attributes: None,
                    relationships: None,
                })),
            },
        );
    }

    #[test]
    fn full_delegators() {
        assert_eq!(
            RelationshipBuilder::default()
                .meta1("foo", 123)
                .meta1("bar", "qwe")
                .link("self", LinkBuilder::new("http://self.com"))
                .link(
                    "prev",
                    LinkBuilder::new("http://prev.com").meta(
                        MetaOrAttrsBuilder::default()
                            .item("foo", 123)
                            .item("bar", "qwe"),
                    )
                )
                .data(DataBuilder::Single(ResourceBuilder::new("qwerties")))
                .unwrap(),
            Relationship {
                meta: Some(meta()),
                links: Some(Links {
                    other: HashMap::new(),
                    self_: Some(Link::String("http://self.com".into())),
                    related: None,
                    first: None,
                    last: None,
                    prev: Some(Link::Object(LinkObject {
                        href: "http://prev.com".into(),
                        meta: Some(meta()),
                    })),
                    next: None,
                }),
                data: Some(Data::Single(Resource {
                    type_: "qwerties".into(),
                    id: None,
                    meta: None,
                    links: None,
                    attributes: None,
                    relationships: None,
                })),
            },
        );
    }

    #[test]
    fn with_data_from_resource() {
        assert_eq!(
            RelationshipBuilder::default()
                .data(ResourceBuilder::new("qwerties"))
                .unwrap(),
            Relationship {
                meta: None,
                links: None,
                data: Some(Data::Single(Resource {
                    type_: "qwerties".into(),
                    id: None,
                    meta: None,
                    links: None,
                    attributes: None,
                    relationships: None,
                })),
            },
        );
    }

    #[test]
    fn with_data_from_resources() {
        assert_eq!(
            RelationshipBuilder::default()
                .data(vec![ResourceBuilder::new("qwerties")])
                .unwrap(),
            Relationship {
                meta: None,
                links: None,
                data: Some(Data::Multiple(vec![Resource {
                    type_: "qwerties".into(),
                    id: None,
                    meta: None,
                    links: None,
                    attributes: None,
                    relationships: None,
                }])),
            },
        );
    }

    #[test]
    fn with_meta1_implicit() {
        assert_eq!(
            RelationshipBuilder::default()
                .meta1("foo", 123)
                .meta1("bar", "car")
                .unwrap(),
            Relationship {
                meta: Some({
                    let mut meta = MetaOrAttrs::new();
                    meta.insert("foo".into(), Value::Number(123.into()));
                    meta.insert("bar".into(), Value::String("car".into()));
                    meta
                }),
                links: None,
                data: None,
            },
        );
    }

    #[test]
    fn with_link_implicit_from_str() {
        assert_eq!(
            RelationshipBuilder::default()
                .link("self", "http://self.com")
                .link("foo", "http://foo.com")
                .unwrap(),
            Relationship {
                meta: None,
                links: Some(Links {
                    other: {
                        let mut other = HashMap::new();
                        other.insert(
                            "foo".into(),
                            Link::String("http://foo.com".into()),
                        );
                        other
                    },
                    self_: Some(Link::String("http://self.com".into())),
                    related: None,
                    first: None,
                    last: None,
                    prev: None,
                    next: None,
                }),
                data: None,
            },
        );
    }
}
