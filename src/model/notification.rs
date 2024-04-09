use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pubstruct Notification {
    pub product_title, String,
    pub product_type: String,
    pub product_url: String,
    pub subscriber_name: String,
    pub status: String
}