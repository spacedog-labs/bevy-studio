use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::request::{self, Outcome};
use rocket::Request;
use serde::{Deserialize, Serialize};

pub struct JWTSecret(pub String);

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

pub fn mint_jwt(jwt_secret: &str, uid: &str) -> String {
    let claims = Claims {
        aud: "bevy-studio".to_string(),
        // 28 day expiry
        exp: Utc::now().timestamp() as usize + 2419200 as usize,
        sub: uid.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap()
}

/// Returns true if `key` is a valid API key string.
pub fn validate_jwt(jwt_secret: &str, key: &str) -> Option<String> {
    match decode::<Claims>(
        &key,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(t) => Some(t.claims.sub),
        Err(_) => None,
    }
}

pub struct JWTAuthorized(String);

#[derive(Debug)]
pub enum JWTError {
    Invalid,
}

#[async_trait]
impl<'r> FromRequest<'r> for JWTAuthorized {
    type Error = JWTError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let jwt = request.headers().get_one("Authorization").unwrap();
        let jwt_secret = request.rocket().state::<JWTSecret>().unwrap();
        match validate_jwt(&jwt_secret.0, jwt) {
            Some(user_id) => {
                return Outcome::Success(JWTAuthorized(user_id));
            }
            None => {
                return Outcome::Failure((Status::Unauthorized, JWTError::Invalid));
            }
        }
    }
}
