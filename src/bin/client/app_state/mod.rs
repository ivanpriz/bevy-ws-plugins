mod app_state;

pub use app_state::{AccountAuthState, AppState};
use app_state::{CharacterServerAuthState, InGameState};
use bevy::prelude::*;

fn log_state_enter(state: Res<State<AppState>>) {
    println!("Entering state {:?}", state.get());
}

fn log_state_exit(state: Res<State<AppState>>) {
    println!("Exiting state {:?}", state.get());
}

pub struct AppStatePlugin {}

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_state::<AccountAuthState>()
            .init_state::<InGameState>()
            .init_state::<CharacterServerAuthState>()
            .add_systems(OnEnter(AppState::LoginScreen), log_state_enter)
            .add_systems(OnEnter(AppState::InGame), log_state_enter)
            .add_systems(OnExit(AppState::LoginScreen), log_state_exit)
            .add_systems(OnExit(AppState::InGame), log_state_exit)
            .add_systems(OnEnter(AppState::Starting), log_state_enter)
            .add_systems(OnExit(AppState::Starting), log_state_exit)
            .add_systems(OnEnter(AppState::LoginScreen), log_state_enter)
            .add_systems(OnExit(AppState::LoginScreen), log_state_exit)
            .add_systems(OnEnter(AppState::CharacterSelectionScreen), log_state_enter)
            .add_systems(OnExit(AppState::CharacterSelectionScreen), log_state_exit);
    }
}
