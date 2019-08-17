
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A simple object containing a vector of objects that hold a number; for testing only. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestVectorIntObject {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // testVectorIntObject
  /// Vector of objects.
  value: Option<Vec<TestInt>>,
  
}



impl Object for TestVectorIntObject {}
impl RObject for TestVectorIntObject {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testVectorIntObject" }
  fn td_type(&self) -> RTDType { RTDType::TestVectorIntObject }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl TestVectorIntObject {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "testVectorIntObject".to_string(),
      value: None,
      
    }
  }
  
  pub fn value(&self) -> Option<Vec<TestInt>> { self.value.clone() }
  #[doc(hidden)] pub fn _set_value(&mut self, value: Vec<TestInt>) -> &mut Self { self.value = Some(value); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



