
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sends a call rating.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendCallRating {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // sendCallRating
  /// Call identifier.
  call_id: Option<i32>,
  /// Call rating; 1-5.
  rating: Option<i32>,
  /// An optional user comment if the rating is less than 5.
  comment: Option<String>,
  
}



impl Object for SendCallRating {}
impl RObject for SendCallRating {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendCallRating" }
  fn td_type(&self) -> RTDType { RTDType::SendCallRating }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SendCallRating {}


impl SendCallRating {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "sendCallRating".to_string(),
      call_id: None,
      rating: None,
      comment: None,
      
    }
  }
  
  pub fn call_id(&self) -> Option<i32> { self.call_id.clone() }
  #[doc(hidden)] pub fn _set_call_id(&mut self, call_id: i32) -> &mut Self { self.call_id = Some(call_id); self }
  
  pub fn rating(&self) -> Option<i32> { self.rating.clone() }
  #[doc(hidden)] pub fn _set_rating(&mut self, rating: i32) -> &mut Self { self.rating = Some(rating); self }
  
  pub fn comment(&self) -> Option<String> { self.comment.clone() }
  #[doc(hidden)] pub fn _set_comment(&mut self, comment: String) -> &mut Self { self.comment = Some(comment); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



