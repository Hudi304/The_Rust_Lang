use serde_json::Value;
use std::fmt;
use std::fmt::Debug;

use crate::{utils::type_utils::extract_type, Import};

// TODO this might be a good idea when
// pub enum PropType {
//     Bool,
//     String,
//     Number,
//     Object,
// }

pub struct PropSchema {
    pub name: String,
    pub prop_type: String,
    pub import: Option<Import>,
}

impl Debug for PropSchema {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} - ({:?})", self.name, self.prop_type, self.import)
    }
}

impl PropSchema {
    pub fn build((prop_name, prop_value): (&String, &Value)) -> PropSchema {
        let (prop_type, import_option) = extract_type(prop_value, "items");

        PropSchema {
            name: prop_name.clone(),
            prop_type,
            import: import_option,
        }
    }
}
