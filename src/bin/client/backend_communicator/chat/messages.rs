use bevy::prelude::*;

#[derive(Event, Clone)]
pub struct IncomingChatMessage {
    pub text: String,
}

#[derive(Event, Clone)]
pub struct OutgoingChatMessage {
    pub text: String,
}
