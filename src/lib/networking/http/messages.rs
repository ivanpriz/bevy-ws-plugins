use serde::Deserialize;
use serde::Serialize;

use std::collections::HashMap;

#[derive(Clone)]
pub enum HTTPRequest {
    GET(GETData),
    POST(POSTData),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GETData {
    pub req_id: String,
    pub url: String,
    pub not_req_id_headers: HashMap<String, String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct POSTData {
    pub req_id: String,
    pub url: String,
    pub not_req_id_headers: HashMap<String, String>,
    pub body: HashMap<String, String>,
}

#[derive(Clone)]
// TODO: maybe also store all headers here
pub struct HTTPResponse {
    pub request_id: String,
    pub all_headers: HashMap<String, String>,
    pub status_code: u16,
    pub body: HashMap<String, String>,
}
