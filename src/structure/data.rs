use std::collections::{HashMap, HashSet};
use serde_tuple::*;

#[derive(GraphQLObject, Serialize_tuple, Deserialize_tuple, Debug, Clone, Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct VariableDefinition {
    pub id: i32,
    pub name: String,
    pub vtype: VType,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum VType {
    #[serde(rename = "string")]
    VTypeString(VTypeString),
    #[serde(rename = "boolean")]
    VTypeBoolean(VTypeBoolean),
    #[serde(rename = "integer")]
    VTypeInteger(VTypeInteger),
}

graphql_union!(VType: () where Scalar = <S> |&self| {
    instance_resolvers: |_| {
        &VTypeString => match *self { VType::VTypeString(ref h) => Some(h), _ => None },
        &VTypeBoolean => match *self { VType::VTypeBoolean(ref h) => Some(h), _ => None },
        &VTypeInteger => match *self { VType::VTypeInteger(ref h) => Some(h), _ => None },
    }
});

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum VTypeContainer {
    VTypeStringContainer(VTypeStringContainer),
    VTypeBooleanContainer(VTypeBooleanContainer),
    VTypeIntegerContainer(VTypeIntegerContainer),
}

graphql_union!(VTypeContainer: () where Scalar = <S> |&self| {
    instance_resolvers: |_| {
        &VTypeStringContainer => match *self { VTypeContainer::VTypeStringContainer(ref h) => Some(h), _ => None },
        &VTypeBooleanContainer => match *self { VTypeContainer::VTypeBooleanContainer(ref h) => Some(h), _ => None },
        &VTypeIntegerContainer => match *self { VTypeContainer::VTypeIntegerContainer(ref h) => Some(h), _ => None },
    }
});

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[graphql(name = "string")]
pub struct VTypeString {
    pub default: Option<String>,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct VTypeStringContainer {
    pub value: String,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct VTypeBoolean {
    pub default: Option<bool>,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct VTypeBooleanContainer {
    pub value: bool,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct VTypeInteger {
    pub default: Option<i32>,
    pub min: Option<i32>,
    pub max: Option<i32>,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct VTypeIntegerContainer {
    pub value: i32,
}

pub trait VTypeVariableDefinition {
    type Container;
    fn new_instance(&self) -> Self::Container;
    fn is_consistent(&self) -> bool {
        true
    }
}

impl VTypeVariableDefinition for VTypeString {
    type Container = VTypeStringContainer;

    fn new_instance(&self) -> Self::Container {
        Self::Container {
            value: self.default.as_ref().unwrap_or(&format!("")).clone(),
        }
    }
}

impl VTypeVariableDefinition for VTypeBoolean {
    type Container = VTypeBooleanContainer;

    fn new_instance(&self) -> Self::Container {
        Self::Container {
            value: self.default.unwrap_or(false),
        }
    }
}

impl VTypeVariableDefinition for VTypeInteger {
    type Container = VTypeIntegerContainer;

    fn new_instance(&self) -> Self::Container {
        Self::Container {
            value: self.default.unwrap_or(0),
        }
    }

    fn is_consistent(&self) -> bool {
        let x = self.default.unwrap_or(0);
        if let Some(min) = self.min {
            if min > x {
                return false;
            }
        }
        if let Some(max) = self.max {
            if max < x {
                return false;
            }
        }
        true
    }
}

pub type VariableDefinitions = Vec<VariableDefinition>;

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct DocumentVariables {
    pub input: VariableDefinitions,
    pub local: VariableDefinitions,
    pub output: VariableDefinitions,
}

impl Default for DocumentVariables {
    fn default() -> Self {
        Self {
            input: Vec::<VariableDefinition>::new(),
            local: Vec::<VariableDefinition>::new(),
            output: Vec::<VariableDefinition>::new(),
        }
    }
}

impl DocumentVariables {
    pub fn get_all_ids(&self) -> HashSet<i32> {
        let mut ids = HashSet::<i32>::new();

        let _ = self.input.iter().map(|p| {
            ids.insert(p.id)
        });

        unimplemented!();

        ids
    }

//    /// Get a `HashSet` of all variable names in input, local and output
//    ///
//    /// # Example
//    /// ```
//    /// use gears::structure::xflow::{XFlow};
//    /// let xfs = XFlow::default();
//    /// let names = xfs.get_all_variable_names();
//    /// assert_eq!(names.len(), 0);
//    /// ```
//    pub fn all_variable_names(&self) -> HashSet<String> {
//        let mut vars = HashSet::<String>::new();
//
//        for xvar in &self.variables.input {
//            vars.insert(xvar.name.clone());
//        }
//
//        for xvar in &self.variables.local {
//            vars.insert(xvar.name.clone());
//        }
//
//        for xvar in &self.variables.output {
//            vars.insert(xvar.name.clone());
//        }
//
//        vars
//    }

}

mod test {
    #[test]
    fn test_vtype_string() {
        use super::VTypeVariableDefinition;
        let vtype = super::VTypeString {
            default: Some((&"thing").to_string()),
        };

        let vval = vtype.new_instance();
        assert_eq!(vval.value, "thing");
    }

    #[test]
    fn test_vtype_boolean() {
        use super::VTypeVariableDefinition;
        let vtype = super::VTypeBoolean {
            default: Some(true),
        };

        let vval = vtype.new_instance();
        assert_eq!(vval.value, true);
    }

    #[test]
    fn test_vtype_integer() {
        use super::VTypeVariableDefinition;
        let vtype = super::VTypeInteger {
            default: Some(200),
            min: Some(0),
            max: Some(500),
        };

        let vval = vtype.new_instance();
        assert_eq!(vval.value, 200);
    }
}
