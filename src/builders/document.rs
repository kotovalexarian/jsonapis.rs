use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocumentBuilder {
    jsonapi: Option<JsonApiBuilder>,
    meta: Option<MetaOrAttrsBuilder>,
    links: Option<LinksBuilder>,
    data: Option<DataBuilder>,
}

impl Default for DocumentBuilder {
    fn default() -> Self {
        Self {
            jsonapi: None,
            meta: None,
            links: None,
            data: None,
        }
    }
}

impl Builder for DocumentBuilder {
    type Entity = Document;

    fn finish(self) -> Result<Self::Entity, ()> {
        Ok(Self::Entity {
            jsonapi: match self.jsonapi {
                None => None,
                Some(jsonapi) => Some(jsonapi.finish()?),
            },
            meta: match self.meta {
                None => None,
                Some(meta) => Some(meta.finish()?),
            },
            links: match self.links {
                None => None,
                Some(links) => Some(links.finish()?),
            },
            data: match self.data {
                None => None,
                Some(data) => Some(data.finish()?),
            },
        })
    }
}

impl DocumentBuilder {
    pub fn jsonapi<J: Into<JsonApiBuilder>>(self, jsonapi: J) -> Self {
        Self {
            jsonapi: Some(jsonapi.into()),
            ..self
        }
    }

    pub fn meta(self, meta: MetaOrAttrsBuilder) -> Self {
        Self {
            meta: Some(meta),
            ..self
        }
    }

    pub fn links(self, links: LinksBuilder) -> Self {
        Self {
            links: Some(links),
            ..self
        }
    }

    pub fn data<D: Into<DataBuilder>>(self, data: D) -> Self {
        Self {
            data: Some(data.into()),
            ..self
        }
    }

    pub fn meta1<V: Into<Value>>(self, name: &str, meta1: V) -> Self {
        let meta = self
            .meta
            .unwrap_or(MetaOrAttrsBuilder::default())
            .item(name, meta1);

        Self {
            meta: Some(meta),
            ..self
        }
    }

