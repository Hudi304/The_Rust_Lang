pub mod model_generator {
    use serde_json::{Map, Value};

    pub fn filter_models(components: &Map<String, Value>)
    // -> Vec<(&String, &Value)>
    {
        let schemas = components.get("schemas").unwrap();
        let schema_obj = schemas.as_object().unwrap();

        let models: Vec<(&String, &Value)> = schema_obj
            .into_iter()
            .filter(|(_, values)| values.get("type").unwrap() == "object")
            .collect();

        for (model_name, values) in models.into_iter() {
            println!("  {} ", model_name);
        }

        


    }
}
