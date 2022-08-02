use super::*;

use http::StatusCode;

impl Entity<'_> for HttpStatus {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpStatus(pub StatusCode);

impl Serialize for HttpStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for HttpStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyVisitor;

        impl<'de> Visitor<'de> for MyVisitor {
            type Value = HttpStatus;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("HTTP status code")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value.parse::<u16>() {
                    Err(_) => Err(E::custom("invalid HTTP status code")),
                    Ok(number) => match StatusCode::from_u16(number) {
                        Err(_) => Err(E::custom("invalid HTTP status code")),
                        Ok(status_code) => Ok(HttpStatus(status_code)),
                    },
                }
            }
        }

        deserializer.deserialize_any(MyVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn debug() {
        let http_status = HttpStatus(StatusCode::OK);
        assert_eq!(format!("{:?}", http_status), "HttpStatus(200)");

        let http_status = HttpStatus(StatusCode::NOT_FOUND);
        assert_eq!(format!("{:?}", http_status), "HttpStatus(404)");
    }

    #[test]
    fn serialize() {
        let http_status = HttpStatus(StatusCode::OK);
        let json = serde_json::to_string(&http_status).unwrap();
        let value: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value, json!("200"));

        let http_status = HttpStatus(StatusCode::NOT_FOUND);
        let json = serde_json::to_string(&http_status).unwrap();
        let value: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value, json!("404"));
    }

    #[test]
    fn deserialize() {
        let value = json!("200");
        let json = serde_json::to_string(&value).unwrap();
        let http_status: HttpStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(http_status, HttpStatus(StatusCode::OK));

        let value = json!("404");
        let json = serde_json::to_string(&value).unwrap();
        let http_status: HttpStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(http_status, HttpStatus(StatusCode::NOT_FOUND));
    }
}
