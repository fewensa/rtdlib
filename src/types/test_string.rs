
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A simple object containing a string; for testing only. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestString {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // testString
  /// String.
  value: Option<String>,
  
}



impl Object for TestString {}
impl RObject for TestString {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testString" }
  fn td_type(&self) -> RTDType { RTDType::TestString }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl TestString {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "testString".to_string(),
      value: None,
      
    }
  }
  
  pub fn value(&self) -> Option<String> { self.value.clone() }
  #[doc(hidden)] pub fn _set_value(&mut self, value: String) -> &mut Self { self.value = Some(value); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



