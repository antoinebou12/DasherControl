use std::net::{SocketAddr, TcpStream};
use tokio::net::{TcpListener as TTcpListener, TcpStream as TTcpStream };
use tokio_tungstenite::{accept_async, tungstenite::Error};
use tokio_tungstenite::tungstenite::Result;
use futures::{SinkExt, StreamExt};

use ssh2::Session;
use std::io::Read;

pub async fn connnect_ssh(username: &str, password: &str) {
    // Connect to the local SSH Server
    let tcp = TcpStream::connect("10.0.0.250:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    sess.userauth_password("username", "password").unwrap();
    assert!(sess.authenticated());
}

pub async fn exec_ssh(sess: Session) {
    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
}


async fn accept_connection(peer: SocketAddr, stream: TTcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => println!("Error processing connection: {}", err),
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TTcpStream) -> Result<()> {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

    println!("New WebSocket connection: {}", peer);

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {
            ws_stream.send(msg).await?;
        }
    }
    Ok(())
}


#[tokio::main]
pub(crate) async fn main() {
    let addr = "0.0.0.0:9000";
    let listener = TTcpListener::bind(&addr).await.expect("Can't listen");
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("connected streams should have a peer address");
        println!("Peer address: {}", peer);

        tokio::spawn(accept_connection(peer, stream));
    }
}

