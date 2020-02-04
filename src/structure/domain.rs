use super::common::{Document, DocumentReference};
use crate::messages::AddStringAttributeToEntity;
use super::data::*;
use std::error;
use std::fmt;
use uuid::Uuid;

pub type DomainDocument = Document<Domain>;

impl Default for DomainDocument {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            project_id: crate::util::naming::empty_uuid(),
            name: "New Domain".to_owned(),
            doctype: "domain".to_owned(),
            version: 0,
            body: Domain::default(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DomainError {
    EntityDoesNotExist(i32),
    AttributeDoesNotExist(i32, i32),
    ReferenceDoesNotExist(i32, i32),
    EntityAlreadyExists(String),
    AttributeAlreadyExists(i32, String),
    ReferenceAlreadyExists(i32, String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DomainError::EntityDoesNotExist(e) => write!(f, "id:{}", e),
            DomainError::AttributeDoesNotExist(e, a) => write!(f, "id:{}/id:{}", e, a),
            DomainError::ReferenceDoesNotExist(e, r) => write!(f, "id:{}/id:{}", e, r),
            DomainError::EntityAlreadyExists(e) => write!(f, "{}", e),
            DomainError::AttributeAlreadyExists(e, a) => write!(f, "id:{}/{}", e, a),
            DomainError::ReferenceAlreadyExists(e, r) => write!(f, "id:{}/{}", e, r),
        }
    }
}

impl error::Error for DomainError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[juniper::object]
impl DomainDocument {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn doctype(&self) -> &str {
        &self.doctype
    }

    fn body(&self) -> &Domain {
        &self.body
    }
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Domain {
    pub events: Events,
    pub entities: Entities,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Events {
    pub change: Vec<DocumentReference>,
    pub update: Vec<DocumentReference>,
    pub read: Vec<DocumentReference>,
    pub delete: Vec<DocumentReference>,
    pub all: Vec<DocumentReference>,
}

pub type Entities = Vec<Entity>;
pub type Attributes = Vec<Attribute>;
pub type References = Vec<Reference>;
pub type Validations = Vec<Validation>;

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Validation {
    pub message: String,
    pub xflow: DocumentReference,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Attribute {
    pub id: i32,
    pub name: String,
    pub vtype: VType,
    pub validations: Vec<Validation>,
}

#[derive(GraphQLEnum, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum ReferenceType {
    #[serde(rename = "has_many")]
    HasMany,
    #[serde(rename = "belongs_to")]
    BelongsTo,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Reference {
    pub id: i32,
    pub name: String,
    pub reftype: ReferenceType,
    pub other: String,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Entity {
    pub id: i32,
    pub name: String,
    pub attributes: Attributes,
    pub references: References,
}

impl Default for Events {
    fn default() -> Self {
        Events {
            change: Vec::<DocumentReference>::new(),
            update: Vec::<DocumentReference>::new(),
            read: Vec::<DocumentReference>::new(),
            delete: Vec::<DocumentReference>::new(),
            all: Vec::<DocumentReference>::new(),
        }
    }
}

impl Attribute {
    pub fn new(id: i32, name: &str, vtype: VType) -> Self {
        Attribute {
            id: id,
            name: name.to_string(),
            vtype: vtype,
            validations: Validations::new(),
        }
    }
}

impl Domain {
    pub fn next_id(&self) -> i32 {
        let mut ids = Vec::<i32>::new();
        ids.append(&mut self.entities.iter().map(|e| e.id).collect());
        for entity in &self.entities {
            ids.append(&mut entity.attributes.iter().map(|e| e.id).collect());
            ids.append(&mut entity.references.iter().map(|e| e.id).collect());
        }
        let id = ids.iter().max().unwrap_or(&0).to_owned() + 1;
        id
    }

    pub fn has_entity_name(&mut self, name: &str) -> bool {
        self.get_entity_name(name).is_ok()
    }

    pub fn has_entity(&mut self, id: i32) -> bool {
        let res: Vec<&Entity> = self.entities.iter().filter(|&e| e.id.eq(&id)).collect();
        res.len() == 1
    }

    pub fn get_entity(&mut self, id: i32) -> Result<&Entity, DomainError> {
        let res: Vec<&Entity> = self.entities.iter().filter({ |e| e.id.eq(&id) }).collect();
        if res.len() == 1 {
            Ok(&res[0])
        } else {
            Err(DomainError::EntityDoesNotExist(id))
        }
    }

    pub fn get_entity_mut(&mut self, id: i32) -> Result<&mut Entity, DomainError> {
        let index = self.entities.iter().position({ |e| e.id.eq(&id) });

        match index {
            Some(idx) => Ok(self.entities.get_mut(idx).unwrap()),
            None => Err(DomainError::EntityDoesNotExist(id)),
        }
    }

    pub fn get_entity_name(&mut self, name: &str) -> Result<&Entity, DomainError> {
        let res: Vec<&Entity> = self
            .entities
            .iter()
            .filter({ |e| e.name.eq(name) })
            .collect();
        if res.len() == 1 {
            Ok(&res[0])
        } else {
            Err(DomainError::EntityDoesNotExist(0))
        }
    }

    pub fn add_entity(&mut self, name: &str) -> Result<Entity, DomainError> {
        if self.has_entity_name(&name) {
            Err(DomainError::EntityAlreadyExists((&name).to_string()))
        } else {
            let entity = Entity::new(self.next_id(), name);
            self.entities.push(entity.clone());
            Ok(entity)
        }
    }

    pub fn remove_entity(&mut self, id: i32) -> Result<(), DomainError> {
        let entities = self.entities.clone();

        let index = entities.into_iter().position({ |e| e.id.eq(&id) });
        match index {
            Some(idx) => {
                &mut self.entities.remove(idx);
                Ok(())
            }
            None => Err(DomainError::EntityDoesNotExist(id)),
        }
    }

    pub fn entity_add_string_attribute(
        &mut self,
        entity_id: i32,
        message: &AddStringAttributeToEntity,
    ) -> Result<Attribute, DomainError> {

        let id = self.next_id();
        let entity = self.get_entity_mut(entity_id)?;

        let vtype = VTypeString {
            default: message.default.clone(),
        };
        let attribute = Attribute {
            id: id,
            name: message.name.clone(),
            vtype: VType::VTypeString(vtype),
            validations: Validations::new(),
        };
        let attr_clone = attribute.clone();
        entity.attributes.push(attribute);
        Ok(attr_clone)
    }
}

impl Entity {
    fn new(id: i32, name: &str) -> Self {
        Entity {
            id: id,
            name: name.to_string(),
            attributes: Attributes::new(),
            references: References::new(),
        }
    }

    pub fn get_attribute(self, name: &str) -> Result<Attribute, String> {
        let res: Vec<&Attribute> = self
            .attributes
            .iter()
            .filter({ |e| e.name.eq(name) })
            .collect();
        if res.len() == 1 {
            Ok(res[0].clone())
        } else {
            Err(format!("Attribute {} does not exist", name))
        }
    }

    pub fn add_attribute(&mut self, attr: Attribute) -> Result<(), String> {
        self.attributes.push(attr);
        Ok(())
    }
}

impl Default for Domain {
    fn default() -> Self {
        Domain {
            events: Events::default(),
            entities: Entities::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::DomainDocument;
    #[test]
    fn test_init_domain() {
        let doc = DomainDocument::new(&crate::util::naming::empty_uuid(), "domain".into());
        assert_eq!(doc.doctype, "domain");
    }
}
