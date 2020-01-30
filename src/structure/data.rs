#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum VType {
    VTypeString(VTypeString),
    VTypeBoolean(VTypeBoolean),
    VTypeInteger(VTypeInteger),
}

/*
#[juniper::graphql_union]
impl<'a> GraphQLUnion for &'a dyn VTypePrimitive {
    fn resolve(&self) {
        match self {
            VT => self.as_human(),
            Droid => self.as_droid(),
        }
    }
}
*/

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum VTypeContainer {
    VTypeStringContainer(VTypeStringContainer),
    VTypeBooleanContainer(VTypeBooleanContainer),
    VTypeIntegerContainer(VTypeIntegerContainer),
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct VTypeString {
    default: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct VTypeStringContainer {
    value: String,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct VTypeBoolean {
    default: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct VTypeBooleanContainer {
    value: bool,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct VTypeInteger {
    default: Option<i32>,
    min: Option<i32>,
    max: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct VTypeIntegerContainer {
    value: i32,
}

pub trait VTypePrimitive {
    type Container;
    fn new_instance(&self) -> Self::Container;
    fn is_consistent(&self) -> bool {
        true
    }
}

impl VTypePrimitive for VTypeString {
    type Container = VTypeStringContainer;

    fn new_instance(&self) -> Self::Container {
        Self::Container {
            value: self.default.as_ref().unwrap_or(&format!("")).clone(),
        }
    }
}

impl VTypePrimitive for VTypeBoolean {
    type Container = VTypeBooleanContainer;

    fn new_instance(&self) -> Self::Container {
        Self::Container {
            value: self.default.unwrap_or(false),
        }
    }
}

impl VTypePrimitive for VTypeInteger {
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
                return false
            }
        }
        if let Some(max) = self.max {
            if max < x {
                return false
            }
        }
        true
    }
}

mod test {
    #[test]
    fn test_vtype_string() {
        use super::VTypePrimitive;
        let vtype = super::VTypeString {
            default: Some((&"thing").to_string()),
        };

        let vval = vtype.new_instance();
        assert_eq!(vval.value, "thing");
    }

    #[test]
    fn test_vtype_boolean() {
        use super::VTypePrimitive;
        let vtype = super::VTypeBoolean {
            default: Some(true),
        };

        let vval = vtype.new_instance();
        assert_eq!(vval.value, true);
    }

    #[test]
    fn test_vtype_integer() {
        use super::VTypePrimitive;
        let vtype = super::VTypeInteger {
            default: Some(200),
            min: Some(0),
            max: Some(500),
        };

        let vval = vtype.new_instance();
        assert_eq!(vval.value, 200);
    }
}