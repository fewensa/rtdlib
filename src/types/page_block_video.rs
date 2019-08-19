
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A video. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBlockVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockVideo
  /// Video file; may be null.
  video: Option<Video>,
  /// Video caption.
  caption: Option<PageBlockCaption>,
  /// True, if the video should be played automatically.
  need_autoplay: Option<bool>,
  /// True, if the video should be looped.
  is_looped: Option<bool>,
  
}



impl Object for PageBlockVideo {}
impl RObject for PageBlockVideo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockVideo" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockVideo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockVideo {}


impl PageBlockVideo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockVideo".to_string(),
      video: None,
      caption: None,
      need_autoplay: None,
      is_looped: None,
      
    }
  }
  
  pub fn video(&self) -> Option<Video> { self.video.clone() }
  #[doc(hidden)] pub fn _set_video(&mut self, video: Video) -> &mut Self { self.video = Some(video); self }
  
  pub fn caption(&self) -> Option<PageBlockCaption> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: PageBlockCaption) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn need_autoplay(&self) -> Option<bool> { self.need_autoplay.clone() }
  #[doc(hidden)] pub fn _set_need_autoplay(&mut self, need_autoplay: bool) -> &mut Self { self.need_autoplay = Some(need_autoplay); self }
  
  pub fn is_looped(&self) -> Option<bool> { self.is_looped.clone() }
  #[doc(hidden)] pub fn _set_is_looped(&mut self, is_looped: bool) -> &mut Self { self.is_looped = Some(is_looped); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



