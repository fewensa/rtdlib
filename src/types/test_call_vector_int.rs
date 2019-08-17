
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the received vector of numbers; for testing only. This is an offline method. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCallVectorInt {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // testCallVectorInt
  /// Vector of numbers to return.
  x: Option<Vec<i32>>,
  
}



impl Object for TestCallVectorInt {}
impl RObject for TestCallVectorInt {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testCallVectorInt" }
  fn td_type(&self) -> RTDType { RTDType::TestCallVectorInt }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for TestCallVectorInt {}


impl TestCallVectorInt {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "testCallVectorInt".to_string(),
      x: None,
      
    }
  }
  
  pub fn x(&self) -> Option<Vec<i32>> { self.x.clone() }
  #[doc(hidden)] pub fn _set_x(&mut self, x: Vec<i32>) -> &mut Self { self.x = Some(x); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



