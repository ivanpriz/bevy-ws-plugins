mod app_state;
mod backend_communicator;
mod state_specific;

use app_state::AppStatePlugin;
use backend_communicator::account_login::AccountLoginPlugin;
use bevy::prelude::*;
use bevy_iced::IcedPlugin;
use bevy_io_game::networking::http::HTTPPlugin;
use state_specific::character_selection::CharacterSelectionPlugin;
use state_specific::in_game::InGamePlugin;
use state_specific::login_screen::LoginScreenPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            IcedPlugin::default(),
            HTTPPlugin {},
            AppStatePlugin {},
            AccountLoginPlugin {},
            LoginScreenPlugin {},
            CharacterSelectionPlugin {},
            InGamePlugin::new("ws://127.0.0.1:3000/ws"),
        ))
        .run();
}
