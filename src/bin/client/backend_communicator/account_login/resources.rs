use bevy::prelude::*;

#[derive(Resource)]
pub struct AccountToken {
    pub account_token: Option<String>,
}

#[derive(Resource, Debug)]
pub struct LoginResponse {
    pub data: Option<LoginResponseData>,
}

#[derive(Debug)]
pub struct LoginResponseData {
    // pub username: String,
    pub token: String,
}

#[derive(Resource, Debug)]
pub struct LoginRequest {
    pub data: Option<LoginRequestData>,
    pub sent: bool,
}

#[derive(Debug)]
pub struct LoginRequestData {
    pub id: String,
    pub username: String,
    pub password: String,
}
