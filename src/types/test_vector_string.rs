
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A simple object containing a vector of strings; for testing only. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestVectorString {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // testVectorString
  /// Vector of strings.
  value: Option<Vec<String>>,
  
}



impl Object for TestVectorString {}
impl RObject for TestVectorString {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testVectorString" }
  fn td_type(&self) -> RTDType { RTDType::TestVectorString }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl TestVectorString {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "testVectorString".to_string(),
      value: None,
      
    }
  }
  
  pub fn value(&self) -> Option<Vec<String>> { self.value.clone() }
  #[doc(hidden)] pub fn _set_value(&mut self, value: Vec<String>) -> &mut Self { self.value = Some(value); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



