use serde_json::{Map, Value};
use std::fmt::{self, Debug};

use super::model_prop::PropSchema;
use crate::utils::type_utils::clean_model_name;

struct ModelSchema {
    name: String,
    props: Vec<PropSchema>,
    // schema_type: String,
}

impl Debug for ModelSchema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {{\n  props : {:#?} }}", self.name, self.props)
    }
}

impl ModelSchema {
    fn build((model_name, model_properties): (&String, &Value)) -> ModelSchema {
        let model_name = clean_model_name(&model_name);

        let model_properties = model_properties
            .as_object()
            .expect("Model Schema properties is not and object");

        let properties = model_properties.get("properties").unwrap();
        let properties = properties.as_object().unwrap();
        // let add_props = model_properties.get("additionalProperties").unwrap();

        let props_schema_vector: Vec<PropSchema> = properties
            .into_iter()
            .map(|(key, value)| PropSchema::build((key, value)))
            .collect();

        return ModelSchema {
            name: model_name,
            props: props_schema_vector,
        };
    }
}

// fn get_constructor() {
//     !todo!();
// }

pub fn extract_models(models: &Map<String, Value>) {
    for (name, value) in models.iter() {
        let _model_schema = ModelSchema::build((name, value));
        // println!("{:#?}", _model_schema);
    }
}
