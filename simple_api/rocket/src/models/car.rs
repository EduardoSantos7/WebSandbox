use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Car {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub region: String,
    pub price: i16,
    pub year: String,
    pub manufacturer: String,
    pub model: String,
    pub condition: String,
}
