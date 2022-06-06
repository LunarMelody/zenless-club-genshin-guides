use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Guide {
    id: String,
    html: String,
}

impl Guide {
    pub fn new(id: String, html: String) -> Self {
        Guide {
            id,
            html,
        }
    }
}
