use regex::Regex;
use actix_web::{
    Error, FromRequest,
};
use futures::future::{ok, err, Ready};
use actix_web::error::ErrorBadRequest;

pub struct  ClientState {
   value: String,
}

impl FromRequest for ClientState {
    type Config = ();
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &::actix_web::HttpRequest,
                    payload: &mut actix_web::dev::Payload,) -> Self::Future {

        let client_state: &str = req.headers().get("X-Client-State").unwrap()
                                        .to_str().unwrap();
        lazy_static! {
        let re = Regex::new(r"\^[a-zA-Z0-9\._\-]{1,32}$").unwrap();
        }

        if re.is_match(client_state) {
            ok(ClientState{ value: client_state.to_string().into() })

        } else {
            err(ErrorBadRequest("Invalid Client State."))

        }
    }
}