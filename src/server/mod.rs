//! Main application server

use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{
    dev, http::StatusCode, middleware::errhandlers::ErrorHandlers, web, App, HttpRequest,
    HttpResponse, HttpServer,
};
use cadence::StatsdClient;

use crate::error::ApiError;
use crate::metrics;

impl Server {
    pub fn with_settings(settings: Settings) -> Result<dev::Server, ApiError> {
        let metrics = metrics::metrics_from_opts(&settings)?;
        let port = settings.port;

        let server = HttpServer::new(move || {
            App::new()
                //.data(state)
                .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, ApiError::render_404))
                .wrap(Cors::default())
                // TODO: Add endpoints and handlers here.
                // Dockerflow
                // Remember to update .::web::middleware::DOCKER_FLOW_ENDPOINTS
                // when applying changes to endpoint names.
                .service(web::resource("/__heartbeat__").route(web::get().to(handlers::heartbeat)))
                .service(
                    web::resource("/__lbheartbeat__").route(web::get().to(|_: HttpRequest| {
                        // used by the load balancers, just return OK.
                        HttpResponse::Ok()
                            .content_type("application/json")
                            .body("{}")
                    })),
                )
                .service(
                    web::resource("/__version__").route(web::get().to(|_: HttpRequest| {
                        // return the contents of the version.json file created by circleci
                        // and stored in the docker root
                        HttpResponse::Ok()
                            .content_type("application/json")
                            .body(include_str!("../../version.json"))
                    })),
                )
                .service(web::resource("/__error__").route(web::get().to(handlers::test_error)))
        })
    }
}
