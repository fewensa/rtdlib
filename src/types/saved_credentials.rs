
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about saved card credentials. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedCredentials {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // savedCredentials
  /// Unique identifier of the saved credentials.
  id: Option<String>,
  /// Title of the saved credentials.
  title: Option<String>,
  
}



impl Object for SavedCredentials {}
impl RObject for SavedCredentials {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "savedCredentials" }
  fn td_type(&self) -> RTDType { RTDType::SavedCredentials }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl SavedCredentials {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "savedCredentials".to_string(),
      id: None,
      title: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



