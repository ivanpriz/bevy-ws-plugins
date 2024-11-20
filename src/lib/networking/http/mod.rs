mod messages;
mod resources;
mod systems;

use bevy::prelude::*;

pub use messages::{GETData, HTTPRequest, HTTPResponse, POSTData};
pub use resources::{HTTPClient, HTTPResponsesStorage};
use systems::receive_responses;

pub struct HTTPPlugin {}

impl Plugin for HTTPPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HTTPResponsesStorage::new())
            .insert_resource(HTTPClient::new())
            .add_systems(Update, receive_responses);
    }
}
