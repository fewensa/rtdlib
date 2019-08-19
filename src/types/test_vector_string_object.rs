
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A simple object containing a vector of objects that hold a string; for testing only. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestVectorStringObject {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // testVectorStringObject
  /// Vector of objects.
  value: Option<Vec<TestString>>,
  
}



impl Object for TestVectorStringObject {}
impl RObject for TestVectorStringObject {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testVectorStringObject" }
  fn td_type(&self) -> RTDType { RTDType::TestVectorStringObject }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl TestVectorStringObject {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "testVectorStringObject".to_string(),
      value: None,
      
    }
  }
  
  pub fn value(&self) -> Option<Vec<TestString>> { self.value.clone() }
  #[doc(hidden)] pub fn _set_value(&mut self, value: Vec<TestString>) -> &mut Self { self.value = Some(value); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



