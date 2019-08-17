
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a counter. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Count {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // count
  /// Count.
  count: Option<i32>,
  
}



impl Object for Count {}
impl RObject for Count {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "count" }
  fn td_type(&self) -> RTDType { RTDType::Count }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Count {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "count".to_string(),
      count: None,
      
    }
  }
  
  pub fn count(&self) -> Option<i32> { self.count.clone() }
  #[doc(hidden)] pub fn _set_count(&mut self, count: i32) -> &mut Self { self.count = Some(count); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



