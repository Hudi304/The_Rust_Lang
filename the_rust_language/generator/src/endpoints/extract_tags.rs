use serde_json::Value;
use crate::unwrap_or_return_default;

#[allow(unused_variables)]
pub fn get_tags(endpoints_values: &Value) -> Vec<String> {
    
  let err_msg = "Warning : found endpoint with no tags key!";
  let tags = unwrap_or_return_default!(endpoints_values.get("tags"), vec![], err_msg);

  let err_msg = "Warning : found endpoint with tags key that is not an array!";
  let tags = unwrap_or_return_default!(tags.as_array(), vec![], err_msg);

  if tags.len() > 1 {
      println!("Warning: more then one tag was found")
  }

  tags.into_iter()
      .map(|tag| tag.as_str().unwrap_or("").to_owned())
      .collect()
}

// TODO test this 