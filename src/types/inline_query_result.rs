
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Represents a single result of an inline query. 
#[typetag::serde(tag = "@struct")]
pub trait InlineQueryResult: Object + RObject + Debug {}






impl InlineQueryResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<InlineQueryResult> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDInlineQueryResultType {
  InlineQueryResultAnimation,
  InlineQueryResultArticle,
  InlineQueryResultAudio,
  InlineQueryResultContact,
  InlineQueryResultDocument,
  InlineQueryResultGame,
  InlineQueryResultLocation,
  InlineQueryResultPhoto,
  InlineQueryResultSticker,
  InlineQueryResultVenue,
  InlineQueryResultVideo,
  InlineQueryResultVoiceNote,
  
}
impl RTDInlineQueryResultType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDInlineQueryResultType)(text.as_ref()) }
}



