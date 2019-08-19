
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A new incoming query; for bots only. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNewCustomQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateNewCustomQuery
  /// The query identifier.
  id: Option<i64>,
  /// JSON-serialized query data.
  data: Option<String>,
  /// Query timeout.
  timeout: Option<i32>,
  
}



impl Object for UpdateNewCustomQuery {}
impl RObject for UpdateNewCustomQuery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNewCustomQuery" }
  fn td_type(&self) -> RTDType { RTDType::UpdateNewCustomQuery }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateNewCustomQuery {}


impl UpdateNewCustomQuery {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateNewCustomQuery".to_string(),
      id: None,
      data: None,
      timeout: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i64> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i64) -> &mut Self { self.id = Some(id); self }
  
  pub fn data(&self) -> Option<String> { self.data.clone() }
  #[doc(hidden)] pub fn _set_data(&mut self, data: String) -> &mut Self { self.data = Some(data); self }
  
  pub fn timeout(&self) -> Option<i32> { self.timeout.clone() }
  #[doc(hidden)] pub fn _set_timeout(&mut self, timeout: i32) -> &mut Self { self.timeout = Some(timeout); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