    pub fn link<L: Into<LinkBuilder>>(self, name: &str, link: L) -> Self {
        let links = self
            .links
            .unwrap_or(LinksBuilder::default())
            .link(name, link);

        Self {
            links: Some(links),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn meta() -> MetaOrAttrs {
        let mut meta = MetaOrAttrs::new();
        meta.insert("foo".into(), 123.into());
        meta.insert("bar".into(), "qwe".into());
        meta
    }

    #[test]
    fn empty() {
        assert_eq!(
            DocumentBuilder::default().unwrap(),
            Document {
                jsonapi: None,
                meta: None,
                links: None,
                data: None,
            },
        );
    }

    #[test]
    fn full() {
        assert_eq!(
            DocumentBuilder::default()
                .jsonapi(JsonApiBuilder::default().version(Version::new(456)))
                .meta(
                    MetaOrAttrsBuilder::default()
                        .item("foo", 123)
                        .item("bar", "qwe"),
                )
                .links(
                    LinksBuilder::default()
                        .self_(LinkBuilder::new("http://self.com"))
                        .link("qwe", LinkBuilder::new("http://qwe.com")),
                )
                .data(DataBuilder::Single(ResourceBuilder::new("qwerties")))
                .unwrap(),
            Document {
                jsonapi: Some(JsonApi {
                    version: Some(Version::new(456)),
                    meta: None,
                }),
                meta: Some(meta()),
                links: Some(Links {
                    other: {
                        let mut other = HashMap::new();
                        other.insert(
                            "qwe".into(),
                            Link::String("http://qwe.com".into()),
                        );
                        other
                    },
                    self_: Some(Link::String("http://self.com".into())),
                    related: None,
                    first: None,
                    last: None,
                    prev: None,
                    next: None,
                }),
                data: Some(Data::Single(Resource {
                    type_: "qwerties".into(),
                    id: None,
                    meta: None,
                    links: None,
                    attributes: None,
                    relationships: None,
                })),
            },
        );
    }

    #[test]
    fn full_delegators() {
        assert_eq!(
            DocumentBuilder::default()
                .jsonapi(JsonApiBuilder::default().version(Version::new(456)))
                .meta1("foo", 123)
                .meta1("bar", "qwe")
                .link("self", LinkBuilder::new("http://self.com"))
                .link("qwe", LinkBuilder::new("http://qwe.com"))
                .data(DataBuilder::Single(ResourceBuilder::new("qwerties")))
                .unwrap(),
            Document {
                jsonapi: Some(JsonApi {
                    version: Some(Version::new(456)),
                    meta: None,
                }),
                meta: Some(meta()),
                links: Some(Links {
                    other: {
                        let mut other = HashMap::new();
                        other.insert(
                            "qwe".into(),
                            Link::String("http://qwe.com".into()),
                        );
                        other
                    },
                    self_: Some(Link::String("http://self.com".into())),
                    related: None,
                    first: None,
                    last: None,
                    prev: None,
                    next: None,
                }),
                data: Some(Data::Single(Resource {
                    type_: "qwerties".into(),
                    id: None,
                    meta: None,
                    links: None,
                    attributes: None,
                    relationships: None,
                })),
            },
        );
    }

    #[test]
    fn with_jsonapi() {
        assert_eq!(
            DocumentBuilder::default()
                .jsonapi(JsonApiBuilder::default().version(Version::new(456)))
                .unwrap(),
            Document {
                jsonapi: Some(JsonApi {
                    version: Some(Version::new(456)),
                    meta: None,
                }),
                meta: None,
                links: None,
                data: None,
            },
        );
    }

    #[test]
    fn with_jsonapi_implicit_from_version() {
        assert_eq!(
            DocumentBuilder::default()
                .jsonapi(Version::new(456))
                .unwrap(),
            Document {
                jsonapi: Some(JsonApi {
                    version: Some(Version::new(456)),
                    meta: None,
                }),
                meta: None,
                links: None,
                data: None,
            },
        );
    }

    #[test]
    fn with_meta() {
        assert_eq!(
            DocumentBuilder::default()
                .meta(
                    MetaOrAttrsBuilder::default()
                        .item("foo", 123)
                        .item("bar", "qwe"),
                )
                .unwrap(),
            Document {
                jsonapi: None,
                meta: Some(meta()),
                links: None,
                data: None,
            },
        );
    }

    #[test]
    fn with_links() {
        assert_eq!(
            DocumentBuilder::default()
                .links(
                    LinksBuilder::default()
                        .self_(LinkBuilder::new("http://self.com"))
                        .prev(
                            LinkBuilder::new("http://prev.com").meta(
                                MetaOrAttrsBuilder::default()
                                    .item("foo", 123)
                                    .item("bar", "qwe"),
                            ),
                        ),
                )
                .unwrap(),
            Document {
                jsonapi: None,
                meta: None,
                links: Some(Links {
                    other: HashMap::new(),
                    self_: Some(Link::String("http://self.com".into())),
                    related: None,
                    first: None,
                    last: None,
                    prev: Some(Link::Object(LinkObject {
                        href: "http://prev.com".into(),
                        meta: Some(meta()),
                    })),
                    next: None,
                }),
                data: None,
            },
        );
    }

    #[test]
    fn with_data() {
        assert_eq!(
            DocumentBuilder::default()
                .data(DataBuilder::Multiple(vec![ResourceBuilder::new(
                    "qwerties"
                )]))
                .unwrap(),
            Document {
                jsonapi: None,
                meta: None,
                links: None,
                data: Some(Data::Multiple(vec![Resource {
                    type_: "qwerties".into(),
                    id: None,
                    meta: None,
                    links: None,
                    attributes: None,
                    relationships: None,
                }])),
            },
        );
    }

    #[test]
    fn with_data_from_resource() {
        assert_eq!(
            DocumentBuilder::default()
                .data(ResourceBuilder::new("qwerties"))
                .unwrap(),
            Document {
                jsonapi: None,
                meta: None,
                links: None,
                data: Some(Data::Single(Resource {
                    type_: "qwerties".into(),
                    id: None,
                    meta: None,
                    links: None,
                    attributes: None,
                    relationships: None,
                })),
            },
        );
    }

    #[test]
    fn with_data_from_resources() {
        assert_eq!(
            DocumentBuilder::default()
                .data(vec![ResourceBuilder::new("qwerties")])
                .unwrap(),
            Document {
                jsonapi: None,
                meta: None,
                links: None,
                data: Some(Data::Multiple(vec![Resource {
                    type_: "qwerties".into(),
                    id: None,
                    meta: None,
                    links: None,
                    attributes: None,
                    relationships: None,
                }])),
            },
        );
    }
}
