
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Represents a filter for message search results. 
#[typetag::serde(tag = "@struct")]
pub trait SearchMessagesFilter: Object + RObject + Debug {}






impl SearchMessagesFilter {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<SearchMessagesFilter> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDSearchMessagesFilterType {
  SearchMessagesFilterAnimation,
  SearchMessagesFilterAudio,
  SearchMessagesFilterCall,
  SearchMessagesFilterChatPhoto,
  SearchMessagesFilterDocument,
  SearchMessagesFilterEmpty,
  SearchMessagesFilterMention,
  SearchMessagesFilterMissedCall,
  SearchMessagesFilterPhoto,
  SearchMessagesFilterPhotoAndVideo,
  SearchMessagesFilterUnreadMention,
  SearchMessagesFilterUrl,
  SearchMessagesFilterVideo,
  SearchMessagesFilterVideoNote,
  SearchMessagesFilterVoiceAndVideoNote,
  SearchMessagesFilterVoiceNote,
  
}
impl RTDSearchMessagesFilterType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDSearchMessagesFilterType)(text.as_ref()) }
}



