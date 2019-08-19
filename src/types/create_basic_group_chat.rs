
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns an existing chat corresponding to a known basic group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBasicGroupChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // createBasicGroupChat
  /// Basic group identifier.
  basic_group_id: Option<i32>,
  /// If true, the chat will be created without network request. In this case all information about the chat except its type, title and photo can be incorrect.
  force: Option<bool>,
  
}



impl Object for CreateBasicGroupChat {}
impl RObject for CreateBasicGroupChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "createBasicGroupChat" }
  fn td_type(&self) -> RTDType { RTDType::CreateBasicGroupChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CreateBasicGroupChat {}


impl CreateBasicGroupChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "createBasicGroupChat".to_string(),
      basic_group_id: None,
      force: None,
      
    }
  }
  
  pub fn basic_group_id(&self) -> Option<i32> { self.basic_group_id.clone() }
  #[doc(hidden)] pub fn _set_basic_group_id(&mut self, basic_group_id: i32) -> &mut Self { self.basic_group_id = Some(basic_group_id); self }
  
  pub fn force(&self) -> Option<bool> { self.force.clone() }
  #[doc(hidden)] pub fn _set_force(&mut self, force: bool) -> &mut Self { self.force = Some(force); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



