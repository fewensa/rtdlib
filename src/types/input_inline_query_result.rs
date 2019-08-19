
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Represents a single result of an inline query; for bots only. 
#[typetag::serde(tag = "@struct")]
pub trait InputInlineQueryResult: Object + RObject + Debug {}






impl InputInlineQueryResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<InputInlineQueryResult> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDInputInlineQueryResultType {
  InputInlineQueryResultAnimatedGif,
  InputInlineQueryResultAnimatedMpeg4,
  InputInlineQueryResultArticle,
  InputInlineQueryResultAudio,
  InputInlineQueryResultContact,
  InputInlineQueryResultDocument,
  InputInlineQueryResultGame,
  InputInlineQueryResultLocation,
  InputInlineQueryResultPhoto,
  InputInlineQueryResultSticker,
  InputInlineQueryResultVenue,
  InputInlineQueryResultVideo,
  InputInlineQueryResultVoiceNote,
  
}
impl RTDInputInlineQueryResultType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDInputInlineQueryResultType)(text.as_ref()) }
}



