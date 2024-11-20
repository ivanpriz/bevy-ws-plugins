use async_channel::{bounded, Receiver, Sender, TryRecvError};
use tokio::net::{TcpListener, TcpStream};

use std::{net::SocketAddr, thread};

use bevy::{prelude::*, tasks::futures_lite::StreamExt};
use futures_util::SinkExt;
use tokio_tungstenite::{
    accept_async, connect_async,
    tungstenite::{Error, Message, Result},
};

pub struct ClientMessage {
    addr: SocketAddr,
    msg: String,
}

#[derive(Resource)]
pub struct WsServer {
    pub send_sender: Sender<ClientMessage>,
    pub recv_receiver: Receiver<ClientMessage>,

    send_receiver: Receiver<ClientMessage>,
    recv_sender: Sender<ClientMessage>,
    tokio_thread: Option<thread::JoinHandle<()>>,
}

impl WsServer {
    pub fn new() -> Self {
        let (ws_recv_sender, ws_recv_receiver) = bounded::<ClientMessage>(42);
        let (ws_send_sender, ws_send_receiver) = bounded::<ClientMessage>(42);

        Self {
            send_sender: ws_send_sender,
            send_receiver: ws_send_receiver,
            recv_sender: ws_recv_sender,
            recv_receiver: ws_recv_receiver,
            tokio_thread: None,
        }
    }

    pub fn start(&mut self) {
        let recv_sender = self.recv_sender.clone();
        self.tokio_thread = Some(thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            runtime.block_on(Self::start_listener(recv_sender));
        }))
    }

    async fn start_listener(recv_sender: Sender<ClientMessage>) {
        let addr = "127.0.0.1:9002";

        let listener = TcpListener::bind(&addr).await.expect("Can't listen");
        println!("Listening on: {}", addr);

        while let Ok((stream, _)) = listener.accept().await {
            let peer = stream
                .peer_addr()
                .expect("Connected streams should have a peer address");
            println!("Peer address: {}", peer);

            tokio::spawn(Self::accept_connection(peer, stream, recv_sender.clone()));
        }
    }

    async fn accept_connection(
        peer: SocketAddr,
        stream: TcpStream,
        recv_sender: Sender<ClientMessage>,
    ) {
        if let Err(e) = Self::handle_connection(peer, stream, recv_sender).await {
            match e {
                Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                err => error!("Error processing connection: {}", err),
            }
        }
    }

    async fn handle_connection(
        peer: SocketAddr,
        stream: TcpStream,
        recv_sender: Sender<ClientMessage>,
    ) -> Result<()> {
        let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

        println!("New websocket connection: {}", peer);

        while let Some(msg) = ws_stream.next().await {
            let msg = msg?;
            recv_sender
                .send(ClientMessage {
                    addr: peer,
                    msg: msg.into_text().unwrap(),
                })
                .await
                .expect("Failed to send to channel");
        }

        Ok(())
    }

    fn get_received_message(&mut self) -> Option<ClientMessage> {
        todo!();
    }

    fn broadcast_message(&mut self, msg: String) {
        todo!();
    }

    fn send_message_to_client(&mut self, msg: String, client_addr: SocketAddr) {
        todo!();
    }
}

impl Default for WsServer {
    fn default() -> Self {
        let mut res = Self::new();
        res.start();
        res
    }
}
