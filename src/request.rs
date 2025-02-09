/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! A representation of a request for
//! the SocketLabs [Injection API](https://www.socketlabs.com/api-reference/injection-api/).

use reqwest::{header::CONTENT_TYPE, Client};
use serde_json;

use error::Result;
use message::Message;
use response::Response;

static API_URL: &'static str = "https://inject.socketlabs.com/api/v1/email";

/// This is the struct that will hold
/// all  tokens needed for
/// Injection API authentication and also
/// the vector with all the messages to send
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Request<'a> {
    server_id: u16,
    api_key: String,
    messages: Vec<Message<'a>>,
}

impl<'a> Request<'a> {
    /// Creates a new request object with
    /// the given credentials and messages.
    pub fn new(server_id: u16, api_key: String, messages: Vec<Message<'a>>) -> Result<Request> {
        Ok(Request {
            server_id: server_id,
            api_key: api_key,
            messages: messages,
        })
    }

    /// Sends an email using the  Injection API
    pub fn send(&self) -> Result<Response> {
        let body = serde_json::to_string(&self)?;
        let client = Client::new();
        let mut response = client
            .post(API_URL)
            .header(CONTENT_TYPE, "json")
            .body(body)
            .send()?;
        serde_json::from_str::<Response>(&response.text()?).map_err(From::from)
    }
}
