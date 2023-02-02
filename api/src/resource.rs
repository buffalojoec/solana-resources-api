use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Resource {
    pub id: String,
    pub resource_scope: String,
    pub resource_topic: String,
    pub resource_title: String,
    pub resource_type: String,
    pub resource_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewResourceRequest {
    pub resource_scope: String,
    pub resource_topic: String,
    pub resource_title: String,
    pub resource_type: String,
    pub resource_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateResourceRequest {
    pub resource_scope: Option<String>,
    pub resource_topic: Option<String>,
    pub resource_title: Option<String>,
    pub resource_type: Option<String>,
    pub resource_url: Option<String>,
}