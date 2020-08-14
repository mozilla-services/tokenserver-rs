use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn get_handler(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("{}")
}
#[actix_rt::test]
async fn test_index() {
    use super::*;
    use actix_web::test;
    let mut app = test::init_service(
        App::new().service(web::resource("/1.0/sync/1.5").route(web::get().to(get_handler))),
    )
    .await;

    let req = test::TestRequest::get().uri("/1.0/sync/1.5").to_request();
    let res = test::call_service(&mut app, req).await;

    assert_eq!(res.status(), 200, "/1.0/sync/1.5 should return 200");
}
