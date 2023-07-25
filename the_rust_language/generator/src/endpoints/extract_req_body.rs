use serde_json::Value;

use crate::{common::import::Import, unwrap_or_return_default, utils::type_utils::extract_type};

// TODO print warnings / errors
// this might have to return void for get, delete and others!
#[allow(unused_variables)]
pub fn get_request_body_type(values: &Value) -> (String, Option<Import>) {
  let default = (String::from("any"), None);

  let err_msg = "Endpoint does not have a requestBody";
  let req_body = unwrap_or_return_default!(values.get("requestBody"), default, err_msg);
  
  let err_msg = "Endpoint requestBody does not have a content key";
  let req_body = unwrap_or_return_default!(req_body.get("content"), default, err_msg);
  
  let err_msg = "Endpoint requestBody.content does not have 'application/json' key";
  let req_body = unwrap_or_return_default!(req_body.get("application/json"), default, err_msg);
  
  let err_msg = "Endpoint requestBody.content.'application/json' is not an object";
  let req_body = unwrap_or_return_default!(req_body.as_object(), default);

  for (_req_type, value) in req_body.into_iter() {
    // TODO this returns the first response body 
    // TODO return all of them or just the app_json_one
      return extract_type(value, "schema");
  }

  return ("any".to_owned(), None);
}

//TODO test this with gets and deletes

#[cfg(test)]
mod request_body {

    use super::*;

    //TODO maybe find a way to do this in some kind of loop
    // this is a lot of code repetition
    #[test]
    fn req_body() {
        // Arrange
        let number_data = r#"{ "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "$ref": "/components/schemas/ReviewsFilter"
              }
            },
            "text/json": {
              "schema": {
                "$ref": "/components/schemas/ReviewsFilter"
              }
            },
            "application/*+json": {
              "schema": {
                "$ref": "/components/schemas/ReviewsFilter"
              }
            }
          }
        }}"#;

        // Act
        let prop_json: Value = serde_json::from_str(number_data).unwrap();
        let (request_type, import_option) = get_request_body_type(&prop_json);

        // Assert
        assert_eq!(request_type.eq("ReviewsFilter"), true);
        assert_eq!(import_option.unwrap().name.eq("ReviewsFilter"), true);
    }
}