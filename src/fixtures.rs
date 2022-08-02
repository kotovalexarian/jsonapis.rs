use super::*;

use serde_json::{json, Value};

pub fn meta_or_attrs() -> MetaOrAttrs {
    let mut expected_meta_or_attrs: MetaOrAttrs = MetaOrAttrs::new();
    expected_meta_or_attrs.insert("foo".into(), Value::Number(123.into()));
    expected_meta_or_attrs.insert("bar".into(), Value::String("qwe".into()));
    expected_meta_or_attrs
}

pub fn meta_or_attrs_value() -> Value {
    json!({
        "foo": json!(123),
        "bar": json!("qwe"),
    })
}
