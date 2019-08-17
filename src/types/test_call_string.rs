
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the received string; for testing only. This is an offline method. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCallString {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // testCallString
  /// String to return.
  x: Option<String>,
  
}



impl Object for TestCallString {}
impl RObject for TestCallString {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testCallString" }
  fn td_type(&self) -> RTDType { RTDType::TestCallString }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for TestCallString {}


impl TestCallString {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "testCallString".to_string(),
      x: None,
      
    }
  }
  
  pub fn x(&self) -> Option<String> { self.x.clone() }
  #[doc(hidden)] pub fn _set_x(&mut self, x: String) -> &mut Self { self.x = Some(x); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



