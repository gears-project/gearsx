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
}
