use axum::Json;

use crate::database::user;
use crate::error::{CustomError, CustomResult};
use crate::handler::auth::{generate_jwt, Claims};
use crate::handler::idl::*;

pub async fn register(Json(req): Json<RegisterRequest>) -> CustomResult<Json<RegisterResponse>> {
    user::create(&req.username, &req.password)
        .await
        .map(|_| Json(RegisterResponse { success: true }))
}

pub async fn login(Json(req): Json<LoginRequest>) -> CustomResult<Json<LoginResponse>> {
    let user = user::find(&req.username)
        .await?
        .ok_or(CustomError::AuthError)?;

    if user.password != req.password {
        tracing::info!("password mismatch");
        return Err(CustomError::AuthError);
    }

    let jwt_token = generate_jwt(&Claims {
        username: user.username,
    })?;

    Ok(Json(LoginResponse {
        success: true,
        token: jwt_token,
    }))
}
