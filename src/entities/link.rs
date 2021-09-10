use super::*;

impl Entity for Link {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Link {
    String(String),
    Object(LinkObject),
}

impl Serialize for Link {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::String(string) => serializer.serialize_str(string),
            Self::Object(object) => object.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Link {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyVisitor;

        impl<'de> Visitor<'de> for MyVisitor {
            type Value = Link;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("JSON API link")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Link::String(value.to_string()))
            }

            fn visit_map<A>(self, value: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                Ok(Link::Object(Deserialize::deserialize(
                    serde::de::value::MapAccessDeserializer::new(value),
                )?))
            }
        }

        deserializer.deserialize_any(MyVisitor)
    }
}
