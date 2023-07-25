use serde_json::Value;

use crate::{common::import::Import, unwrap_or_return_default, utils::type_utils::extract_type};

// returns any if any of the check fail
// TODO there is a bug here test with /supplier-portal/invoices

#[allow(unused_variables)]
pub fn get_return_types(endpoints_values: &Value) -> (String, Option<Import>) {
  let default = (String::from("any"), None);
  
  let responses = endpoints_values.get("responses");
  
  let err_msg = "Warning endpoint with no response key";
  let responses = unwrap_or_return_default!(responses, default, err_msg);
  
  let err_msg = "Endpoint 'responses' is not and object";
  let response = unwrap_or_return_default!(responses.as_object(), default, err_msg);

  for (status_code, values) in response.into_iter() {
      match status_code.as_str() {
          "200" => (),
          "201" => (),
          _ => continue,
      }
      
      let err_msg = "Endpoint responses does not have a 'content' key";
      let resp_type = unwrap_or_return_default!(values.get("content"), default, err_msg);
      
      let err_msg = "Endpoint response.content does not have a 'application/json' key";
      let resp_type = unwrap_or_return_default!(resp_type.get("application/json"), default);
      
      let err_msg = "Endpoint response.content.'application/json' does not have a 'schema' key";
      let resp_type = unwrap_or_return_default!(resp_type.get("schema"), default);
      
      let response_type = extract_type(resp_type, "items");

      return response_type;
  }

  return default;
}
// TODO test this