use super::*;

#[derive(Clone)]
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

    pub fn data(self, data: DataBuilder) -> Self {
        Self {
            data: Some(data),
            ..self
        }
    }

    pub fn data_single(self, resource: ResourceBuilder) -> Self {
        self.data(DataBuilder::Single(resource))
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
}
