use crate::messages::DocumentProperties;
use serde;
use serde_json;
use serde_yaml;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Document<T> {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub doctype: String,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub body: T,
}

pub struct RawDocument<'a> {
    pub id: &'a Uuid,
    pub project_id: &'a Uuid,
    pub name: &'a str,
    pub doctype: &'a str,
    pub version: &'a i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub body: serde_json::Value,
}

#[macro_export]
macro_rules! gears_doc {
    ($source:ty, $name:ident, $doctype:expr) => {
        pub type $name = Document<$source>;

        impl Default for $name {
            fn default() -> Self {
                Self {
                    id: Uuid::new_v4(),
                    project_id: crate::util::naming::empty_uuid(),
                    name: "New".to_owned(),
                    doctype: stringify!($doctype).to_owned(),
                    version: 0,
                    created_at: NaiveDateTime::from_timestamp(0, 0),
                    updated_at: NaiveDateTime::from_timestamp(0, 0),
                    body: <$source>::default(),
                }
            }
        }
    };
}

// pub type DocumentList<T> = Vec<Document<T>>;

#[derive(GraphQLObject, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DocumentReference {
    pub id: Uuid,
    pub doctype: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ModelLoadError {
    UnParseable(String),
    BadStructure(String),
}

impl<T> Document<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned + Eq + Default,
{
    pub fn new(project_id: &Uuid, doctype: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            project_id: project_id.clone(),
            name: "default".to_owned(),
            doctype: doctype,
            version: 0,
            created_at: NaiveDateTime::from_timestamp(0, 0),
            updated_at: NaiveDateTime::from_timestamp(0, 0),
            body: <T>::default(),
        }
    }

    pub fn change_props(&mut self, props: &DocumentProperties) -> &Self {
        if let Some(name) = &props.name {
            self.name = name.clone();
        }
        self
    }

    pub fn as_raw(&self) -> RawDocument {
        RawDocument {
            id: &self.id,
            project_id: &self.project_id,
            name: &self.name,
            doctype: &self.doctype,
            version: &self.version,
            created_at: self.created_at,
            updated_at: self.updated_at,
            body: serde_json::to_value(&self.body).unwrap(),
        }
    }

    /// Return a string representation of the Document
    ///
    pub fn to_string(&self) -> String {
        format!("document {}", self.id)
    }

    /// Return a summary of the Document
    ///
    pub fn summary(&self) -> String {
        format!("Doc {:?} - {:?} - {:?}", self.doctype, self.id, self.name)
    }

    /// Return an indented JSON representation of the Document
    ///
    /// partof: SPC-serialization-json
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    /// Return a compact JSON representation of the Document
    ///
    /// partof: SPC-serialization-json
    pub fn to_json_compact(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// Initialize a Document from a JSON string
    ///
    /// partof: SPC-serialization-json
    pub fn from_json(s: &str) -> Result<Self, ModelLoadError> {
        match serde_json::from_str(s) {
            Ok(res) => Ok(res),
            Err(err) => {
                let msg = format!("{}", err);
                Err(ModelLoadError::BadStructure(msg))
            }
        }
    }

    /// Update a Document from a JSON string
    ///
    /// partof: SPC-serialization-json
    pub fn update_from_json(&mut self, s: &str) -> Result<&Self, String> {
        let value = serde_json::from_str(s).unwrap();
        *self = serde_json::from_value(value).unwrap();
        Ok(self)
    }

    /// Return a YAML representation of the Document
    ///
    /// partof: #SPC-serialization-yaml
    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }

    /// Initialize a Document from a JSON string
    ///
    /// partof: SPC-serialization-yaml
    pub fn from_yaml(s: &str) -> Result<Self, ModelLoadError> {
        match serde_yaml::from_str(s) {
            Ok(res) => Ok(res),
            Err(err) => {
                let msg = format!("{}", err);
                Err(ModelLoadError::BadStructure(msg))
            }
        }
    }

    /// Update a Document from a YAML string
    ///
    /// partof: SPC-serialization-yaml
    pub fn update_from_yaml(&mut self, s: &str) -> Result<&Self, String> {
        let value = serde_yaml::from_str(s).unwrap();
        *self = serde_yaml::from_value(value).unwrap();
        Ok(self)
    }

    pub fn change(&mut self) -> i32 {
        self.version += 1;
        self.version
    }
}

/*
impl<T> Default for Document<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            project_id: crate::util::naming::empty_uuid(),
            name: "default".to_owned(),
            doctype: "none".to_owned(),
            version: 0,
            body: <T>::default(),
        }
    }

}
*/
