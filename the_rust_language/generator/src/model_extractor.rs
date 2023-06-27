use serde_json::{Map, Value};

fn clean_model_name(name: &String) -> String {
    return name.replace("`", "");
}

pub enum PropType {
    Bool,
    String,
    Number,
    Object,
}

pub fn extract_models(models: &Map<String, Value>) {
    for (name, value) in models.iter() {
        let model_name = clean_model_name(name);
        println!("{}", model_name);

        let value = value.as_object().unwrap();

        let properties = value.get("properties").unwrap();
        let schema_type = value.get("type").unwrap();
        let add_props = value.get("additionalProperties").unwrap();

        let props = properties.as_object().unwrap();
        for (key, value) in props.into_iter() {
            let prop_name = key;

            let value = value.as_object().unwrap();

            let type_option = value.get("type");

            let mut prop_type: String = "".to_owned();

            if (type_option.is_some()) {
                println!("  {} : {:?}", key, type_option.unwrap().as_str().unwrap());
                prop_type = type_option.unwrap().as_str().unwrap().to_owned();
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
                prop_type = reference;
            }

            println!("  {} : {} ", prop_name, prop_type);
        }
    }
}
