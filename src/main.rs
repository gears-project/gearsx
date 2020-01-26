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

#[macro_use]
extern crate log;

mod db;
mod graphql;
mod structure;
mod messages;
mod util;

use actix_cors::Cors;
use actix_web::{http, middleware, web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info,debug");
    env_logger::init();

    let yaml = load_yaml!("cli.yml");
    let _matches = clap::App::from(yaml).get_matches();

    let pool = db::connection::get_connection_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["GET", "POST", "HEAD", "PUT"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .configure(graphql::handler::register)
            .default_service(web::to(|| async { "404" }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
