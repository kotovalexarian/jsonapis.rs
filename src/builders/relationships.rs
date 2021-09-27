use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationshipsBuilder(HashMap<String, RelationshipBuilder>);

impl Default for RelationshipsBuilder {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl Builder for RelationshipsBuilder {
    type Entity = Relationships;

    fn finish(self) -> Result<Self::Entity, ()> {
        let mut relationships = Relationships::new();

        for (name, relationship) in self.0 {
            relationships.insert(name, relationship.finish()?);
        }

        Ok(relationships)
    }
}

impl From<Relationships> for RelationshipsBuilder {
    fn from(relationships: Relationships) -> Self {
        let mut new_relationships = HashMap::new();
        for (key, value) in relationships {
            new_relationships.insert(key, value.into());
        }
        Self(new_relationships)
    }
}

impl RelationshipsBuilder {
    pub fn rel<R: Into<RelationshipBuilder>>(self, name: &str, relationship: R) -> Self {
        let mut relationships = self.0;
        relationships.insert(name.into(), relationship.into());
        Self(relationships)
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
            RelationshipsBuilder::default().unwrap(),
            Relationships::new(),
        );
    }

    #[test]
    fn full() {
        assert_eq!(
            RelationshipsBuilder::default()
                .rel("foo", RelationshipBuilder::default())
                .rel(
                    "bar",
                    RelationshipBuilder::default().meta(
                        MetaOrAttrsBuilder::default()
                            .item("foo", 123)
                            .item("bar", "qwe"),
                    ),
                )
                .rel(
                    "car",
                    RelationshipBuilder::default().data(DataBuilder::Single(
                        ResourceBuilder::new("qwerties")
                    )),
                )
                .unwrap(),
            {
                let mut relationships = Relationships::new();
                relationships.insert(
                    "foo".into(),
                    Relationship {
                        meta: None,
                        links: None,
                        data: None,
                    },
                );
                relationships.insert(
                    "bar".into(),
                    Relationship {
                        meta: Some(meta()),
                        links: None,
                        data: None,
                    },
                );
                relationships.insert(
                    "car".into(),
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
                relationships
            },
        );
    }

    #[test]
    fn with_rel_implicit_from_entity() {
        assert_eq!(
            RelationshipsBuilder::default().rel("qwerty", Relationship {
                meta: Some(meta()),
                links: Some(links()),
                data: None,
            }).unwrap(),
            {
                let mut relationships = HashMap::new();
                relationships.insert(
                    "qwerty".into(),
                    Relationship {
                        meta: Some(meta()),
                        links: Some(links()),
                        data: None,
                    },
                );
                relationships
            },
        );
    }

    // TODO: implicit tests
}
