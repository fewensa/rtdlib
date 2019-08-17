
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message with a user contact. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageContact {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageContact
  /// Message content.
  contact: Option<Contact>,
  
}



impl Object for MessageContact {}
impl RObject for MessageContact {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageContact" }
  fn td_type(&self) -> RTDType { RTDType::MessageContact }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageContact {}


impl MessageContact {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageContact".to_string(),
      contact: None,
      
    }
  }
  
  pub fn contact(&self) -> Option<Contact> { self.contact.clone() }
  #[doc(hidden)] pub fn _set_contact(&mut self, contact: Contact) -> &mut Self { self.contact = Some(contact); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



