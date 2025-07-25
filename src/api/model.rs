use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegistrationData {
    pub display_name: String,
    pub password: String,
    pub entity_type: String,
    pub contact_number: String,
    pub email: String,
    pub website_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorBody {
    pub code: i64,
}
