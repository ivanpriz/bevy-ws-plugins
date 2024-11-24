use bevy::prelude::*;

#[derive(Event, Clone)]
pub enum UiMessage {
    CharacterSelected(String),
}
