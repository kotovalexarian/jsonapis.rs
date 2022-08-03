use super::*;

use http::StatusCode;

impl Entity<'_> for HttpStatus {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpStatus(StatusCode);

impl Display for HttpStatus {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}

impl FromStr for HttpStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse().map_err(|_| ())?))
    }
}

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

impl From<StatusCode> for HttpStatus {
    fn from(status_code: StatusCode) -> Self {
        Self(status_code)
    }
}

macro_rules! constants {
    ($($konst:ident)+) => {
        impl HttpStatus {
            $(
                pub const $konst: HttpStatus = HttpStatus(StatusCode::$konst);
            )+
        }
    }
}

constants! {
    CONTINUE                        // 100
    SWITCHING_PROTOCOLS             // 101
    PROCESSING                      // 102

    OK                              // 200
    CREATED                         // 201
    ACCEPTED                        // 202
    NON_AUTHORITATIVE_INFORMATION   // 203
    NO_CONTENT                      // 204
    RESET_CONTENT                   // 205
    PARTIAL_CONTENT                 // 206
    MULTI_STATUS                    // 207
    ALREADY_REPORTED                // 208
    IM_USED                         // 226

    MULTIPLE_CHOICES                // 300
    MOVED_PERMANENTLY               // 301
    FOUND                           // 302
    SEE_OTHER                       // 303
    NOT_MODIFIED                    // 304
    USE_PROXY                       // 305
    TEMPORARY_REDIRECT              // 307
    PERMANENT_REDIRECT              // 308

    BAD_REQUEST                     // 400
    UNAUTHORIZED                    // 401
    PAYMENT_REQUIRED                // 402
    FORBIDDEN                       // 403
    NOT_FOUND                       // 404
    METHOD_NOT_ALLOWED              // 405
    NOT_ACCEPTABLE                  // 406
    PROXY_AUTHENTICATION_REQUIRED   // 407
    REQUEST_TIMEOUT                 // 408
    CONFLICT                        // 409
    GONE                            // 410
    LENGTH_REQUIRED                 // 411
    PRECONDITION_FAILED             // 412
    PAYLOAD_TOO_LARGE               // 413
    URI_TOO_LONG                    // 414
    UNSUPPORTED_MEDIA_TYPE          // 415
    RANGE_NOT_SATISFIABLE           // 416
    EXPECTATION_FAILED              // 417
    IM_A_TEAPOT                     // 418
    MISDIRECTED_REQUEST             // 421
    UNPROCESSABLE_ENTITY            // 422
    LOCKED                          // 423
    FAILED_DEPENDENCY               // 424
    UPGRADE_REQUIRED                // 426
    PRECONDITION_REQUIRED           // 428
    TOO_MANY_REQUESTS               // 429
    REQUEST_HEADER_FIELDS_TOO_LARGE // 431
    UNAVAILABLE_FOR_LEGAL_REASONS   // 451

    INTERNAL_SERVER_ERROR           // 500
    NOT_IMPLEMENTED                 // 501
    BAD_GATEWAY                     // 502
    SERVICE_UNAVAILABLE             // 503
    GATEWAY_TIMEOUT                 // 504
    HTTP_VERSION_NOT_SUPPORTED      // 505
    VARIANT_ALSO_NEGOTIATES         // 506
    INSUFFICIENT_STORAGE            // 507
    LOOP_DETECTED                   // 508
    NOT_EXTENDED                    // 510
    NETWORK_AUTHENTICATION_REQUIRED // 511
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn clone() {
        assert_eq!(HttpStatus::OK.clone(), HttpStatus::OK);
        assert_eq!(HttpStatus::NOT_FOUND.clone(), HttpStatus::NOT_FOUND);
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", HttpStatus::OK), "HttpStatus(200)");
        assert_eq!(format!("{:?}", HttpStatus::NOT_FOUND), "HttpStatus(404)");
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", HttpStatus::OK), "200 OK");
        assert_eq!(format!("{}", HttpStatus::NOT_FOUND), "404 Not Found");
    }

    #[test]
    fn to_string() {
        assert_eq!(HttpStatus::OK.to_string(), "200 OK");
        assert_eq!(HttpStatus::NOT_FOUND.to_string(), "404 Not Found");
    }

    #[test]
    fn parse() {
        let http_status: HttpStatus = "200".parse().unwrap();
        assert_eq!(http_status, HttpStatus::OK);

        let http_status: HttpStatus = "404".parse().unwrap();
        assert_eq!(http_status, HttpStatus::NOT_FOUND);
    }

    #[test]
    fn serialize_and_deserialize() {
        let http_status = HttpStatus::OK;
        let json = serde_json::to_string(&http_status).unwrap();
        let deserialized: HttpStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(http_status, deserialized);
    }

    #[test]
    fn serialize() {
        let http_status = HttpStatus::OK;
        let json = serde_json::to_string(&http_status).unwrap();
        let value: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value, json!("200"));

        let http_status = HttpStatus::NOT_FOUND;
        let json = serde_json::to_string(&http_status).unwrap();
        let value: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value, json!("404"));
    }

    #[test]
    fn deserialize() {
        let value = json!("200");
        let json = serde_json::to_string(&value).unwrap();
        let http_status: HttpStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(http_status, HttpStatus::OK);

        let value = json!("404");
        let json = serde_json::to_string(&value).unwrap();
        let http_status: HttpStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(http_status, HttpStatus::NOT_FOUND);
    }
}
