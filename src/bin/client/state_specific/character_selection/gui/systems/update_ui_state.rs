use bevy::prelude::*;

use crate::state_specific::character_selection::{
    gui::events::UiMessage, state_resources::Characters,
};

pub fn update_ui_state(
    mut ui_messages: EventReader<UiMessage>,
    mut characters: ResMut<Characters>,
) {
    for msg in ui_messages.read() {
        match msg {
            UiMessage::CharacterSelected(char_id) => {
                characters.selected_character_id = Some(char_id.clone());
            }
        }
    }
}
