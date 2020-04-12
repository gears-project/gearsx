#[macro_use]
use crate as root;
use super::common::{Document, DocumentReference};
use super::data::*;
use std::error;
use std::fmt;

root::gears_doc!(Fngroup, FngroupDocument, fngroup);

#[derive(Debug, PartialEq)]
pub enum FngroupError {
    FnDoesNotExist(i32),
}

impl fmt::Display for FngroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FngroupError::FnDoesNotExist(e) => write!(f, "id:{}", e),
        }
    }
}

impl error::Error for FngroupError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[juniper::object]
impl FngroupDocument {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn doctype(&self) -> &str {
        &self.doctype
    }

    fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
    fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }

    fn body(&self) -> &Fngroup {
        &self.body
    }
}
#[derive(GraphQLEnum, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum FnLanguage {
    AssemblyScript
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct FnDefinition {
    pub id: i32,
    pub lang: FnLanguage,
    pub name: String,
    pub description: String,
    pub input: VariableDefinitions,
    pub output: VariableDefinition,
    pub body: String,
}

pub type FnDefinitions = Vec<FnDefinition>;

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Fngroup {
    fns: FnDefinitions,
}

impl Default for Fngroup {
    fn default() -> Self {
        Fngroup {
            fns: FnDefinitions::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::FngroupDocument;
    #[test]
    fn test_init_fngroup() {
        let doc = FngroupDocument::new(&crate::util::naming::empty_uuid(), "docstore".into());
        assert_eq!(doc.doctype, "fngroup");
    }
}
