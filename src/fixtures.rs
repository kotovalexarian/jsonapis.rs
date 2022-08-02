use super::*;

use serde_json::{json, Value};

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
