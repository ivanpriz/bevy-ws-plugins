use bevy::prelude::*;

use super::resources::{HTTPClient, HTTPResponsesStorage};

pub fn receive_responses(
    mut responses_storage: ResMut<HTTPResponsesStorage>,
    mut http_client: ResMut<HTTPClient>,
) {
    while let Some(response) = http_client.get_received_http_response() {
        println!(
            "Received some response with request_id: {}",
            response.request_id
        );
        responses_storage.set_response(response).unwrap();
    }
}
