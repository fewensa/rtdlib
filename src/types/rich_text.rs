
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Describes a text object inside an instant-view web page. 
#[typetag::serde(tag = "@struct")]
pub trait RichText: Object + RObject + Debug {}






impl RichText {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<RichText> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDRichTextType {
  RichTextAnchor,
  RichTextBold,
  RichTextEmailAddress,
  RichTextFixed,
  RichTextIcon,
  RichTextItalic,
  RichTextMarked,
  RichTextPhoneNumber,
  RichTextPlain,
  RichTexts,
  RichTextStrikethrough,
  RichTextSubscript,
  RichTextSuperscript,
  RichTextUnderline,
  RichTextUrl,
  
}
impl RTDRichTextType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDRichTextType)(text.as_ref()) }
}



