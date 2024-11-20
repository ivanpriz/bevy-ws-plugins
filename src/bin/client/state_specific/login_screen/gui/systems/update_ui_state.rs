use bevy::prelude::*;

use crate::backend_communicator::account_login::{LoginRequest, LoginRequestData};

use super::super::events::UiMessage;
use super::super::resources::UIState;

pub fn update_ui_state(
    mut ui_messages: EventReader<UiMessage>,
    mut ui_state: ResMut<UIState>,
    mut login_request: ResMut<LoginRequest>,
) {
    for msg in ui_messages.read() {
        match msg {
            UiMessage::CredentialsInputSubmitted => {
                login_request.data = Some(LoginRequestData {
                    id: String::from("login_req"),
                    username: ui_state.login.clone(),
                    password: ui_state.password.clone(),
                });
            }
            UiMessage::LoginInputUpdated(login) => {
                ui_state.login = login.clone();
            }
            UiMessage::PasswordInputUpdated(password) => {
                ui_state.password = password.clone();
            }
        }
    }
}
