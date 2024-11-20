use bevy::prelude::*;

#[derive(Default, States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    LoginScreen,
    CharacterSelectionScreen,
    InGame,
}
