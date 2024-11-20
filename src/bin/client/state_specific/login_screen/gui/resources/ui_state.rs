use bevy::prelude::Resource;

#[derive(Resource)]
pub struct UIState {
    pub login: String,
    pub password: String,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            login: String::new(),
            password: String::new(),
        }
    }
}
