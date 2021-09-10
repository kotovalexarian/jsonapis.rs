use super::*;

#[derive(Clone)]
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

impl RelationshipsBuilder {
    pub fn rel(self, name: &str, relationship: RelationshipBuilder) -> Self {
        let mut relationships = self.0;
        relationships.insert(name.into(), relationship);
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
}
