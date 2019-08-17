
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the received vector of strings; for testing only. This is an offline method. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCallVectorString {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // testCallVectorString
  /// Vector of strings to return.
  x: Option<Vec<String>>,
  
}



impl Object for TestCallVectorString {}
impl RObject for TestCallVectorString {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testCallVectorString" }
  fn td_type(&self) -> RTDType { RTDType::TestCallVectorString }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for TestCallVectorString {}


impl TestCallVectorString {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "testCallVectorString".to_string(),
      x: None,
      
    }
  }
  
  pub fn x(&self) -> Option<Vec<String>> { self.x.clone() }
  #[doc(hidden)] pub fn _set_x(&mut self, x: Vec<String>) -> &mut Self { self.x = Some(x); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



