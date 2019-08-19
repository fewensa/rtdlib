
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Does nothing and ensures that the Error object is used; for testing only. This is an offline method. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestUseError {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // testUseError
  
}



impl Object for TestUseError {}
impl RObject for TestUseError {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testUseError" }
  fn td_type(&self) -> RTDType { RTDType::TestUseError }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for TestUseError {}


impl TestUseError {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "testUseError".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



