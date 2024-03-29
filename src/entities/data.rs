use super::*;

impl Entity<'_> for Data {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Data {
    Single(Resource),
    Multiple(Vec<Resource>),
}

impl Serialize for Data {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Single(single) => single.serialize(serializer),
            Self::Multiple(multiple) => multiple.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Data {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyVisitor;

        impl<'de> Visitor<'de> for MyVisitor {
            type Value = Data;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("JSON API data")
            }

            fn visit_map<A>(self, value: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                Ok(Data::Single(Deserialize::deserialize(
                    serde::de::value::MapAccessDeserializer::new(value),
                )?))
            }

            fn visit_seq<A>(self, value: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                Ok(Data::Multiple(Deserialize::deserialize(
                    serde::de::value::SeqAccessDeserializer::new(value),
                )?))
            }
        }

        deserializer.deserialize_any(MyVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", Data::Multiple(vec![])), "Multiple([])",);

        assert_eq!(
            format!(
                "{:?}",
                Data::Single(Resource {
                    type_: "qwerties".into(),
                    id: None,
                    meta: None,
                    links: None,
                    attributes: None,
                    relationships: None,
                })
            ),
            "Single(Resource { \
                type_: \"qwerties\", \
                id: None, \
                meta: None, \
                links: None, \
                attributes: None, \
                relationships: None \
            })",
        );

        assert_eq!(
            format!(
                "{:?}",
                Data::Multiple(vec![Resource {
                    type_: "qwerties".into(),
                    id: None,
                    meta: None,
                    links: None,
                    attributes: None,
                    relationships: None,
                }]),
            ),
            "Multiple([Resource { \
                type_: \"qwerties\", \
                id: None, \
                meta: None, \
                links: None, \
                attributes: None, \
                relationships: None \
            }])",
        );
    }

    #[test]
    fn equality() {
        assert_eq!(Data::Multiple(vec![]), Data::Multiple(vec![]));

        assert_eq!(
            Data::Single(Resource {
                type_: "qwerties".into(),
                id: None,
                meta: None,
                links: None,
                attributes: None,
                relationships: None,
            }),
            Data::Single(Resource {
                type_: "qwerties".into(),
                id: None,
                meta: None,
                links: None,
                attributes: None,
                relationships: None,
            }),
        );

        assert_eq!(
            Data::Multiple(vec![Resource {
                type_: "qwerties".into(),
                id: None,
                meta: None,
                links: None,
                attributes: None,
                relationships: None,
            }]),
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
}
