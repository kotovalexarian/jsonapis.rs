use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceBuilder {
    type_: String,
    id: Option<String>,
    meta: Option<MetaOrAttrsBuilder>,
    links: Option<LinksBuilder>,
    attributes: Option<MetaOrAttrsBuilder>,
    relationships: Option<RelationshipsBuilder>,
}

impl ResourceBuilder {
    pub fn new(type_: &str) -> Self {
        Self {
            type_: type_.into(),
            id: None,
            meta: None,
            links: None,
            attributes: None,
            relationships: None,
        }
    }

    pub fn new_with_id<I: ToString>(type_: &str, id: I) -> Self {
        Self {
            type_: type_.into(),
            id: Some(id.to_string()),
            meta: None,
            links: None,
            attributes: None,
            relationships: None,
        }
    }
}

impl Builder for ResourceBuilder {
    type Entity = Resource;

    fn finish(self) -> Result<Self::Entity, ()> {
        Ok(Self::Entity {
            type_: self.type_,
            id: self.id,
            meta: match self.meta {
                None => None,
                Some(meta) => Some(meta.finish()?),
            },
            links: match self.links {
                None => None,
                Some(links) => Some(links.finish()?),
            },
            attributes: match self.attributes {
                None => None,
                Some(attributes) => Some(attributes.finish()?),
            },
            relationships: match self.relationships {
                None => None,
                Some(relationships) => Some(relationships.finish()?),
            },
        })
    }
}

impl From<Resource> for ResourceBuilder {
    fn from(resource: Resource) -> Self {
        Self {
            type_: resource.type_,
            id: resource.id,
            meta: match resource.meta {
                None => None,
                Some(meta) => Some(meta.into()),
            },
            links: match resource.links {
                None => None,
                Some(links) => Some(links.into()),
            },
            attributes: match resource.attributes {
                None => None,
                Some(attributes) => Some(attributes.into()),
            },
            relationships: match resource.relationships {
                None => None,
                Some(relationships) => Some(relationships.into()),
            },
        }
    }
}

impl ResourceBuilder {
    pub fn id(self, id: &str) -> Self {
        Self {
            id: Some(id.into()),
            ..self
        }
    }

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

    pub fn attributes(self, attributes: MetaOrAttrsBuilder) -> Self {
        Self {
            attributes: Some(attributes),
            ..self
        }
    }

