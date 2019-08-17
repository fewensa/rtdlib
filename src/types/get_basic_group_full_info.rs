
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns full information about a basic group by its identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBasicGroupFullInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getBasicGroupFullInfo
  /// Basic group identifier.
  basic_group_id: Option<i32>,
  
}



impl Object for GetBasicGroupFullInfo {}
impl RObject for GetBasicGroupFullInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getBasicGroupFullInfo" }
  fn td_type(&self) -> RTDType { RTDType::GetBasicGroupFullInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetBasicGroupFullInfo {}


impl GetBasicGroupFullInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getBasicGroupFullInfo".to_string(),
      basic_group_id: None,
      
    }
  }
  
  pub fn basic_group_id(&self) -> Option<i32> { self.basic_group_id.clone() }
  #[doc(hidden)] pub fn _set_basic_group_id(&mut self, basic_group_id: i32) -> &mut Self { self.basic_group_id = Some(basic_group_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



