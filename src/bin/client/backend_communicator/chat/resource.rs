use bevy::prelude::*;

use bevy_io_game::networking::websockets::WsClient;

use super::{messages::IncomingChatMessage, OutgoingChatMessage};

#[derive(Resource)]
pub struct ChatServer {
    server_adress: String,
    ws_client: WsClient,
}

impl ChatServer {
    pub fn new(server_adress: &str) -> Self {
        Self {
            server_adress: server_adress.to_owned(),
            ws_client: WsClient::new(server_adress),
        }
    }

    pub fn start(&mut self) {
        self.ws_client.start();
    }

    // todo create schema for chat messages (channel like say local(can be some subregion id)/region/guild, prompt type - whisp/yell/say etc.)
    pub fn send_chat_message(&mut self, msg: OutgoingChatMessage) {
        self.ws_client.send_ws_message(&msg.text);
    }

    pub fn get_received_chat_message(&mut self) -> Option<IncomingChatMessage> {
        // todo validate that received is a valid chat message
        match self.ws_client.get_received_ws_message() {
            Some(text) => Some(IncomingChatMessage { text }),
            None => None,
        }
    }

    pub fn disconnect(&mut self) {
        self.ws_client.disconnect();
    }
}
