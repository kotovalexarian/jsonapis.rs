use super::*;

use std::{fmt::Display, str::Utf8Error};

use reqwest::{
    blocking::{Client as ReqClient, RequestBuilder},
    header::{HeaderValue, ACCEPT, CONTENT_TYPE, LOCATION},
    Error as ReqError, StatusCode, Url,
};
use serde::Serialize;
use serde_json::Error as JsonError;
use url::ParseError;

const MIME: &str = "application/vnd.api+json";
const MIME_PREFIX: &str = "application/vnd.api+json;";

#[derive(Clone, Debug)]
pub struct Client {
    url: String,
    add_json_ext: bool,
}

pub type Result = std::result::Result<Response, Error>;

#[derive(Clone, Debug, Serialize)]
pub struct Response {
    document: Document,
    location: Option<String>,
}

#[derive(Debug)]
pub enum Error {
    Response(Box<Response>),
    Url(ParseError),
    Http(ReqError),
    InvalidStatus(StatusCode),
    NoContentType,
    InvalidContentType(HeaderValue),
    InvalidLocationUtf8(Utf8Error),
    Text(ReqError),
    Json(JsonError),
}

impl Response {
    pub fn document(&self) -> &Document {
        &self.document
    }
}

impl Client {
    pub fn new<U: Into<String>>(url: U) -> Self {
        Self {
            url: url.into(),
            add_json_ext: false,
        }
    }

    pub fn add_json_ext(self, add_json_ext: bool) -> Self {
        Self {
            add_json_ext,
            ..self
        }
    }

    pub fn get<P, I, K, V>(&self, path: P, params: I) -> Result
    where
        P: Display,
        I: IntoIterator,
        K: AsRef<str>,
        V: AsRef<str>,
        <I as IntoIterator>::Item: std::borrow::Borrow<(K, V)>,
    {
        let url = self.url_for_get(path, params).map_err(Error::Url)?;

        let (status, response) = Self::make_request(ReqClient::new().get(url))?;

        // TODO: Implement status handling accorging to specification
        // https://jsonapi.org/format/#fetching-resources-responses
        // https://jsonapi.org/format/#fetching-relationships-responses
        if status.is_success() {
            if status == StatusCode::OK {
                Ok(response)
            } else {
                Err(Error::InvalidStatus(status))
            }
        } else {
            Err(Error::Response(Box::new(response)))
        }
    }

    pub fn post<'d, P, D>(&self, path: P, document: D) -> Result
    where
        P: Display,
        D: Into<&'d Document>,
    {
        let url = self.url_for_post(path).map_err(Error::Url)?;

        let document: &Document = document.into();

        let (status, response) =
            Self::make_request(ReqClient::new().post(url).json(document))?;

        // TODO: Implement status handling accorging to specification
        // https://jsonapi.org/format/#crud-creating-responses
        // https://jsonapi.org/format/#crud-updating-responses
        // https://jsonapi.org/format/#crud-updating-relationship-responses
        // https://jsonapi.org/format/#crud-deleting-responses
        if status.is_success() {
            if status == StatusCode::CREATED {
                Ok(response)
            } else {
                Err(Error::InvalidStatus(status))
            }
        } else {
            Err(Error::Response(Box::new(response)))
        }
    }

    fn url_for_get<P, I, K, V>(
        &self,
        path: P,
        params: I,
    ) -> std::result::Result<Url, ParseError>
    where
        P: Display,
        I: IntoIterator,
        K: AsRef<str>,
        V: AsRef<str>,
        <I as IntoIterator>::Item: std::borrow::Borrow<(K, V)>,
    {
        Url::parse_with_params(
            &if self.add_json_ext {
                format!("{}{}.json", self.url, path)
            } else {
                format!("{}{}", self.url, path)
            },
            params,
        )
    }

    fn url_for_post<P>(&self, path: P) -> std::result::Result<Url, ParseError>
    where
        P: Display,
    {
        Url::parse(&if self.add_json_ext {
            format!("{}{}.json", self.url, path)
        } else {
            format!("{}{}", self.url, path)
        })
    }

    fn make_request(
        request_builder: RequestBuilder,
    ) -> std::result::Result<(StatusCode, Response), Error> {
        let response = request_builder
            .header(ACCEPT, MIME)
            .header(CONTENT_TYPE, MIME)
            .send()
            .map_err(Error::Http)?;

        let status = response.status();

        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .ok_or(Error::NoContentType)?;

        if content_type != MIME
            && !content_type.as_bytes().starts_with(MIME_PREFIX.as_bytes())
        {
            return Err(Error::InvalidContentType(content_type.clone()));
        }

        let location = match response.headers().get(LOCATION) {
            None => None,
            Some(header) => match std::str::from_utf8(header.as_bytes()) {
                Err(error) => return Err(Error::InvalidLocationUtf8(error)),
                Ok(location) => Some(location.to_string()),
            },
        };

        let json = response.text().map_err(Error::Text)?;

        let document = serde_json::from_str(&json).map_err(Error::Json)?;

        Ok((status, Response { document, location }))
    }
}
