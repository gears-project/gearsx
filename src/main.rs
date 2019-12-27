#[macro_use]
extern crate clap;
use clap::App;
mod db;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();
}
