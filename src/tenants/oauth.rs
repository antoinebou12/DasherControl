//! OAuth2 (Google + GitHub) authorization-code login.
//! Configure `OAUTH_PUBLIC_APP_URL` and provider client IDs/secrets (see `.env.example`).

use std::env;

use chrono::Local;
use rocket::form::FromForm;
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::response::Redirect;
use serde::Deserialize;

use crate::db::DbConn;
use crate::tenants::model::{
    DBJsonType, NewOAuthTenant, NewTenantConfiguration, Tenant, TenantConfiguration,
};
use crate::tenants::token::create_token;
use crate::schema::tenants;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use serde_json::json;

const STATE_COOKIE: &str = "oauth-csrf";
const STATE_TTL_SECS: usize = 600;

pub fn public_app_origin() -> Result<String, ()> {
    let base = env::var("OAUTH_PUBLIC_APP_URL").map_err(|_| ())?;
    let base = base.trim_end_matches('/').to_string();
    if base.is_empty() {
        return Err(());
    }
    Ok(base)
}

fn redirect_uri(provider: &str) -> Result<String, ()> {
    Ok(format!(
        "{}/tenants/api/oauth/{}/callback",
        public_app_origin()?,
        provider
    ))
}

fn post_login_redirect() -> Result<String, ()> {
    Ok(format!("{}/", public_app_origin()?))
}

fn fail_redirect() -> Result<String, ()> {
    Ok(format!("{}/?oauth_error=1", public_app_origin()?))
}

fn require_env(name: &str) -> Result<String, ()> {
    env::var(name).map_err(|_| ())
}

#[derive(FromForm)]
pub struct OAuthCallback<'r> {
    code: Option<&'r str>,
    state: Option<&'r str>,
    error: Option<&'r str>,
    #[allow(dead_code)]
    error_description: Option<&'r str>,
}

fn set_oauth_state_cookie(jar: &CookieJar<'_>, state: &str) {
    let mut c = Cookie::new(STATE_COOKIE, state.to_string());
    c.set_path("/tenants");
    c.set_same_site(SameSite::Lax);
    c.set_max_age(rocket::time::Duration::seconds(STATE_TTL_SECS as i64));
    jar.add_private(c);
}

fn verify_oauth_state(jar: &CookieJar<'_>, state: Option<&str>) -> bool {
    let Some(expected) = jar.get_private(STATE_COOKIE).map(|c| c.value().to_string()) else {
        return false;
    };
    jar.remove_private(Cookie::from(STATE_COOKIE));
    let Some(got) = state else {
        return false;
    };
    expected == got && !expected.is_empty()
}

fn sanitize_username_base(raw: &str) -> String {
    let mut s: String = raw
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();
    s = s.trim_matches('_').to_string();
    if s.is_empty() {
        s = "user".to_string();
    }
    s.chars().take(48).collect()
}

fn unique_username(base: &str, conn: &PgConnection) -> String {
    let slug = sanitize_username_base(base);
    if !Tenant::username_taken(&slug, conn) {
        return slug;
    }
    for n in 0u32..10_000 {
        let candidate = format!("{}-{}", slug.chars().take(40).collect::<String>(), n);
        if !Tenant::username_taken(&candidate, conn) {
            return candidate;
        }
    }
    format!("u-{}", uuid::Uuid::new_v4().simple())
}

fn issue_session(jar: &CookieJar<'_>, tenant: &Tenant) {
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
}

