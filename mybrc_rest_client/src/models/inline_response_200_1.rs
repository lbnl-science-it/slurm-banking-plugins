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
pub struct InlineResponse2001 {
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(rename = "previous", skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    #[serde(rename = "results")]
    pub results: Vec<crate::models::Job>,
}

impl InlineResponse2001 {
    pub fn new(count: i32, results: Vec<crate::models::Job>) -> InlineResponse2001 {
        InlineResponse2001 {
            count,
            next: None,
            previous: None,
            results,
        }
    }
}
