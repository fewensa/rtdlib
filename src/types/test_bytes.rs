
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A simple object containing a sequence of bytes; for testing only. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestBytes {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // testBytes
  /// Bytes.
  value: Option<String>,
  
}



impl Object for TestBytes {}
impl RObject for TestBytes {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testBytes" }
  fn td_type(&self) -> RTDType { RTDType::TestBytes }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl TestBytes {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "testBytes".to_string(),
      value: None,
      
    }
  }
  
  pub fn value(&self) -> Option<String> { self.value.clone() }
  #[doc(hidden)] pub fn _set_value(&mut self, value: String) -> &mut Self { self.value = Some(value); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



