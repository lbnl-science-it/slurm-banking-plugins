/* 
 * myBRC REST API
 *
 * REST API for myBRC
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Partition {
  #[serde(rename = "name")]
  name: String,
  #[serde(rename = "shared")]
  shared: Option<bool>,
  #[serde(rename = "num_nodes")]
  num_nodes: Option<i32>,
  #[serde(rename = "num_cores")]
  num_cores: Option<i32>,
  #[serde(rename = "su_per_core_hour")]
  su_per_core_hour: Option<String>
}

impl Partition {
  pub fn new(name: String) -> Partition {
    Partition {
      name: name,
      shared: None,
      num_nodes: None,
      num_cores: None,
      su_per_core_hour: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> Partition {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_shared(&mut self, shared: bool) {
    self.shared = Some(shared);
  }

  pub fn with_shared(mut self, shared: bool) -> Partition {
    self.shared = Some(shared);
    self
  }

  pub fn shared(&self) -> Option<&bool> {
    self.shared.as_ref()
  }

  pub fn reset_shared(&mut self) {
    self.shared = None;
  }

  pub fn set_num_nodes(&mut self, num_nodes: i32) {
    self.num_nodes = Some(num_nodes);
  }

  pub fn with_num_nodes(mut self, num_nodes: i32) -> Partition {
    self.num_nodes = Some(num_nodes);
    self
  }

  pub fn num_nodes(&self) -> Option<&i32> {
    self.num_nodes.as_ref()
  }

  pub fn reset_num_nodes(&mut self) {
    self.num_nodes = None;
  }

  pub fn set_num_cores(&mut self, num_cores: i32) {
    self.num_cores = Some(num_cores);
  }

  pub fn with_num_cores(mut self, num_cores: i32) -> Partition {
    self.num_cores = Some(num_cores);
    self
  }

  pub fn num_cores(&self) -> Option<&i32> {
    self.num_cores.as_ref()
  }

  pub fn reset_num_cores(&mut self) {
    self.num_cores = None;
  }

  pub fn set_su_per_core_hour(&mut self, su_per_core_hour: String) {
    self.su_per_core_hour = Some(su_per_core_hour);
  }

  pub fn with_su_per_core_hour(mut self, su_per_core_hour: String) -> Partition {
    self.su_per_core_hour = Some(su_per_core_hour);
    self
  }

  pub fn su_per_core_hour(&self) -> Option<&String> {
    self.su_per_core_hour.as_ref()
  }

  pub fn reset_su_per_core_hour(&mut self) {
    self.su_per_core_hour = None;
  }

}


