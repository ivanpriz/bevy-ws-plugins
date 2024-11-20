use async_channel::{bounded, Receiver, Sender, TryRecvError};
use futures_util::stream::SplitSink;
use futures_util::stream::SplitStream;
use tokio::net::TcpStream;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;

use std::str::FromStr;
use std::thread;

use bevy::{prelude::*, tasks::futures_lite::StreamExt};
use futures_util::SinkExt;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        self,
        handshake::client::generate_key,
        http::{self, Uri},
        Message,
    },
};

#[derive(Resource)]
pub struct WsClient {
    server_adress: String,

    ws_send_sender: Sender<String>,
    ws_send_receiver: Receiver<String>,

    ws_recv_receiver: Receiver<String>,
    ws_recv_sender: Sender<String>,

    tokio_thread: Option<thread::JoinHandle<()>>,
}

impl WsClient {
    pub fn new(server_adress: &str) -> Self {
        let (ws_recv_sender, ws_recv_receiver) = bounded::<String>(42);
        let (ws_send_sender, ws_send_receiver) = bounded::<String>(42);

        Self {
            server_adress: server_adress.to_owned(),
            ws_send_sender,
            ws_send_receiver,
            ws_recv_sender,
            ws_recv_receiver,
            tokio_thread: None,
        }
    }

    async fn connect(
        server_adress: String,
    ) -> (
        SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
    ) {
        println!("Trying to connect to ws chat server...");
        let uri = Uri::from_str(&server_adress).unwrap();
        println!("Chat uri: {}", uri);
        let req = http::Request::builder()
            .uri(server_adress)
            .header("Sec-Websocket-Protocol", "sss")
            .header("Sec-WebSocket-Key", generate_key())
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", 13)
            .header("Host", uri.host().unwrap());
        println!("Building request with headers...");
        let built_req = req.body(()).expect("Building req failed");
        let tung_req = tungstenite::handshake::client::Request::from(built_req);
        println!("Tung request constructed");
        let (ws_stream, _) = connect_async(tung_req)
            .await
            .expect("Failed to connect to server");
        println!("Connected to ws server");

        futures_util::StreamExt::split(ws_stream)
    }

    pub fn start(&mut self) {
        let ws_send_receiver = self.ws_send_receiver.clone();
        let ws_recv_sender = self.ws_recv_sender.clone();

        let server_adress = self.server_adress.clone();

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        self.tokio_thread = Some(thread::spawn(move || {
            runtime.block_on(async move {
                let (mut write, mut read) = WsClient::connect(server_adress).await;

                println!("Stream created.");

                /*
                There are several ways to organize listening for sync mpsc queue and async ws stream.
                First is with select, but then reading from mpsc should be wrapped into a coro
                Second is spaning two tasks, but we can't send mpsc received to another task that easily and generally no reason to do so
                Third (how its done) is to spin a task for receiving messages from stream and loop over msgs from mpsc,
                while giving the control back to tokio runtime with tokio::time::sleep(0) */

                let jh1 = tokio::spawn(async move {
                    loop {
                        match read.next().await {
                            Some(Ok(message)) => {
                                println!("Received ws message: {:?}", message);
                                let m = message.into_text().unwrap();
                                ws_recv_sender
                                    .send(m)
                                    .await
                                    .expect("Sending of message into channel failed");
                            }
                            Some(Err(err)) => {
                                println!("Error while receiving message: {:?}", err);
                            }
                            None => {
                                println!("No more messages on websocket!");
                                return;
                            }
                        }
                    }
                });

                println!("Tokio task for ws messages receiving spawned");

                let jh2 = tokio::spawn(async move {
                    loop {
                        println!("Checking for messages to send");
                        let msg = ws_send_receiver
                            .recv()
                            .await
                            .expect("Could not recv message from channel");
                        let message = Message::Text(msg);
                        println!("Got msg to send to ws {:?}", message);
                        match write.send(message).await {
                            Ok(_) => println!("Successfully sent msg"),
                            Err(err) => match err {
                                tungstenite::Error::AlreadyClosed => {
                                    println!("CAN'T SEND! ALREADY CLOSED SOCKET! RECONNECT NEEDED")
                                }
                                _ => println!("{:?}", err),
                            },
                        }
                    }
                });

                println!("Tokio task for ws messages sending spawned");

                tokio::try_join!(async move { jh1.await }, async move { jh2.await },)
                    .expect("One of tasks exited");
            });
        }));
        println!("Tokio thread spawned")
    }

    pub fn get_received_ws_message(&mut self) -> Option<String> {
        match self.ws_recv_receiver.try_recv() {
            Ok(r) => Some(r),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Closed) => panic!("channel closed omg"),
        }
    }

    pub fn send_ws_message(&mut self, msg: &str) {
        self.ws_send_sender
            .try_send(msg.to_owned())
            .expect("WTF cant send");
    }

    pub fn disconnect(&mut self) {
        println!("Disconnecting from ws server not implemented yet.")
    }
}

impl Drop for WsClient {
    fn drop(&mut self) {
        println!("Dropping WsResource");
    }
}
