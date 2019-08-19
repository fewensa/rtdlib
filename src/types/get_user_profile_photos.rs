
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the profile photos of a user. The result of this query may be outdated: some photos might have been deleted already.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserProfilePhotos {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getUserProfilePhotos
  /// User identifier.
  user_id: Option<i32>,
  /// The number of photos to skip; must be non-negative.
  offset: Option<i32>,
  /// Maximum number of photos to be returned; up to 100.
  limit: Option<i32>,
  
}



impl Object for GetUserProfilePhotos {}
impl RObject for GetUserProfilePhotos {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getUserProfilePhotos" }
  fn td_type(&self) -> RTDType { RTDType::GetUserProfilePhotos }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetUserProfilePhotos {}


impl GetUserProfilePhotos {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getUserProfilePhotos".to_string(),
      user_id: None,
      offset: None,
      limit: None,
      
    }
  }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn offset(&self) -> Option<i32> { self.offset.clone() }
  #[doc(hidden)] pub fn _set_offset(&mut self, offset: i32) -> &mut Self { self.offset = Some(offset); self }
  
  pub fn limit(&self) -> Option<i32> { self.limit.clone() }
  #[doc(hidden)] pub fn _set_limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



