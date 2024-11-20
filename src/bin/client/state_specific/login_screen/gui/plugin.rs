use bevy::prelude::*;

use crate::app_state::AppState;

use super::{
    events::UiMessage,
    resources::UIState,
    systems::{render_ui, update_ui_state},
};

pub struct LoginScreenGUIPlugin {}

impl Plugin for LoginScreenGUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UIState>()
            .add_event::<UiMessage>()
            .add_systems(
                Update,
                (update_ui_state, render_ui).run_if(in_state(AppState::LoginScreen)),
            );
    }
}
