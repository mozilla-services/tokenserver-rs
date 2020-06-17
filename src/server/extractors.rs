use regex::Regex;
use futures::future::LocalBoxFuture;
use actix_web::{
    http::{
        header::HeaderMap,
    },
    Error, FromRequest,
};

use crate::tags::Tags;

pub struct  ClientState {
   extractedClientState: String,
   validatedClientState: String,
}

impl FromRequest for ClientState {
    type Config = ();
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(headers: &HeaderMap, _: Option<Tags>) -> ClientState {
        let client_state: &str = headers.get("X-Client-State").unwrap()
                                        .to_str().unwrap();

        let re = Regex::new(r"\^[a-zA-Z0-9._-]{1,32}$").unwrap();

        if re.is_match(client_state) {
            ClientState{ validatedClientState: client_state.to_string(),
                extractedClientState: client_state.to_string() }
        } else {
            ClientState{ validatedClientState: "".to_string(),
                extractedClientState: client_state.to_string() }
        }
    }
}
