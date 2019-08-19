
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The list of saved animations was updated. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSavedAnimations {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateSavedAnimations
  /// The new list of file identifiers of saved animations.
  animation_ids: Option<Vec<i32>>,
  
}



impl Object for UpdateSavedAnimations {}
impl RObject for UpdateSavedAnimations {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateSavedAnimations" }
  fn td_type(&self) -> RTDType { RTDType::UpdateSavedAnimations }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateSavedAnimations {}


impl UpdateSavedAnimations {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateSavedAnimations".to_string(),
      animation_ids: None,
      
    }
  }
  
  pub fn animation_ids(&self) -> Option<Vec<i32>> { self.animation_ids.clone() }
  #[doc(hidden)] pub fn _set_animation_ids(&mut self, animation_ids: Vec<i32>) -> &mut Self { self.animation_ids = Some(animation_ids); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



