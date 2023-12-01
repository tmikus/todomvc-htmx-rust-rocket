use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Todo {
    pub completed: bool,
    pub id: i32,
    pub title: String,
}
