
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Removes users from the contact list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveContacts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // removeContacts
  /// Identifiers of users to be deleted.
  user_ids: Option<Vec<i32>>,
  
}



impl Object for RemoveContacts {}
impl RObject for RemoveContacts {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeContacts" }
  fn td_type(&self) -> RTDType { RTDType::RemoveContacts }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for RemoveContacts {}


impl RemoveContacts {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "removeContacts".to_string(),
      user_ids: None,
      
    }
  }
  
  pub fn user_ids(&self) -> Option<Vec<i32>> { self.user_ids.clone() }
  #[doc(hidden)] pub fn _set_user_ids(&mut self, user_ids: Vec<i32>) -> &mut Self { self.user_ids = Some(user_ids); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



