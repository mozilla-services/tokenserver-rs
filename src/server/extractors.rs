use actix_web::error::ErrorBadRequest;
use actix_web::{Error, FromRequest, Responder};
use futures::future::{err, ok, Ready};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
lazy_static! {
    static ref RE_EXP: Regex = Regex::new(r"^[a-zA-Z0-9\._\-]{1,32}$").unwrap();
}
#[derive(Debug, Deserialize)]
pub struct ClientState {
    value: String,
}

impl FromRequest for ClientState {
    type Config = ();
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    // type Result = Future<Item = Self, Error = Error>;

    fn from_request(
        req: &::actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let client_state: &str = req
            .headers()
            .get("X-Client-State")
            .unwrap()
            .to_str()
            .unwrap();

        if RE_EXP.is_match(client_state) {
            ok(ClientState {
                value: client_state.to_string(),
            })
        } else {
            err(ErrorBadRequest("Invalid Client State."))
        }
    }
}

async fn extract_client_state(state: ClientState) -> impl Responder {
    state.value
}

//Negative tests
#[actix_rt::test]
async fn test_extractor1() {
    use super::*;
    use actix_web::test;

    let mut app = test::init_service(
        App::new().service(web::resource("/test/extractor").route(web::to(extract_client_state))),
    )
    .await;
    let req = test::TestRequest::with_header("X-Client-State", "12345678~90ab-567890ab")
        .uri("/test/extractor")
        .to_request();
    let res = test::call_service(&mut app, req).await;

    let var = test::read_body(res).await;
    let var2 = std::str::from_utf8(&var).unwrap();
    if !RE_EXP.is_match(var2) {
        assert_eq!(var2, "Invalid Client State.");
    }
}

#[actix_rt::test]
async fn test_extractor2() {
    use super::*;
    use actix_web::test;

    let mut app = test::init_service(
        App::new().service(web::resource("/test/extractor").route(web::to(extract_client_state))),
    )
    .await;
    let req = test::TestRequest::with_header(
        "X-Client-State",
        "12345678-1234-1234-1234-1234567890abcdef-1234",
    )
    .uri("/test/extractor")
    .to_request();
    let res = test::call_service(&mut app, req).await;

    let var = test::read_body(res).await;
    let var2 = std::str::from_utf8(&var).unwrap();
    if !RE_EXP.is_match(var2) {
        assert_eq!(var2, "Invalid Client State.");
    }
}

#[actix_rt::test]
async fn test_extractor3() {
    use super::*;
    use actix_web::test;

    let mut app = test::init_service(
        App::new().service(web::resource("/test/extractor").route(web::to(extract_client_state))),
    )
    .await;
    let req = test::TestRequest::with_header("X-Client-State", "12345678|1234-@#$%90abcdef")
        .uri("/test/extractor")
        .to_request();
    let res = test::call_service(&mut app, req).await;

    let var = test::read_body(res).await;
    let var2 = std::str::from_utf8(&var).unwrap();
    if !RE_EXP.is_match(var2) {
        assert_eq!(var2, "Invalid Client State.");
    }
}
