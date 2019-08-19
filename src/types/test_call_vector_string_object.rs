
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the received vector of objects containing a string; for testing only. This is an offline method. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCallVectorStringObject {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // testCallVectorStringObject
  /// Vector of objects to return.
  x: Option<Vec<TestString>>,
  
}



impl Object for TestCallVectorStringObject {}
impl RObject for TestCallVectorStringObject {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testCallVectorStringObject" }
  fn td_type(&self) -> RTDType { RTDType::TestCallVectorStringObject }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for TestCallVectorStringObject {}


impl TestCallVectorStringObject {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "testCallVectorStringObject".to_string(),
      x: None,
      
    }
  }
  
  pub fn x(&self) -> Option<Vec<TestString>> { self.x.clone() }
  #[doc(hidden)] pub fn _set_x(&mut self, x: Vec<TestString>) -> &mut Self { self.x = Some(x); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



