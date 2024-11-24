use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_token(user_id: &str) -> String {
    let claims = Claims {
        sub: user_id.to_string(),
        exp: 10000000000, // Set expiration time
    };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
    token
}

pub fn validate_token(token: &str) -> Result<String, String> {
    let validation = Validation::default();
    let decoded = decode::<Claims>(token, &DecodingKey::from_secret("secret".as_ref()), &validation)
        .map_err(|_| "Invalid token".to_string())?;
    Ok(decoded.claims.sub)
}