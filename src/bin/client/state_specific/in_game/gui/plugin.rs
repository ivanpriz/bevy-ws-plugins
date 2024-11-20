use crate::app_state::AppState;

use super::events::UiMessage;
use super::resources::UIState;
use super::systems::{render_ui, update_ui_state};
use bevy::prelude::*;

pub struct InGameGUIPlugin {}

impl Plugin for InGameGUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UIState>()
            .add_event::<UiMessage>()
            .add_systems(
                Update,
                (render_ui, update_ui_state).run_if(in_state(AppState::InGame)),
            );
    }
}
