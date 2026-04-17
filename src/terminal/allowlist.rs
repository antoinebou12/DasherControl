use std::env;
use std::net::{IpAddr, SocketAddr};

/// Entries are normalized to lowercase `host:port` (default port 22 if omitted in entry).
pub fn parse_allowlist_env() -> Vec<String> {
    env::var("TERMINAL_SSH_ALLOWLIST")
        .ok()
        .map(|raw| {
            raw.split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(normalize_allowlist_entry)
                .collect()
        })
        .unwrap_or_default()
}

fn normalize_allowlist_entry(entry: &str) -> String {
    let entry = entry.to_lowercase();
    if entry.contains(':') {
        entry
    } else {
        format!("{entry}:22")
    }
}

fn target_key(host: &str, port: u16) -> String {
    format!("{}:{}", host.trim().to_lowercase(), port)
}

/// When `allowlist` is non-empty, only those `host:port` pairs may be used.
/// When empty, resolved addresses must all be loopback, private, or unique-local IPv6.
pub async fn assert_ssh_target_allowed(host: &str, port: u16, allowlist: &[String]) -> Result<(), &'static str> {
    if host.trim().is_empty() {
        return Err("empty host");
    }
    if port == 0 {
        return Err("invalid port");
    }

    if !allowlist.is_empty() {
        let key = target_key(host, port);
        if allowlist.iter().any(|e| e == &key) {
            return Ok(());
        }
        return Err("host not in TERMINAL_SSH_ALLOWLIST");
    }

    let addrs: Vec<SocketAddr> = tokio::net::lookup_host((host.trim(), port))
        .await
        .map_err(|_| "dns resolution failed")?
        .collect();
    if addrs.is_empty() {
        return Err("no addresses for host");
    }
    for addr in &addrs {
        if !is_allowed_addr(*addr) {
            return Err(
                "host resolves to a non-private address; set TERMINAL_SSH_ALLOWLIST or use a private IP",
            );
        }
    }
    Ok(())
}

fn is_allowed_addr(addr: SocketAddr) -> bool {
    match addr.ip() {
        IpAddr::V4(v4) => v4.is_private() || v4.is_loopback() || v4.is_link_local(),
        IpAddr::V6(v6) => v6.is_loopback() || v6.is_unique_local(),
    }
}
