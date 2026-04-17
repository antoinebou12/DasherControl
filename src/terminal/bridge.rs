use std::io::Cursor;
use std::sync::Arc;

use rocket::futures::{SinkExt, StreamExt};
use rocket_ws::stream::DuplexStream;
use rocket_ws::Message;
use russh::client;
use russh::{ChannelMsg, Disconnect};

#[derive(Debug, serde::Deserialize)]
pub struct TerminalConnect {
    pub host: String,
    #[serde(default = "default_ssh_port")]
    pub port: u16,
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub cols: u32,
    #[serde(default)]
    pub rows: u32,
}

fn default_ssh_port() -> u16 {
    22
}

struct SshClientHandler;

impl client::Handler for SshClientHandler {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &russh::keys::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

pub async fn run_ssh_over_ws(
    c: TerminalConnect,
    duplex: &mut DuplexStream,
) -> Result<(), String> {
    let host = c.host.trim();
    let cols = if c.cols == 0 {
        80
    } else {
        c.cols.clamp(40, 512)
    };
    let rows = if c.rows == 0 {
        24
    } else {
        c.rows.clamp(10, 200)
    };

    if c.password.len() > 4096 {
        return Err("password too long".into());
    }

    let config = Arc::new(client::Config::default());
    let mut handle = client::connect(config, (host, c.port), SshClientHandler)
        .await
        .map_err(|e| format!("ssh connect: {e}"))?;

    let auth = handle
        .authenticate_password(c.username.clone(), c.password.clone())
        .await
        .map_err(|e| format!("ssh auth: {e}"))?;
    if !auth.success() {
        return Err("SSH authentication failed".into());
    }

    let mut channel = handle
        .channel_open_session()
        .await
        .map_err(|e| format!("open channel: {e}"))?;

    channel
        .request_pty(true, "xterm", cols, rows, 0, 0, &[])
        .await
        .map_err(|e| format!("pty: {e}"))?;
    channel
        .request_shell(true)
        .await
        .map_err(|e| format!("shell: {e}"))?;

    loop {
        tokio::select! {
            ws_in = duplex.next() => {
                match ws_in {
                    Some(Ok(Message::Text(t))) => {
                        let b = t.into_bytes();
                        if !b.is_empty() {
                            let mut cur = Cursor::new(b);
                            channel.data(&mut cur).await.map_err(|e| format!("ssh write: {e}"))?;
                        }
                    }
                    Some(Ok(Message::Binary(b))) => {
                        if !b.is_empty() {
                            let mut cur = Cursor::new(b);
                            channel.data(&mut cur).await.map_err(|e| format!("ssh write: {e}"))?;
                        }
                    }
                    Some(Ok(Message::Ping(p))) => {
                        let _ = duplex.send(Message::Pong(p)).await;
                    }
                    Some(Ok(Message::Close(_))) | None => {
                        let _ = channel.close().await;
                        break;
                    }
                    _ => {}
                }
            }
            ch_msg = channel.wait() => {
                match ch_msg {
                    Some(ChannelMsg::Data { data }) => {
                        let payload = Vec::from(data.as_ref());
                        duplex.send(Message::binary(payload)).await.map_err(|e| format!("ws send: {e}"))?;
                    }
                    Some(ChannelMsg::ExtendedData { data, .. }) => {
                        let payload = Vec::from(data.as_ref());
                        duplex.send(Message::binary(payload)).await.map_err(|e| format!("ws send: {e}"))?;
                    }
                    Some(ChannelMsg::Eof) | Some(ChannelMsg::Close) | None => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    let _ = handle
        .disconnect(Disconnect::ByApplication, "", "")
        .await;
    Ok(())
}
