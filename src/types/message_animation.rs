
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An animation message (GIF-style). 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageAnimation
  /// Message content.
  animation: Option<Animation>,
  /// Animation caption.
  caption: Option<FormattedText>,
  /// True, if the animation thumbnail must be blurred and the animation must be shown only while tapped.
  is_secret: Option<bool>,
  
}



impl Object for MessageAnimation {}
impl RObject for MessageAnimation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageAnimation" }
  fn td_type(&self) -> RTDType { RTDType::MessageAnimation }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageAnimation {}


impl MessageAnimation {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageAnimation".to_string(),
      animation: None,
      caption: None,
      is_secret: None,
      
    }
  }
  
  pub fn animation(&self) -> Option<Animation> { self.animation.clone() }
  #[doc(hidden)] pub fn _set_animation(&mut self, animation: Animation) -> &mut Self { self.animation = Some(animation); self }
  
  pub fn caption(&self) -> Option<FormattedText> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: FormattedText) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn is_secret(&self) -> Option<bool> { self.is_secret.clone() }
  #[doc(hidden)] pub fn _set_is_secret(&mut self, is_secret: bool) -> &mut Self { self.is_secret = Some(is_secret); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



