
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An audio file. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBlockAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockAudio
  /// Audio file; may be null.
  audio: Option<Audio>,
  /// Audio file caption.
  caption: Option<PageBlockCaption>,
  
}



impl Object for PageBlockAudio {}
impl RObject for PageBlockAudio {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockAudio" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockAudio }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockAudio {}


impl PageBlockAudio {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockAudio".to_string(),
      audio: None,
      caption: None,
      
    }
  }
  
  pub fn audio(&self) -> Option<Audio> { self.audio.clone() }
  #[doc(hidden)] pub fn _set_audio(&mut self, audio: Audio) -> &mut Self { self.audio = Some(audio); self }
  
  pub fn caption(&self) -> Option<PageBlockCaption> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: PageBlockCaption) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



