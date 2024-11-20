use async_channel::{bounded, Receiver, Sender, TryRecvError};
use futures_util::Future;
use tokio::pin;

use std::pin::Pin;
use std::thread;

use bevy::{prelude::*, tasks::futures_lite::StreamExt};

#[derive(Resource)]
pub struct AsyncThreadManager<
    MsgSendType: Sync + Send + 'static,
    MsgReceiveType: Sync + Send + 'static,
> {
    pub send_sender: Sender<MsgSendType>,
    pub send_receiver: Receiver<MsgSendType>,

    pub recv_sender: Sender<MsgReceiveType>,
    pub recv_receiver: Receiver<MsgReceiveType>,

    tokio_thread: Option<thread::JoinHandle<()>>,
}

impl<MsgSendType: Sync + Send + 'static, MsgReceiveType: Sync + Send + 'static>
    AsyncThreadManager<MsgSendType, MsgReceiveType>
{
    pub fn new() -> Self {
        let (ws_recv_sender, ws_recv_receiver) = bounded::<MsgReceiveType>(42);
        let (ws_send_sender, ws_send_receiver) = bounded::<MsgSendType>(42);

        Self {
            send_sender: ws_send_sender,
            send_receiver: ws_send_receiver,
            recv_sender: ws_recv_sender,
            recv_receiver: ws_recv_receiver,
            tokio_thread: None,
        }
    }

    pub async fn send_to_send_channel(send_sender: Sender<MsgSendType>, msg: MsgSendType) {
        send_sender
            .send(msg)
            .await
            .expect("Failed to send to send channel")
    }

    pub async fn receive_from_send_channel(send_receiver: Receiver<MsgSendType>) -> MsgSendType {
        send_receiver
            .recv()
            .await
            .expect("Failed to receive from send channel")
    }

    pub async fn send_to_receive_channel(
        recv_sender: &Sender<MsgReceiveType>,
        msg: MsgReceiveType,
    ) {
        recv_sender
            .send(msg)
            .await
            .expect("Failed to send to receive channel")
    }

    pub async fn receive_from_receive_channel(
        recv_receiver: Receiver<MsgReceiveType>,
    ) -> MsgReceiveType {
        recv_receiver
            .recv()
            .await
            .expect("Failed to receive from receive channel")
    }

    pub fn start(
        &mut self,
        thread_task_spawner: impl FnOnce() -> Pin<Box<dyn Future<Output = ()>>>
            + std::marker::Send
            + 'static,
    ) {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        self.tokio_thread = Some(thread::spawn(move || {
            runtime.block_on((thread_task_spawner)());
        }));
    }
}
