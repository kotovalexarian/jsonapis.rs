use super::*;

use std::collections::HashMap;

use serde_json::{json, Value};

/*****************
 * meta_or_attrs *
 *****************/

pub fn meta_or_attrs() -> MetaOrAttrs {
    let mut meta_or_attrs: MetaOrAttrs = MetaOrAttrs::new();
    meta_or_attrs.insert("foo".into(), Value::Number(123.into()));
    meta_or_attrs.insert("bar".into(), Value::String("qwe".into()));
    meta_or_attrs
}

pub fn meta_or_attrs_value() -> Value {
    json!({
        "foo": json!(123),
        "bar": json!("qwe"),
    })
}

/***********************
 * simple_links        *
 *                     *
 * For easier builders *
 ***********************/

pub fn simple_links() -> Links {
    Links {
        other: {
            let mut other = HashMap::new();
            other.insert("qwe".into(), Link::String("http://qwe.com".into()));
            other
        },
        self_: Some(Link::String("http://self.com".into())),
        related: None,
        first: None,
        last: None,
        prev: None,
        next: None,
        about: None,
    }
}

/****************************
 * different_links          *
 *                          *
 * - one common link string *
 * - one common link object *
 * - one custom link string *
 * - one custom link object *
 ****************************/

pub fn different_links() -> Links {
    let mut links: Links = Links {
        other: HashMap::new(),
        self_: Some(Link::String("http://example.com".into())),
        related: None,
        first: None,
        last: None,
        prev: None,
        next: Some(Link::Object(LinkObject {
            href: "http://example.com".into(),
            meta: Some(meta_or_attrs()),
        })),
        about: None,
    };

    links
        .other
        .insert("foo".into(), Link::String("http://foo.com".into()));
    links.other.insert(
        "bar".into(),
        Link::Object(LinkObject {
            href: "http://bar.com".into(),
            meta: Some(meta_or_attrs()),
        }),
    );

    links
}

pub fn different_links_value() -> Value {
    json!({
        "self": json!("http://example.com"),
        "related": json!(null),
        "first": json!(null),
        "last": json!(null),
        "prev": json!(null),
        "next": json!({
            "href": json!("http://example.com"),
            "meta": meta_or_attrs_value(),
        }),
        "foo": json!("http://foo.com"),
        "bar": json!({
            "href": json!("http://bar.com"),
            "meta": meta_or_attrs_value(),
        }),
        "about": json!(null),
    })
}
