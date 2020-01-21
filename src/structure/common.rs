use serde;
use serde_json;
use serde_yaml;
use uuid::Uuid;

#[derive(Queryable)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Document<T> {
    pub id: Uuid,
    pub name: String,
    pub doctype: String,
    pub version: i32,
    pub body: T,
}

pub type DocumentList<T> = Vec<Document<T>>;

#[derive(GraphQLObject)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DocumentReference {
    pub id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ModelLoadError {
    UnParseable(String),
    BadStructure(String),
}

impl<T> Document<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned + Eq + Default
{
    pub fn new_from_header(header: &DocumentHeader) -> Self {
        Self {
            id: header.id.clone(),
            name: header.name.clone(),
            doctype: header.doctype.clone(),
            version: header.version.clone(),
            body: <T>::default(),
        }
    }

    pub fn get_header(&self) -> DocumentHeader {
        DocumentHeader {
            id: self.id.clone(),
            name: self.name.clone(),
            doctype: self.doctype.clone(),
            version: self.version.clone(),
        }
    }

    pub fn set_header(&mut self, header: &DocumentHeader) -> () {
        self.id = header.id.clone();
        self.name = header.name.clone();
        self.doctype = header.doctype.clone();
        self.version = header.version.clone();
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
        self.version = self.version + 1;
        self.version
    }

}

impl<T> Default for Document<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "default".to_owned(),
            doctype: "none".to_owned(),
            version: 0,
            body: <T>::default(),
        }
    }
}

//
// This struct only exists to make the top-level Model object serializable into a project's
// model.json
//

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct DocumentHeader {
    pub id: Uuid,
    pub name: String,
    pub doctype: String,
    pub version: i32,
}

impl DocumentHeader {
    /// Return a string representation of the DocumentHeader
    ///
    pub fn to_string(&self) -> String {
        format!("document {}", self.id)
    }

    /// Return an indented JSON representation of the DocumentHeader
    ///
    /// partof: SPC-serialization-json
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    /// Return a compact JSON representation of the DocumentHeader
    ///
    /// partof: SPC-serialization-json
    pub fn to_json_compact(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// Initialize a DocumentHeader from a JSON string
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

    /// Return a YAML representation of the DocumentHeader
    ///
    /// partof: SPC-serialization-yaml
    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }

    /// Initialize a DocumentHeader from a JSON string
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
}
