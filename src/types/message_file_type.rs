
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Contains information about a file with messages exported from another app
pub trait TDMessageFileType: Debug + RObject {}

/// Contains information about a file with messages exported from another app
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MessageFileType {
  #[doc(hidden)] _Default(()),
  /// Returns information about a file with messages exported from another app
  GetMessageFileType(GetMessageFileType),
  /// The messages was exported from a group chat
  Group(MessageFileTypeGroup),
  /// The messages was exported from a private chat
  Private(MessageFileTypePrivate),
  /// The messages was exported from a chat of unknown type
  Unknown(MessageFileTypeUnknown),

}

impl Default for MessageFileType {
  fn default() -> Self { MessageFileType::_Default(()) }
}

impl<'de> Deserialize<'de> for MessageFileType {
  fn deserialize<D>(deserializer: D) -> Result<MessageFileType, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      MessageFileType,
      (getMessageFileType, GetMessageFileType);
      (messageFileTypeGroup, Group);
      (messageFileTypePrivate, Private);
      (messageFileTypeUnknown, Unknown);

    )(deserializer)
  }
}

impl RObject for MessageFileType {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      MessageFileType::GetMessageFileType(t) => t.td_name(),
      MessageFileType::Group(t) => t.td_name(),
      MessageFileType::Private(t) => t.td_name(),
      MessageFileType::Unknown(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      MessageFileType::GetMessageFileType(t) => t.extra(),
      MessageFileType::Group(t) => t.extra(),
      MessageFileType::Private(t) => t.extra(),
      MessageFileType::Unknown(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl MessageFileType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let MessageFileType::_Default(_) = self { true } else { false } }

  pub fn is_get_message_file_type(&self) -> bool { if let MessageFileType::GetMessageFileType(_) = self { true } else { false } }
  pub fn is_group(&self) -> bool { if let MessageFileType::Group(_) = self { true } else { false } }
  pub fn is_private(&self) -> bool { if let MessageFileType::Private(_) = self { true } else { false } }
  pub fn is_unknown(&self) -> bool { if let MessageFileType::Unknown(_) = self { true } else { false } }

  pub fn on_get_message_file_type<F: FnOnce(&GetMessageFileType)>(&self, fnc: F) -> &Self { if let MessageFileType::GetMessageFileType(t) = self { fnc(t) }; self }
  pub fn on_group<F: FnOnce(&MessageFileTypeGroup)>(&self, fnc: F) -> &Self { if let MessageFileType::Group(t) = self { fnc(t) }; self }
  pub fn on_private<F: FnOnce(&MessageFileTypePrivate)>(&self, fnc: F) -> &Self { if let MessageFileType::Private(t) = self { fnc(t) }; self }
  pub fn on_unknown<F: FnOnce(&MessageFileTypeUnknown)>(&self, fnc: F) -> &Self { if let MessageFileType::Unknown(t) = self { fnc(t) }; self }

  pub fn as_get_message_file_type(&self) -> Option<&GetMessageFileType> { if let MessageFileType::GetMessageFileType(t) = self { return Some(t) } None }
  pub fn as_group(&self) -> Option<&MessageFileTypeGroup> { if let MessageFileType::Group(t) = self { return Some(t) } None }
  pub fn as_private(&self) -> Option<&MessageFileTypePrivate> { if let MessageFileType::Private(t) = self { return Some(t) } None }
  pub fn as_unknown(&self) -> Option<&MessageFileTypeUnknown> { if let MessageFileType::Unknown(t) = self { return Some(t) } None }



  pub fn get_message_file_type<T: AsRef<GetMessageFileType>>(t: T) -> Self { MessageFileType::GetMessageFileType(t.as_ref().clone()) }

  pub fn group<T: AsRef<MessageFileTypeGroup>>(t: T) -> Self { MessageFileType::Group(t.as_ref().clone()) }

  pub fn private<T: AsRef<MessageFileTypePrivate>>(t: T) -> Self { MessageFileType::Private(t.as_ref().clone()) }

  pub fn unknown<T: AsRef<MessageFileTypeUnknown>>(t: T) -> Self { MessageFileType::Unknown(t.as_ref().clone()) }

}

impl AsRef<MessageFileType> for MessageFileType {
  fn as_ref(&self) -> &MessageFileType { self }
}







/// The messages was exported from a group chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageFileTypeGroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Title of the group chat; may be empty if unrecognized
  title: String,
  
}

impl RObject for MessageFileTypeGroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageFileTypeGroup" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageFileType for MessageFileTypeGroup {}



impl MessageFileTypeGroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageFileTypeGroupBuilder {
    let mut inner = MessageFileTypeGroup::default();
    inner.td_name = "messageFileTypeGroup".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDMessageFileTypeGroupBuilder { inner }
  }

  pub fn title(&self) -> &String { &self.title }

}

#[doc(hidden)]
pub struct RTDMessageFileTypeGroupBuilder {
  inner: MessageFileTypeGroup
}

impl RTDMessageFileTypeGroupBuilder {
  pub fn build(&self) -> MessageFileTypeGroup { self.inner.clone() }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

}

impl AsRef<MessageFileTypeGroup> for MessageFileTypeGroup {
  fn as_ref(&self) -> &MessageFileTypeGroup { self }
}

impl AsRef<MessageFileTypeGroup> for RTDMessageFileTypeGroupBuilder {
  fn as_ref(&self) -> &MessageFileTypeGroup { &self.inner }
}







/// The messages was exported from a private chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageFileTypePrivate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the other party; may be empty if unrecognized
  name: String,
  
}

impl RObject for MessageFileTypePrivate {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageFileTypePrivate" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageFileType for MessageFileTypePrivate {}



impl MessageFileTypePrivate {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageFileTypePrivateBuilder {
    let mut inner = MessageFileTypePrivate::default();
    inner.td_name = "messageFileTypePrivate".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDMessageFileTypePrivateBuilder { inner }
  }

  pub fn name(&self) -> &String { &self.name }

}

#[doc(hidden)]
pub struct RTDMessageFileTypePrivateBuilder {
  inner: MessageFileTypePrivate
}

impl RTDMessageFileTypePrivateBuilder {
  pub fn build(&self) -> MessageFileTypePrivate { self.inner.clone() }

   
  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

}

impl AsRef<MessageFileTypePrivate> for MessageFileTypePrivate {
  fn as_ref(&self) -> &MessageFileTypePrivate { self }
}

impl AsRef<MessageFileTypePrivate> for RTDMessageFileTypePrivateBuilder {
  fn as_ref(&self) -> &MessageFileTypePrivate { &self.inner }
}







/// The messages was exported from a chat of unknown type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageFileTypeUnknown {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for MessageFileTypeUnknown {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageFileTypeUnknown" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageFileType for MessageFileTypeUnknown {}



impl MessageFileTypeUnknown {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageFileTypeUnknownBuilder {
    let mut inner = MessageFileTypeUnknown::default();
    inner.td_name = "messageFileTypeUnknown".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDMessageFileTypeUnknownBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDMessageFileTypeUnknownBuilder {
  inner: MessageFileTypeUnknown
}

impl RTDMessageFileTypeUnknownBuilder {
  pub fn build(&self) -> MessageFileTypeUnknown { self.inner.clone() }

}

impl AsRef<MessageFileTypeUnknown> for MessageFileTypeUnknown {
  fn as_ref(&self) -> &MessageFileTypeUnknown { self }
}

impl AsRef<MessageFileTypeUnknown> for RTDMessageFileTypeUnknownBuilder {
  fn as_ref(&self) -> &MessageFileTypeUnknown { &self.inner }
}



