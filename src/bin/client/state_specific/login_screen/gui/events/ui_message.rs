use bevy::prelude::*;

#[derive(Event, Clone)]
pub enum UiMessage {
    LoginInputUpdated(String),
    PasswordInputUpdated(String),
    CredentialsInputSubmitted,
}
