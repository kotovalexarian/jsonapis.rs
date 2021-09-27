use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataBuilder {
    Single(ResourceBuilder),
    Multiple(Vec<ResourceBuilder>),
}

impl Builder for DataBuilder {
    type Entity = Data;

    fn finish(self) -> Result<Self::Entity, ()> {
        Ok(match self {
            Self::Single(resource) => Data::Single(resource.finish()?),
            Self::Multiple(resources) => Data::Multiple({
                let mut new_resources = vec![];
                for resource in resources {
                    new_resources.push(resource.finish()?);
                }
                new_resources
            }),
        })
    }
}

impl From<Data> for DataBuilder {
    fn from(data: Data) -> Self {
        match data {
            Data::Single(resource) => Self::Single(resource.into()),
            Data::Multiple(resources) => Self::Multiple({
                let mut new_resources = vec![];
                for resource in resources {
                    new_resources.push(resource.into());
                }
                new_resources
            }),
        }
    }
}

impl From<Resource> for DataBuilder {
    fn from(resource: Resource) -> Self {
        Self::Single(resource.into())
    }
}

impl From<Vec<Resource>> for DataBuilder {
    fn from(resources: Vec<Resource>) -> Self {
        Self::Multiple({
            let mut new_resources = vec![];
            for resource in resources {
                new_resources.push(resource.into());
            }
            new_resources
        })
    }
}

impl From<ResourceBuilder> for DataBuilder {
    fn from(resource: ResourceBuilder) -> Self {
        Self::Single(resource)
    }
}

impl From<Vec<ResourceBuilder>> for DataBuilder {
    fn from(resources: Vec<ResourceBuilder>) -> Self {
        Self::Multiple(resources)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single() {
        assert_eq!(
            DataBuilder::Single(ResourceBuilder::new("qwerties")).unwrap(),
            Data::Single(Resource {
                type_: "qwerties".into(),
                id: None,
                meta: None,
                links: None,
                attributes: None,
                relationships: None,
            }),
        );
    }

    #[test]
    fn multiple_zero() {
        assert_eq!(
            DataBuilder::Multiple(vec![]).unwrap(),
            Data::Multiple(vec![]),
        );
    }

    #[test]
    fn multiple_one() {
        assert_eq!(
            DataBuilder::Multiple(vec![ResourceBuilder::new("qwerties")])
                .unwrap(),
            Data::Multiple(vec![Resource {
                type_: "qwerties".into(),
                id: None,
                meta: None,
                links: None,
                attributes: None,
                relationships: None,
            }]),
        );
    }

    #[test]
    fn multiple_two() {
        assert_eq!(
            DataBuilder::Multiple(vec![
                ResourceBuilder::new("qwerties"),
                ResourceBuilder::new("foobars"),
            ])
            .unwrap(),
            Data::Multiple(vec![
                Resource {
                    type_: "qwerties".into(),
                    id: None,
                    meta: None,
                    links: None,
                    attributes: None,
                    relationships: None,
                },
                Resource {
                    type_: "foobars".into(),
                    id: None,
                    meta: None,
                    links: None,
                    attributes: None,
                    relationships: None,
                },
            ]),
        );
    }

    // TODO: 3 or 5 implicit tests
}
