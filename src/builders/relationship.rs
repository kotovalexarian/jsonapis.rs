use super::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RelationshipBuilder {
    meta: Option<MetaOrAttrsBuilder>,
    links: Option<LinksBuilder>,
    data: Option<DataBuilder>,
}

impl Builder<'_> for RelationshipBuilder {
    type Entity = Relationship;

    fn finish(self) -> Result<Self::Entity, BuildErrors> {
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
    pub fn meta<M: Into<MetaOrAttrsBuilder>>(self, meta: M) -> Self {
        Self {
            meta: Some(meta.into()),
            ..self
        }
    }

    pub fn links<L: Into<LinksBuilder>>(self, links: L) -> Self {
        Self {
            links: Some(links.into()),
            ..self
        }
    }

    pub fn data<D: Into<DataBuilder>>(self, data: D) -> Self {
        Self {
            data: Some(data.into()),
            ..self
        }
    }

    pub fn meta1<N: ToString, V: Into<Value>>(self, name: N, meta1: V) -> Self {
        let meta = self.meta.unwrap_or_default().item(name, meta1);

        Self {
            meta: Some(meta),
            ..self
        }
    }

    pub fn link<N: ToString, L: Into<LinkBuilder>>(
        self,
        name: N,
        link: L,
    ) -> Self {
        let links = self.links.unwrap_or_default().link(name, link);

        Self {
            links: Some(links),
            ..self
        }
    }
}

impl From<Relationship> for RelationshipBuilder {
    fn from(relationship: Relationship) -> Self {
        Self {
            meta: relationship.meta.map(|meta| meta.into()),
            links: relationship.links.map(|links| links.into()),
            data: relationship.data.map(|data| data.into()),
        }
    }
}

impl<R: Into<ResourceBuilder>> From<R> for RelationshipBuilder {
    fn from(resource: R) -> Self {
        Self::default().data(resource.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixtures;

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
                meta: Some(fixtures::meta_or_attrs()),
                links: Some(Links {
                    other: HashMap::new(),
                    self_: Some(Link::String("http://self.com".into())),
                    related: None,
                    first: None,
                    last: None,
                    prev: Some(Link::Object(LinkObject {
                        href: "http://prev.com".into(),
                        meta: Some(fixtures::meta_or_attrs()),
                    })),
                    next: None,
                    about: None,
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
                meta: Some(fixtures::meta_or_attrs()),
                links: Some(Links {
                    other: HashMap::new(),
                    self_: Some(Link::String("http://self.com".into())),
                    related: None,
                    first: None,
                    last: None,
                    prev: Some(Link::Object(LinkObject {
                        href: "http://prev.com".into(),
                        meta: Some(fixtures::meta_or_attrs()),
                    })),
                    next: None,
                    about: None,
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
                .link("qwe", "http://qwe.com")
                .unwrap(),
            Relationship {
                meta: None,
                links: Some(fixtures::simple_links()),
                data: None,
            },
        );
    }

    #[test]
    fn implicit_from_entity() {
        let relationship = Relationship {
            meta: Some(fixtures::meta_or_attrs()),
            links: Some(fixtures::simple_links()),
            data: Some(Data::Single(Resource {
                type_: "qwerties".into(),
                id: Some("123".into()),
                meta: None,
                links: None,
                attributes: None,
                relationships: None,
            })),
        };

        let builder: RelationshipBuilder = relationship.clone().into();

        assert_eq!(builder.unwrap(), relationship);
    }

    #[test]
    fn with_meta_implicit_from_entity() {
        assert_eq!(
            RelationshipBuilder::default()
                .meta(fixtures::meta_or_attrs())
                .unwrap(),
            Relationship {
                meta: Some(fixtures::meta_or_attrs()),
                links: None,
                data: None,
            },
        );
    }

    #[test]
    fn with_links_implicit_from_entity() {
        assert_eq!(
            RelationshipBuilder::default()
                .links(fixtures::simple_links())
                .unwrap(),
            Relationship {
                meta: None,
                links: Some(fixtures::simple_links()),
                data: None,
            },
        );
    }

    #[test]
    fn with_data_implicit_from_entity() {
        assert_eq!(
            RelationshipBuilder::default()
                .data(Data::Single(Resource {
                    type_: "qwerties".into(),
                    id: Some("123".into()),
                    meta: None,
                    links: None,
                    attributes: None,
                    relationships: None,
                }))
                .unwrap(),
            Relationship {
                meta: None,
                links: None,
                data: Some(Data::Single(Resource {
                    type_: "qwerties".into(),
                    id: Some("123".into()),
                    meta: None,
                    links: None,
                    attributes: None,
                    relationships: None,
                })),
            },
        );
    }

    #[test]
    fn with_data_single_implicit_from_entity() {
        assert_eq!(
            RelationshipBuilder::default()
                .data(Resource {
                    type_: "qwerties".into(),
                    id: Some("123".into()),
                    meta: None,
                    links: None,
                    attributes: None,
                    relationships: None,
                })
                .unwrap(),
            Relationship {
                meta: None,
                links: None,
                data: Some(Data::Single(Resource {
                    type_: "qwerties".into(),
                    id: Some("123".into()),
                    meta: None,
                    links: None,
                    attributes: None,
                    relationships: None,
                })),
            },
        );
    }
}
