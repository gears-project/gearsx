#[macro_use]
extern crate clap;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;

#[macro_use]
extern crate juniper;

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
mod structure;
mod graphql;

use actix_web::{middleware, web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info,info");
    env_logger::init();

    let yaml = load_yaml!("cli.yml");
    let matches = clap::App::from(yaml).get_matches();

    let pool = db::connection::get_connection_pool();

    /*
    let conn = db::connection::establish_connection();
    let new_project = db::models::Project::create("First", &conn).unwrap();
    let project = db::models::Project::by_id(&new_project.id, &conn).unwrap();
    print!("Project {:?}", project);
    let new_domain = db::models::Document::create(&project.id, "Yay", &conn).unwrap();
    print!("Dom {:?}", new_domain);
    */


    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(graphql::handler::register)
            .default_service(web::to(|| async { "404" }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await

}
