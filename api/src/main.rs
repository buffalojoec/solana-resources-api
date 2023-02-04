#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;

use {
    actix_web::{middleware, App, HttpServer},
    actix_web::web::Data,
    diesel::r2d2::ConnectionManager,
    diesel::PgConnection,
    r2d2::{Pool, PooledConnection},
    std::{env, io},
};

mod controller;
mod resource;
mod schema;
mod util;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_connection_to_pool(pool: Data<DBPool>) -> DBPooledConnection {
    pool.get().expect("Failed to reach DB connection pool.")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {

    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not detected.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to initialize DB connection pool.");

    HttpServer::new(move|| {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(controller::list_resources)
            .service(controller::get_resource)
            .service(controller::create_resource)
            // .service(controller::update_resource)
            .wrap(middleware::Logger::default())
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}