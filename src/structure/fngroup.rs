#[macro_use]
use crate as root;
use super::common::{Document, DocumentReference};
use super::data::*;
use crate::messages::{FnGroupFnNew, FnGroupFnUpdate};
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
    AssemblyScript,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct FnDefinition {
    pub id: i32,
    pub lang: FnLanguage,
    pub name: String,
    pub description: String,
    pub input: VariableDefinitions,
    pub output: VariableDefinitions,
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

impl Fngroup {
    fn next_id(&self) -> i32 {
        let mut ids = Vec::<i32>::new();
        ids.append(&mut self.fns.iter().map(|e| e.id).collect());
        let id = ids.iter().max().unwrap_or(&0).to_owned() + 1;
        id
    }

    pub fn add_fn(&mut self, name: &str) -> Result<FnDefinition, FngroupError> {
        let f = FnDefinition {
            id: self.next_id(),
            name: name.to_owned(),
            description: "No description".to_owned(),
            body: "".to_owned(),
            lang: FnLanguage::AssemblyScript,
            input: VariableDefinitions::new(),
            output: VariableDefinitions::new(),
        };
        self.fns.push(f.clone());
        Ok(f)
    }

    pub fn update_fn(&mut self, input: &FnGroupFnUpdate) -> Result<FnDefinition, FngroupError> {
        let mut f = self.get_fn_mut(input.id)?;
        if let Some(name) = &input.name {
            f.name = name.to_string();
        }
        if let Some(description) = &input.description {
            f.description = description.to_string();
        }
        if let Some(body) = &input.body {
            f.body = body.to_string();
        }
        Ok(f.clone())
    }

    pub fn get_fn_mut(&mut self, id: i32) -> Result<&mut FnDefinition, FngroupError> {
        let index = self.fns.iter().position({ |e| e.id.eq(&id) });

        match index {
            Some(idx) => Ok(self.fns.get_mut(idx).unwrap()),
            None => Err(FngroupError::FnDoesNotExist(id)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::FngroupDocument;
    #[test]
    fn test_init_fngroup() {
        let doc = FngroupDocument::new(&crate::util::naming::empty_uuid(), "fngroup".into());
        assert_eq!(doc.doctype, "fngroup");
    }

    #[test]
    fn test_next_id() {
        let doc = FngroupDocument::new(&crate::util::naming::empty_uuid(), "docstore".into());
        assert_eq!(doc.doctype, "fngroup");
    }
}
