
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An animation. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBlockAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockAnimation
  /// Animation file; may be null.
  animation: Option<Animation>,
  /// Animation caption.
  caption: Option<PageBlockCaption>,
  /// True, if the animation should be played automatically.
  need_autoplay: Option<bool>,
  
}



impl Object for PageBlockAnimation {}
impl RObject for PageBlockAnimation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockAnimation" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockAnimation }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockAnimation {}


impl PageBlockAnimation {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockAnimation".to_string(),
      animation: None,
      caption: None,
      need_autoplay: None,
      
    }
  }
  
  pub fn animation(&self) -> Option<Animation> { self.animation.clone() }
  #[doc(hidden)] pub fn _set_animation(&mut self, animation: Animation) -> &mut Self { self.animation = Some(animation); self }
  
  pub fn caption(&self) -> Option<PageBlockCaption> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: PageBlockCaption) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn need_autoplay(&self) -> Option<bool> { self.need_autoplay.clone() }
  #[doc(hidden)] pub fn _set_need_autoplay(&mut self, need_autoplay: bool) -> &mut Self { self.need_autoplay = Some(need_autoplay); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



