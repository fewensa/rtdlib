
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Describes the way the text should be parsed for TextEntities. 
#[typetag::serde(tag = "@struct")]
pub trait TextParseMode: Object + RObject + Debug {}






impl TextParseMode {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<TextParseMode> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDTextParseModeType {
  TextParseModeHTML,
  TextParseModeMarkdown,
  
}
impl RTDTextParseModeType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDTextParseModeType)(text.as_ref()) }
}



