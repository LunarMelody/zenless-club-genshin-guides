use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Guide {
    pub id: String,
    pub html: String,
}

impl Guide {
    pub fn new(id: String, html: String) -> Self {
        Guide {
            id,
            html,
        }
    }
}
