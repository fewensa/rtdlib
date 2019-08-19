
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message containing a user contact. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMessageContact {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputMessageContact
  /// Contact to send.
  contact: Option<Contact>,
  
}



impl Object for InputMessageContact {}
impl RObject for InputMessageContact {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageContact" }
  fn td_type(&self) -> RTDType { RTDType::InputMessageContact }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputMessageContent for InputMessageContact {}


impl InputMessageContact {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputMessageContact".to_string(),
      contact: None,
      
    }
  }
  
  pub fn contact(&self) -> Option<Contact> { self.contact.clone() }
  #[doc(hidden)] pub fn _set_contact(&mut self, contact: Contact) -> &mut Self { self.contact = Some(contact); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



