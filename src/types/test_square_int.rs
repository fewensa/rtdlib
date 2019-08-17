
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the squared received number; for testing only. This is an offline method. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSquareInt {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // testSquareInt
  /// Number to square.
  x: Option<i32>,
  
}



impl Object for TestSquareInt {}
impl RObject for TestSquareInt {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testSquareInt" }
  fn td_type(&self) -> RTDType { RTDType::TestSquareInt }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for TestSquareInt {}


impl TestSquareInt {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "testSquareInt".to_string(),
      x: None,
      
    }
  }
  
  pub fn x(&self) -> Option<i32> { self.x.clone() }
  #[doc(hidden)] pub fn _set_x(&mut self, x: i32) -> &mut Self { self.x = Some(x); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



