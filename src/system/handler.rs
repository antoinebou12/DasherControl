use std::time::{Duration, Instant};

use futures_util::StreamExt;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use super::metrics::SystemMetricsState;
use super::model::{MetricsResponse, NetworkSpeedTestError, NetworkSpeedTestResponse};

const DEFAULT_SPEED_TEST_BYTES: u64 = 2_000_000;
const MAX_SPEED_TEST_BYTES: u64 = 10_000_000;
const MIN_SPEED_TEST_BYTES: u64 = 250_000;
const SPEED_TEST_TIMEOUT: Duration = Duration::from_secs(30);

fn speed_test_byte_cap() -> u64 {
    std::env::var("DASHER_NETWORK_SPEED_TEST_BYTES")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(DEFAULT_SPEED_TEST_BYTES)
        .clamp(MIN_SPEED_TEST_BYTES, MAX_SPEED_TEST_BYTES)
}

fn speed_test_url() -> String {
    let bytes = speed_test_byte_cap();
    format!("https://speed.cloudflare.com/__down?bytes={bytes}")
}

#[get("/api/metrics")]
pub fn get_metrics(state: &State<SystemMetricsState>) -> Json<MetricsResponse> {
    Json(state.snapshot())
}

#[post("/api/network_speed_test")]
pub async fn network_speed_test() -> Result<Json<NetworkSpeedTestResponse>, (Status, Json<NetworkSpeedTestError>)> {
    let cap = speed_test_byte_cap();
    let url = speed_test_url();

    let client = reqwest::Client::builder()
        .timeout(SPEED_TEST_TIMEOUT)
        .build()
        .map_err(|e| {
            (
                Status::InternalServerError,
                Json(NetworkSpeedTestError {
                    error: format!("http client: {e}"),
                }),
            )
        })?;

    let started = Instant::now();
    let response = client.get(url).send().await.map_err(|e| {
        (
            Status::BadGateway,
            Json(NetworkSpeedTestError {
                error: format!("request failed: {e}"),
            }),
        )
    })?;

    if !response.status().is_success() {
        return Err((
            Status::BadGateway,
            Json(NetworkSpeedTestError {
                error: format!("unexpected status {}", response.status()),
            }),
        ));
    }

    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| {
            (
                Status::BadGateway,
                Json(NetworkSpeedTestError {
                    error: format!("read error: {e}"),
                }),
            )
        })?;
        downloaded += chunk.len() as u64;
        if downloaded >= cap {
            break;
        }
    }

    let elapsed = started.elapsed();
    let duration_ms = elapsed.as_millis() as u64;
    let secs = elapsed.as_secs_f64().max(0.001);
    let megabits_per_second = (downloaded as f64 * 8.0) / secs / 1_000_000.0;

    Ok(Json(NetworkSpeedTestResponse {
        megabits_per_second,
        bytes_downloaded: downloaded,
        duration_ms,
    }))
}
