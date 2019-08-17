
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents the result of an ImportContacts request. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportedContacts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // importedContacts
  /// User identifiers of the imported contacts in the same order as they were specified in the request; 0 if the contact is not yet a registered user.
  user_ids: Option<Vec<i32>>,
  /// The number of users that imported the corresponding contact; 0 for already registered users or if unavailable.
  importer_count: Option<Vec<i32>>,
  
}



impl Object for ImportedContacts {}
impl RObject for ImportedContacts {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "importedContacts" }
  fn td_type(&self) -> RTDType { RTDType::ImportedContacts }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl ImportedContacts {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "importedContacts".to_string(),
      user_ids: None,
      importer_count: None,
      
    }
  }
  
  pub fn user_ids(&self) -> Option<Vec<i32>> { self.user_ids.clone() }
  #[doc(hidden)] pub fn _set_user_ids(&mut self, user_ids: Vec<i32>) -> &mut Self { self.user_ids = Some(user_ids); self }
  
  pub fn importer_count(&self) -> Option<Vec<i32>> { self.importer_count.clone() }
  #[doc(hidden)] pub fn _set_importer_count(&mut self, importer_count: Vec<i32>) -> &mut Self { self.importer_count = Some(importer_count); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



