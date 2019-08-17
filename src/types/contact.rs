
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes a user contact. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // contact
  /// Phone number of the user.
  phone_number: Option<String>,
  /// First name of the user; 1-255 characters in length.
  first_name: Option<String>,
  /// Last name of the user.
  last_name: Option<String>,
  /// Additional data about the user in a form of vCard; 0-2048 bytes in length.
  vcard: Option<String>,
  /// Identifier of the user, if known; otherwise 0.
  user_id: Option<i32>,
  
}



impl Object for Contact {}
impl RObject for Contact {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "contact" }
  fn td_type(&self) -> RTDType { RTDType::Contact }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Contact {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "contact".to_string(),
      phone_number: None,
      first_name: None,
      last_name: None,
      vcard: None,
      user_id: None,
      
    }
  }
  
  pub fn phone_number(&self) -> Option<String> { self.phone_number.clone() }
  #[doc(hidden)] pub fn _set_phone_number(&mut self, phone_number: String) -> &mut Self { self.phone_number = Some(phone_number); self }
  
  pub fn first_name(&self) -> Option<String> { self.first_name.clone() }
  #[doc(hidden)] pub fn _set_first_name(&mut self, first_name: String) -> &mut Self { self.first_name = Some(first_name); self }
  
  pub fn last_name(&self) -> Option<String> { self.last_name.clone() }
  #[doc(hidden)] pub fn _set_last_name(&mut self, last_name: String) -> &mut Self { self.last_name = Some(last_name); self }
  
  pub fn vcard(&self) -> Option<String> { self.vcard.clone() }
  #[doc(hidden)] pub fn _set_vcard(&mut self, vcard: String) -> &mut Self { self.vcard = Some(vcard); self }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



