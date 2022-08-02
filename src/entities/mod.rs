mod data;
mod document;
mod error_source;
mod http_status;
mod jsonapi;
mod link;
mod link_object;
mod links;
mod meta_or_attrs;
mod relationship;
mod relationships;
mod resource;
mod version;

pub use data::Data;
pub use document::Document;
pub use error_source::ErrorSource;
pub use http_status::HttpStatus;
pub use jsonapi::JsonApi;
pub use link::Link;
pub use link_object::LinkObject;
pub use links::Links;
pub use meta_or_attrs::MetaOrAttrs;
pub use relationship::Relationship;
pub use relationships::Relationships;
pub use resource::Resource;
pub use version::Version;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

use serde::{
    de::Visitor,
    ser::{SerializeMap, Serializer},
    Deserialize, Deserializer, Serialize,
};
use serde_json::Value;

pub trait Entity<'de>:
    Clone + Debug + Deserialize<'de> + Eq + PartialEq + Serialize + Sized
{
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    fn expected_meta_or_attrs() -> MetaOrAttrs {
        let mut expected_meta_or_attrs: MetaOrAttrs = MetaOrAttrs::new();
        expected_meta_or_attrs.insert("foo".into(), Value::Number(123.into()));
        expected_meta_or_attrs
            .insert("bar".into(), Value::String("qwe".into()));
        expected_meta_or_attrs
    }

    fn expected_meta_or_attrs_value() -> Value {
        json!({
            "foo": json!(123),
            "bar": json!("qwe"),
        })
    }

    fn expected_links() -> Links {
        let mut expected_links: Links = Links {
            other: HashMap::new(),
            self_: Some(Link::String("http://example.com".into())),
            related: None,
            first: None,
            last: None,
            prev: None,
            next: Some(Link::Object(LinkObject {
                href: "http://example.com".into(),
                meta: Some(expected_meta_or_attrs()),
            })),
            about: None,
        };

        expected_links
            .other
            .insert("foo".into(), Link::String("http://foo.com".into()));
        expected_links.other.insert(
            "bar".into(),
            Link::Object(LinkObject {
                href: "http://bar.com".into(),
                meta: Some(expected_meta_or_attrs()),
            }),
        );

        expected_links
    }

    fn expected_links_value() -> Value {
        json!({
            "self": json!("http://example.com"),
            "related": json!(null),
            "first": json!(null),
            "last": json!(null),
            "prev": json!(null),
            "next": json!({
                "href": json!("http://example.com"),
                "meta": expected_meta_or_attrs_value(),
            }),
            "foo": json!("http://foo.com"),
            "bar": json!({
                "href": json!("http://bar.com"),
                "meta": expected_meta_or_attrs_value(),
            }),
            "about": json!(null),
        })
    }

    fn expected_relationships() -> Relationships {
        let mut expected_relationships: Relationships = Relationships::new();
        expected_relationships.insert(
            "car".into(),
            Relationship {
                meta: None,
                links: None,
                data: None,
            },
        );
        expected_relationships.insert(
            "cdr".into(),
            Relationship {
                meta: Some(expected_meta_or_attrs()),
                links: Some(expected_links()),
                data: Some(Data::Single(Resource {
                    type_: "qwerties".into(),
                    id: Some("123".into()),
                    meta: Some(expected_meta_or_attrs()),
                    links: Some(expected_links()),
                    attributes: Some(expected_meta_or_attrs()),
                    relationships: None,
                })),
            },
        );
        expected_relationships
    }

    fn expected_relationships_value() -> Value {
        json!({
            "car": json!({
                "meta": json!(null),
                "links": json!(null),
                "data": json!(null),
            }),
            "cdr": json!({
                "meta": expected_meta_or_attrs_value(),
                "links": expected_links_value(),
                "data": json!({
                    "type": json!("qwerties"),
                    "id": json!("123"),
                    "meta": expected_meta_or_attrs_value(),
                    "links": expected_links_value(),
                    "attributes": expected_meta_or_attrs_value(),
                    "relationships": json!(null),
                }),
            }),
        })
    }

    #[test]
    fn serialize_and_deserialize() {
        let document = Document {
            jsonapi: Some(JsonApi {
                version: Some(Version::new(0)),
                meta: Some(expected_meta_or_attrs()),
            }),
            meta: Some(expected_meta_or_attrs()),
            links: Some(expected_links()),
            data: Some(Data::Multiple(vec![Resource {
                type_: "qwerties".into(),
                id: Some("123".into()),
                meta: Some(expected_meta_or_attrs()),
                links: Some(expected_links()),
                attributes: Some(expected_meta_or_attrs()),
                relationships: Some(expected_relationships()),
            }])),
        };

        let serialized = serde_json::to_string(&document).unwrap();

        let deserialized: Document = serde_json::from_str(&serialized).unwrap();

        assert_eq!(document, deserialized);
    }

    mod deserialize {
        use super::*;

        #[test]
        fn default() {
            let expected_document = Document {
                jsonapi: Some(JsonApi {
                    version: Some(Version::new(0)),
                    meta: Some(expected_meta_or_attrs()),
                }),
                meta: Some(expected_meta_or_attrs()),
                links: Some(expected_links()),
                data: None,
            };

            let value = json!({
                "jsonapi": json!({
                    "version": json!("1.0"),
                    "meta": expected_meta_or_attrs_value(),
                }),
                "meta": expected_meta_or_attrs_value(),
                "links": expected_links_value(),
            });

            let json = serde_json::to_string(&value).unwrap();

            let document: Document = serde_json::from_str(&json).unwrap();

            assert_eq!(document, expected_document);
        }

        #[test]
        fn data_as_null() {
            let expected_document = Document {
                jsonapi: None,
                meta: None,
                links: None,
                data: None,
            };

            let json = "{\"data\": null}";

            let document: Document = serde_json::from_str(json).unwrap();

            assert_eq!(document, expected_document);
        }

        #[test]
        fn data_as_empty_array() {
            let expected_document = Document {
                jsonapi: None,
                meta: None,
                links: None,
                data: Some(Data::Multiple(vec![])),
            };

            let json = "{\"data\": []}";

            let document: Document = serde_json::from_str(json).unwrap();

            assert_eq!(document, expected_document);
        }

        #[test]
        fn data_as_single_resource() {
            let expected_document = Document {
                jsonapi: None,
                meta: None,
                links: None,
                data: Some(Data::Single(Resource {
                    type_: "qwerties".into(),
                    id: Some("123".into()),
                    meta: Some(expected_meta_or_attrs()),
                    links: Some(expected_links()),
                    attributes: Some(expected_meta_or_attrs()),
                    relationships: Some(expected_relationships()),
                })),
            };

            let value = json!({
                "data": json!({
                    "type": json!("qwerties"),
                    "id": json!("123"),
                    "meta": expected_meta_or_attrs_value(),
                    "links": expected_links_value(),
                    "attributes": expected_meta_or_attrs_value(),
                    "relationships": expected_relationships_value(),
                }),
            });

            let json = serde_json::to_string(&value).unwrap();

            let document: Document = serde_json::from_str(&json).unwrap();

            assert_eq!(document, expected_document);
        }

        #[test]
        fn data_as_multiple_resources() {
            let expected_document = Document {
                jsonapi: None,
                meta: None,
                links: None,
                data: Some(Data::Multiple(vec![Resource {
                    type_: "qwerties".into(),
                    id: Some("123".into()),
                    meta: Some(expected_meta_or_attrs()),
                    links: Some(expected_links()),
                    attributes: Some(expected_meta_or_attrs()),
                    relationships: Some(expected_relationships()),
                }])),
            };

            let value = json!({
                "data": json!([
                    json!({
                        "type": json!("qwerties"),
                        "id": json!("123"),
                        "meta": expected_meta_or_attrs_value(),
                        "links": expected_links_value(),
                        "attributes": expected_meta_or_attrs_value(),
                        "relationships": expected_relationships_value(),
                    }),
                ]),
            });

            let json = serde_json::to_string(&value).unwrap();

            let document: Document = serde_json::from_str(&json).unwrap();

            assert_eq!(document, expected_document);
        }
    }

    mod serialize {
        use super::*;

        #[test]
        fn empty() {
            let document = Document {
                jsonapi: None,
                meta: None,
                links: None,
                data: None,
            };

            let json = serde_json::to_string(&document).unwrap();

            let value: Value = serde_json::from_str(&json).unwrap();

            assert_eq!(
                value,
                json!({
                    "jsonapi": json!(null),
                    "meta": json!(null),
                    "links": json!(null),
                    "data": json!(null),
                })
            );
        }

        #[test]
        fn default() {
            let document = Document {
                jsonapi: Some(JsonApi {
                    version: Some(Version::new(0)),
                    meta: Some(expected_meta_or_attrs()),
                }),
                meta: Some(expected_meta_or_attrs()),
                links: Some(expected_links()),
                data: Some(Data::Multiple(vec![Resource {
                    type_: "qwerties".into(),
                    id: Some("123".into()),
                    meta: Some(expected_meta_or_attrs()),
                    links: Some(expected_links()),
                    attributes: Some(expected_meta_or_attrs()),
                    relationships: Some(expected_relationships()),
                }])),
            };

            let json = serde_json::to_string(&document).unwrap();

            let value: Value = serde_json::from_str(&json).unwrap();

            assert_eq!(
                value,
                json!({
                    "jsonapi": json!({
                        "version": json!("1.0"),
                        "meta": expected_meta_or_attrs_value(),
                    }),
                    "meta": expected_meta_or_attrs_value(),
                    "links": expected_links_value(),
                    "data": json!([
                        json!({
                            "type": json!("qwerties"),
                            "id": json!("123"),
                            "meta": expected_meta_or_attrs_value(),
                            "links": expected_links_value(),
                            "attributes": expected_meta_or_attrs_value(),
                            "relationships": expected_relationships_value(),
                        }),
                    ]),
                })
            );
        }

        #[test]
        fn links_empty() {
            let links = Links {
                other: HashMap::new(),
                self_: None,
                related: None,
                first: None,
                last: None,
                prev: None,
                next: None,
                about: None,
            };

            let json = serde_json::to_string(&links).unwrap();

            let value: Value = serde_json::from_str(&json).unwrap();

            assert_eq!(
                value,
                json!({
                    "self": json!(null),
                    "related": json!(null),
                    "first": json!(null),
                    "last": json!(null),
                    "prev": json!(null),
                    "next": json!(null),
                    "about": json!(null),
                })
            );
        }

        #[test]
        fn links_default() {
            let links = Links {
                other: {
                    let mut other = HashMap::new();
                    other.insert(
                        "foo".into(),
                        Link::String("http://foo.com".into()),
                    );
                    other.insert(
                        "bar".into(),
                        Link::Object(LinkObject {
                            href: "http://bar.com".into(),
                            meta: None,
                        }),
                    );
                    other
                },
                self_: Some(Link::String("http://self.com".into())),
                related: Some(Link::String("http://related.com".into())),
                first: Some(Link::Object(LinkObject {
                    href: "http://first.com".into(),
                    meta: None,
                })),
                last: Some(Link::String("http://last.com".into())),
                prev: Some(Link::Object(LinkObject {
                    href: "http://prev.com".into(),
                    meta: Some({
                        let mut meta = HashMap::new();
                        meta.insert("qwerty".into(), json!(123456));
                        meta
                    }),
                })),
                next: Some(Link::String("http://next.com".into())),
                about: Some(Link::String("http://about.com".into())),
            };

            let json = serde_json::to_string(&links).unwrap();

            let value: Value = serde_json::from_str(&json).unwrap();

            assert_eq!(
                value,
                json!({
                    "self": json!("http://self.com"),
                    "related": json!("http://related.com"),
                    "first": json!({
                        "href": json!("http://first.com"),
                        "meta": json!(null),
                    }),
                    "last": json!("http://last.com"),
                    "prev": json!({
                        "href": json!("http://prev.com"),
                        "meta": json!({ "qwerty": json!(123456) }),
                    }),
                    "next": json!("http://next.com"),
                    "about": json!("http://about.com"),
                    "foo": json!("http://foo.com"),
                    "bar": json!({
                        "href": json!("http://bar.com"),
                        "meta": json!(null),
                    }),
                })
            );
        }

        #[test]
        fn resource_empty() {
            let resource = Resource {
                type_: "qwerties".into(),
                id: None,
                meta: None,
                links: None,
                attributes: None,
                relationships: None,
            };

            let json = serde_json::to_string(&resource).unwrap();

            let value: Value = serde_json::from_str(&json).unwrap();

            assert_eq!(
                value,
                json!({
                    "type": json!("qwerties"),
                    "id": json!(null),
                    "meta": json!(null),
                    "links": json!(null),
                    "attributes": json!(null),
                    "relationships": json!(null),
                })
            );
        }
    }
}
