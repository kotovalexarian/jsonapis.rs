use super::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ErrorBuilder {
    id: Option<String>,
    links: Option<LinksBuilder>,
    status: Option<HttpStatus>,
    code: Option<String>,
    title: Option<String>,
    detail: Option<String>,
    source: Option<ErrorSourceBuilder>,
    meta: Option<MetaOrAttrsBuilder>,
}

impl Builder<'_> for ErrorBuilder {
    type Entity = Error;

    fn finish(self) -> Result<Self::Entity, BuildErrors> {
        Ok(Self::Entity {
            id: self.id,
            links: match self.links {
                None => None,
                Some(links) => Some(links.finish()?),
            },
            status: self.status,
            code: self.code,
            title: self.title,
            detail: self.detail,
            source: match self.source {
                None => None,
                Some(source) => Some(source.finish()?),
            },
            meta: match self.meta {
                None => None,
                Some(meta) => Some(meta.finish()?),
            },
        })
    }
}

impl ErrorBuilder {
    pub fn id<I: ToString>(self, id: I) -> Self {
        Self {
            id: Some(id.to_string()),
            ..self
        }
    }

    pub fn links<L: Into<LinksBuilder>>(self, links: L) -> Self {
        Self {
            links: Some(links.into()),
            ..self
        }
    }

    pub fn status<S: Into<HttpStatus>>(self, status: S) -> Self {
        Self {
            status: Some(status.into()),
            ..self
        }
    }

    pub fn code<C: ToString>(self, code: C) -> Self {
        Self {
            code: Some(code.to_string()),
            ..self
        }
    }

    pub fn title<T: ToString>(self, title: T) -> Self {
        Self {
            title: Some(title.to_string()),
            ..self
        }
    }

    pub fn detail<D: ToString>(self, detail: D) -> Self {
        Self {
            detail: Some(detail.to_string()),
            ..self
        }
    }

    pub fn source<S: Into<ErrorSourceBuilder>>(self, source: S) -> Self {
        Self {
            source: Some(source.into()),
            ..self
        }
    }

    pub fn meta<M: Into<MetaOrAttrsBuilder>>(self, meta: M) -> Self {
        Self {
            meta: Some(meta.into()),
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

    pub fn pointer<P: ToString>(self, pointer: P) -> Self {
        let source = self.source.unwrap_or_default().pointer(pointer);

        Self {
            source: Some(source),
            ..self
        }
    }

    pub fn parameter<P: ToString>(self, parameter: P) -> Self {
        let source = self.source.unwrap_or_default().parameter(parameter);

        Self {
            source: Some(source),
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
}

impl From<Error> for ErrorBuilder {
    fn from(error: Error) -> Self {
        Self {
            id: error.id,
            links: error.links.map(|links| links.into()),
            status: error.status,
            code: error.code,
            title: error.title,
            detail: error.detail,
            source: error.source.map(|source| source.into()),
            meta: error.meta.map(|meta| meta.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixtures;

    #[test]
    fn empty() {
        assert_eq!(
            ErrorBuilder::default().unwrap(),
            Error {
                id: None,
                links: None,
                status: None,
                code: None,
                title: None,
                detail: None,
                source: None,
                meta: None,
            },
        );
    }

    #[test]
    fn full() {
        assert_eq!(
            ErrorBuilder::default()
                .id("123")
                .links(
                    LinksBuilder::default()
                        .self_(LinkBuilder::new("http://self.com"))
                        .link("qwe", LinkBuilder::new("http://qwe.com")),
                )
                .status(HttpStatus::OK)
                .code("some code")
                .title("some title")
                .detail("some detail")
                .source(
                    ErrorSourceBuilder::default()
                        .pointer("/foo/0/bar/1")
                        .parameter("car"),
                )
                .meta(
                    MetaOrAttrsBuilder::default()
                        .item("foo", 123)
                        .item("bar", "qwe"),
                )
                .unwrap(),
            Error {
                id: Some("123".into()),
                links: Some(fixtures::simple_links()),
                status: Some(HttpStatus::OK),
                code: Some("some code".into()),
                title: Some("some title".into()),
                detail: Some("some detail".into()),
                source: Some(ErrorSource {
                    pointer: Some("/foo/0/bar/1".into()),
                    parameter: Some("car".into()),
                }),
                meta: Some(fixtures::meta_or_attrs()),
            },
        );
    }

    #[test]
    fn full_delegators() {
        assert_eq!(
            ErrorBuilder::default()
                .id("123")
                .link("self", LinkBuilder::new("http://self.com"))
                .link("qwe", LinkBuilder::new("http://qwe.com"))
                .status(HttpStatus::OK)
                .code("some code")
                .title("some title")
                .detail("some detail")
                .pointer("/foo/0/bar/1")
                .parameter("car")
                .meta1("foo", 123)
                .meta1("bar", "qwe")
                .unwrap(),
            Error {
                id: Some("123".into()),
                links: Some(fixtures::simple_links()),
                status: Some(HttpStatus::OK),
                code: Some("some code".into()),
                title: Some("some title".into()),
                detail: Some("some detail".into()),
                source: Some(ErrorSource {
                    pointer: Some("/foo/0/bar/1".into()),
                    parameter: Some("car".into()),
                }),
                meta: Some(fixtures::meta_or_attrs()),
            },
        );
    }
}
