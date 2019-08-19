
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A simple object containing a vector of numbers; for testing only. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestVectorInt {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // testVectorInt
  /// Vector of numbers.
  value: Option<Vec<i32>>,
  
}



impl Object for TestVectorInt {}
impl RObject for TestVectorInt {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testVectorInt" }
  fn td_type(&self) -> RTDType { RTDType::TestVectorInt }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl TestVectorInt {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "testVectorInt".to_string(),
      value: None,
      
    }
  }
  
  pub fn value(&self) -> Option<Vec<i32>> { self.value.clone() }
  #[doc(hidden)] pub fn _set_value(&mut self, value: Vec<i32>) -> &mut Self { self.value = Some(value); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



