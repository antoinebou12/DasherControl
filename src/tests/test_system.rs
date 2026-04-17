use rocket::http::Status;
use rocket::local::blocking::Client;
use serde_json::Value;

use crate::create_rocket;

#[test]
fn system_metrics_endpoint() {
    let client = Client::tracked(create_rocket()).expect("rocket");
    let response = client.get("/system/api/metrics").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let ct = response.headers().get_one("Content-Type").unwrap_or("");
    assert!(
        ct.contains("json"),
        "expected JSON content-type, got {ct:?}"
    );
    let body: Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();
    for key in [
        "cpu_usage_percent",
        "cpus",
        "memory_used_bytes",
        "memory_total_bytes",
        "disks",
        "network_interfaces",
        "process_count",
        "gpu",
    ] {
        assert!(
            body.get(key).is_some(),
            "missing JSON field {key}: {body}"
        );
    }
    assert!(body["cpus"].is_array());
    assert!(body["disks"].is_array());
    assert!(body["network_interfaces"].is_array());
}
