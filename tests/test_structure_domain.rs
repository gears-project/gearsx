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

#[test]
fn test_add_and_remove_entities() {
    let _ = env_logger::try_init();

    let mut domain = Domain::default();
    let order_entity = domain.add_entity("Order").expect("Expect to be able to add an Order Entity");
    assert_eq!(domain.entities.len(), 1);
    let order_item_entity = domain.add_entity("OrderItem").expect("Expect to be able to add an OrderItem Entity");
    assert_eq!(domain.entities.len(), 2);
    domain.remove_entity(order_item_entity.id).expect("Expect to be able to remove an OrderItem Entity");
    assert_eq!(domain.entities.len(), 1);
    domain.remove_entity(order_entity.id).expect("Expect to be able to remove an Order Entity");
    assert_eq!(domain.entities.len(), 0);

}

#[test]
fn test_add_and_remove_entities_error_scenarios() {
    let _ = env_logger::try_init();

    let mut domain = Domain::default();

    let order_entity = domain.add_entity("Order").expect("Expect to be able to add an Order Entity");
    assert_eq!(domain.entities.len(), 1);

    let order_entity_2 = domain.add_entity("Order");
    assert_eq!(order_entity_2, Err(DomainError::EntityAlreadyExists("Order".into())));

    let remove_entity = domain.remove_entity(55);
    assert_eq!(remove_entity, Err(DomainError::EntityDoesNotExist(55)));

    let remove_entity_2 = domain.remove_entity(order_entity.id);
    assert_eq!(remove_entity_2, Ok(()));
}
