#[macro_use]
extern crate clap;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;

#[macro_use]
extern crate juniper;
extern crate chrono;

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate log;

mod db;
mod graphql;
mod messages;
mod structure;
mod util;

use warp::{Reply, Filter};

fn create_graphql_filter() -> warp::filters::BoxedFilter<(impl Reply,)> {
    let state = warp::any().map(move || graphql::schema::Context::new());
    let graphql_filter = juniper_warp::make_graphql_filter(graphql::schema::create_schema(), state.boxed());
    let graphql_filter = warp::path("graphql").and(graphql_filter);
    graphql_filter.boxed()
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let yaml = load_yaml!("cli.yml");
    let _matches = clap::App::from(yaml).get_matches();

    let graphql_filter = create_graphql_filter();
    let log = warp::log("warp_server");

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["*"])
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]);

    let cors_route = warp::any()
        .map(warp::reply)
        .with(cors);

    let graphql_routes = 
        warp::get2()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql"))
            .or(graphql_filter);

    warp::serve(
            graphql_routes
            .with(log),
    )
    .run(([127, 0, 0, 1], 8080));
}
