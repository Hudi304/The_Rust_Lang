use serde_json::Value;

use crate::unwrap_or_return_default;

#[allow(unused_variables)]
pub fn get_fn_name(endpoints_values: &Value) -> String {
  let fn_name = endpoints_values.get("operationId");

  let err_msg = "Warning : operationId not found";
  let fn_name = unwrap_or_return_default!(fn_name, String::new(), err_msg);

  let fn_name = fn_name.as_str().unwrap_or("default_fn_name");
  fn_name.to_owned()
}

// TODO test this
