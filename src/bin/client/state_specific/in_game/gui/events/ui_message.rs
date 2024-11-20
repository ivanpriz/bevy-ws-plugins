use bevy::prelude::*;

#[derive(Event, Clone)]
pub enum UiMessage {
    ChatInputUpdated(String),
    ChatInputSubmitted,
}
