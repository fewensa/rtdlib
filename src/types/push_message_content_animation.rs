
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An animation message (GIF-style.) 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentAnimation
  /// Message content; may be null.
  animation: Option<Animation>,
  /// Animation caption.
  caption: Option<String>,
  /// True, if the message is a pinned message with the specified content.
  is_pinned: Option<bool>,
  
}



impl Object for PushMessageContentAnimation {}
impl RObject for PushMessageContentAnimation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentAnimation" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentAnimation }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentAnimation {}


impl PushMessageContentAnimation {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentAnimation".to_string(),
      animation: None,
      caption: None,
      is_pinned: None,
      
    }
  }
  
  pub fn animation(&self) -> Option<Animation> { self.animation.clone() }
  #[doc(hidden)] pub fn _set_animation(&mut self, animation: Animation) -> &mut Self { self.animation = Some(animation); self }
  
  pub fn caption(&self) -> Option<String> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: String) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn is_pinned(&self) -> Option<bool> { self.is_pinned.clone() }
  #[doc(hidden)] pub fn _set_is_pinned(&mut self, is_pinned: bool) -> &mut Self { self.is_pinned = Some(is_pinned); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



