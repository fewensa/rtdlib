
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a sponsored message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SponsoredMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Message identifier; unique for the chat to which the sponsored message belongs among both ordinary and sponsored messages
  message_id: i64,
  /// Chat identifier
  sponsor_chat_id: i64,
  /// An internal link to be opened when the sponsored message is clicked; may be null. If null, the sponsor chat needs to be opened instead
  link: Option<InternalLinkType>,
  /// Content of the message. Currently, can be only of the type messageText
  content: MessageContent,
  
}

impl RObject for SponsoredMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sponsoredMessage" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl SponsoredMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSponsoredMessageBuilder {
    let mut inner = SponsoredMessage::default();
    inner.td_name = "sponsoredMessage".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSponsoredMessageBuilder { inner }
  }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn sponsor_chat_id(&self) -> i64 { self.sponsor_chat_id }

  pub fn link(&self) -> &Option<InternalLinkType> { &self.link }

  pub fn content(&self) -> &MessageContent { &self.content }

}

#[doc(hidden)]
pub struct RTDSponsoredMessageBuilder {
  inner: SponsoredMessage
}

impl RTDSponsoredMessageBuilder {
  pub fn build(&self) -> SponsoredMessage { self.inner.clone() }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn sponsor_chat_id(&mut self, sponsor_chat_id: i64) -> &mut Self {
    self.inner.sponsor_chat_id = sponsor_chat_id;
    self
  }

   
  pub fn link<T: AsRef<InternalLinkType>>(&mut self, link: T) -> &mut Self {
    self.inner.link = Some(link.as_ref().clone());
    self
  }

   
  pub fn content<T: AsRef<MessageContent>>(&mut self, content: T) -> &mut Self {
    self.inner.content = content.as_ref().clone();
    self
  }

}

impl AsRef<SponsoredMessage> for SponsoredMessage {
  fn as_ref(&self) -> &SponsoredMessage { self }
}

impl AsRef<SponsoredMessage> for RTDSponsoredMessageBuilder {
  fn as_ref(&self) -> &SponsoredMessage { &self.inner }
}



