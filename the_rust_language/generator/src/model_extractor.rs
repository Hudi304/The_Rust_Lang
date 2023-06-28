use serde_json::{Map, Value};

#[derive(Debug)]
pub enum PropType {
    Bool,
    String,
    Number,
    Object,
}
#[derive(Debug)]
struct PropertySchema {
    prop_type: PropType,
    is_array: bool,
}

impl PropertySchema {
    fn build((key, value): (&String, &Value)) -> PropertySchema {
        let prop_name = key;
        let value = value.as_object().unwrap();
        let type_option = value.get("type");
        let mut prop_type_string: String = "".to_owned();

        if (type_option.is_some()) {
            println!("  {} : {:?}", key, type_option.unwrap().as_str().unwrap());
            prop_type_string = type_option.unwrap().as_str().unwrap().to_owned();
        } else {
            let reference = value
                .get("$ref")
                .unwrap()
                .as_str()
                .unwrap()
                .split("/")
                .last()
                .unwrap()
                .to_owned();
            prop_type_string = reference;
        }

        let prop_type = match prop_type_string.as_str() {
            "string" => PropType::String,
            "number" => PropType::Number,
            "bool" => PropType::Bool,
            "object" => PropType::Object,
            something_else => panic!("unmatched type {:?}", something_else),
        };

        return PropertySchema {
            prop_type,
            is_array: false,
        };
    }
}

#[derive(Debug)]
struct ModelSchema {
    name: String,
    props: Vec<PropertySchema>,
    schema_type: String,
}

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

        println!("{:?}", model_schema);
    }
}
