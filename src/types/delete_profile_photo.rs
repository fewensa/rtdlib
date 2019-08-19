
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Deletes a profile photo. If something changes, 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProfilePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // deleteProfilePhoto
  /// Identifier of the profile photo to delete.
  profile_photo_id: Option<i64>,
  
}



impl Object for DeleteProfilePhoto {}
impl RObject for DeleteProfilePhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deleteProfilePhoto" }
  fn td_type(&self) -> RTDType { RTDType::DeleteProfilePhoto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for DeleteProfilePhoto {}


impl DeleteProfilePhoto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "deleteProfilePhoto".to_string(),
      profile_photo_id: None,
      
    }
  }
  
  pub fn profile_photo_id(&self) -> Option<i64> { self.profile_photo_id.clone() }
  #[doc(hidden)] pub fn _set_profile_photo_id(&mut self, profile_photo_id: i64) -> &mut Self { self.profile_photo_id = Some(profile_photo_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



