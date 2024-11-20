use bevy::prelude::*;

use super::gui::InGameGUIPlugin;
use crate::backend_communicator::chat::ChatPlugin;

pub struct InGamePlugin {
    chat_server_address: String,
}

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ChatPlugin::new(&self.chat_server_address))
            .add_plugins(InGameGUIPlugin {});
    }
}

impl InGamePlugin {
    pub fn new(chat_server_address: &str) -> Self {
        Self {
            chat_server_address: chat_server_address.to_owned(),
        }
    }
}
