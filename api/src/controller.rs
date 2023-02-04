use {
    actix_web::HttpResponse,
    actix_web::web::{Data, Json, Path},
    uuid::Uuid,

    crate::DBPool,
    crate::util::{NotFoundMessage, ResponseType},
    crate::resource::*,
};


// List all Resources
#[get("/resources")]
pub async fn list_resources(pool: Data<DBPool>) -> HttpResponse {
    let mut conn = crate::get_connection_to_pool(pool);
    let resources: Vec<ResourceResponse> = fetch_all_resources(&mut conn);
    ResponseType::Ok(resources).get_response()
}

// Get a resource
#[get("/resources/{id}")]
pub async fn get_resource(path: Path<(String,)>, pool: Data<DBPool>) -> HttpResponse {
    let mut conn = crate::get_connection_to_pool(pool);
    let resource: Option<ResourceResponse> = fetch_resource_by_id(
        Uuid::parse_str(path.0.as_str()).unwrap(), &mut conn);
    match resource {
        Some(resource) => ResponseType::Ok(resource).get_response(),
        None => ResponseType::NotFound(
            NotFoundMessage::new("Resource not found".to_string())
        ).get_response(),
    }
}

// Create New Resource
#[post("/resources")]
pub async fn create_resource(new_resource_request: Json<NewResourceRequest>, pool: Data<DBPool>) -> HttpResponse {
    let mut conn = crate::get_connection_to_pool(pool);
    match create_new_resource(new_resource_request.0, &mut conn) {
        Ok(created_resource) => ResponseType::Created(created_resource).get_response(),
        Err(_) => ResponseType::NotFound(
            NotFoundMessage::new("Error creating resource.".to_string())
        ).get_response(),
    }
}

// Create New Resource
#[put("/resources/{id}")]
pub async fn update_resource(path: Path<(String,)>, update_resource_request: Json<UpdateResourceRequest>, pool: Data<DBPool>) -> HttpResponse {
    let mut conn = crate::get_connection_to_pool(pool);
    match update_existing_resource(Uuid::parse_str(path.0.as_str()).unwrap(), update_resource_request.0, &mut conn) {
        Ok(created_resource) => ResponseType::Created(created_resource).get_response(),
        Err(_) => ResponseType::NotFound(
            NotFoundMessage::new("Error updating resource.".to_string())
        ).get_response(),
    }
}