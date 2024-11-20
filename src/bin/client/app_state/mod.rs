mod app_state;

pub use app_state::AppState;
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
        app.init_state::<AppState>();
        /*
        .add_systems(OnEnter(AppState::LoginScreen), log_state_enter)
        .add_systems(OnEnter(AppState::InGame), log_state_enter)
        .add_systems(OnExit(AppState::LoginScreen), log_state_exit)
        .add_systems(OnExit(AppState::InGame), log_state_exit);
         */
    }
}
