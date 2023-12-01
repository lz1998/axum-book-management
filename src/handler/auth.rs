use crate::error::{CustomError, CustomResult};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    extract::TypedHeader,
    headers::{authorization::Bearer, Authorization},
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref JWT_SECRET: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub username: String,
}

pub fn generate_jwt(claims: &Claims) -> CustomResult<String> {
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .map_err(Into::into)
}

/// defines how to extract the claims from the request
#[async_trait]
impl<B> FromRequestParts<B> for Claims
where
    B: Send + Sync,
{
    /// defines what to return when the request is rejected.
    /// IntoResponse must be implemented for Rejection.
    type Rejection = CustomError;

    async fn from_request_parts(parts: &mut Parts, state: &B) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| CustomError::AuthError)?;

        // Decode the user data
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| CustomError::AuthError)?;

        Ok(token_data.claims)
    }
}
