use serde::{Deserialize, Serialize};

use csrf::{AesGcmCsrfProtection, CsrfProtection, CsrfToken};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde_json::Value;
use std::convert::TryFrom;
use std::env;

use chrono::{Duration, Local};
use diesel::pg::PgConnection;
use dotenvy::dotenv;

use crate::db::DbConn;
use crate::tenants::error::MyError;
use crate::tenants::model::Tenant;

static KEY: &[u8; 16] = include_bytes!("../../certs/secret.key");

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iat: i64,
    pub sub: i32,
    pub exp: i64,
    pub xsrf_token: String,
    pub email: String,
    pub username: String,
    pub role: String,
    pub login_session: String,
}

pub struct TinyTenant {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub role: String,
    pub login_session: String,
}

impl From<Claims> for TinyTenant {
    fn from(claims: Claims) -> Self {
        TinyTenant {
            id: claims.sub,
            email: claims.email,
            username: claims.username,
            role: claims.role,
            login_session: claims.login_session,
        }
    }
}

impl Claims {
    fn create(id: i32, email: &str, username: &str, role: &str, login_session: &str) -> Self {
        Claims {
            sub: id,
            email: email.to_string(),
            username: username.to_string(),
            role: role.to_string(),
            login_session: login_session.to_string(),
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(24)).timestamp(),
            xsrf_token: generate_csrf().b64_string(),
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = Local::now().timestamp();
        now >= self.exp
    }

    pub fn is_claimed_user(&self, id: i32) -> bool {
        self.sub == id
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.role == role
    }

    pub fn get_tenant_id(&self) -> i32 {
        self.sub
    }
}

pub fn create_token(id: i32, email: &str, username: &str, role: &str, login_session: &str) -> String {
    let claims = Claims::create(id, email, username, role, login_session);
    let key = EncodingKey::from_secret(KEY);
    encode(&Header::default(), &claims, &key).expect("jwt encode")
}

pub fn decode_token(token: &str) -> Result<Claims, MyError> {
    let key = DecodingKey::from_secret(KEY);
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = false;
    let data = decode::<Claims>(token, &key, &validation).map_err(|e| MyError::BadToken(e.to_string()))?;
    Ok(data.claims)
}

/// Same validation as the HTTP [`Claims`] guard: JWT must be valid, non-expired, and session must exist in DB.
pub fn claims_from_jwt_string(jwt: &str, conn: &PgConnection) -> Result<Claims, ()> {
    let token = jwt.trim();
    if token.is_empty() {
        return Err(());
    }
    let token_data = decode_token(token).map_err(|_| ())?;
    if token_data.is_expired() {
        return Err(());
    }
    if !Tenant::is_valid_login_session(&token_data, conn) {
        return Err(());
    }
    Ok(token_data)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomResponse {
    pub status_code: u16,
    pub message: String,
    pub data: Value,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let conn = match req.guard::<DbConn>().await {
            Outcome::Success(c) => c,
            _ => return Outcome::Error((Status::InternalServerError, ())),
        };

        if let Some(auth_header) = req.headers().get_one("Authorization") {
            let auth_str = auth_header.to_string();
            if auth_str.starts_with("Bearer") {
                let token = auth_str[6..].trim();
                if let Ok(token_data) = decode_token(token) {
                    if !token_data.is_expired() {
                        if Tenant::is_valid_login_session(&token_data, &*conn) {
                            return Outcome::Success(token_data);
                        }
                    }
                }
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}

pub fn generate_csrf() -> CsrfToken {
    dotenv().ok();
    let csrf_token = env::var("CSRF_TOKEN_KEY").expect("CSRF_TOKEN_KEY must be set");
    let aead_key = <[u8; 32]>::try_from(csrf_token.into_bytes()).unwrap();
    let protect = AesGcmCsrfProtection::from_key(aead_key);
    let (token, _cookie) = protect
        .generate_token_pair(None, 3600)
        .expect("couldn't generate token/cookie pair");
    token
}
