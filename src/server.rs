use crate::{packet::JoinPacket, Args, Subcommands};
use futures::prelude::*;
use log::{debug, info};
use once_cell::sync::Lazy;
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};
use tokio::{net::TcpListener, sync::Mutex};
use tokio_serde::{formats::SymmetricalJson, SymmetricallyFramed};
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};
use uuid::Uuid;

pub static CODEC: Lazy<LengthDelimitedCodec> = Lazy::new(|| {
    LengthDelimitedCodec::builder()
        .length_field_length(4)
        .length_field_type::<u16>()
        .new_codec()
});

pub struct Server {
    clients: HashMap<Uuid, Client>,
    config: ServerConfig,
}

impl Server {
    fn new(config: ServerConfig) -> Self {
        Self {
            config,
            clients: HashMap::new(),
        }
    }
}

pub struct ServerConfig {
    addr: SocketAddr,
}

#[derive(Debug)]
pub struct Client {
    name: String,
}

impl From<Args> for ServerConfig {
    fn from(value: Args) -> Self {
        let default_addr = IpAddr::V4(Ipv4Addr::LOCALHOST);

        Self {
            addr: match value.subcommand {
                Subcommands::Server { addr, port } => {
                    SocketAddr::new(addr.unwrap_or(default_addr), port.unwrap_or(8080))
                }
                _ => SocketAddr::new(default_addr, 8080),
            },
        }
    }
}

pub async fn server(config: ServerConfig) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(&config.addr).await?;
    let server = Arc::new(Mutex::new(Server::new(config)));

    info!("TCP server started on {}", listener.local_addr()?);

    {
        let server = Arc::clone(&server);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs_f32(5.0)).await;
                let players = &server.lock().await.clients;
                debug!("{:#?}", players.values());
            }
        });
    }

    loop {
        let server = Arc::clone(&server);
        let (mut socket, _) = listener.accept().await?;

        info!("Client connected from {}", socket.peer_addr()?);
        tokio::spawn(async move {
            let (rd, wr) = socket.split();

            let read = FramedRead::new(rd, CODEC.clone());
            let write = FramedWrite::new(wr, CODEC.clone());

            let mut deserializer =
                SymmetricallyFramed::new(read, SymmetricalJson::<JoinPacket>::default());

            let mut serializer =
                SymmetricallyFramed::new(write, SymmetricalJson::<Value>::default());

            loop {
                while let Ok(Some(p)) = deserializer.try_next().await {
                    info!("Client connected with name {}!", p.name);

                    let uuid = Uuid::new_v4();

                    server
                        .lock()
                        .await
                        .clients
                        .insert(uuid, Client { name: p.name });

                    serializer
                        .send(json!({
                            "uuid": uuid.to_string()
                        }))
                        .await
                        .expect("Failed send");
                }
            }
        });
    }
}
