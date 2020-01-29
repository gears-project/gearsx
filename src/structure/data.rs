
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum VType {
    VTypeString(VTypeString),
    VTypeBoolean(VTypeBoolean),
    VTypeInteger(VTypeInteger),
}

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

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct VTypeStringContainer {
    value: String,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct VTypeBoolean {
    default: Option<bool>,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct VTypeBooleanContainer {
    value: bool,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct VTypeInteger {
    default: Option<i32>,
    min: Option<i32>,
    max: Option<i32>,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct VTypeIntegerContainer {
    value: i32,
}

pub trait VTypeI {
    type Container;
    fn new_instance(&self) -> Self::Container;
}

impl VTypeI for VTypeString {
    type Container = VTypeStringContainer;

    fn new_instance(&self) -> VTypeStringContainer {
        VTypeStringContainer {
            value: self.default.as_ref().unwrap_or(&format!("")).clone(),
        }
    }
}

mod test {
    use super::VTypeI;
    #[test]
    fn test_vtype_string() {
        let vtype = super::VTypeString {
            default: Some((&"thing").to_string()),
        };

        let vval = vtype.new_instance();
        assert_eq!(vval.value, "thing");
    }
}