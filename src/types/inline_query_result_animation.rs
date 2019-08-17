
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents an animation file. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inlineQueryResultAnimation
  /// Unique identifier of the query result.
  id: Option<String>,
  /// Animation file.
  animation: Option<Animation>,
  /// Animation title.
  title: Option<String>,
  
}



impl Object for InlineQueryResultAnimation {}
impl RObject for InlineQueryResultAnimation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultAnimation" }
  fn td_type(&self) -> RTDType { RTDType::InlineQueryResultAnimation }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InlineQueryResult for InlineQueryResultAnimation {}


impl InlineQueryResultAnimation {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inlineQueryResultAnimation".to_string(),
      id: None,
      animation: None,
      title: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn animation(&self) -> Option<Animation> { self.animation.clone() }
  #[doc(hidden)] pub fn _set_animation(&mut self, animation: Animation) -> &mut Self { self.animation = Some(animation); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



