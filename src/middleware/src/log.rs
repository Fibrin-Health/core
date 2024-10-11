use core::error;
use std::time::SystemTime;

use crate::{ctx::Ctx, error::ClientError, Error, Result};
use axum::http::{Method, Uri};
use hyper::{client, service};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
// use tower_cookies::cookie::time::serde::timestamp;
use uuid::Uuid;


pub async fn log_request(
    uuid: Uuid,
    req_method: Method,
    uri: Uri,
    ctx: Option<Ctx>,
    service_error: Option<&Error>,
    client_error: Option<ClientError>,
) -> Result<()> {

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let error_type = service_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(service_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));


    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: timestamp.to_string(),
        user_id: ctx.map(|c| c.user_id()),

        req_path: uri.path().to_string(),
        req_method: req_method.to_string(),

        client_error_type: client_error.map(|ce| ce.as_ref().to_string()),
        error_type,
        error_value: error_data,
    };


    // Integrate cloudwatch service here


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
    error_value: Option<Value>,
}

