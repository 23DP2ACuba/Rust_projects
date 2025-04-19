use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "not completed")]
    NotCompleted,
}

fn default_status() -> Status {
    Status::NotCompleted
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String,

    #[serde(default = "default_status")]
    pub status: Status,
}

