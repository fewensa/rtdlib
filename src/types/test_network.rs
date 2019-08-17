
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sends a simple network request to the Telegram servers; for testing only. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestNetwork {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // testNetwork
  
}



impl Object for TestNetwork {}
impl RObject for TestNetwork {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testNetwork" }
  fn td_type(&self) -> RTDType { RTDType::TestNetwork }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for TestNetwork {}


impl TestNetwork {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "testNetwork".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



