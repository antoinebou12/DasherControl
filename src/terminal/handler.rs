use rocket::futures::{SinkExt, StreamExt};
use rocket::get;
use rocket_ws::{Channel, Message, WebSocket};

use crate::db::DbConn;
use crate::terminal::allowlist::{assert_ssh_target_allowed, parse_allowlist_env};
use crate::terminal::bridge::{run_ssh_over_ws, TerminalConnect};
use crate::tenants::token::claims_from_jwt_string;

#[get("/ws?<token>")]
pub fn terminal_ws(ws: WebSocket, token: String, conn: DbConn) -> Channel<'static> {
    let allowlist = parse_allowlist_env();
    let auth_ok = claims_from_jwt_string(&token, &*conn)
        .map(|c| c.has_role("Admin"))
        .unwrap_or(false);

    ws.channel(move |mut duplex| {
        Box::pin(async move {
            if !auth_ok {
                let _ = duplex
                    .send(Message::text(r#"{"error":"unauthorized"}"#))
                    .await;
                return Ok(());
            }

            let first = match duplex.next().await {
                Some(Ok(m)) => m,
                _ => return Ok(()),
            };

            let body = match first {
                Message::Text(t) => t,
                Message::Binary(b) => match String::from_utf8(b) {
                    Ok(s) => s,
                    Err(_) => {
                        let _ = duplex
                            .send(Message::text(r#"{"error":"invalid_utf8"}"#))
                            .await;
                        return Ok(());
                    }
                },
                _ => {
                    let _ = duplex
                        .send(Message::text(r#"{"error":"expected_json_connect"}"#))
                        .await;
                    return Ok(());
                }
            };

            let connect: TerminalConnect = match serde_json::from_str(&body) {
                Ok(c) => c,
                Err(_) => {
                    let _ = duplex.send(Message::text(r#"{"error":"bad_json"}"#)).await;
                    return Ok(());
                }
            };

            if let Err(msg) =
                assert_ssh_target_allowed(&connect.host, connect.port, &allowlist).await
            {
                let err = serde_json::json!({ "error": "policy", "message": msg });
                let _ = duplex.send(Message::text(err.to_string())).await;
                return Ok(());
            }

            if let Err(e) = run_ssh_over_ws(connect, &mut duplex).await {
                let err = serde_json::json!({ "error": "ssh", "message": e });
                let _ = duplex.send(Message::text(err.to_string())).await;
            }

            Ok(())
        })
    })
}
