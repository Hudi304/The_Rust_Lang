use super::response_info::ResponseInfo;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwaggerFormat {
    pub openapi: String,
    pub info: ResponseInfo,
    pub paths: Value,      // components: String,
    pub components: Value, // components: String,
}
