use std::env::var;

use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::{Response}, headers::{HeaderMapExt, Authorization, authorization::Bearer}
};

pub async fn guard_middleware<T>(request: Request<T>, next: Next<T>) -> Result<Response, (StatusCode, String)> {
    let auth_token = match var("REQUEST_AUTH_TOKEN") {
        Ok(token) => token,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };
    
    let token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or((StatusCode::BAD_REQUEST, String::from("Authorization header malformed")))?
        .token()
        .to_owned();

    match auth_token == token {
        true => Ok(next.run(request).await),
        false => Err((StatusCode::UNAUTHORIZED, String::from("Token doesnt match with requirements"))),
    }
}