fn provision_or_login_oauth(
    conn: &PgConnection,
    provider: &'static str,
    subject: &str,
    email: &str,
    display_name: &str,
) -> Result<Tenant, ()> {
    if email.is_empty() {
        return Err(());
    }

    let mut tenant = if let Some(t) = Tenant::find_by_oauth(provider, subject, conn) {
        t
    } else if let Some(mut existing) = Tenant::find_tenant_by_email(email, conn) {
        if existing.oauth_provider.is_none() && existing.oauth_subject.is_none() {
            Tenant::link_oauth_account(existing.id, provider, subject, conn).map_err(|_| ())?;
            existing.oauth_provider = Some(provider.to_string());
            existing.oauth_subject = Some(subject.to_string());
        }
        existing
    } else {
        let username = unique_username(
            if display_name.is_empty() {
                email.split('@').next().unwrap_or("user")
            } else {
                display_name
            },
            conn,
        );
        let name = if display_name.is_empty() {
            username.clone()
        } else {
            display_name.chars().take(120).collect()
        };
        let password = Tenant::hash_password(uuid::Uuid::new_v4().to_string()).map_err(|_| ())?;
        let row: Tenant = diesel::insert_into(tenants::table)
            .values(NewOAuthTenant {
                email: email.to_string(),
                name,
                username: username.clone(),
                password,
                role: "tenant".to_string(),
                created_at: Local::now().naive_local(),
                oauth_provider: provider.to_string(),
                oauth_subject: subject.to_string(),
            })
            .get_result(conn)
            .map_err(|_| ())?;

        let _ = TenantConfiguration::create(
            conn,
            NewTenantConfiguration {
                tenant_id: row.id,
                config: DBJsonType::from_json(json!({})),
            },
        );
        row
    };

    tenant.establish_login_session(conn).map_err(|_| ())?;
    Ok(tenant)
}

// --- Google ---

#[get("/api/oauth/google/login")]
pub async fn google_login(jar: &CookieJar<'_>) -> Result<Redirect, Status> {
    let client_id = require_env("OAUTH_GOOGLE_CLIENT_ID").map_err(|_| Status::ServiceUnavailable)?;
    let redirect_uri = redirect_uri("google").map_err(|_| Status::ServiceUnavailable)?;
    let state = uuid::Uuid::new_v4().simple().to_string();
    set_oauth_state_cookie(jar, &state);
    let scope = "openid%20email%20profile";
    let url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}&access_type=offline&include_granted_scopes=true",
        urlencoding::encode(&client_id),
        urlencoding::encode(&redirect_uri),
        scope,
        urlencoding::encode(&state),
    );
    Ok(Redirect::to(url))
}

#[derive(Debug, Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
}

#[derive(Debug, Deserialize)]
struct GoogleUserInfo {
    sub: String,
    email: Option<String>,
    name: Option<String>,
}

async fn google_profile(code: &str) -> Result<(String, String, String), ()> {
    let client_id = require_env("OAUTH_GOOGLE_CLIENT_ID")?;
    let client_secret = require_env("OAUTH_GOOGLE_CLIENT_SECRET")?;
    let redirect_uri = redirect_uri("google")?;
    let client = reqwest::Client::builder()
        .user_agent("DasherControl/0.1")
        .build()
        .map_err(|_| ())?;

    let token: GoogleTokenResponse = client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", redirect_uri.as_str()),
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
        ])
        .send()
        .await
        .map_err(|_| ())?
        .json()
        .await
        .map_err(|_| ())?;

    let user: GoogleUserInfo = client
        .get("https://openidconnect.googleapis.com/v1/userinfo")
        .bearer_auth(&token.access_token)
        .send()
        .await
        .map_err(|_| ())?
        .json()
        .await
        .map_err(|_| ())?;

    let email = user.email.ok_or(())?;
    let name = user.name.unwrap_or_default();
    Ok((user.sub, email, name))
}

#[get("/api/oauth/google/callback?<q..>")]
pub async fn google_callback(
    conn: DbConn,
    jar: &CookieJar<'_>,
    q: OAuthCallback<'_>,
) -> Result<Redirect, Redirect> {
    if q.error.is_some() || !verify_oauth_state(jar, q.state) {
        return Err(Redirect::to(fail_redirect().unwrap_or_else(|_| "/".into())));
    }
    let Some(code) = q.code else {
        return Err(Redirect::to(fail_redirect().unwrap_or_else(|_| "/".into())));
    };

    let profile = google_profile(code).await;
    let Ok((sub, email, name)) = profile else {
        return Err(Redirect::to(fail_redirect().unwrap_or_else(|_| "/".into())));
    };

    let tenant = provision_or_login_oauth(&*conn, "google", &sub, &email, &name);
    let Ok(tenant) = tenant else {
        return Err(Redirect::to(fail_redirect().unwrap_or_else(|_| "/".into())));
    };

    issue_session(jar, &tenant);
    Ok(Redirect::to(
        post_login_redirect().unwrap_or_else(|_| "/".into()),
    ))
}

