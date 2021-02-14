
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Contains information about the initial sender of a forwarded message
pub trait TDMessageForwardInfo: Debug + RObject {}

/// Contains information about the initial sender of a forwarded message
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MessageForwardInfo {
  #[doc(hidden)] _Default(()),
  /// The message was originally written by a known user
  MessageForwardedFromUser(MessageForwardedFromUser),
  /// The message was originally a post in a channel
  MessageForwardedPost(MessageForwardedPost),

}

impl Default for MessageForwardInfo {
  fn default() -> Self { MessageForwardInfo::_Default(()) }
}

impl<'de> Deserialize<'de> for MessageForwardInfo {
  fn deserialize<D>(deserializer: D) -> Result<MessageForwardInfo, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      MessageForwardInfo,
      (messageForwardedFromUser, MessageForwardedFromUser);
      (messageForwardedPost, MessageForwardedPost);

    )(deserializer)
  }
}

impl RObject for MessageForwardInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      MessageForwardInfo::MessageForwardedFromUser(t) => t.td_name(),
      MessageForwardInfo::MessageForwardedPost(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      MessageForwardInfo::MessageForwardedFromUser(t) => t.extra(),
      MessageForwardInfo::MessageForwardedPost(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl MessageForwardInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let MessageForwardInfo::_Default(_) = self { true } else { false } }

  pub fn is_message_forwarded_from_user(&self) -> bool { if let MessageForwardInfo::MessageForwardedFromUser(_) = self { true } else { false } }
  pub fn is_message_forwarded_post(&self) -> bool { if let MessageForwardInfo::MessageForwardedPost(_) = self { true } else { false } }

  pub fn on_message_forwarded_from_user<F: FnOnce(&MessageForwardedFromUser)>(&self, fnc: F) -> &Self { if let MessageForwardInfo::MessageForwardedFromUser(t) = self { fnc(t) }; self }
  pub fn on_message_forwarded_post<F: FnOnce(&MessageForwardedPost)>(&self, fnc: F) -> &Self { if let MessageForwardInfo::MessageForwardedPost(t) = self { fnc(t) }; self }

  pub fn as_message_forwarded_from_user(&self) -> Option<&MessageForwardedFromUser> { if let MessageForwardInfo::MessageForwardedFromUser(t) = self { return Some(t) } None }
  pub fn as_message_forwarded_post(&self) -> Option<&MessageForwardedPost> { if let MessageForwardInfo::MessageForwardedPost(t) = self { return Some(t) } None }



  pub fn message_forwarded_from_user<T: AsRef<MessageForwardedFromUser>>(t: T) -> Self { MessageForwardInfo::MessageForwardedFromUser(t.as_ref().clone()) }

  pub fn message_forwarded_post<T: AsRef<MessageForwardedPost>>(t: T) -> Self { MessageForwardInfo::MessageForwardedPost(t.as_ref().clone()) }

}

impl AsRef<MessageForwardInfo> for MessageForwardInfo {
  fn as_ref(&self) -> &MessageForwardInfo { self }
}







/// The message was originally written by a known user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageForwardedFromUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user that originally sent this message
  sender_user_id: i64,
  /// Point in time (Unix timestamp) when the message was originally sent
  date: i64,
  /// For messages forwarded to the chat with the current user (saved messages), the identifier of the chat from which the message was forwarded; 0 if unknown
  forwarded_from_chat_id: i64,
  /// For messages forwarded to the chat with the current user (saved messages) the identifier of the original message from which the new message was forwarded; 0 if unknown
  forwarded_from_message_id: i64,
  
}

impl RObject for MessageForwardedFromUser {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageForwardedFromUser" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageForwardInfo for MessageForwardedFromUser {}



impl MessageForwardedFromUser {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageForwardedFromUserBuilder {
    let mut inner = MessageForwardedFromUser::default();
    inner.td_name = "messageForwardedFromUser".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDMessageForwardedFromUserBuilder { inner }
  }

