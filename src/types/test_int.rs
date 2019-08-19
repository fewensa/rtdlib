
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A simple object containing a number; for testing only. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestInt {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // testInt
  /// Number.
  value: Option<i32>,
  
}



impl Object for TestInt {}
impl RObject for TestInt {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testInt" }
  fn td_type(&self) -> RTDType { RTDType::TestInt }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl TestInt {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "testInt".to_string(),
      value: None,
      
    }
  }
  
  pub fn value(&self) -> Option<i32> { self.value.clone() }
  #[doc(hidden)] pub fn _set_value(&mut self, value: i32) -> &mut Self { self.value = Some(value); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