// --- GitHub ---

#[get("/api/oauth/github/login")]
pub async fn github_login(jar: &CookieJar<'_>) -> Result<Redirect, Status> {
    let client_id = require_env("OAUTH_GITHUB_CLIENT_ID").map_err(|_| Status::ServiceUnavailable)?;
    let redirect_uri = redirect_uri("github").map_err(|_| Status::ServiceUnavailable)?;
    let state = uuid::Uuid::new_v4().simple().to_string();
    set_oauth_state_cookie(jar, &state);
    let scope = "read:user%20user:email";
    let url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope={}&state={}",
        urlencoding::encode(&client_id),
        urlencoding::encode(&redirect_uri),
        scope,
        urlencoding::encode(&state),
    );
    Ok(Redirect::to(url))
}

#[derive(Debug, Deserialize)]
struct GitHubTokenResponse {
    access_token: String,
}

#[derive(Debug, Deserialize)]
struct GitHubUser {
    id: i64,
    login: String,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GitHubEmail {
    email: String,
    primary: Option<bool>,
    verified: Option<bool>,
}

async fn github_profile(code: &str) -> Result<(String, String, String), ()> {
    let client_id = require_env("OAUTH_GITHUB_CLIENT_ID")?;
    let client_secret = require_env("OAUTH_GITHUB_CLIENT_SECRET")?;
    let redirect_uri = redirect_uri("github")?;
    let client = reqwest::Client::builder()
        .user_agent("DasherControl/0.1")
        .build()
        .map_err(|_| ())?;

    let token: GitHubTokenResponse = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("code", code),
            ("redirect_uri", redirect_uri.as_str()),
        ])
        .send()
        .await
        .map_err(|_| ())?
        .json()
        .await
        .map_err(|_| ())?;

    let user: GitHubUser = client
        .get("https://api.github.com/user")
        .header("Accept", "application/vnd.github+json")
        .bearer_auth(&token.access_token)
        .send()
        .await
        .map_err(|_| ())?
        .json()
        .await
        .map_err(|_| ())?;

    let emails: Vec<GitHubEmail> = client
        .get("https://api.github.com/user/emails")
        .header("Accept", "application/vnd.github+json")
        .bearer_auth(&token.access_token)
        .send()
        .await
        .map_err(|_| ())?
        .json()
        .await
        .map_err(|_| ())?;

    let email = emails
        .iter()
        .find(|e| e.primary == Some(true))
        .or_else(|| emails.first())
        .map(|e| e.email.clone())
        .ok_or(())?;

    let display = user.name.unwrap_or_else(|| user.login.clone());
    let subject = user.id.to_string();
    Ok((subject, email, display))
}

#[get("/api/oauth/github/callback?<q..>")]
pub async fn github_callback(
    conn: DbConn,
    jar: &CookieJar<'_>,
    q: OAuthCallback<'_>,
) -> Result<Redirect, Redirect> {
    if q.error.is_some() || !verify_oauth_state(jar, q.state) {
        return Err(Redirect::to(fail_redirect().unwrap_or_else(|_| "/".into())));
    }
    let Some(code) = q.code else {
        return Err(Redirect::to(fail_redirect().unwrap_or_else(|_| "/".into())));
    };

    let profile = github_profile(code).await;
    let Ok((sub, email, name)) = profile else {
        return Err(Redirect::to(fail_redirect().unwrap_or_else(|_| "/".into())));
    };

    let tenant = provision_or_login_oauth(&*conn, "github", &sub, &email, &name);
    let Ok(tenant) = tenant else {
        return Err(Redirect::to(fail_redirect().unwrap_or_else(|_| "/".into())));
    };

    issue_session(jar, &tenant);
    Ok(Redirect::to(
        post_login_redirect().unwrap_or_else(|_| "/".into()),
    ))
}
