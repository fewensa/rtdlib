
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A text shows instead of a raw mention of the user (e.g., when the user has no username). 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEntityTypeMentionName {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // textEntityTypeMentionName
  /// Identifier of the mentioned user.
  user_id: Option<i32>,
  
}



impl Object for TextEntityTypeMentionName {}
impl RObject for TextEntityTypeMentionName {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeMentionName" }
  fn td_type(&self) -> RTDType { RTDType::TextEntityTypeMentionName }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TextEntityType for TextEntityTypeMentionName {}


impl TextEntityTypeMentionName {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "textEntityTypeMentionName".to_string(),
      user_id: None,
      
    }
  }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