  pub fn sender_user_id(&self) -> i64 { self.sender_user_id }

  pub fn date(&self) -> i64 { self.date }

  pub fn forwarded_from_chat_id(&self) -> i64 { self.forwarded_from_chat_id }

  pub fn forwarded_from_message_id(&self) -> i64 { self.forwarded_from_message_id }

}

#[doc(hidden)]
pub struct RTDMessageForwardedFromUserBuilder {
  inner: MessageForwardedFromUser
}

impl RTDMessageForwardedFromUserBuilder {
  pub fn build(&self) -> MessageForwardedFromUser { self.inner.clone() }

   
  pub fn sender_user_id(&mut self, sender_user_id: i64) -> &mut Self {
    self.inner.sender_user_id = sender_user_id;
    self
  }

   
  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

   
  pub fn forwarded_from_chat_id(&mut self, forwarded_from_chat_id: i64) -> &mut Self {
    self.inner.forwarded_from_chat_id = forwarded_from_chat_id;
    self
  }

   
  pub fn forwarded_from_message_id(&mut self, forwarded_from_message_id: i64) -> &mut Self {
    self.inner.forwarded_from_message_id = forwarded_from_message_id;
    self
  }

}

impl AsRef<MessageForwardedFromUser> for MessageForwardedFromUser {
  fn as_ref(&self) -> &MessageForwardedFromUser { self }
}

impl AsRef<MessageForwardedFromUser> for RTDMessageForwardedFromUserBuilder {
  fn as_ref(&self) -> &MessageForwardedFromUser { &self.inner }
}







/// The message was originally a post in a channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageForwardedPost {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat from which the message was forwarded
  chat_id: i64,
  /// Post author signature
  author_signature: String,
  /// Point in time (Unix timestamp) when the message was originally sent
  date: i64,
  /// Message identifier of the original message from which the new message was forwarded; 0 if unknown
  message_id: i64,
  /// For messages forwarded to the chat with the current user (saved messages), the identifier of the chat from which the message was forwarded; 0 if unknown
  forwarded_from_chat_id: i64,
  /// For messages forwarded to the chat with the current user (saved messages), the identifier of the original message from which the new message was forwarded; 0 if unknown
  forwarded_from_message_id: i64,
  
}

impl RObject for MessageForwardedPost {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageForwardedPost" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageForwardInfo for MessageForwardedPost {}



impl MessageForwardedPost {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageForwardedPostBuilder {
    let mut inner = MessageForwardedPost::default();
    inner.td_name = "messageForwardedPost".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDMessageForwardedPostBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn author_signature(&self) -> &String { &self.author_signature }

  pub fn date(&self) -> i64 { self.date }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn forwarded_from_chat_id(&self) -> i64 { self.forwarded_from_chat_id }

  pub fn forwarded_from_message_id(&self) -> i64 { self.forwarded_from_message_id }

}

#[doc(hidden)]
pub struct RTDMessageForwardedPostBuilder {
  inner: MessageForwardedPost
}

impl RTDMessageForwardedPostBuilder {
  pub fn build(&self) -> MessageForwardedPost { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn author_signature<T: AsRef<str>>(&mut self, author_signature: T) -> &mut Self {
    self.inner.author_signature = author_signature.as_ref().to_string();
    self
  }

   
  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn forwarded_from_chat_id(&mut self, forwarded_from_chat_id: i64) -> &mut Self {
    self.inner.forwarded_from_chat_id = forwarded_from_chat_id;
    self
  }

   
  pub fn forwarded_from_message_id(&mut self, forwarded_from_message_id: i64) -> &mut Self {
    self.inner.forwarded_from_message_id = forwarded_from_message_id;
    self
  }

}

impl AsRef<MessageForwardedPost> for MessageForwardedPost {
  fn as_ref(&self) -> &MessageForwardedPost { self }
}

impl AsRef<MessageForwardedPost> for RTDMessageForwardedPostBuilder {
  fn as_ref(&self) -> &MessageForwardedPost { &self.inner }
}



