
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Forces an updates.getDifference call to the Telegram servers; for testing only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestGetDifference {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // testGetDifference
  
}



impl Object for TestGetDifference {}
impl RObject for TestGetDifference {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testGetDifference" }
  fn td_type(&self) -> RTDType { RTDType::TestGetDifference }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for TestGetDifference {}


impl TestGetDifference {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "testGetDifference".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



