
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a video. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inlineQueryResultVideo
  /// Unique identifier of the query result.
  id: Option<String>,
  /// Video.
  video: Option<Video>,
  /// Title of the video.
  title: Option<String>,
  /// Description of the video.
  description: Option<String>,
  
}



impl Object for InlineQueryResultVideo {}
impl RObject for InlineQueryResultVideo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultVideo" }
  fn td_type(&self) -> RTDType { RTDType::InlineQueryResultVideo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InlineQueryResult for InlineQueryResultVideo {}


impl InlineQueryResultVideo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inlineQueryResultVideo".to_string(),
      id: None,
      video: None,
      title: None,
      description: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn video(&self) -> Option<Video> { self.video.clone() }
  #[doc(hidden)] pub fn _set_video(&mut self, video: Video) -> &mut Self { self.video = Some(video); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn description(&self) -> Option<String> { self.description.clone() }
  #[doc(hidden)] pub fn _set_description(&mut self, description: String) -> &mut Self { self.description = Some(description); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



