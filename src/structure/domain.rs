use uuid::Uuid;
use super::common::{Document, DocumentReference};

pub type DomainDocument = Document<Domain>;

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

#[derive(GraphQLObject)]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Domain {
    pub events: Events,
    pub entities: Entities,
}

#[derive(GraphQLObject)]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
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

#[derive(GraphQLObject)]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Validation {
    pub message: String,
    pub xflow: DocumentReference,
}

#[derive(GraphQLObject)]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Attribute {
    pub id: i32,
    pub name: String,
    pub vtype: String,
    pub default: String,
    pub validations: Vec<Validation>,
}

#[derive(GraphQLEnum)]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum ReferenceType {
    #[serde(rename = "has_many")]
    HasMany,
    #[serde(rename = "belongs_to")]
    BelongsTo,
}

#[derive(GraphQLObject)]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Reference {
    pub id: i32,
    pub name: String,
    pub reftype: ReferenceType,
    pub other: String,
}

#[derive(GraphQLObject)]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
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
            name: name.to_string().clone(),
            vtype: attr_type.to_string().clone(),
            default: "".to_string(),
            validations: Validations::new(),
        }
    }
}

impl Domain {
    pub fn next_id(&self) -> Result<i32, String> {
        let entity_ids: Vec<i32> = self.entities
            .iter()
            .map(|e| e.id)
            .collect();
        Ok(entity_ids.iter().max().unwrap_or(&0).to_owned() + 1)
    }

    pub fn has_entity(&mut self, name: &str) -> bool {
        match self.get_entity(name) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn get_entity(&mut self, name: &str) -> Result<&Entity, String> {
        let mut res: Vec<&Entity> = self.entities
            .iter()
            .filter({
                |e| e.name.eq(name)
            })
            .collect();
        if res.len() == 1 {
            Ok(&mut res[0])
        } else {
            Err(format!("Entity {} does not exist", name))
        }
    }

    pub fn add_entity(&mut self, name: &str) -> Result<(), String> {
        if self.has_entity(&name) {
            Err(format!("Entity {} already exists", name))
        } else {
            self.entities.push(Entity::new(self.next_id().unwrap(), name));
            Ok(())
        }
    }

    pub fn remove_entity(&mut self, entity: &str) -> Result<(), String> {
        let entities = self.entities.clone();

        self.entities = entities
            .into_iter()
            .filter({
                |e| e.name.ne(entity)
            })
            .collect();

        Ok(())
    }
}

/*
impl Default for Entity {
    fn default() -> Self {
        Entity {
            name: "".to_owned(),
            attributes: Attributes::new(),
            references: References::new(),
        }
    }
}
*/

impl Entity {
    pub fn new(id: i32, name: &str) -> Self {
        Entity {
            id: id,
            name: name.to_string().to_owned(),
            attributes: Attributes::new(),
            references: References::new(),
        }
    }

    pub fn get_attribute(self, name: &str) -> Result<Attribute, String> {
        let res: Vec<&Attribute> = self.attributes
            .iter()
            .filter({
                |e| e.name.eq(name)
            })
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

