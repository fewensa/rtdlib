
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes imported contacts using the list of current user contacts saved on the device. Imports newly added contacts and, if at least the file database is enabled, deletes recently deleted contacts. Query result depends on the result of the previous query, so only one query is possible at the same time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeImportedContacts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // changeImportedContacts
  /// The new list of contacts, contact's vCard are ignored and are not imported.
  contacts: Option<Vec<Contact>>,
  
}



impl Object for ChangeImportedContacts {}
impl RObject for ChangeImportedContacts {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "changeImportedContacts" }
  fn td_type(&self) -> RTDType { RTDType::ChangeImportedContacts }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ChangeImportedContacts {}


impl ChangeImportedContacts {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "changeImportedContacts".to_string(),
      contacts: None,
      
    }
  }
  
  pub fn contacts(&self) -> Option<Vec<Contact>> { self.contacts.clone() }
  #[doc(hidden)] pub fn _set_contacts(&mut self, contacts: Vec<Contact>) -> &mut Self { self.contacts = Some(contacts); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



