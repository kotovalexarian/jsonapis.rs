use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinksBuilder {
    pub other: HashMap<String, LinkBuilder>,
    pub self_: Option<LinkBuilder>,
    pub related: Option<LinkBuilder>,
    pub first: Option<LinkBuilder>,
    pub last: Option<LinkBuilder>,
    pub prev: Option<LinkBuilder>,
    pub next: Option<LinkBuilder>,
}

impl Default for LinksBuilder {
    fn default() -> Self {
        Self {
            other: HashMap::new(),
            self_: None,
            related: None,
            first: None,
            last: None,
            prev: None,
            next: None,
        }
    }
}

impl Builder for LinksBuilder {
    type Entity = Links;

    fn finish(self) -> Result<Self::Entity, ()> {
        let mut other = HashMap::new();

        for (key, value) in self.other {
            other.insert(key, value.finish()?);
        }

        Ok(Self::Entity {
            other,
            self_: match self.self_ {
                None => None,
                Some(self_) => Some(self_.finish()?),
            },
            related: match self.related {
                None => None,
                Some(related) => Some(related.finish()?),
            },
            first: match self.first {
                None => None,
                Some(first) => Some(first.finish()?),
            },
            last: match self.last {
                None => None,
                Some(last) => Some(last.finish()?),
            },
            prev: match self.prev {
                None => None,
                Some(prev) => Some(prev.finish()?),
            },
            next: match self.next {
                None => None,
                Some(next) => Some(next.finish()?),
            },
        })
    }
}

impl LinksBuilder {
    pub fn self_<L: Into<LinkBuilder>>(self, self_: L) -> Self {
        Self {
            self_: Some(self_.into()),
            ..self
        }
    }

    pub fn related<L: Into<LinkBuilder>>(self, related: L) -> Self {
        Self {
            related: Some(related.into()),
            ..self
        }
    }

    pub fn first<L: Into<LinkBuilder>>(self, first: L) -> Self {
        Self {
            first: Some(first.into()),
            ..self
        }
    }

    pub fn last<L: Into<LinkBuilder>>(self, last: L) -> Self {
        Self {
            last: Some(last.into()),
            ..self
        }
    }

    pub fn prev<L: Into<LinkBuilder>>(self, prev: L) -> Self {
        Self {
            prev: Some(prev.into()),
            ..self
        }
    }

    pub fn next<L: Into<LinkBuilder>>(self, next: L) -> Self {
        Self {
            next: Some(next.into()),
            ..self
        }
    }

    pub fn link<L: Into<LinkBuilder>>(self, name: &str, link: L) -> Self {
        if name == "self" {
            return Self {
                self_: Some(link.into()),
                ..self
            };
        }

        if name == "related" {
            return Self {
                related: Some(link.into()),
                ..self
            };
        }

        if name == "first" {
            return Self {
                first: Some(link.into()),
                ..self
            };
        }

        if name == "last" {
            return Self {
                last: Some(link.into()),
                ..self
            };
        }

        if name == "prev" {
            return Self {
                prev: Some(link.into()),
                ..self
            };
        }

        if name == "next" {
            return Self {
                next: Some(link.into()),
                ..self
            };
        }

        let mut other = self.other;
        other.insert(name.into(), link.into());

        Self { other, ..self }
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
            LinksBuilder::default().unwrap(),
            Links {
                other: HashMap::new(),
                self_: None,
                related: None,
                first: None,
                last: None,
                prev: None,
                next: None,
            },
        );
    }

    #[test]
    fn full() {
        assert_eq!(
            LinksBuilder::default()
                .self_(LinkBuilder::new("http://self.com"))
                .related(LinkBuilder::new("http://related.com"))
                .first(
                    LinkBuilder::new("http://first.com").meta(
                        MetaOrAttrsBuilder::default()
                            .item("foo", 123)
                            .item("bar", "qwe"),
                    ),
                )
                .last(LinkBuilder::new("http://last.com"))
                .prev(LinkBuilder::new("http://prev.com"))
                .next(LinkBuilder::new("http://next.com"))
                .link("foo", LinkBuilder::new("http://foo.com"))
                .link(
                    "bar",
                    LinkBuilder::new("http://bar.com").meta(
                        MetaOrAttrsBuilder::default()
                            .item("foo", 123)
                            .item("bar", "qwe"),
                    ),
                )
                .unwrap(),
            Links {
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
                            meta: Some(meta()),
                        }),
                    );
                    other
                },
                self_: Some(Link::String("http://self.com".into())),
                related: Some(Link::String("http://related.com".into())),
                first: Some(Link::Object(LinkObject {
                    href: "http://first.com".into(),
                    meta: Some(meta()),
                })),
                last: Some(Link::String("http://last.com".into())),
                prev: Some(Link::String("http://prev.com".into())),
                next: Some(Link::String("http://next.com".into())),
            },
        );
    }

    #[test]
    fn full_common_with_link() {
        assert_eq!(
            LinksBuilder::default()
                .link("self", LinkBuilder::new("http://self.com"))
                .link("related", LinkBuilder::new("http://related.com"))
                .link("first", LinkBuilder::new("http://first.com"))
                .link(
                    "last",
                    LinkBuilder::new("http://last.com").meta(
                        MetaOrAttrsBuilder::default()
                            .item("foo", 123)
                            .item("bar", "qwe"),
                    ),
                )
                .link("prev", LinkBuilder::new("http://prev.com"))
                .link("next", LinkBuilder::new("http://next.com"))
                .link("foo", LinkBuilder::new("http://foo.com"))
                .unwrap(),
            Links {
                other: {
                    let mut other = HashMap::new();
                    other.insert(
                        "foo".into(),
                        Link::String("http://foo.com".into()),
                    );
                    other
                },
                self_: Some(Link::String("http://self.com".into())),
                related: Some(Link::String("http://related.com".into())),
                first: Some(Link::String("http://first.com".into())),
                last: Some(Link::Object(LinkObject {
                    href: "http://last.com".into(),
                    meta: Some(meta()),
                })),
                prev: Some(Link::String("http://prev.com".into())),
                next: Some(Link::String("http://next.com".into())),
            },
        );
    }

    #[test]
    fn full_implicit_from_str() {
        assert_eq!(
            LinksBuilder::default()
                .self_("http://self.com")
                .related("http://related.com")
                .first("http://first.com")
                .last("http://last.com")
                .prev("http://prev.com")
                .next("http://next.com")
                .link("foo", "http://foo.com")
                .link("bar", "http://bar.com")
                .unwrap(),
            Links {
                other: {
                    let mut other = HashMap::new();
                    other.insert(
                        "foo".into(),
                        Link::String("http://foo.com".into()),
                    );
                    other.insert(
                        "bar".into(),
                        Link::String("http://bar.com".into()),
                    );
                    other
                },
                self_: Some(Link::String("http://self.com".into())),
                related: Some(Link::String("http://related.com".into())),
                first: Some(Link::String("http://first.com".into())),
                last: Some(Link::String("http://last.com".into())),
                prev: Some(Link::String("http://prev.com".into())),
                next: Some(Link::String("http://next.com".into())),
            },
        );
    }
}
