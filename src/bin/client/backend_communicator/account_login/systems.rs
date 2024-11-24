use std::collections::HashMap;

use bevy::prelude::*;
use bevy_io_game::networking::http::{HTTPClient, HTTPRequest, HTTPResponsesStorage, POSTData};

use crate::app_state::{AccountAuthState, AppState};

use super::resources::{AccountToken, LoginRequest, LoginResponse, LoginResponseData};

pub fn obtain_login_token(
    mut http_client: ResMut<HTTPClient>,
    mut login_request: ResMut<LoginRequest>,
    mut login_response: ResMut<LoginResponse>,
    mut account_token: ResMut<AccountToken>,
    mut http_responses_storage: ResMut<HTTPResponsesStorage>,
    mut next_state: ResMut<NextState<AccountAuthState>>,
    mut next_state_app: ResMut<NextState<AppState>>, // shouldn't be here, need separate system
) {
    match &account_token.account_token {
        Some(_) => {
            // If account token is already set, we not going to send any requests.
            // If we want to reobtain token it should be set to null first, and the the login request should be set
        }
        None => match &login_response.data {
            Some(login_response_data) => {
                // If there is no token and we have response data, we set the token
                // Not nullifying anything
                account_token.account_token = Some(login_response_data.token.to_owned());
                next_state.set(AccountAuthState::LoggedIn);
                next_state_app.set(AppState::CharacterSelectionScreen);
            }
            None => match &login_request.sent {
                // No response - checking for login request
                true => {
                    // if login request is sent, we are checking the response
                    match http_responses_storage.take_response(
                        &login_request
                            .data
                            .as_mut()
                            .expect("Login request should have data when is sent")
                            .id,
                    ) {
                        Ok(Some(login_response_http)) => match login_response_http.data {
                            // result for response for login request found
                            Ok(data) => {
                                // request successful
                                match data.status_code {
                                    200 => {
                                        login_response.data =
                                            Some(LoginResponseData { token: data.body });
                                    }
                                    _ => {
                                        todo!("Implement handling other than 200 responses for account token")
                                    }
                                }
                            }
                            Err(_) => {
                                // request errored out - couln't get the response from server.
                                // In this case we are resending the request
                                todo!("Implement retry logic for login request")
                            }
                        },
                        Ok(None) => {
                            // login request id found in the responses storage, but the response has not arrived yet
                        }
                        Err(_e) => {
                            panic!("Login request id not in http responses map, even though login request is marked as sent.")
                        }
                    }
                }
                // if login request is not sent we are sending it as soon as we have data
                false => match &login_request.data {
                    // data set - this means we want to send it actually
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
                            url: String::from("http://localhost:5555/auth"),
                            not_req_id_headers: HashMap::new(),
                            body: user_data,
                        }));
                        http_responses_storage.create_response_promise(&login_request_data.id);
                        login_request.sent = true;
                    }
                    None => {
                        // means we don't have data yet - so we do not intend to send the request
                    }
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
    println!("Nullified account login state");
}

// todo: extract all resources startup like this to a separate module, where we will change the state to LoginScreen when all are set up
pub fn start_http_client(
    mut http_client: ResMut<HTTPClient>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    println!("Entered start_http_client");
    http_client.start();
    next_state.set(AppState::LoginScreen);
    println!("Transfered to login screen state");
}
