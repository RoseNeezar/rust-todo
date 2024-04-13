use axum::http::HeaderValue;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub async fn get_user(token: Option<HeaderValue>, key: String) -> Option<uuid::Uuid> {
    let jwt = token?;

    let claims = decode::<TokenClaims>(
        jwt.to_str().unwrap().trim_start_matches("Bearer "),
        &DecodingKey::from_secret(key.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| None::<uuid::Uuid>)
    .ok()?
    .claims;

    let user_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|_| None::<uuid::Uuid>)
        .ok()?;

    Some(user_id)
}
