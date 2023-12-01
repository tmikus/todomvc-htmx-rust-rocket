use rocket::form::FromFormField;
use rocket::serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, FromFormField, Serialize)]
#[serde(crate = "rocket::serde", rename_all = "lowercase")]
pub enum Filter {
    #[default]
    All,
    Active,
    Completed,
}
