/*
 * myBRC REST API
 *
 * REST API for myBRC
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Partition {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "shared", skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    #[serde(rename = "num_nodes", skip_serializing_if = "Option::is_none")]
    pub num_nodes: Option<i32>,
    #[serde(rename = "num_cores", skip_serializing_if = "Option::is_none")]
    pub num_cores: Option<i32>,
    #[serde(rename = "su_per_core_hour", skip_serializing_if = "Option::is_none")]
    pub su_per_core_hour: Option<String>,
}

impl Partition {
    pub fn new(name: String) -> Partition {
        Partition {
            name,
            shared: None,
            num_nodes: None,
            num_cores: None,
            su_per_core_hour: None,
        }
    }
}
