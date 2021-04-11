extern crate csrf;

use std::env;

use chrono::{Duration, Local};
use csrf::{AesGcmCsrfProtection, CsrfProtection, CsrfToken};
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, Header, Validation};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32,
    email: String,
    username: String,
    exp: usize
}

// We're using a struct so we can implement a conversion from
// Claims to TinyTenant, useful in the decode function.
pub struct TinyTenant {
    pub id: i32,
    pub email: String,
    pub username: String
}

impl From<Claims> for TinyTenant {
    fn from(claims: Claims) -> Self {
        TinyTenant {
            id: claims.sub,
            email: claims.email,
            username: claims.username
        }
    }
}

impl Claims {
    fn with_email(id: i32, email: &str, username: &str) -> Self {
        Claims {
            sub: id.into(),
            email: email.into(),
            username: username.into(),
            exp: (Local::now() + Duration::hours(24)).timestamp() as usize
        }
    }
}

pub fn create_token(id: i32, email: &str, username: &str) -> jsonwebtoken::errors::Result<String> {
    let claims = Claims::with_email(id, email, username);
        return encode(
            &Header::default(),
            &claims,
            get_secret("JWT_TOKEN").as_ref(),
        );
}

pub fn decode_token(token: &str) -> TinyTenant {
    let data = match decode::<Claims>(
        token,
        get_secret("JWT_TOKEN").as_ref(),
        &Validation::default(),
    ){
        Ok(data) => data,
        Err(_) => panic!(),
    };

    return data.claims.into();

}

fn get_secret(secret: &str) -> String {
    dotenv().ok();
    return env::var(secret).expect(&*format!("{} must be set", secret));
}

pub(crate) fn generate_csrf() -> CsrfToken {
    let _csrf_token = get_secret("CSRF_TOKEN");
    let protect =
        AesGcmCsrfProtection::from_key(*b"01234567012345670123456701234567");

    let (token, _cookie) =
        protect.generate_token_pair(None, 3600)
        .expect("couldn't generate token/cookie pair");

    return token;
}


