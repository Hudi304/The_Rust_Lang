use serde_json::{Map, Value};
use std::collections::HashSet;

pub const MODELS_PATH: &str = "src/swagger_generator/models/";

pub fn filter_models(components: &Map<String, Value>) -> Vec<(&String, &Value)> {
    let schemas = components.get("schemas").unwrap();
    let schema_obj = schemas.as_object().unwrap();
    let models: Vec<(&String, &Value)> = schema_obj
        .into_iter()
        .filter(|(_, values)| values.get("type").unwrap() == "object")
        .collect();
    return models;
}

pub fn get_model_file_content(
    (model_name, model_values): (&String, &Value),
    models_array: &Vec<(&String, &Value)>,
) -> Option<(String, String)> {
    let properties = match model_values.get("properties") {
        None => return None, //? return early
        Some(properties) => properties,
    };

    let file_path = MODELS_PATH.to_owned() + &model_name + ".model.ts";
    let (attributes, constructor) = get_model_attributes(properties);
    let imports = get_imports(properties, &models_array);
    let declaration = format!("export class {model_name} {{\n");
    let file_content = imports + &declaration + &attributes + &constructor + "}";

    return Some((file_path, file_content));
}

pub fn get_model_attributes(properties: &Value) -> (String, String) {
    let model_properties = match properties.as_object() {
        None => return ("".to_string(), "".to_string()),
        Some(properties) => properties,
    };
    let mut accumulator: String = String::from("");
    let constructor = get_constructor(model_properties);

    for (attribute_key, attribute_value) in model_properties.into_iter() {
        let row = get_attribute_row((attribute_key, attribute_value));
        accumulator = accumulator + row.as_str();
    }

    return (accumulator, constructor);
}

pub fn get_attribute_row((attribute_name, attribute_properties): (&String, &Value)) -> String {
    let attribute_properties: &Map<String, Value> = attribute_properties.as_object().unwrap();
    let attribute_nullable = attribute_properties.get("nullable");

    let attribute_type = get_attribute_type(attribute_properties);

    let attribute_nullable = match attribute_nullable {
        None => "", //? key does not exist
        Some(nullable) => match nullable.as_bool() {
            None => "", //? key exists but parsing to bool fails
            Some(false) => "",
            Some(true) => "?",
        },
    };

    let initial_value = get_initial_value(&attribute_type);

    //TODO handle this with a matching engine
    let attribute_row = format!(
        "  public {attribute_name}{attribute_nullable}: {attribute_type} = {initial_value} \n"
    );

    return attribute_row;
}

pub fn get_reference_type(reference: Option<&Value>) -> Option<String> {
    let reference = match reference {
        None => return None,
        Some(reference) => reference,
    };

    let reference = match reference.as_str() {
        None => return None,
        Some(str) => str,
    };

    let reference_type = match reference.split("/").last() {
        None => return None,
        Some(str) => str,
    };

    return Some(String::from(reference_type));
}

pub fn get_array_type(items: Option<&Value>) -> Option<String> {
    let items = match items {
        None => return None,
        Some(reference) => reference,
    };

    let reference = match items.get("$ref") {
        None => return None,
        Some(reference) => reference,
    };

    let reference = match reference.as_str() {
        None => return None,
        Some(reference) => reference,
    };

    let array_reference_type = match reference.split("/").last() {
        None => return None,
        Some(str) => str,
    };

    let array_reference_type = String::from(format!("{}[]", array_reference_type));

    return Some(array_reference_type);
}

//TODO add matching engine here
pub fn get_initial_value(attribute_type: &String) -> String {
    let type_length = attribute_type.len();
    let is_array = (attribute_type[type_length - 2..]).eq("[]");

    if is_array {
        return String::from("[]");
    }

    match attribute_type.as_str() {
        "integer" | "number" => "0",
        "string" => "\"\"",
        "boolean" => "false",
        "array" => "[]",
        _ => "{} as any;",
    }
    .to_owned()
}

pub fn get_imports(properties: &Value, models: &Vec<(&String, &Value)>) -> String {
    let properties = match properties.as_object() {
        None => return "".to_string(),
        Some(properties) => properties,
    };

    let mut model_imports_set = HashSet::new();
    let mut enum_imports_set = HashSet::new();

    for (_, props_value) in properties.into_iter() {
        let reference_name = get_property_reference_name(props_value);

        if reference_name.is_none() {
            continue;
        }
        let reference_name = reference_name.unwrap();

        let reference_model_name = match reference_name.split("/").last() {
            None => continue,
            Some(str) => str,
        }
        .to_owned();

        if is_model(&reference_model_name, models) {
            let model_import_statement = format!(
                "import {{ {0} }} from '@/common/model/{0}.model` \n",
                reference_model_name
            );
            model_imports_set.insert(model_import_statement);
        } else {
            let enum_import_statement = format!(
                "import {{ {0} }} from '@/common/enums/{0}.enum` \n",
                reference_model_name
            );
            enum_imports_set.insert(enum_import_statement);
        }
    }
    let model_imports_str: String = model_imports_set.into_iter().collect();
    let enum_imports_str: String = enum_imports_set.into_iter().collect();
    let imports = enum_imports_str + model_imports_str.as_str() + "\n";

    return imports;
}

