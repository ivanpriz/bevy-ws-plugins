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

// #[derive(Debug, Clone)]
// pub enum ResponseBody {
//     Text(String),
//     JSON(HashMap<String, String>),
// }

#[derive(Debug, Clone)]
pub struct HTTPResponseData {
    pub all_headers: HashMap<String, String>,
    pub status_code: u16,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct HTTPResponse {
    pub request_id: String,
    pub data: Result<HTTPResponseData, ()>,
}
