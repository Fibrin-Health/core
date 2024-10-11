use crate::{web, Error, Result};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use axum::routing::post;
use tower_cookies::{Cookie, Cookies};


pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}


async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // Fix -- Implement authentication logic here
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));

    Ok(body)
}





#[derive(Debug, serde::Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}