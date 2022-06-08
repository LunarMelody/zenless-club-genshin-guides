use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Guide {
    id: String,
    html: String,
}

impl Guide {
    pub fn new<T: Into<String>>(id: T, html: T) -> Self {
        Guide {
            id: id.into(),
            html: html.into(),
        }
    }
}
