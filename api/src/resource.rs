use {
    diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl},
    diesel::query_dsl::methods::FilterDsl,
    diesel::result::Error,
    serde::{Deserialize, Serialize},
    uuid::Uuid,

    super::schema::resources,
    crate::DBPooledConnection,
};

#[derive(Queryable, Insertable)]
#[table_name = "resources"]
pub struct Resource {
    pub id: Uuid,
    pub resource_scope: String,
    pub resource_topic: String,
    pub resource_title: String,
    pub resource_type: String,
    pub resource_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceResponse {
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

impl Resource {
    fn to_resource_response(&self) -> ResourceResponse {
        ResourceResponse {
            id: self.id.to_string(),
            resource_scope: self.resource_scope.clone(),
            resource_topic: self.resource_topic.clone(),
            resource_title: self.resource_title.clone(),
            resource_type: self.resource_type.clone(),
            resource_url: self.resource_url.clone(),
        }
    }
}

pub fn fetch_all_resources(conn: &mut DBPooledConnection) -> Vec<ResourceResponse> {
    use crate::schema::resources::dsl::*;
    match resources.load::<Resource>(conn) {
        Ok(result) => result.iter().map(|r| { r.to_resource_response() }).collect(),
        Err(_) => vec![],
    }
}

pub fn fetch_resource_by_id(_id: Uuid, conn: &mut DBPooledConnection) -> Option<ResourceResponse> {
    use crate::schema::resources::dsl::*;
    match resources.filter(id.eq(_id)).load::<Resource>(conn) {
        Ok(mut result) => Some(result.pop()?.to_resource_response()),
        Err(_) => None,
    }
}

pub fn create_new_resource(new_resource_request: NewResourceRequest, conn: &mut DBPooledConnection) -> Result<ResourceResponse, Error> {
    use crate::schema::resources::dsl::*;
    let new_resource = Resource {
        id: Uuid::new_v4(),
        resource_scope: new_resource_request.resource_scope,
        resource_topic: new_resource_request.resource_topic,
        resource_title: new_resource_request.resource_title,
        resource_type: new_resource_request.resource_type,
        resource_url: new_resource_request.resource_url,
    };
    match diesel::insert_into(resources).values(&new_resource).execute(conn) {
        Ok(_) => Ok(new_resource.to_resource_response()),
        Err(e) => Err(e),
    }
    
}