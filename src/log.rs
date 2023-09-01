use std::time::{SystemTime, UNIX_EPOCH};

use crate::{ctx::Ctx, error::ClientError, Error, Result};
use axum::http::Uri;
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;

pub async fn log_request(
    uuid: String,
    req_method: String,
    uri: Uri,
    ctx: Option<Ctx>,

    service_error: Option<&Error>,
    client_error: Option<ClientError>,
) -> Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let error_type = service_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(service_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    let log_line = RequestLogLine {
        client_error_type: client_error.map(|ce| ce.as_ref().to_string()),
        error_data,
        error_type,
        req_method,
        req_path: uri.to_string(),
        timestamp: timestamp.to_string(),
        user_id: ctx.map(|ctx| ctx.user_id()),
        uuid,
    };

    // log_line
    println!("->> log_request {}\n", json!(log_line));

    Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,
    timestamp: String,

    user_id: Option<u64>,

    req_path: String,
    req_method: String,

    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}
