
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the received vector of objects containing a number; for testing only. This is an offline method. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCallVectorIntObject {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // testCallVectorIntObject
  /// Vector of objects to return.
  x: Option<Vec<TestInt>>,
  
}



impl Object for TestCallVectorIntObject {}
impl RObject for TestCallVectorIntObject {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testCallVectorIntObject" }
  fn td_type(&self) -> RTDType { RTDType::TestCallVectorIntObject }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for TestCallVectorIntObject {}


impl TestCallVectorIntObject {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "testCallVectorIntObject".to_string(),
      x: None,
      
    }
  }
  
  pub fn x(&self) -> Option<Vec<TestInt>> { self.x.clone() }
  #[doc(hidden)] pub fn _set_x(&mut self, x: Vec<TestInt>) -> &mut Self { self.x = Some(x); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



