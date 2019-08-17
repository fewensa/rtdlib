
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Removes an animation from the list of saved animations.
#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveSavedAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // removeSavedAnimation
  /// Animation file to be removed.
  animation: Option<Box<InputFile>>,
  
}


impl Clone for RemoveSavedAnimation {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for RemoveSavedAnimation {}
impl RObject for RemoveSavedAnimation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeSavedAnimation" }
  fn td_type(&self) -> RTDType { RTDType::RemoveSavedAnimation }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for RemoveSavedAnimation {}


impl RemoveSavedAnimation {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "removeSavedAnimation".to_string(),
      animation: None,
      
    }
  }
  
  pub fn animation(&self) -> Option<Box<InputFile>> { self.animation.clone() }
  #[doc(hidden)] pub fn _set_animation(&mut self, animation: Box<InputFile>) -> &mut Self { self.animation = Some(animation); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



