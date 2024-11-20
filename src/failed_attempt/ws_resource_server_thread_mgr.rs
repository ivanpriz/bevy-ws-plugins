use async_channel::{bounded, Receiver, Sender, TryRecvError};
use tokio::net::{TcpListener, TcpStream};

use std::{
    cell::RefCell,
    net::SocketAddr,
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

use bevy::{prelude::*, tasks::futures_lite::StreamExt};
use futures_util::SinkExt;
use tokio_tungstenite::{
    accept_async, connect_async,
    tungstenite::{Error, Message, Result},
};

use super::async_thread_manager::AsyncThreadManager;

pub struct ClientMessage {
    addr: SocketAddr,
    msg: String,
}

#[derive(Resource)]
pub struct WsResourceServer {
    thread_manager: Arc<Mutex<AsyncThreadManager<ClientMessage, ClientMessage>>>,
}

impl WsResourceServer {
    pub fn new() -> Self {
        Self {
            thread_manager: Arc::new(Mutex::new(
                AsyncThreadManager::<ClientMessage, ClientMessage>::new(),
            )),
        }
    }

    pub fn start(&mut self) {
        let mut mgr = Arc::clone(&self.thread_manager);
        self.thread_manager
            .lock()
            .unwrap()
            .start(move || Box::pin(Self::start_listener(Arc::clone(&self.thread_manager))));
        // self.thread_manager = Some(thread_manager);
    }

    async fn start_listener(
        thread_manager: Arc<Mutex<AsyncThreadManager<ClientMessage, ClientMessage>>>,
    ) {
        let addr = "127.0.0.1:9002";

        let listener = TcpListener::bind(&addr).await.expect("Can't listen");
        println!("Listening on: {}", addr);

        while let Ok((stream, _)) = listener.accept().await {
            let peer = stream
                .peer_addr()
                .expect("Connected streams should have a peer address");
            println!("Peer address: {}", peer);

            tokio::spawn(Self::accept_connection(
                peer,
                stream,
                Arc::clone(&thread_manager),
            ));
        }
    }

    async fn accept_connection(
        peer: SocketAddr,
        stream: TcpStream,
        thread_manager: Arc<Mutex<AsyncThreadManager<ClientMessage, ClientMessage>>>,
    ) {
        if let Err(e) = Self::handle_connection(peer, stream, Arc::clone(&thread_manager)).await {
            match e {
                Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                err => error!("Error processing connection: {}", err),
            }
        }
    }

    async fn handle_connection(
        peer: SocketAddr,
        stream: TcpStream,
        thread_manager: Arc<Mutex<AsyncThreadManager<ClientMessage, ClientMessage>>>,
    ) -> Result<()> {
        let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

        println!("New websocket connection: {}", peer);

        while let Some(msg) = ws_stream.next().await {
            let msg = msg?;
            AsyncThreadManager::<ClientMessage, ClientMessage>::send_to_receive_channel(
                &thread_manager.lock().unwrap().recv_sender,
                ClientMessage {
                    addr: peer,
                    msg: msg.into_text().unwrap(),
                },
            )
            .await;
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

impl Default for WsResourceServer {
    fn default() -> Self {
        let mut res = Self::new();
        res.start();
        res
    }
}
