use bevy::prelude::Resource;

#[derive(Resource)]
pub struct UIState {
    pub chat_messages: Vec<String>,
    pub chat_input_value: String,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            chat_messages: vec![],
            chat_input_value: String::new(),
        }
    }
}
