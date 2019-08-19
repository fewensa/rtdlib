
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains full information about a user profile photo. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfilePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // userProfilePhoto
  /// Unique user profile photo identifier.
  id: Option<i64>,
  /// Point in time (Unix timestamp) when the photo has been added.
  added_date: Option<i32>,
  /// Available variants of the user photo, in different sizes.
  sizes: Option<Vec<PhotoSize>>,
  
}



impl Object for UserProfilePhoto {}
impl RObject for UserProfilePhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userProfilePhoto" }
  fn td_type(&self) -> RTDType { RTDType::UserProfilePhoto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl UserProfilePhoto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "userProfilePhoto".to_string(),
      id: None,
      added_date: None,
      sizes: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i64> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i64) -> &mut Self { self.id = Some(id); self }
  
  pub fn added_date(&self) -> Option<i32> { self.added_date.clone() }
  #[doc(hidden)] pub fn _set_added_date(&mut self, added_date: i32) -> &mut Self { self.added_date = Some(added_date); self }
  
  pub fn sizes(&self) -> Option<Vec<PhotoSize>> { self.sizes.clone() }
  #[doc(hidden)] pub fn _set_sizes(&mut self, sizes: Vec<PhotoSize>) -> &mut Self { self.sizes = Some(sizes); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



