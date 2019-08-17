
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Creates a new supergroup or channel and sends a corresponding 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNewSupergroupChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // createNewSupergroupChat
  /// Title of the new chat; 1-128 characters.
  title: Option<String>,
  /// True, if a channel chat should be created.
  is_channel: Option<bool>,
  /// Chat description; 0-255 characters.
  description: Option<String>,
  
}



impl Object for CreateNewSupergroupChat {}
impl RObject for CreateNewSupergroupChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "createNewSupergroupChat" }
  fn td_type(&self) -> RTDType { RTDType::CreateNewSupergroupChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CreateNewSupergroupChat {}


impl CreateNewSupergroupChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "createNewSupergroupChat".to_string(),
      title: None,
      is_channel: None,
      description: None,
      
    }
  }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn is_channel(&self) -> Option<bool> { self.is_channel.clone() }
  #[doc(hidden)] pub fn _set_is_channel(&mut self, is_channel: bool) -> &mut Self { self.is_channel = Some(is_channel); self }
  
  pub fn description(&self) -> Option<String> { self.description.clone() }
  #[doc(hidden)] pub fn _set_description(&mut self, description: String) -> &mut Self { self.description = Some(description); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



