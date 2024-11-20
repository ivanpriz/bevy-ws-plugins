use std::collections::HashMap;

use bevy::prelude::*;

use super::super::messages::HTTPResponse;

#[derive(Resource)]
pub struct HTTPResponsesStorage {
    responses_by_ids: HashMap<String, Option<HTTPResponse>>,
}

impl HTTPResponsesStorage {
    pub fn new() -> Self {
        Self {
            responses_by_ids: HashMap::new(),
        }
    }

    pub fn set_response(&mut self, response: HTTPResponse) -> Result<(), String> {
        match self.responses_by_ids.get_mut(&response.request_id) {
            Some(Some(_)) => Err(String::from("Response for given request id already set")),
            Some(val @ None) => {
                println!(
                    "Successfully set response in http storage (id: {})",
                    response.request_id
                );
                *val = Some(response);
                Ok(())
            }
            None => Err(String::from(
                "Could not set http response: request with given id not found",
            )),
        }
    }

    pub fn take_response(&mut self, request_id: &str) -> Result<Option<HTTPResponse>, String> {
        // If the response is there will delete the entry from the map
        match self.responses_by_ids.get(request_id) {
            None => Err(String::from("Given request id not in http requests map")),
            Some(Some(resp)) => {
                println!("Successfully returned resp with id {}", request_id);
                let res = Ok(Some(resp.clone()));
                self.responses_by_ids.remove(request_id);
                res
            }
            Some(None) => Ok(None),
        }
    }

    pub fn create_response_promise(&mut self, request_id: &str) {
        // todo here we should expect none
        self.responses_by_ids.insert(request_id.to_owned(), None);
        println!("Response promise with id {} created", request_id);
    }
}
