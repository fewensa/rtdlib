
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Does nothing; for testing only. This is an offline method. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCallEmpty {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // testCallEmpty
  
}



impl Object for TestCallEmpty {}
impl RObject for TestCallEmpty {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testCallEmpty" }
  fn td_type(&self) -> RTDType { RTDType::TestCallEmpty }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for TestCallEmpty {}


impl TestCallEmpty {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "testCallEmpty".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



