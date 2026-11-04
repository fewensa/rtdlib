use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a message that can be used for quick reply
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuickReplyMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique message identifier among all quick replies
  id: i64,
  /// The sending state of the message; may be null if the message isn't being sent and didn't fail to be sent
  sending_state: Option<MessageSendingState>,
  /// True, if the message can be edited
  can_be_edited: bool,
  /// The identifier of the quick reply message to which the message replies; 0 if none
  reply_to_message_id: i64,
  /// If non-zero, the user identifier of the bot through which this message was sent
  via_bot_user_id: i64,
  /// Unique identifier of an album this message belongs to; 0 if none. Only audios, documents, photos and videos can be grouped together in albums
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  media_album_id: isize,
  /// Content of the message
  content: MessageContent,
  /// Inline keyboard reply markup for the message; may be null if none
  reply_markup: Option<ReplyMarkup>,
}

impl RObject for QuickReplyMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "quickReplyMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl QuickReplyMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDQuickReplyMessageBuilder {
    let inner = QuickReplyMessage {
      td_name: "quickReplyMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDQuickReplyMessageBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn sending_state(&self) -> &Option<MessageSendingState> {
    &self.sending_state
  }

  pub fn can_be_edited(&self) -> bool {
    self.can_be_edited
  }

  pub fn reply_to_message_id(&self) -> i64 {
    self.reply_to_message_id
  }

  pub fn via_bot_user_id(&self) -> i64 {
    self.via_bot_user_id
  }

  pub fn media_album_id(&self) -> isize {
    self.media_album_id
  }

  pub fn content(&self) -> &MessageContent {
    &self.content
  }

  pub fn reply_markup(&self) -> &Option<ReplyMarkup> {
    &self.reply_markup
  }
}

#[doc(hidden)]
pub struct RTDQuickReplyMessageBuilder {
  inner: QuickReplyMessage,
}

impl RTDQuickReplyMessageBuilder {
  pub fn build(&self) -> QuickReplyMessage {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn sending_state<T: AsRef<MessageSendingState>>(&mut self, sending_state: T) -> &mut Self {
    self.inner.sending_state = Some(sending_state.as_ref().clone());
    self
  }

  pub fn can_be_edited(&mut self, can_be_edited: bool) -> &mut Self {
    self.inner.can_be_edited = can_be_edited;
    self
  }

  pub fn reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self {
    self.inner.reply_to_message_id = reply_to_message_id;
    self
  }

  pub fn via_bot_user_id(&mut self, via_bot_user_id: i64) -> &mut Self {
    self.inner.via_bot_user_id = via_bot_user_id;
    self
  }

  pub fn media_album_id(&mut self, media_album_id: isize) -> &mut Self {
    self.inner.media_album_id = media_album_id;
    self
  }

  pub fn content<T: AsRef<MessageContent>>(&mut self, content: T) -> &mut Self {
    self.inner.content = content.as_ref().clone();
    self
  }

  pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
    self.inner.reply_markup = Some(reply_markup.as_ref().clone());
    self
  }
}

impl AsRef<QuickReplyMessage> for QuickReplyMessage {
  fn as_ref(&self) -> &QuickReplyMessage {
    self
  }
}

impl AsRef<QuickReplyMessage> for RTDQuickReplyMessageBuilder {
  fn as_ref(&self) -> &QuickReplyMessage {
    &self.inner
  }
}
