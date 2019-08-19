
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Creates a new basic group and sends a corresponding 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNewBasicGroupChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // createNewBasicGroupChat
  /// Identifiers of users to be added to the basic group.
  user_ids: Option<Vec<i32>>,
  /// Title of the new basic group; 1-128 characters.
  title: Option<String>,
  
}



impl Object for CreateNewBasicGroupChat {}
impl RObject for CreateNewBasicGroupChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "createNewBasicGroupChat" }
  fn td_type(&self) -> RTDType { RTDType::CreateNewBasicGroupChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CreateNewBasicGroupChat {}


impl CreateNewBasicGroupChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "createNewBasicGroupChat".to_string(),
      user_ids: None,
      title: None,
      
    }
  }
  
  pub fn user_ids(&self) -> Option<Vec<i32>> { self.user_ids.clone() }
  #[doc(hidden)] pub fn _set_user_ids(&mut self, user_ids: Vec<i32>) -> &mut Self { self.user_ids = Some(user_ids); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



