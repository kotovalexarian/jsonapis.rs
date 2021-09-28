use super::*;

use std::{fmt::Display, str::Utf8Error};

use reqwest::{
    header::{HeaderValue, ACCEPT, CONTENT_TYPE, LOCATION},
    Client as ReqClient, Error as ReqError, RequestBuilder, StatusCode, Url,
    UrlError,
};
use serde::Serialize;
use serde_json::Error as JsonError;

const MIME: &str = "application/vnd.api+json";
const MIME_PREFIX: &str = "application/vnd.api+json;";

#[derive(Clone, Debug)]
pub struct Client(String);

pub type Result = std::result::Result<Response, Error>;

#[derive(Clone, Debug, Serialize)]
pub struct Response {
    document: Document,
    location: Option<String>,
}

#[derive(Debug)]
pub enum Error {
    Response(Response),
    URL(UrlError),
    HTTP(ReqError),
    InvalidStatus(StatusCode),
    NoContentType,
    InvalidContentType(HeaderValue),
    InvalidLocationUtf8(Utf8Error),
    Text(ReqError),
    JSON(JsonError),
}

impl Response {
    pub fn document(&self) -> &Document {
        &self.document
    }
}

impl Client {
    pub fn new<U: Into<String>>(url: U) -> Self {
        Self(url.into())
    }

    pub fn get<P, I, K, V>(&self, path: P, params: I) -> Result
    where
        P: Display,
        I: IntoIterator,
        K: AsRef<str>,
        V: AsRef<str>,
        <I as IntoIterator>::Item: std::borrow::Borrow<(K, V)>,
    {
        let url =
            Url::parse_with_params(&format!("{}{}.json", self.0, path), params)
                .map_err(|error| Error::URL(error))?;

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
            Err(Error::Response(response))
        }
    }

    pub fn post<'d, P, D>(&self, path: P, document: D) -> Result
    where
        P: Display,
        D: Into<&'d Document>,
    {
        let url = Url::parse(&format!("{}{}.json", self.0, path))
            .map_err(|error| Error::URL(error))?;

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
            Err(Error::Response(response))
        }
    }

    fn make_request<'a>(
        request_builder: RequestBuilder,
    ) -> std::result::Result<(StatusCode, Response), Error> {
        let mut response = request_builder
            .header(ACCEPT, MIME)
            .header(CONTENT_TYPE, MIME)
            .send()
            .map_err(|error| Error::HTTP(error))?;

        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .ok_or(Error::NoContentType)?;

        if content_type != MIME
            && !content_type.as_bytes().starts_with(MIME_PREFIX.as_bytes())
        {
            return Err(Error::InvalidContentType(content_type.clone()));
        }

        let json = response.text().map_err(|error| Error::Text(error))?;

        let document =
            serde_json::from_str(&json).map_err(|error| Error::JSON(error))?;

        let location = match response.headers().get(LOCATION) {
            None => None,
            Some(header) => match std::str::from_utf8(header.as_bytes()) {
                Err(error) => return Err(Error::InvalidLocationUtf8(error)),
                Ok(location) => Some(location.to_string()),
            },
        };

        Ok((response.status(), Response { document, location }))
    }
}
