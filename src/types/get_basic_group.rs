
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns information about a basic group by its identifier. This is an offline request if the current user is not a bot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBasicGroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getBasicGroup
  /// Basic group identifier.
  basic_group_id: Option<i32>,
  
}



impl Object for GetBasicGroup {}
impl RObject for GetBasicGroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getBasicGroup" }
  fn td_type(&self) -> RTDType { RTDType::GetBasicGroup }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetBasicGroup {}


impl GetBasicGroup {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getBasicGroup".to_string(),
      basic_group_id: None,
      
    }
  }
  
  pub fn basic_group_id(&self) -> Option<i32> { self.basic_group_id.clone() }
  #[doc(hidden)] pub fn _set_basic_group_id(&mut self, basic_group_id: i32) -> &mut Self { self.basic_group_id = Some(basic_group_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



