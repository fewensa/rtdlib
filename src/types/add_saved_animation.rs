
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Manually adds a new animation to the list of saved animations. The new animation is added to the beginning of the list. If the animation was already in the list, it is removed first. Only non-secret video animations with MIME type "video/mp4" can be added to the list.
#[derive(Debug, Serialize, Deserialize)]
pub struct AddSavedAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // addSavedAnimation
  /// The animation file to be added. Only animations known to the server (i.e. successfully sent via a message) can be added to the list.
  animation: Option<Box<InputFile>>,
  
}


impl Clone for AddSavedAnimation {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for AddSavedAnimation {}
impl RObject for AddSavedAnimation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addSavedAnimation" }
  fn td_type(&self) -> RTDType { RTDType::AddSavedAnimation }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for AddSavedAnimation {}


impl AddSavedAnimation {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "addSavedAnimation".to_string(),
      animation: None,
      
    }
  }
  
  pub fn animation(&self) -> Option<Box<InputFile>> { self.animation.clone() }
  #[doc(hidden)] pub fn _set_animation(&mut self, animation: Box<InputFile>) -> &mut Self { self.animation = Some(animation); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



