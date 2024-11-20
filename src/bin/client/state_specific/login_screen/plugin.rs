use bevy::prelude::*;

use super::gui::LoginScreenGUIPlugin;

pub struct LoginScreenPlugin;

impl Plugin for LoginScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LoginScreenGUIPlugin {});
    }
}
