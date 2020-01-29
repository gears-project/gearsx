use crate::structure::common::ModelLoadError;
use crate::structure::domain::DomainDocument;
use crate::structure::modelx::ModelxDocument;

use std;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_json_file(path: &Path) -> String {
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, Error::description(&why));
    };

    s
}

fn write_file(filename: &str, data: &str) -> () {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => {
            error!("couldn't create {}: {}", display, why.description());
            panic!("couldn't create {}: {}", display, why.description());
        }
        Ok(file) => file,
    };

    match file.write_all(data.as_bytes()) {
        Err(why) => {
            error!("couldn't write to {}: {}", display, why.description());
            panic!("couldn't write to {}: {}", display, why.description());
        }
        Ok(_) => debug!("successfully wrote to {}", display),
    }
}

fn create_dir(path: &str) -> () {
    debug!("Creating directory '{:?}'", path);
    if !Path::new(path).exists() {
        match std::fs::create_dir(&path) {
            Ok(_) => {
                debug!("Created directory '{:?}' : OK", path);
            }
            Err(_) => {
                error!("Error creating directory '{:?}'", path);
            }
        };
    } else {
        debug!("Directory '{:?}' exists, not creating", path);
    }
}

pub fn model_to_fs(model: &ModelxDocument, path: &str) -> Result<(), ModelLoadError> {
    // XXX Error handling, assumption checking

    debug!(
        "Writing model id:'{}', version:'{}' to directory '{}'",
        model.id, model.version, path
    );

    let model_doc_filename = format!("{}/modelx.json", path);
    write_file(&model_doc_filename, &model.to_json());

    /*
    let doc_filename = format!("{}/domain.json", path);
    write_file(&doc_filename, &model.body.domain.to_json());
    */

    Ok(())
}

pub fn model_from_fs(path: &str) -> Result<ModelxDocument, ModelLoadError> {
    // XXX Error handling, assumption checking

    debug!("Reading model from directory '{}'", path);
    let model_filename = format!("{}/modelx.json", path);
    let model_path = Path::new(&model_filename);
    let model_json = read_json_file(model_path);
    debug!(
        "model_from_fs : Deserializing model JSON from {}",
        model_filename
    );
    let modeldoc: ModelxDocument = match ModelxDocument::from_json(&model_json) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    /*
    let domain_filename = format!("{}/domain.json", path);
    let domain_path = Path::new(&domain_filename);
    let json = read_json_file(domain_path);
    debug!("model_from_fs : Deserializing domain JSON from {}", domain_filename);
    let _domain: DomainDocument = match DomainDocument::from_json(&json) {
        Ok(res) => res,
        Err(err) => return Err(err)
    };
    */

    Ok(modeldoc)
}

pub fn init_new_model_dir(path: &str) -> Result<(), ModelLoadError> {
    create_dir(path);
    let model = ModelxDocument::new(&crate::util::naming::empty_uuid(), "modelx".into());
    model_to_fs(&model, &path)
}

pub fn is_model_dir(path: &str) -> bool {
    model_from_fs(path).is_ok()
}
