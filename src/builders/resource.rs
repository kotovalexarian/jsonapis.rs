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
    pub fn new<T: ToString>(type_: T) -> Self {
        Self {
            type_: type_.to_string(),
            id: None,
            meta: None,
            links: None,
            attributes: None,
            relationships: None,
        }
    }

    pub fn new_with_id<T: ToString, I: ToString>(type_: T, id: I) -> Self {
        Self {
            type_: type_.to_string(),
            id: Some(id.to_string()),
            meta: None,
            links: None,
            attributes: None,
            relationships: None,
        }
    }
}

impl Builder<'_> for ResourceBuilder {
    type Entity = Resource;

    fn finish(self) -> Result<Self::Entity, BuildErrors> {
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

impl ResourceBuilder {
    pub fn id<I: ToString>(self, id: I) -> Self {
        Self {
            id: Some(id.to_string()),
            ..self
        }
    }

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

    pub fn attributes<M: Into<MetaOrAttrsBuilder>>(
        self,
        attributes: M,
    ) -> Self {
        Self {
            attributes: Some(attributes.into()),
            ..self
        }
    }

    pub fn relationships<R: Into<RelationshipsBuilder>>(
        self,
        relationships: R,
    ) -> Self {
        Self {
            relationships: Some(relationships.into()),
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

    pub fn attr<N: ToString, V: Into<Value>>(
        self,
        name: N,
        attribute: V,
    ) -> Self {
        let attributes =
            self.attributes.unwrap_or_default().item(name, attribute);

        Self {
            attributes: Some(attributes),
            ..self
        }
    }

    pub fn rel<N: ToString, R: Into<RelationshipBuilder>>(
        self,
        name: N,
        relationship: R,
    ) -> Self {
        let relationships = self
            .relationships
            .unwrap_or_default()
            .rel(name, relationship.into());

        Self {
            relationships: Some(relationships),
            ..self
        }
    }
}

impl From<Resource> for ResourceBuilder {
    fn from(resource: Resource) -> Self {
        Self {
            type_: resource.type_,
            id: resource.id,
            meta: resource.meta.map(|meta| meta.into()),
            links: resource.links.map(|links| links.into()),
            attributes: resource.attributes.map(|attributes| attributes.into()),
            relationships: resource
                .relationships
                .map(|relationships| relationships.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixtures;

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
                meta: Some(fixtures::meta_or_attrs()),
                links: Some(Links {
                    other: HashMap::new(),
                    self_: Some(Link::String("http://self.com".into())),
                    related: None,
                    first: None,
                    last: None,
                    prev: None,
                    next: Some(Link::Object(LinkObject {
                        href: "http://next.com".into(),
                        meta: Some(fixtures::meta_or_attrs()),
                    })),
                    about: None,
                }),
                attributes: Some(fixtures::meta_or_attrs()),
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
                meta: Some(fixtures::meta_or_attrs()),
                links: Some(Links {
                    other: HashMap::new(),
                    self_: Some(Link::String("http://self.com".into())),
                    related: None,
                    first: None,
                    last: None,
                    prev: None,
                    next: Some(Link::Object(LinkObject {
                        href: "http://next.com".into(),
                        meta: Some(fixtures::meta_or_attrs()),
                    })),
                    about: None,
                }),
                attributes: Some(fixtures::meta_or_attrs()),
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
                links: Some(fixtures::simple_links()),
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

    #[test]
    fn with_meta_implicit_from_entity() {
        assert_eq!(
            ResourceBuilder::new("qwerties")
                .meta(fixtures::meta_or_attrs())
                .unwrap(),
            Resource {
                type_: "qwerties".into(),
                id: None,
                meta: Some(fixtures::meta_or_attrs()),
                links: None,
                attributes: None,
                relationships: None,
            },
        );
    }

    #[test]
    fn with_links_implicit_from_entity() {
        assert_eq!(
            ResourceBuilder::new("qwerties")
                .links(fixtures::simple_links())
                .unwrap(),
            Resource {
                type_: "qwerties".into(),
                id: None,
                meta: None,
                links: Some(fixtures::simple_links()),
                attributes: None,
                relationships: None,
            },
        );
    }

    #[test]
    fn with_attributes_implicit_from_entity() {
        assert_eq!(
            ResourceBuilder::new("qwerties")
                .attributes(fixtures::meta_or_attrs())
                .unwrap(),
            Resource {
                type_: "qwerties".into(),
                id: None,
                meta: None,
                links: None,
                attributes: Some(fixtures::meta_or_attrs()),
                relationships: None,
            },
        );
    }

    #[test]
    fn with_relationships_implicit_from_entity() {
        assert_eq!(
            ResourceBuilder::new("qwerties")
                .relationships({
                    let mut relationships = HashMap::new();
                    relationships.insert(
                        "foo".into(),
                        Relationship {
                            meta: Some(fixtures::meta_or_attrs()),
                            links: Some(fixtures::simple_links()),
                            data: None,
                        },
                    );
                    relationships
                })
                .unwrap(),
            Resource {
                type_: "qwerties".into(),
                id: None,
                meta: None,
                links: None,
                attributes: None,
                relationships: Some({
                    let mut relationships = HashMap::new();
                    relationships.insert(
                        "foo".into(),
                        Relationship {
                            meta: Some(fixtures::meta_or_attrs()),
                            links: Some(fixtures::simple_links()),
                            data: None,
                        },
                    );
                    relationships
                }),
            },
        );
    }

    #[test]
    fn with_rel_implicit_from_entity() {
        assert_eq!(
            ResourceBuilder::new("qwerties")
                .rel(
                    "foo",
                    Relationship {
                        meta: Some(fixtures::meta_or_attrs()),
                        links: Some(fixtures::simple_links()),
                        data: None,
                    }
                )
                .unwrap(),
            Resource {
                type_: "qwerties".into(),
                id: None,
                meta: None,
                links: None,
                attributes: None,
                relationships: Some({
                    let mut relationships = HashMap::new();
                    relationships.insert(
                        "foo".into(),
                        Relationship {
                            meta: Some(fixtures::meta_or_attrs()),
                            links: Some(fixtures::simple_links()),
                            data: None,
                        },
                    );
                    relationships
                }),
            },
        );
    }

    // TODO: implicit tests
}
