use std::collections::HashMap;

use bevy::prelude::Resource;

pub struct CharacterData {
    pub id: String,
    pub name: String,
    // later we will store all the data returned by characters service here
}

#[derive(Resource)]
pub struct Characters {
    pub characters_data: HashMap<String, CharacterData>,
    pub selected_character_id: Option<String>,
}

impl Default for Characters {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(
            String::from("char1"),
            CharacterData {
                name: String::from("NagibatorXxX"),
                id: String::from("char1"),
            },
        );
        Self {
            characters_data: map,
            selected_character_id: None,
        }
    }
}
