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

#[cfg(test)]
mod tags {

    // use super::*;

    //TODO maybe find a way to do this in some kind of loop
    // this is a lot of code repetition
    #[test]
    #[ignore]
    fn multiple_methods_endpoint() {
        // Arrange
        // let number_data = r#"{ "tags": ["tag1", "tag2"] }"#;

        // Act
        // let prop_json: Value = serde_json::from_str(number_data).unwrap();
        // let tags : Vec<String>= get_tags(&prop_json);
        // println!("{}, {:?} ", return_type, import);

        // Assert

        //TODO find a way to compare arrays
        // assert_eq!(tags[0].eq("tag1".to_owned()), true);
    }
}