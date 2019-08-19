
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A document message (general file). 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputMessageDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputMessageDocument
  /// Document to be sent.
  document: Option<Box<InputFile>>,
  /// Document thumbnail, if available.
  thumbnail: Option<InputThumbnail>,
  /// Document caption; 0-GetOption("message_caption_length_max") characters.
  caption: Option<FormattedText>,
  
}


impl Clone for InputMessageDocument {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputMessageDocument {}
impl RObject for InputMessageDocument {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageDocument" }
  fn td_type(&self) -> RTDType { RTDType::InputMessageDocument }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputMessageContent for InputMessageDocument {}


impl InputMessageDocument {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputMessageDocument".to_string(),
      document: None,
      thumbnail: None,
      caption: None,
      
    }
  }
  
  pub fn document(&self) -> Option<Box<InputFile>> { self.document.clone() }
  #[doc(hidden)] pub fn _set_document(&mut self, document: Box<InputFile>) -> &mut Self { self.document = Some(document); self }
  
  pub fn thumbnail(&self) -> Option<InputThumbnail> { self.thumbnail.clone() }
  #[doc(hidden)] pub fn _set_thumbnail(&mut self, thumbnail: InputThumbnail) -> &mut Self { self.thumbnail = Some(thumbnail); self }
  
  pub fn caption(&self) -> Option<FormattedText> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: FormattedText) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



