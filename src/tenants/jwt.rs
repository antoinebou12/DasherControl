use std::env;

use chrono::{Duration, Local};
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, Header, Validation};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32,
    email: String,
    name: String,
    exp: usize
}

// We're using a struct so we can implement a conversion from
// Claims to TinyTenant, useful in the decode function.
pub struct TinyTenant {
    pub id: i32,
    pub email: String,
    pub name: String
}

impl From<Claims> for TinyTenant {
    fn from(claims: Claims) -> Self {
        TinyTenant {
            id: claims.sub,
            email: claims.email,
            name: claims.name
        }
    }
}

impl Claims {
    fn with_email(id: i32, email: &str, name: &str) -> Self {
        Claims {
            sub: id.into(),
            email: email.into(),
            name: name.into(),
            exp: (Local::now() + Duration::hours(24)).timestamp() as usize
        }
    }
}

pub fn create_token(id: i32, email: &str, name: &str) -> jsonwebtoken::errors::Result<String> {
    let claims = Claims::with_email(id, email, name);
        return encode(
            &Header::default(),
            &claims,
            get_secret().as_ref(),
        );
}

pub fn decode_token(token: &str) -> TinyTenant {
    let data = match decode::<Claims>(
        token,
        get_secret().as_ref(),
        &Validation::default(),
    ){
        Ok(data) => data,
        Err(_) => panic!(),
    };

    return data.claims.into();

}

fn get_secret() -> String {
    dotenv().ok();
    return env::var("JWT_TOKEN").expect("JWT_TOKEN must be set");
    
}