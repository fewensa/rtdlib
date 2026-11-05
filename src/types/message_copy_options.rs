use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Options to be used when a message content is copied without reference to the original sender. Service messages, messages with messageInvoice, messagePaidMedia, messageGiveaway, or messageGiveawayWinners content can't be copied
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageCopyOptions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if content of the message needs to be copied without reference to the original sender. Always true if the message is forwarded to a secret chat or is local. Use messageProperties.can_be_copied and messageProperties.can_be_copied_to_secret_chat to check whether the message is suitable
  send_copy: bool,
  /// True, if media caption of the message copy needs to be replaced. Ignored if send_copy is false
  replace_caption: bool,
  /// New message caption; pass null to copy message without caption. Ignored if replace_caption is false
  new_caption: FormattedText,
  /// True, if new caption must be shown above the media; otherwise, new caption must be shown below the media; not supported in secret chats. Ignored if replace_caption is false
  new_show_caption_above_media: bool,
}

impl RObject for MessageCopyOptions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageCopyOptions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl MessageCopyOptions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageCopyOptionsBuilder {
    let inner = MessageCopyOptions {
      td_name: "messageCopyOptions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageCopyOptionsBuilder { inner }
  }

  pub fn send_copy(&self) -> bool {
    self.send_copy
  }

  pub fn replace_caption(&self) -> bool {
    self.replace_caption
  }

  pub fn new_caption(&self) -> &FormattedText {
    &self.new_caption
  }

  pub fn new_show_caption_above_media(&self) -> bool {
    self.new_show_caption_above_media
  }
}

#[doc(hidden)]
pub struct RTDMessageCopyOptionsBuilder {
  inner: MessageCopyOptions,
}

impl RTDMessageCopyOptionsBuilder {
  pub fn build(&self) -> MessageCopyOptions {
    self.inner.clone()
  }

  pub fn send_copy(&mut self, send_copy: bool) -> &mut Self {
    self.inner.send_copy = send_copy;
    self
  }

  pub fn replace_caption(&mut self, replace_caption: bool) -> &mut Self {
    self.inner.replace_caption = replace_caption;
    self
  }

  pub fn new_caption<T: AsRef<FormattedText>>(&mut self, new_caption: T) -> &mut Self {
    self.inner.new_caption = new_caption.as_ref().clone();
    self
  }

  pub fn new_show_caption_above_media(&mut self, new_show_caption_above_media: bool) -> &mut Self {
    self.inner.new_show_caption_above_media = new_show_caption_above_media;
    self
  }
}

impl AsRef<MessageCopyOptions> for MessageCopyOptions {
  fn as_ref(&self) -> &MessageCopyOptions {
    self
  }
}

impl AsRef<MessageCopyOptions> for RTDMessageCopyOptionsBuilder {
  fn as_ref(&self) -> &MessageCopyOptions {
    &self.inner
  }
}
