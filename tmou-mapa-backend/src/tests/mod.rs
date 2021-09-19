#[cfg(test)]
pub mod discovery_tests;
pub mod rocket_tests;
pub mod skip_tests;
pub mod standings_tests;
pub mod tmou22_tests;

use crate::models::db as db;
use crate::models::api as api;

// helper functions
#[allow(unused)]
pub fn item(t: &str, l: i16, n: &str) -> db::Item {
    db::Item {
        type_: t.to_string(),
        url: "Dummy".to_string(),
        level: l,
        name: n.to_string(),
        description: None,
        condition: None,
    }
}

pub fn item_with_condition(t: &str, l: i16, n: &str, c: &str) -> db::Item {
    db::Item {
        type_: t.to_string(),
        url: "Dummy".to_string(),
        level: l,
        name: n.to_string(),
        description: None,
        condition: Some(c.to_string()),
    }
}

#[allow(unused)]
pub fn api_item(t: &str, l: i16, n: &str) -> api::Item {
    api::Item {
        r#type: t.to_string(),
        url: "Dummy".to_string(),
        level: l,
        name: n.to_string(),
        description: "".to_string(),
        timestamp: None,
    }
}
