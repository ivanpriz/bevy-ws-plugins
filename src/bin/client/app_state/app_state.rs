use bevy::prelude::*;

#[derive(Default, States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountAuthState {
    #[default]
    LoggedOut,
    LoggedIn,
}

#[derive(Default, States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CharacterServerAuthState {
    #[default]
    LoggedOut,
    LoggedIn,
}

#[derive(Default, States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InGameState {
    #[default]
    Running,
    Loading,
}

// todo consider renaming to ScreenState with variants of screen names
#[derive(Default, States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Starting,
    LoginScreen,
    ServerSelectionScreen,
    CharacterSelectionScreen,
    InGame,
    LoadingOutGame,
}
