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

/***********************
 * simple_error[s]     *
 *                     *
 * For easier builders *
 ***********************/

pub fn simple_errors() -> Errors {
    vec![simple_error()]
}

pub fn simple_error() -> ErrorObject {
    ErrorObject {
        id: Some("789".into()),
        links: None,
        status: None,
        code: None,
        title: None,
        detail: None,
        source: None,
        meta: None,
    }
}

/*******************
 * full_error[s]   *
 *                 *
 * With all fields *
 *******************/

pub fn full_errors() -> Errors {
    vec![full_error()]
}

pub fn full_errors_value() -> Value {
    json!([full_error_value()])
}

pub fn full_error() -> ErrorObject {
    ErrorObject {
        id: Some("789".into()),
        links: Some(different_links()),
        status: Some(HttpStatus::OK),
        code: Some("some code".into()),
        title: Some("some title".into()),
        detail: Some("some detail".into()),
        source: Some(ErrorSource {
            pointer: Some("/foo/0/bar/1".into()),
            parameter: Some("car".into()),
        }),
        meta: Some(meta_or_attrs()),
    }
}

pub fn full_error_value() -> Value {
    json!({
        "id": json!("789"),
        "links": different_links_value(),
        "status": json!("200"),
        "code": json!("some code"),
        "title": json!("some title"),
        "detail": json!("some detail"),
        "source": json!({
            "pointer": json!("/foo/0/bar/1"),
            "parameter": json!("car"),
        }),
        "meta": meta_or_attrs_value(),
    })
}
