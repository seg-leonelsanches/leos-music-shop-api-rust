use serde::{Deserialize, Serialize};

use crate::schema::microphones;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Microphone {
    pub id: i32,
    pub model: String,
    pub manufacturer: String,
    pub description: String,
    pub main_image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMicrophone {
    pub model: String,
    pub manufacturer: String,
    pub description: String,
    pub main_image: Option<String>,
}