use std::collections::HashMap;

use bevy::prelude::*;
use bevy_io_game::networking::http::{HTTPClient, HTTPRequest, HTTPResponsesStorage, POSTData};

use crate::app_state::AppState;

use super::resources::{AccountToken, LoginRequest, LoginResponse, LoginResponseData};

pub fn obtain_login_token(
    mut http_client: ResMut<HTTPClient>,
    mut login_request: ResMut<LoginRequest>,
    mut login_response: ResMut<LoginResponse>,
    mut account_token: ResMut<AccountToken>,
    mut http_responses_storage: ResMut<HTTPResponsesStorage>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    match &account_token.account_token {
        Some(_) => {}
        None => match &login_response.data {
            Some(login_response_data) => {
                account_token.account_token = Some(login_response_data.token.to_owned());
                next_state.set(AppState::InGame);
            }
            None => match &login_request.sent {
                // if login request is sent, we are checking the response
                true => {
                    match http_responses_storage.take_response(
                        &login_request
                            .data
                            .as_mut()
                            .expect("Login request should have data when is sent")
                            .id,
                    ) {
                        Ok(Some(login_response_http)) => {
                            login_response.data = Some(LoginResponseData {
                                token: login_response_http
                                    .body
                                    .get("username") // IMPORTANT: THIS SHOULD BE "token"
                                    .expect("Token resp body should contain token")
                                    .clone(),
                            });
                        }
                        Ok(None) => {}
                        Err(_e) => {
                            panic!("Login request id not in http responses map, even though login request is marked as sent.")
                        }
                    }
                }
                // if not we are sending it as soon as we have data
                false => match &login_request.data {
                    Some(login_request_data) => {
                        let mut user_data = HashMap::new();
                        user_data.insert(
                            String::from("username"),
                            login_request_data.username.to_owned(),
                        );
                        user_data.insert(
                            String::from("password"),
                            login_request_data.password.to_owned(),
                        );
                        http_client.send_http_request(HTTPRequest::POST(POSTData {
                            req_id: login_request_data.id.to_owned(),
                            url: String::from("https://echo.free.beeceptor.com"),
                            not_req_id_headers: HashMap::new(),
                            body: user_data,
                        }));
                        http_responses_storage.create_response_promise(&login_request_data.id);
                        login_request.sent = true;
                    }
                    // means we don't have data yet, just skipping;
                    None => {}
                },
            },
        },
    }
}

pub fn nullify_login_data(
    mut login_request: ResMut<LoginRequest>,
    mut login_response: ResMut<LoginResponse>,
    mut account_token: ResMut<AccountToken>,
) {
    login_request.sent = false;
    login_request.data = None;
    login_response.data = None;
    account_token.account_token = None;
}

pub fn start_http_client(mut http_client: ResMut<HTTPClient>) {
    http_client.start();
}
