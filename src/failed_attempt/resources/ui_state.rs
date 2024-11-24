use bevy::prelude::Resource;

#[derive(Resource)]
pub struct UIState {
    pub token: String,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            token: String::new(),
        }
    }
}
