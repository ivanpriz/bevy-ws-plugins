mod gui;
mod state_resources;

use bevy::prelude::*;
use gui::{
    events::UiMessage,
    systems::{render_ui, update_ui_state},
};
use state_resources::Characters;

use crate::app_state::AppState;

pub struct CharacterSelectionPlugin {}

impl Plugin for CharacterSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Characters>()
            .add_event::<UiMessage>()
            .add_systems(
                Update,
                (render_ui, update_ui_state).run_if(in_state(AppState::CharacterSelectionScreen)),
            );
    }
}
