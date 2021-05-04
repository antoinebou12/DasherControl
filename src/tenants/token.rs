extern crate csrf;

use serde_json::Value;
use dotenv::dotenv;
use std::env;
use std::convert::TryFrom;
use chrono::{Duration, Local};
use csrf::{AesGcmCsrfProtection, CsrfProtection, CsrfToken};
use jsonwebtoken::{decode, encode, Header, Validation, Algorithm};
use rocket::request::FromRequest;
use rocket::http::Status;
use rocket::{Outcome, Request, request};
use crate::db::DbConn;
use crate::tenants::error::MyError;
use crate::tenants::model::Tenant;


static KEY: &'static [u8; 16] = include_bytes!("../../certs/secret.key");


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // jwt
    pub iat: i64,
    pub sub: i32,
    pub exp: i64,
    pub xsrf_token: String,

    // data
    pub email: String,
    pub username: String,
    pub role: String,
    pub login_session: String

}

// We're using a struct so we can implement a conversion from
// Claims to TinyTenant, useful in the decode function.
pub struct TinyTenant {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub role: String,
    pub login_session: String
}

impl From<Claims> for TinyTenant {
    fn from(claims: Claims) -> Self {
        TinyTenant {
            id: claims.sub,
            email: claims.email,
            username: claims.username,
            role: claims.role,
            login_session: claims.login_session
        }
    }
}


impl Claims {
    fn create(id: i32, email: &str, username: &str, role: &str, login_session: &str) -> Self {
        return Claims {
            sub: id,
            email: email.to_string(),
            username: username.to_string(),
            role: role.to_string(),
            login_session: login_session.to_string(),
            iat: (Local::now()).timestamp(),
            exp: (Local::now() + Duration::hours(24)).timestamp(),
            xsrf_token: generate_csrf().b64_string()
        };
    }

    pub fn is_expired(&self) -> bool {
        let now = (Local::now()).timestamp();
        now >= self.exp
    }

    pub fn is_claimed_user(&self, id: i32) -> bool {
        self.sub == id
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.role == role.to_string()
    }

    pub fn get_tenant_id(&self) -> i32 {
        return self.sub;
    }
}

pub fn create_token(id: i32, email: &str, username: &str, role: &str, login_session: &str) -> String {
    let claims = Claims::create(id, email, username, role, login_session);
        return encode(
            &Header::default(),
            &claims,
            KEY
        ).unwrap();
}

pub fn decode_token(token: &str) -> Result<Claims, MyError> {
    let data = match decode::<Claims>(
        token,
        KEY,
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(data) => data,
        Err(e) => return Err(MyError::BadToken(e.to_string())),
    };

    return Ok(data.claims.into());

}


#[derive(Debug, Serialize, Deserialize)]
pub struct CustomResponse {
    pub status_code: u16,
    pub message: String,
    pub data: Value,

}

impl<'a, 'r> FromRequest<'a, 'r> for Claims {
    type Error = Status;
    fn from_request(
        request: &'a Request<'r>,
    ) -> request::Outcome<Self, Status> {
        let conn = request.guard::<DbConn>().unwrap();
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            let auth_str = auth_header.to_string();
            if auth_str.starts_with("Bearer") {
                let token = auth_str[6..auth_str.len()].trim();
                if let Ok(token_data) = decode_token(token) {
                    if !token_data.is_expired() {
                        if Tenant::is_valid_login_session(&token_data, &conn) {
                            return Outcome::Success(token_data);
                        }
                    }
                }
            }
        }

        Outcome::Failure((
            Status::Unauthorized,
            Status::Unauthorized,
        ))
    }
}


pub fn generate_csrf() -> CsrfToken {
    dotenv().ok();
    let csrf_token= env::var("CSRF_TOKEN_KEY").expect("CSRF_TOKEN_KEY must be set");
    let aead_key = <[u8; 32]>::try_from(csrf_token.into_bytes()).unwrap();
    let protect =
        AesGcmCsrfProtection::from_key(aead_key);

    let (token, _cookie) =
        protect.generate_token_pair(None, 3600)
            .expect("couldn't generate token/cookie pair");

    return token;
}

