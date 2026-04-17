use diesel::result::Error;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::serde::json::{json, Json};
use serde_json::Value;

use crate::db::DbConn;
use crate::tenants::error::MyError;
use crate::tenants::model::{AuthTenant, RegisterTenant, RegisterTenantNoRole, Tenant, TenantInfo};
use crate::tenants::token::*;

#[get("/api/profile")]
pub async fn profile(conn: DbConn, jar: &CookieJar<'_>) -> Result<Json<TenantInfo>, Status> {
    let Some(cookie) = jar.get_private("session-token") else {
        return Err(Status::Unauthorized);
    };
    let claims = decode_token(cookie.value()).map_err(|_| Status::Unauthorized)?;
    if claims.is_expired() || !Tenant::is_valid_login_session(&claims, &*conn) {
        return Err(Status::Unauthorized);
    }
    Ok(Json(TenantInfo {
        email: claims.email,
        username: claims.username,
    }))
}

#[get("/api/token")]
pub async fn get_token(jar: &CookieJar<'_>) -> Result<Json<String>, Json<String>> {
    match jar.get_private("session-token") {
        Some(c) => Ok(Json(c.value().to_string())),
        None => Err(Json(String::new())),
    }
}

#[get("/api/list")]
pub async fn all_tenants(conn: DbConn, token: Claims) -> Result<Json<Vec<Tenant>>, Status> {
    if token.has_role("Admin") {
        Tenant::all(&conn)
            .map_err(error_status)
            .map(Json)
    } else {
        Err(Status::Unauthorized)
    }
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[post("/api/create", format = "json", data = "<tenant>")]
pub async fn create_tenant(
    conn: DbConn,
    tenant: Json<RegisterTenantNoRole>,
) -> Result<Json<String>, (Status, Json<Value>)> {
    let register_tenant_no_role = tenant.into_inner();
    let register_tenant = RegisterTenant {
        email: register_tenant_no_role.email,
        name: register_tenant_no_role.name,
        username: register_tenant_no_role.username,
        role: "tenant".to_string(),
        password: register_tenant_no_role.password,
        password_confirmation: register_tenant_no_role.password_confirmation,
    };
    let register_tenant = match register_tenant.validates(&conn) {
        Ok(register_tenant) => register_tenant,
        Err(e) => {
            return Err((
                Status::Conflict,
                Json(json!({ "error": e.to_string() })),
            ))
        }
    };
    match Tenant::create(register_tenant, &conn) {
        Ok(_) => Ok(Json("tenant created".to_string())),
        Err(e) => Err((
            Status::Conflict,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

#[post("/api/login", format = "json", data = "<auth_tenant>")]
pub async fn login(
    conn: DbConn,
    auth_tenant: Json<AuthTenant>,
    jar: &CookieJar<'_>,
) -> Result<Json<TenantInfo>, (Status, Json<Value>)> {
    const AUTH_FAIL: &str = "Invalid email or password.";

    if jar.get_private("session-token").is_none() {
        let tenant = match auth_tenant.login(&conn) {
            Ok(tenant) => tenant,
            Err(e) => {
                return match e {
                    MyError::NotFoundTenant(_) | MyError::WrongPassword(_) => Err((
                        Status::Unauthorized,
                        Json(json!({ "error": AUTH_FAIL })),
                    )),
                    _ => Err((
                        Status::InternalServerError,
                        Json(json!({ "error": "Login failed. Try again later." })),
                    )),
                };
            }
        };

        let token = create_token(
            tenant.id,
            &tenant.email,
            &tenant.username,
            &tenant.role,
            &tenant.login_session,
        );

        let mut session = Cookie::new("session-token", token);
        session.set_path("/");
        jar.add_private(session);

        Ok(Json(TenantInfo {
            email: tenant.email,
            username: tenant.username,
        }))
    } else {
        let token = jar
            .get_private("session-token")
            .map(|c| c.value().to_string())
            .unwrap_or_default();
        match decode_token(&token) {
            Ok(claims) => Ok(Json(TenantInfo {
                email: claims.email,
                username: claims.username,
            })),
            Err(_) => {
                let mut c = Cookie::new("session-token", "");
                c.set_path("/");
                jar.remove_private(c);
                Err((
                    Status::Unauthorized,
                    Json(json!({
                        "error": "Session expired or invalid. Please log in again."
                    })),
                ))
            }
        }
    }
}

#[post("/api/logout")]
pub async fn logout(jar: &CookieJar<'_>) -> Json<String> {
    let mut c = Cookie::new("session-token", "");
    c.set_path("/");
    jar.remove_private(c);
    Json("logout".to_string())
}
