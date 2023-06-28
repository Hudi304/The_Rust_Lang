use serde_json::{Map, Value};
use std::fmt::{self, Debug};

use super::model_prop::{PropType, PropertySchema};

#[derive(Debug)]
struct ModelSchema {
    name: String,
    props: Vec<PropertySchema>,
    schema_type: String,
}

// impl Debug for ModelSchema {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let self_clone = self.clone();
//         let props_string = self
//             .props
//             .clone()
//             .into_iter()
//             .map(|prop| prop.name.clone().to_owned() + " " + prop.prop_type.clone().as_str())
//             .collect::<Vec<String>>()
//             .join("\n");
//         write!(f, "{} {}", self_clone.name, props_string)
//     }
// }

impl ModelSchema {
    fn build((model_name, model_properties): (String, Value)) -> ModelSchema {
        let model_name = clean_model_name(&model_name);

        let model_properties = model_properties
            .as_object()
            .expect("Model Schema properties is not and object");

        let schema_type = model_properties.get("type").unwrap();
        let properties = model_properties.get("properties").unwrap();
        let add_props = model_properties.get("additionalProperties").unwrap();
        let properties = properties.as_object().unwrap();

        let props_schema_vector: Vec<PropertySchema> = properties
            .into_iter()
            .map(|(key, value)| PropertySchema::build((key, value)))
            .collect();

        return ModelSchema {
            schema_type: schema_type.to_string(),
            name: model_name,
            props: props_schema_vector,
        };
    }
}

fn clean_model_name(name: &String) -> String {
    return name.replace("`", "");
}

fn get_default_value() {
    !todo!();
}

fn get_constructor() {
    !todo!();
}

pub fn extract_models(models: &Map<String, Value>) {
    for (name, value) in models.iter() {
        let model_clone = (name.clone(), value.clone());
        let model_schema = ModelSchema::build(model_clone);

        // println!("{:#?}", model_schema);
    }
}
