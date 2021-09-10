use super::*;

use std::fmt::Display;

use reqwest::{
    header::{ACCEPT, CONTENT_TYPE},
    Client as ReqClient, Error as ReqError, Url, UrlError,
};
use serde_json::Error as JsonError;

const MIME: &str = "application/vnd.api+json";

#[derive(Clone, Debug)]
pub struct Client(String);

pub type Result = std::result::Result<Response, Error>;

#[derive(Clone, Debug)]
pub struct Response {
    document: Document,
}

#[derive(Debug)]
pub enum Error {
    Response(Response),
    URL(UrlError),
    HTTP(ReqError),
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

        let mut response = ReqClient::new()
            .get(url)
            .header(ACCEPT, MIME)
            .header(CONTENT_TYPE, MIME)
            .send()
            .map_err(|error| Error::HTTP(error))?;

        let json = response.text().map_err(|error| Error::Text(error))?;

        let document: Document =
            serde_json::from_str(&json).map_err(|error| Error::JSON(error))?;

        if response.status().is_success() {
            Ok(Response { document })
        } else {
            Err(Error::Response(Response { document }))
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

        let mut response = ReqClient::new()
            .post(url)
            .json(document)
            .header(ACCEPT, MIME)
            .header(CONTENT_TYPE, MIME)
            .send()
            .map_err(|error| Error::HTTP(error))?;

        let json = response.text().map_err(|error| Error::Text(error))?;

        let document: Document =
            serde_json::from_str(&json).map_err(|error| Error::JSON(error))?;

        if response.status().is_success() {
            Ok(Response { document })
        } else {
            Err(Error::Response(Response { document }))
        }
    }
}
