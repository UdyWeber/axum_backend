use std::env::var;

use axum::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use super::router::UserState;


pub async fn guard_middleware<T>(
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, (StatusCode, String)> {
    let auth_token = match var("REQUEST_AUTH_TOKEN") {
        Ok(token) => token,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };
    
    // Setting Up Middle Extension to get in other routes
    request
        .extensions_mut()
        .insert(UserState { name: "Waj".into() });

    // Token Based Authorization using typed_headers
    let token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or((
            StatusCode::BAD_REQUEST,
            String::from("Authorization header malformed"),
        ))?
        .token()
        .to_owned();

    // Matching auth
    match auth_token == token {
        true => Ok(next.run(request).await),
        false => Err((
            StatusCode::UNAUTHORIZED,
            String::from("Token doesnt match with requirements"),
        )),
    }
}