    pub fn relationships(self, relationships: RelationshipsBuilder) -> Self {
        Self {
            relationships: Some(relationships),
            ..self
        }
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

    pub fn attr<V: Into<Value>>(self, name: &str, attribute: V) -> Self {
        let attributes = self
            .attributes
            .unwrap_or(MetaOrAttrsBuilder::default())
            .item(name, attribute);

        Self {
            attributes: Some(attributes),
            ..self
        }
    }

    pub fn rel(self, name: &str, relationship: RelationshipBuilder) -> Self {
        let relationships = self
            .relationships
            .unwrap_or(RelationshipsBuilder::default())
            .rel(name, relationship);

        Self {
            relationships: Some(relationships),
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

    fn links() -> Links {
        Links {
            other: {
                let mut other = HashMap::new();
                other.insert(
                    "qwe".into(),
                    Link::String("http://qwe.com".into()),
                );
                other
            },
            self_: Some(Link::String("http://self.com".into())),
            related: None,
            first: None,
            last: None,
            prev: None,
            next: None,
        }
    }

    #[test]
    fn empty() {
        assert_eq!(
            ResourceBuilder::new("qwerties").unwrap(),
            Resource {
                type_: "qwerties".into(),
                id: None,
                meta: None,
                links: None,
                attributes: None,
                relationships: None,
            },
        );
    }

    #[test]
    fn empty_with_id() {
        assert_eq!(
            ResourceBuilder::new_with_id("qwerties", 123).unwrap(),
            Resource {
                type_: "qwerties".into(),
                id: Some("123".into()),
                meta: None,
                links: None,
                attributes: None,
                relationships: None,
            },
        );
    }

    #[test]
    fn full() {
        assert_eq!(
            ResourceBuilder::new("qwerties")
                .id("123")
                .meta(
                    MetaOrAttrsBuilder::default()
                        .item("foo", 123)
                        .item("bar", "qwe"),
                )
                .links(
                    LinksBuilder::default()
                        .self_(LinkBuilder::new("http://self.com"))
                        .next(
                            LinkBuilder::new("http://next.com").meta(
                                MetaOrAttrsBuilder::default()
                                    .item("foo", 123)
                                    .item("bar", "qwe"),
                            ),
                        ),
                )
                .attributes(
                    MetaOrAttrsBuilder::default()
                        .item("foo", 123)
                        .item("bar", "qwe"),
                )
                .relationships(
                    RelationshipsBuilder::default()
                        .rel("foo", RelationshipBuilder::default())
                )
                .unwrap(),
            Resource {
                type_: "qwerties".into(),
                id: Some("123".into()),
                meta: Some(meta()),
                links: Some(Links {
                    other: HashMap::new(),
                    self_: Some(Link::String("http://self.com".into())),
                    related: None,
                    first: None,
                    last: None,
                    prev: None,
                    next: Some(Link::Object(LinkObject {
                        href: "http://next.com".into(),
                        meta: Some(meta()),
                    })),
                }),
                attributes: Some(meta()),
                relationships: Some({
                    let mut relationships = Relationships::new();
                    relationships.insert(
                        "foo".into(),
                        Relationship {
                            meta: None,
                            links: None,
                            data: None,
                        },
                    );
                    relationships
                }),
            },
        );
    }

    #[test]
    fn full_delegators() {
        assert_eq!(
            ResourceBuilder::new("qwerties")
                .id("123")
                .meta1("foo", 123)
                .meta1("bar", "qwe")
                .link("self", LinkBuilder::new("http://self.com"))
                .link(
                    "next",
                    LinkBuilder::new("http://next.com").meta(
                        MetaOrAttrsBuilder::default()
                            .item("foo", 123)
                            .item("bar", "qwe"),
                    ),
                )
                .attr("foo", 123)
                .attr("bar", "qwe")
                .rel("foo", RelationshipBuilder::default())
                .unwrap(),
            Resource {
                type_: "qwerties".into(),
                id: Some("123".into()),
                meta: Some(meta()),
                links: Some(Links {
                    other: HashMap::new(),
                    self_: Some(Link::String("http://self.com".into())),
                    related: None,
                    first: None,
                    last: None,
                    prev: None,
                    next: Some(Link::Object(LinkObject {
                        href: "http://next.com".into(),
                        meta: Some(meta()),
                    })),
                }),
                attributes: Some(meta()),
                relationships: Some({
                    let mut relationships = Relationships::new();
                    relationships.insert(
                        "foo".into(),
                        Relationship {
                            meta: None,
                            links: None,
                            data: None,
                        },
                    );
                    relationships
                }),
            },
        );
    }

    #[test]
    fn with_meta1_implicit() {
        assert_eq!(
            ResourceBuilder::new("qwerties")
                .meta1("foo", 123)
                .meta1("bar", "car")
                .unwrap(),
            Resource {
                type_: "qwerties".into(),
                id: None,
                meta: Some({
                    let mut meta = MetaOrAttrs::new();
                    meta.insert("foo".into(), Value::Number(123.into()));
                    meta.insert("bar".into(), Value::String("car".into()));
                    meta
                }),
                links: None,
                attributes: None,
                relationships: None,
            },
        );
    }

    #[test]
    fn with_link_implicit_from_str() {
        assert_eq!(
            ResourceBuilder::new("qwerties")
                .link("self", "http://self.com")
                .link("qwe", "http://qwe.com")
                .unwrap(),
            Resource {
                type_: "qwerties".into(),
                id: None,
                meta: None,
                links: Some(links()),
                attributes: None,
                relationships: None,
            },
        );
    }

    #[test]
    fn with_attr_implicit() {
        assert_eq!(
            ResourceBuilder::new("qwerties")
                .attr("foo", 123)
                .attr("bar", "car")
                .unwrap(),
            Resource {
                type_: "qwerties".into(),
                id: None,
                meta: None,
                links: None,
                attributes: Some({
                    let mut meta = MetaOrAttrs::new();
                    meta.insert("foo".into(), Value::Number(123.into()));
                    meta.insert("bar".into(), Value::String("car".into()));
                    meta
                }),
                relationships: None,
            },
        );
    }

    // TODO: implicit tests
}