pub fn is_model(reference: &String, models: &Vec<(&String, &Value)>) -> bool {
    let referenced_model = models
        .into_iter()
        .find(|(model_name, _)| (*model_name).eq(reference));

    return match referenced_model {
        None => false,
        Some(_) => true,
    };
}

pub fn get_constructor(model_properties: &Map<String, Value>) -> String {
    let mut constructor = "  constructor(obj = {} as any) {\n ".to_owned();
    constructor += "    obj = obj || {};\n";

    for (attribute_name, attribute_properties) in model_properties.into_iter() {
        let attribute_properties = attribute_properties.as_object().unwrap();
        let attribute_type = get_attribute_type(attribute_properties);
        let ternary_check = format!(
            "obj.{attribute_name} === null ? {{}} as {attribute_type} : obj.{attribute_name};\n"
        );
        constructor += format!("    this.{attribute_name} = {ternary_check} ").as_str();
    }

    constructor += "}\n";
    return constructor;
}

pub fn get_attribute_type(attribute_properties: &Map<String, Value>) -> String {
    let attribute_type = attribute_properties.get("type");
    let attribute_ref = get_all_of(attribute_properties);
    let attribute_items = attribute_properties.get("items");
    //TODO handle error messages better
    let attribute_type = match attribute_type {
        None => match get_reference_type(attribute_ref) {
            None => "any /* Attribute does not have neither type nor $ref */".to_owned(),
            Some(attr_type) => attr_type,
        }, //? key does not exist
        Some(attribute_type) => match attribute_type.as_str() {
            None => "any /* parsing failed */".to_owned(), //? key exists but parsing to &str failed
            Some("integer") => "number".to_owned(),
            Some("number") => "number".to_owned(),
            Some("boolean") => "boolean".to_owned(),
            Some("string") => "string".to_owned(),
            Some("array") => match get_array_type(attribute_items) {
                None => "any[] /* could not get array type */".to_owned(),
                Some(array_type) => array_type,
            },
            Some("formData") => "FormData".to_owned(),
            Some(_) => "any /* I have no idea what this is */".to_owned(),
        },
    };

    return attribute_type;
}

//* takes in
// "contextAccount": {
// "allOf": [
//     {
//       "$ref": "#/components/schemas/Address"
//     }
//   ],
//   "nullable": true
// },
//*returns  "#/components/schemas/Address"
fn get_all_of(attribute_properties: &Map<String, Value>) -> Option<&Value> {
    let attribute_ref = attribute_properties.get("allOf");
    if attribute_ref.is_some() {
        let attribute_ref = attribute_ref.unwrap();
        let all_of_arr = attribute_ref.as_array().unwrap();
        let arr_first = all_of_arr.first().unwrap();
        let reference = arr_first.get("$ref").unwrap();
        println!("attribute is some {}", reference.to_string());
        return Some(reference);
    }
    return None;
}
// try $ref
// try allOf :[{$ref}]
// try items :[{$ref}]
fn get_property_reference_name(attribute: &Value) -> Option<String> {
    let refer = attribute.get("$ref");

    if refer.is_some() {
        let refer = refer.unwrap();
        let refer_str = refer.as_str().unwrap().to_owned();
        println!(" getPropertyReferenceName : {}", refer_str);
        return Some(refer_str);
    }

    let refer_arr = attribute.get("allOf");

    if refer_arr.is_some() {
        let refer = refer_arr.unwrap();
        let refer_arr = refer.as_array().unwrap();
        let reference = refer_arr.first().unwrap();
        let refer_str = reference.get("$ref");

        if refer_str.is_some() {
            let refer_str = refer_str.unwrap().as_str().unwrap().to_owned();
            println!(" getPropertyReferenceName : {}", refer_str);
            return Some(refer_str);
        }
    }
    let refer_arr = attribute.get("items");

    if refer_arr.is_some() {
        let refer = refer_arr.unwrap();
        let refer_str = refer.get("$ref");

        if refer_str.is_some() {
            let refer_str = refer_str.unwrap().as_str().unwrap().to_owned();
            println!(" getPropertyReferenceName : {}", refer_str);
            return Some(refer_str);
        }
    }

    println!("None");

    return None;
}
