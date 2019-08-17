
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Represents a part of the text which must be formatted differently. 
#[typetag::serde(tag = "@struct")]
pub trait TextEntityType: Object + RObject + Debug {}






impl TextEntityType {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<TextEntityType> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDTextEntityTypeType {
  TextEntityTypeBold,
  TextEntityTypeBotCommand,
  TextEntityTypeCashtag,
  TextEntityTypeCode,
  TextEntityTypeEmailAddress,
  TextEntityTypeHashtag,
  TextEntityTypeItalic,
  TextEntityTypeMention,
  TextEntityTypeMentionName,
  TextEntityTypePhoneNumber,
  TextEntityTypePre,
  TextEntityTypePreCode,
  TextEntityTypeTextUrl,
  TextEntityTypeUrl,
  
}
impl RTDTextEntityTypeType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDTextEntityTypeType)(text.as_ref()) }
}



