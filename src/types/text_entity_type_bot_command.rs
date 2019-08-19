
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A bot command, beginning with "/". This shouldn't be highlighted if there are no bots in the chat. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEntityTypeBotCommand {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // textEntityTypeBotCommand
  
}



impl Object for TextEntityTypeBotCommand {}
impl RObject for TextEntityTypeBotCommand {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeBotCommand" }
  fn td_type(&self) -> RTDType { RTDType::TextEntityTypeBotCommand }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TextEntityType for TextEntityTypeBotCommand {}


impl TextEntityTypeBotCommand {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "textEntityTypeBotCommand".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



