use std::net::SocketAddr;

use crate::log::log_request;

pub use self::error::{Error, Result};

mod ctx;
mod error;
mod log;
mod model;
mod web;

#[allow(unused)]
use axum::routing::get;
use axum::{
    extract::{Path, Query},
    http::{Method, Uri},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::get_service,
    Json, Router,
};
use ctx::Ctx;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    let mc: model::ModelController = model::ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_all())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // region: --Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    // endregion: --Start server
    Ok(())
}

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper - ", "RES_MAPPER");
    let uuid = Uuid::new_v4().to_string();

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_Error)| {
            let client_error_body = serde_json::json!({
                "error": {
                    "type": client_Error.as_ref(),
                    "req_uuid": uuid,
                }
            });

            println!(
                "->> {:<12} - main_response_mapper - {client_error_body:?} - ",
                "RES_MAPPER"
            );
            (*status_code, Json(client_error_body)).into_response()
        });

    println!("->> server log line - {uuid} - Error: {service_error:?} ");
    let client_error = client_status_error.unzip().1;
    let _ = log_request(
        uuid,
        req_method.to_string(),
        uri,
        ctx,
        service_error,
        client_error,
    )
    .await;

    println!();
    error_response.unwrap_or(res)
}

#[derive(Debug, serde::Deserialize)]
struct HelloParams {
    name: Option<String>,
}

fn routes_all() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?} - ", "HANDLER");

    let name = params.name.as_deref().unwrap_or("default name");
    return Html(format!("Hello, <strong>{name}</strong>"));
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2< - {name:?} - ", "HANDLER");

    // let name = params.name.as_deref().unwrap_or("default name");
    return Html(format!("Hello, <strong>{name}</strong>"));
}
