use super::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ErrorSourceBuilder {
    pointer: Option<String>,
    parameter: Option<String>,
}

impl Builder<'_> for ErrorSourceBuilder {
    type Entity = ErrorSource;

    fn finish(self) -> Result<Self::Entity, ()> {
        Ok(Self::Entity {
            pointer: self.pointer,
            parameter: self.parameter,
        })
    }
}

impl ErrorSourceBuilder {
    pub fn pointer<P: ToString>(self, pointer: P) -> Self {
        Self {
            pointer: Some(pointer.to_string()),
            ..self
        }
    }

    pub fn parameter<P: ToString>(self, parameter: P) -> Self {
        Self {
            parameter: Some(parameter.to_string()),
            ..self
        }
    }
}

impl From<ErrorSource> for ErrorSourceBuilder {
    fn from(error_source: ErrorSource) -> Self {
        Self {
            pointer: error_source.pointer,
            parameter: error_source.parameter,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(
            ErrorSourceBuilder::default().unwrap(),
            ErrorSource {
                pointer: None,
                parameter: None,
            },
        );
    }

    #[test]
    fn full() {
        assert_eq!(
            ErrorSourceBuilder::default()
                .pointer("/foo/0/bar/1")
                .parameter("car")
                .unwrap(),
            ErrorSource {
                pointer: Some("/foo/0/bar/1".into()),
                parameter: Some("car".into()),
            },
        );
    }

    #[test]
    fn with_pointer() {
        assert_eq!(
            ErrorSourceBuilder::default()
                .pointer("/foo/0/bar/1")
                .unwrap(),
            ErrorSource {
                pointer: Some("/foo/0/bar/1".into()),
                parameter: None,
            },
        );
    }

    #[test]
    fn with_parameter() {
        assert_eq!(
            ErrorSourceBuilder::default().parameter("car").unwrap(),
            ErrorSource {
                pointer: None,
                parameter: Some("car".into()),
            },
        );
    }

    #[test]
    fn implicit_from_entity() {
        let error_source = ErrorSource {
            pointer: Some("/foo/0/bar/1".into()),
            parameter: Some("car".into()),
        };

        let builder: ErrorSourceBuilder = error_source.clone().into();

        assert_eq!(builder.unwrap(), error_source);
    }
}
