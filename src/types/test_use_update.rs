
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Does nothing and ensures that the 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestUseUpdate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // testUseUpdate
  
}



impl Object for TestUseUpdate {}
impl RObject for TestUseUpdate {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testUseUpdate" }
  fn td_type(&self) -> RTDType { RTDType::TestUseUpdate }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for TestUseUpdate {}


impl TestUseUpdate {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "testUseUpdate".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



