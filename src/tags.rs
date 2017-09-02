use reqwest::Response;
use reqwest::header::{Link, RelationType};
use serde_json::{self, Value};

use error::Result;

pub struct Tags {
    next: Option<String>,
    value: Value,
}

fn get_next_page_url_from_response(response: &Response) -> Option<String> {
    let link_header: &Link = try_option!(response.headers().get());

    let value = link_header.values().iter().find(|link_value| {
        let rels = match link_value.rel() {
            Some(rels) => rels,
            None => return false,
        };

        rels.iter().any(|rel| *rel == RelationType::Next)
    });

    value.map(|v| v.link().to_string())
}

impl Tags {
    pub fn from_response(response: Response) -> Result<Self> {
        Ok(Tags {
            next: get_next_page_url_from_response(&response),
            value: serde_json::from_reader(response)?,
        })
    }

    pub fn next_page_url(self) -> Option<String> {
        self.next
    }

    pub fn get_value(&self) -> &Value {
        &self.value
    }
}
