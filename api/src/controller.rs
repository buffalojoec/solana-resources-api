use {
    actix_web::HttpResponse,
    actix_web::web::Json,
    
    crate::resource::*,
    crate::util::{NotFoundMessage, ResponseType},
};

// List all resources
#[get("/resources")]
pub async fn list_resources() -> HttpResponse {

    /* 
        TODO: Get all Resources from the DB
    */
    let resources: Vec<Resource> = vec![]; // Empty for now
    ResponseType::Ok(resources).get_response()
}

// Get a resource
#[get("/resources/{id}")]
pub async fn get_resource() -> HttpResponse {

    /* 
        TODO: Get the ResourceDAO object from DB WHERE id = requested id
                and convert to Resource object
    */
    let resource: Option<Resource> = None; // None for now
    match resource {
        Some(resource) => ResponseType::Ok(resource).get_response(),
        None => ResponseType::NotFound(
            NotFoundMessage::new("Resource/Club not found".to_string())
        ).get_response(),
    }
}
// Create New Resource
#[post("/resources")]
pub async fn create_resource(create_resource_request: Json<NewResourceRequest>) -> HttpResponse {
    /* 
        TODO: Create a new ResourceDAO object from requested inputs and write to DB
    */
    let resource: Vec<Resource> = vec![]; // Empty for now
    ResponseType::Created(resource).get_response()
}

// Create New Resource
#[put("/resources/{id}")]
pub async fn update_resource(update_resource_request: Json<UpdateResourceRequest>) -> HttpResponse {
    /* 
        TODO: Create a new ResourceDAO object from requested inputs and write to DB
    */
    let resource: Vec<Resource> = vec![]; // Empty for now
    ResponseType::Created(resource).get_response()
}