use super::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DocumentBuilder {
    jsonapi: Option<JsonApiBuilder>,
    meta: Option<MetaOrAttrsBuilder>,
    links: Option<LinksBuilder>,
    data: Option<DataBuilder>,
    errors: Option<Vec<ErrorBuilder>>,
}

impl Builder<'_> for DocumentBuilder {
    type Entity = Document;

    fn finish(self) -> Result<Self::Entity, BuildErrors> {
        let errors = match self.errors {
            None => None,
            Some(errors) => {
                let mut new_errors = Vec::new();

                for error in errors {
                    new_errors.push(error.finish()?);
                }

                Some(new_errors)
            }
        };

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
            errors,
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

    pub fn meta<M: Into<MetaOrAttrsBuilder>>(self, meta: M) -> Self {
        Self {
            meta: Some(meta.into()),
            ..self
        }
    }

    pub fn links<L: Into<LinksBuilder>>(self, links: L) -> Self {
        Self {
            links: Some(links.into()),
            ..self
        }
    }

    pub fn data<D: Into<DataBuilder>>(self, data: D) -> Self {
        Self {
            data: Some(data.into()),
            ..self
        }
    }

    pub fn errors<E: Into<ErrorBuilder>>(self, errors: Vec<E>) -> Self {
        let mut new_errors = Vec::new();

        for error in errors {
            new_errors.push(error.into());
        }

        Self {
            errors: Some(new_errors),
            ..self
        }
    }

    pub fn meta1<N: ToString, V: Into<Value>>(self, name: N, meta1: V) -> Self {
        let meta = self.meta.unwrap_or_default().item(name, meta1);

        Self {
            meta: Some(meta),
            ..self
        }
    }

    pub fn link<N: ToString, L: Into<LinkBuilder>>(
        self,
        name: N,
        link: L,
    ) -> Self {
        let links = self.links.unwrap_or_default().link(name, link);

        Self {
            links: Some(links),
            ..self
        }
    }

    pub fn error<E: Into<ErrorBuilder>>(self, error: E) -> Self {
        let mut errors = self.errors.unwrap_or_default();
        errors.push(error.into());

        Self {
            errors: Some(errors),
            ..self
        }
    }
}

impl From<Document> for DocumentBuilder {
    fn from(document: Document) -> Self {
        let errors = match document.errors {
            None => None,
            Some(errors) => {
                let mut new_errors = Vec::new();

                for error in errors {
                    new_errors.push(error.into());
                }

                Some(new_errors)
            }
        };

        Self {
            jsonapi: document.jsonapi.map(|jsonapi| jsonapi.into()),
            meta: document.meta.map(|meta| meta.into()),
            links: document.links.map(|links| links.into()),
            data: document.data.map(|data| data.into()),
            errors,
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

    fn links() -> Links {
        Links {
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
            about: None,
        }
    }

    fn errors() -> Vec<Error> {
        vec![Error {
            id: Some("789".into()),
            links: None,
            status: None,
            code: None,
            title: None,
            detail: None,
            source: None,
            meta: None,
        }]
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
                errors: None,
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
                .errors(vec![ErrorBuilder::default().id("789")])
                .unwrap(),
            Document {
                jsonapi: Some(JsonApi {
                    version: Some(Version::new(456)),
                    meta: None,
                }),
                meta: Some(meta()),
                links: Some(links()),
                data: Some(Data::Single(Resource {
                    type_: "qwerties".into(),
                    id: None,
                    meta: None,
                    links: None,
                    attributes: None,
                    relationships: None,
                })),
                errors: Some(errors()),
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
                .error(ErrorBuilder::default().id("789"))
                .unwrap(),
            Document {
                jsonapi: Some(JsonApi {
                    version: Some(Version::new(456)),
                    meta: None,
                }),
                meta: Some(meta()),
                links: Some(links()),
                data: Some(Data::Single(Resource {
                    type_: "qwerties".into(),
                    id: None,
                    meta: None,
                    links: None,
                    attributes: None,
                    relationships: None,
                })),
                errors: Some(errors()),
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
                errors: None,
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
                errors: None,
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
                errors: None,
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
                    about: None,
                }),
                data: None,
                errors: None,
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
                errors: None,
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
                errors: None,
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
                errors: None,
            },
        );
    }

    #[test]
    fn with_errors() {
        assert_eq!(
            DocumentBuilder::default()
                .errors(vec![ErrorBuilder::default().id("789")])
                .unwrap(),
            Document {
                jsonapi: None,
                meta: None,
                links: None,
                data: None,
                errors: Some(errors()),
            },
        );
    }

    #[test]
    fn with_meta1_implicit() {
        assert_eq!(
            DocumentBuilder::default()
                .meta1("foo", 123)
                .meta1("bar", "car")
                .unwrap(),
            Document {
                jsonapi: None,
                meta: Some({
                    let mut meta = MetaOrAttrs::new();
                    meta.insert("foo".into(), Value::Number(123.into()));
                    meta.insert("bar".into(), Value::String("car".into()));
                    meta
                }),
                links: None,
                data: None,
                errors: None,
            },
        );
    }

    #[test]
    fn with_link_implicit_from_str() {
        assert_eq!(
            DocumentBuilder::default()
                .link("self", "http://self.com")
                .link("foo", "http://foo.com")
                .unwrap(),
            Document {
                jsonapi: None,
                meta: None,
                links: Some(Links {
                    other: {
                        let mut other = HashMap::new();
                        other.insert(
                            "foo".into(),
                            Link::String("http://foo.com".into()),
                        );
                        other
                    },
                    self_: Some(Link::String("http://self.com".into())),
                    related: None,
                    first: None,
                    last: None,
                    prev: None,
                    next: None,
                    about: None,
                }),
                data: None,
                errors: None,
            },
        );
    }

    #[test]
    fn implicit_from_entity() {
        let document = Document {
            jsonapi: Some(JsonApi {
                version: Some(Version::new(456)),
                meta: Some(meta()),
            }),
            meta: Some(meta()),
            links: Some(links()),
            data: Some(Data::Single(Resource {
                type_: "qwerties".into(),
                id: Some("123".into()),
                meta: None,
                links: None,
                attributes: None,
                relationships: None,
            })),
            errors: Some(errors()),
        };

        let builder: DocumentBuilder = document.clone().into();

        assert_eq!(builder.unwrap(), document);
    }

    #[test]
    fn with_meta_implicit_from_entity() {
        assert_eq!(
            DocumentBuilder::default().meta(meta()).unwrap(),
            Document {
                jsonapi: None,
                meta: Some(meta()),
                links: None,
                data: None,
                errors: None,
            },
        );
    }

    #[test]
    fn with_links_implicit_from_entity() {
        assert_eq!(
            DocumentBuilder::default().links(links()).unwrap(),
            Document {
                jsonapi: None,
                meta: None,
                links: Some(links()),
                data: None,
                errors: None,
            },
        );
    }

    #[test]
    fn with_errors_implicit_from_entity() {
        assert_eq!(
            DocumentBuilder::default().errors(errors()).unwrap(),
            Document {
                jsonapi: None,
                meta: None,
                links: None,
                data: None,
                errors: Some(errors()),
            },
        );
    }
}
