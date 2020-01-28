use super::common::{Document, DocumentReference};
use uuid::Uuid;
use std::error;
use std::fmt;

pub type DomainDocument = Document<Domain>;

#[derive(Debug)]
pub enum DomainError {
    EntityDoesNotExist(String),
    AttributeDoesNotExist(String, String),
    ReferenceDoesNotExist(String, String),
    EntityAlreadyExists(String),
    AttributeAlreadyExists(String, String),
    ReferenceAlreadyExists(String, String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DomainError::EntityDoesNotExist(e) => {
                write!(f, "Entity {} does not exist", e)
            },
            DomainError::AttributeDoesNotExist(e, a) => {
                write!(f, "Attribute {} does not exist in Enitity {}", a, e)
            },
            DomainError::ReferenceDoesNotExist(e, r) => {
                write!(f, "Reference {} does not exist in Entity {}", r, e)
            },
            DomainError::EntityAlreadyExists(e) => {
                write!(f, "Entity {} already exists", e)
            },
            DomainError::AttributeAlreadyExists(e, a) => {
                write!(f, "Attribute {} already exists in Enitity {}", a, e)
            },
            DomainError::ReferenceAlreadyExists(e, r) => {
                write!(f, "Reference {} already exists in Entity {}", r, e)
            },
        }
    }
}

// This is important for other errors to wrap this one.
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
    pub vtype: String,
    pub default: String,
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
    pub fn new(id: i32, name: &str, attr_type: &str) -> Self {
        Attribute {
            id: id,
            name: name.to_string(),
            vtype: attr_type.to_string(),
            default: "".to_string(),
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
        let res : Vec<&Entity>= self.entities
            .iter()
            .filter(|&e| { e.id.eq(&id)})
            .collect();
        res.len() == 1
    }

    pub fn get_entity(&mut self, id: i32) -> Result<&Entity, DomainError> {
        let res: Vec<&Entity> = self
            .entities
            .iter()
            .filter({ |e| e.id.eq(&id) })
            .collect();
        if res.len() == 1 {
            Ok(&res[0])
        } else {
            Err(
                DomainError::EntityDoesNotExist(format!("Entity id:{} does not exist", id))
            )
        }
    }

    pub fn get_entity_mut(&mut self, id: i32) -> Result<&mut Entity, DomainError> {
        let index = self
            .entities
            .iter()
            .position({ |e| e.id.eq(&id) });

        match index {
            Some(idx) => {
                Ok(self.entities.get_mut(idx).unwrap())
            },
            None => {
                Err(
                    DomainError::EntityDoesNotExist(format!("Entity id:{} does not exist", id))
                )
            }
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
            Err(
                DomainError::EntityDoesNotExist(
                    format!("Entity {} does not exist", name)
                )
            )
        }
    }

    pub fn add_entity(&mut self, name: &str) -> Result<Entity, DomainError> {
        if self.has_entity_name(&name) {
            Err(
                DomainError::EntityDoesNotExist(
                    format!("Entity {} already exists", name)
                )
            )
        } else {
            let entity = Entity::new(self.next_id(), name);
            self.entities
                .push(entity.clone());
            Ok(entity)
        }
    }

    pub fn remove_entity(&mut self, entity: &str) -> Result<(), String> {
        let entities = self.entities.clone();

        self.entities = entities
            .into_iter()
            .filter({ |e| e.name.ne(entity) })
            .collect();

        Ok(())
    }

    pub fn entity_add_attribute(&mut self, entity_id: i32, name: &str) -> Result<Attribute, DomainError> {
        let id = self.next_id();

        match self.get_entity_mut(entity_id) {
            Ok(entity) => {
                let attribute = Attribute::new(id, name, &"default");
                entity.attributes.push(attribute.clone());
                Ok(attribute)
            }
            Err(err) => Err(err)
        }
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
        let doc = DomainDocument::new(
            &crate::util::naming::empty_uuid(),
            "domain".into()
        );
        assert_eq!(doc.doctype, "domain");
            
    }


}
