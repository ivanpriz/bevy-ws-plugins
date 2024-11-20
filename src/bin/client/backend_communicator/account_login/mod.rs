mod resources;
mod systems;

pub use resources::{
    AccountToken, LoginRequest, LoginRequestData, LoginResponse, LoginResponseData,
};
use systems::{nullify_login_data, obtain_login_token, start_http_client};

use bevy::prelude::*;

use crate::app_state::AppState;

pub struct AccountLoginPlugin {}

impl Plugin for AccountLoginPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AccountToken {
            account_token: None,
        })
        .insert_resource(LoginRequest {
            data: None,
            sent: false,
        })
        .insert_resource(LoginResponse { data: None })
        .add_systems(
            Update,
            obtain_login_token.run_if(in_state(AppState::LoginScreen)),
        )
        .add_systems(
            OnEnter(AppState::LoginScreen),
            (nullify_login_data, start_http_client),
        );
    }
}
