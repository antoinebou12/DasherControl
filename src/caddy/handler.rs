use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;

use crate::caddy::generator::{validate_sites, CaddyGenerateBody, generate_caddyfile};
use crate::tenants::token::Claims;

#[derive(Serialize)]
pub struct CaddyfileResponse {
    pub caddyfile: String,
}

#[post("/api/generate", format = "json", data = "<body>")]
pub async fn generate_caddy(
    body: Json<CaddyGenerateBody>,
    _claims: Claims,
) -> Result<Json<CaddyfileResponse>, Status> {
    let inner = body.into_inner();
    if inner.sites.is_empty() {
        return Err(Status::BadRequest);
    }
    validate_sites(&inner.sites).map_err(|_| Status::BadRequest)?;
    let caddyfile = generate_caddyfile(&inner.sites);
    Ok(Json(CaddyfileResponse { caddyfile }))
}
