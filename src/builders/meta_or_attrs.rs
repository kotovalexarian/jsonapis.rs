use super::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MetaOrAttrsBuilder(MetaOrAttrs);

impl Builder<'_> for MetaOrAttrsBuilder {
    type Entity = MetaOrAttrs;

    fn finish(self) -> Result<Self::Entity, BuildErrors> {
        Ok(self.0)
    }
}

impl MetaOrAttrsBuilder {
    pub fn item<N: ToString, V: Into<Value>>(self, name: N, value: V) -> Self {
        let mut meta_or_attrs = self.0;
        meta_or_attrs.insert(name.to_string(), value.into());
        Self(meta_or_attrs)
    }
}

impl From<MetaOrAttrs> for MetaOrAttrsBuilder {
    fn from(meta_or_attrs: MetaOrAttrs) -> Self {
        Self(meta_or_attrs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(MetaOrAttrsBuilder::default().unwrap(), MetaOrAttrs::new(),);
    }

    #[test]
    fn full() {
        assert_eq!(
            MetaOrAttrsBuilder::default()
                .item("false", false)
                .item("true", true)
                .item("int", 123)
                .item("float", 3.1415926536)
                .item("str", "Hello, World!")
                .item("array", vec![1, 2, 3])
                .item("object", {
                    let mut object = serde_json::Map::new();
                    object.insert("foo".into(), Value::Number(123.into()));
                    Value::Object(object)
                })
                .unwrap(),
            {
                let mut meta_or_attrs = MetaOrAttrs::new();
                meta_or_attrs.insert("false".into(), Value::Bool(false));
                meta_or_attrs.insert("true".into(), Value::Bool(true));
                meta_or_attrs.insert("int".into(), Value::Number(123.into()));
                meta_or_attrs.insert(
                    "float".into(),
                    Value::Number(
                        serde_json::Number::from_f64(3.1415926536).unwrap(),
                    ),
                );
                meta_or_attrs.insert(
                    "str".into(),
                    Value::String("Hello, World!".into()),
                );
                meta_or_attrs.insert(
                    "array".into(),
                    vec![
                        Value::Number(1.into()),
                        Value::Number(2.into()),
                        Value::Number(3.into()),
                    ]
                    .into(),
                );
                meta_or_attrs.insert("object".into(), {
                    let mut object = serde_json::Map::new();
                    object.insert("foo".into(), Value::Number(123.into()));
                    Value::Object(object)
                });
                meta_or_attrs
            }
        );
    }

    #[test]
    fn implicit_from_entity() {
        let mut meta_or_attrs = MetaOrAttrs::new();
        meta_or_attrs.insert("foo".into(), 123.into());
        meta_or_attrs.insert("bar".into(), "qwe".into());

        let builder: MetaOrAttrsBuilder = meta_or_attrs.clone().into();

        assert_eq!(builder.unwrap(), meta_or_attrs);
    }
}
