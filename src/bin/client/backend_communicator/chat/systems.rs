use bevy::prelude::*;

use super::{messages::IncomingChatMessage, resource::ChatServer, OutgoingChatMessage};

pub fn connect_to_chat_server(mut chat_res: ResMut<ChatServer>) {
    chat_res.start();
}

pub fn receive_chat_messages(
    mut chat_res: ResMut<ChatServer>,
    mut incoming_chat_messages: EventWriter<IncomingChatMessage>,
) {
    while let Some(msg) = chat_res.get_received_chat_message() {
        // Actually not all ws messages are for chat.
        // However as for now we have only them.
        incoming_chat_messages.send(msg);
    }
}

pub fn send_chat_messages(
    mut chat_res: ResMut<ChatServer>,
    mut outgoing_chat_messages: EventReader<OutgoingChatMessage>,
) {
    for outgoing_msg in outgoing_chat_messages.read() {
        chat_res.send_chat_message(outgoing_msg.clone());
    }
}
