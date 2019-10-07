/* 
 * myBRC REST API
 *
 * REST API for myBRC
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use hyper;
use std::collections::HashMap;

pub struct Configuration<C: hyper::client::Connect> {
  pub base_path: String,
  pub user_agent: Option<String>,
  pub client: hyper::client::Client<C>,
  pub basic_auth: Option<BasicAuth>,
  pub oauth_access_token: Option<String>,
  pub api_key: Option<ApiKey>,
  // TODO: take an oauth2 token source, similar to the go one
}

pub type BasicAuth = (String, Option<String>);

pub struct ApiKey {
  pub prefix: Option<String>,
  pub key: String,
}

impl<C: hyper::client::Connect> Configuration<C> {
  pub fn new(client: hyper::client::Client<C>) -> Configuration<C> {
    Configuration {
      base_path: "https://scgup-dev.lbl.gov/mybrc-rest".to_owned(),
      user_agent: Some("Swagger-Codegen/v1/rust".to_owned()),
      client: client,
      basic_auth: None,
      oauth_access_token: None,
      api_key: None,
    }
  }
}
