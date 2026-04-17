use std::time::Instant;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header, Method};
use rocket::{Data, Request, Response};
use uuid::Uuid;

pub struct CORS();

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options
            || response.content_type() == Some(ContentType::JSON)
        {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, GET, OPTIONS",
            ));
            response.set_header(Header::new(
                "Access-Control-Allow-Headers",
                "Content-Type, Authorization",
            ));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
            response.set_header(Header::new("X-Frame-Options", "Allow"));
        }

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
        }
    }
}

struct RequestStart(Instant);

struct CorrelationId(Uuid);

/// Structured HTTP access log (method, path, status, latency, request id) via `tracing`.
pub struct RequestLogging;

#[rocket::async_trait]
impl Fairing for RequestLogging {
    fn info(&self) -> Info {
        Info {
            name: "Structured HTTP request log",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        req.local_cache(|| RequestStart(Instant::now()));
        req.local_cache(|| CorrelationId(Uuid::new_v4()));
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, response: &mut Response<'r>) {
        let start = req.local_cache(|| RequestStart(Instant::now()));
        let correlation = req.local_cache(|| CorrelationId(Uuid::new_v4()));
        response.set_raw_header("X-Request-Id", correlation.0.to_string());

        let elapsed_ms = start.0.elapsed().as_millis().min(u128::from(u64::MAX)) as u64;
        let status = response.status();
        let method = req.method().to_string();
        let path = req.uri().path().to_string();

        if status.code >= 500 {
            tracing::warn!(
                method = %method,
                path = %path,
                status = status.code,
                latency_ms = elapsed_ms,
                request_id = %correlation.0,
                "request completed"
            );
        } else {
            tracing::info!(
                method = %method,
                path = %path,
                status = status.code,
                latency_ms = elapsed_ms,
                request_id = %correlation.0,
                "request completed"
            );
        }
    }
}
