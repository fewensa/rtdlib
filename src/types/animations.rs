
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a list of animations. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animations {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // animations
  /// List of animations.
  animations: Option<Vec<Animation>>,
  
}



impl Object for Animations {}
impl RObject for Animations {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "animations" }
  fn td_type(&self) -> RTDType { RTDType::Animations }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Animations {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "animations".to_string(),
      animations: None,
      
    }
  }
  
  pub fn animations(&self) -> Option<Vec<Animation>> { self.animations.clone() }
  #[doc(hidden)] pub fn _set_animations(&mut self, animations: Vec<Animation>) -> &mut Self { self.animations = Some(animations); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



