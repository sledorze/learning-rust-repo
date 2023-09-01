use crate::web;
use crate::{Error, Result};
use axum::{routing::post, Json, Router};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFailed);
    }

    println!("SET COOKIES");

    // FIXME: implement real signature
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    // TODO: Set Cookies

    // Create a success response
    Ok(Json(json!({
        "result": {
            "success": true
        }
    })))
}

#[derive(Debug, serde::Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
