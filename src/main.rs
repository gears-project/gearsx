#[macro_use]
extern crate clap;
use clap::App;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
mod structure;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    let conn = db::connection::establish_connection();
    let new_project = db::models::Project::create("First", &conn).unwrap();
    let project = db::models::Project::by_id(&new_project.id, &conn).unwrap();
    print!("Project {:?}", project);
    let new_domain = db::models::Document::create(&project.id, "Yay", &conn).unwrap();
    print!("Dom {:?}", new_domain);


}
