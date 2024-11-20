pub mod messages;
mod resource;
mod systems;

use bevy::prelude::*;
pub use messages::{IncomingChatMessage, OutgoingChatMessage};
use resource::ChatServer;
use systems::{connect_to_chat_server, receive_chat_messages, send_chat_messages};

use crate::app_state::AppState;

pub struct ChatPlugin {
    chat_server_address: String,
}

impl ChatPlugin {
    pub fn new(chat_server_address: &str) -> Self {
        Self {
            chat_server_address: chat_server_address.to_owned(),
        }
    }
}

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<IncomingChatMessage>()
            .add_event::<OutgoingChatMessage>()
            .insert_resource(ChatServer::new(&self.chat_server_address))
            .add_systems(
                Update,
                (receive_chat_messages, send_chat_messages).run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnEnter(AppState::InGame), connect_to_chat_server);
    }
}
