use super::*;

impl Entity<'_> for Links {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Links {
    pub other: HashMap<String, Link>,
    // Basic (https://jsonapi.org/format/#document-links)
    pub self_: Option<Link>,
    pub related: Option<Link>,
    // Pagination (https://jsonapi.org/format/#fetching-pagination)
    pub first: Option<Link>,
    pub last: Option<Link>,
    pub prev: Option<Link>,
    pub next: Option<Link>,
    // Errors (https://jsonapi.org/format/#error-objects)
    pub about: Option<Link>,
}

impl Serialize for Links {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut all: HashMap<String, Option<Link>> = HashMap::new();

        for (key, value) in &self.other {
            all.insert(key.into(), Some(value.clone()));
        }

        // Basic
        all.insert("self".into(), self.self_.clone());
        all.insert("related".into(), self.related.clone());
        // Pagination
        all.insert("first".into(), self.first.clone());
        all.insert("last".into(), self.last.clone());
        all.insert("prev".into(), self.prev.clone());
        all.insert("next".into(), self.next.clone());
        // Errors
        all.insert("about".into(), self.about.clone());

        let mut map = serializer.serialize_map(Some(all.len()))?;

        for (key, value) in all {
            map.serialize_entry(&key, &value)?;
        }

        map.end()
    }
}

impl<'de> Deserialize<'de> for Links {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyVisitor;

        impl<'de> Visitor<'de> for MyVisitor {
            type Value = Links;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("JSON API links")
            }

            fn visit_map<A>(self, value: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                match Deserialize::deserialize(
                    serde::de::value::MapAccessDeserializer::new(value),
                ) {
                    Err(err) => Err(err),
                    Ok(all) => {
                        let mut all: HashMap<String, Option<Link>> = all;

                        // Basic
                        let self_: Option<Option<Link>> = all.remove("self");
                        let related: Option<Option<Link>> =
                            all.remove("related");
                        // Pagination
                        let first: Option<Option<Link>> = all.remove("first");
                        let last: Option<Option<Link>> = all.remove("last");
                        let prev: Option<Option<Link>> = all.remove("prev");
                        let next: Option<Option<Link>> = all.remove("next");
                        // Errors
                        let about: Option<Option<Link>> = all.remove("about");

                        let mut other: HashMap<String, Link> = HashMap::new();

                        for (key, value) in all {
                            if let Some(value) = value {
                                other.insert(key, value);
                            }
                        }

                        Ok(Self::Value {
                            other,
                            // Basic
                            self_: self_.unwrap_or(None),
                            related: related.unwrap_or(None),
                            // Pagination
                            first: first.unwrap_or(None),
                            last: last.unwrap_or(None),
                            prev: prev.unwrap_or(None),
                            next: next.unwrap_or(None),
                            // Errors
                            about: about.unwrap_or(None),
                        })
                    }
                }
            }
        }

        deserializer.deserialize_map(MyVisitor)
    }
}
