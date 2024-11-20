use bevy::prelude::*;

use super::super::events::UiMessage;
use super::super::resources::UIState;
use crate::backend_communicator::chat::{IncomingChatMessage, OutgoingChatMessage};

pub fn update_ui_state(
    mut ui_messages: EventReader<UiMessage>,
    mut outgoing_chat_messages: EventWriter<OutgoingChatMessage>,
    mut incoming_chat_messages: EventReader<IncomingChatMessage>,
    mut ui_state: ResMut<UIState>,
) {
    for chat_msg in incoming_chat_messages.read() {
        ui_state.chat_messages.push(chat_msg.text.clone())
    }

    let l = ui_state.chat_messages.len();
    if l > 20 {
        ui_state.chat_messages.drain(l - 20..l);
    }

    for msg in ui_messages.read() {
        match msg {
            UiMessage::ChatInputSubmitted => {
                outgoing_chat_messages.send(OutgoingChatMessage {
                    text: ui_state.chat_input_value.to_owned(),
                });
                ui_state.chat_input_value = String::new();
            }
            UiMessage::ChatInputUpdated(value) => ui_state.chat_input_value = value.to_owned(),
        }
    }
}
