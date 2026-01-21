use crate::Tunnel;
use russh::*;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

pub struct Client {}

impl client::Handler for Client {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &russh::keys::PublicKey,
    ) -> Result<bool, Self::Error> {
        // TO DO: need to check fingerprint
        Ok(true)
    }
}

pub async fn start_tunnel(tunnel: Tunnel) -> Result<(), Box<dyn std::error::Error>> {
    // Prepare config and session
    let config = Arc::new(client::Config::default());
    let sh = Client {};
    let ssh_addr = format!("{}:{}", tunnel.ssh_host, tunnel.ssh_port);
    println!("🔌 Connecting to SSH: {}", ssh_addr);

    let mut session = client::connect(config, ssh_addr, sh).await?;

    // Authorization
    println!("🔌 Authorizing");
    let key_path = match tunnel.ssh_key_path {
        Some(path) => path,
        None => {
            return Err("No SSH key path provided!".into());
        }
    };
    // decode the ssh key and convert it as usable key (with Hash alg)
    let read_ssh_key = std::fs::read_to_string(key_path)?;
    let key_pair = Arc::new(keys::decode_secret_key(&read_ssh_key, None)?);
    let key_with_alg = keys::PrivateKeyWithHashAlg::new(key_pair, None);

    session
        .authenticate_publickey(tunnel.ssh_user, key_with_alg)
        .await?;

    let session = Arc::new(Mutex::new(session));

    // SSH Tunnel
    let local_addr = format!("127.0.0.1:{}", &tunnel.local_port);
    let listener = TcpListener::bind(&local_addr).await?;

    println!("🚀 Listening on {}", local_addr);

    // Create channel and Streaming
    loop {
        let (stream, ssh_addr) = listener.accept().await?;
        println!("Connection from {}", ssh_addr);

        // Prepare connection conifg
        let shared_session = session.clone();
        let target_host = tunnel.remote_host.clone();
        let target_port = tunnel.remote_port;
        let local_port = tunnel.local_port;

        tokio::spawn(async move {
            // Session create
            // get the locked session, use await to prevent competition
            let session_guard = shared_session.lock().await;
            let channel_result = session_guard
                .channel_open_direct_tcpip(
                    target_host,
                    target_port.into(),
                    "127.0.0.1",
                    local_port.into(),
                )
                .await;
            // once the channel created, release the lock immediately
            drop(session_guard);

            // check the channel is it create successful
            let channel = match channel_result {
                Ok(c) => c,
                Err(e) => {
                    println!("Failed to open channel: {:?}", e);
                    return;
                }
            };

            // println!("{:?}", channel);
            let mut channel_stream = channel.into_stream();
            let mut stream = stream;
            // Upload: Stream -> Channel
            // Download: Channel -> Stream
            // copy_bidirectional: used for I/O
            match tokio::io::copy_bidirectional(&mut stream, &mut channel_stream).await {
                Ok((_sent, _received)) => {
                    println!("Tunnel closed.");
                }
                Err(e) => {
                    println!("Tunnel broken: {:?}", e);
                }
            }
        });
    }
}
