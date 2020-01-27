extern crate env_logger;

extern crate gearsx;
use gearsx::structure::domain::*;

mod common;
use crate::common::load_doc;

#[test]
fn test_load_domain() {
    let _ = env_logger::try_init();

    let domain = load_doc::<DomainDocument>("resource/docs/domain/good/basic.json");

    assert_eq!(std::mem::size_of_val(&domain), 232);
    assert_eq!(domain.body.events.change.len(), 0);
    assert_eq!(domain.body.events.update.len(), 0);
    assert_eq!(domain.body.events.read.len(), 0);
    assert_eq!(domain.body.events.delete.len(), 0);
    assert_eq!(domain.body.events.all.len(), 1);

}
