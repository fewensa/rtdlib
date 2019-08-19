
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the received bytes; for testing only. This is an offline method. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCallBytes {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // testCallBytes
  /// Bytes to return.
  x: Option<String>,
  
}



impl Object for TestCallBytes {}
impl RObject for TestCallBytes {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testCallBytes" }
  fn td_type(&self) -> RTDType { RTDType::TestCallBytes }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for TestCallBytes {}


impl TestCallBytes {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "testCallBytes".to_string(),
      x: None,
      
    }
  }
  
  pub fn x(&self) -> Option<String> { self.x.clone() }
  #[doc(hidden)] pub fn _set_x(&mut self, x: String) -> &mut Self { self.x = Some(x); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



