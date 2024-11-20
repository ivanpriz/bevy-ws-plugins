use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct UnclassifiedResponse {
    pub body: HashMap<String, String>,
}
