
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Adds new contacts or edits existing contacts; contacts' user identifiers are ignored.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportContacts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // importContacts
  /// The list of contacts to import or edit, contact's vCard are ignored and are not imported.
  contacts: Option<Vec<Contact>>,
  
}



impl Object for ImportContacts {}
impl RObject for ImportContacts {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "importContacts" }
  fn td_type(&self) -> RTDType { RTDType::ImportContacts }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ImportContacts {}


impl ImportContacts {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "importContacts".to_string(),
      contacts: None,
      
    }
  }
  
  pub fn contacts(&self) -> Option<Vec<Contact>> { self.contacts.clone() }
  #[doc(hidden)] pub fn _set_contacts(&mut self, contacts: Vec<Contact>) -> &mut Self { self.contacts = Some(contacts); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



