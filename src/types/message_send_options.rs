use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Options to be used when a message is sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSendOptions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Information about the suggested post; pass null if none. For messages to channel direct messages chat only. Applicable only to sendMessage and addOffer
  suggested_post_info: InputSuggestedPostInfo,
  /// Pass true to disable notification for the message
  disable_notification: bool,
  /// Pass true if the message is sent from the background
  from_background: bool,
  /// Pass true if the content of the message must be protected from forwarding and saving; for bots only
  protect_content: bool,
  /// Pass true to allow the message to ignore regular broadcast limits for a small fee; for bots only
  allow_paid_broadcast: bool,
  /// The number of Telegram Stars the user agreed to pay to send the messages
  paid_message_star_count: i64,
  /// Pass true if the user has explicitly chosen a sticker or a custom emoji from an installed sticker set; applicable only to sendMessage and sendMessageAlbum
  update_order_of_installed_sticker_sets: bool,
  /// Message scheduling state; pass null to send message immediately. Messages sent to a secret chat, to a chat with paid messages, to a channel direct messages chat, live location messages and self-destructing messages can't be scheduled
  scheduling_state: MessageSchedulingState,
  /// Identifier of the effect to apply to the message; pass 0 if none; applicable only to sendMessage, sendMessageAlbum in private chats and forwardMessages with one message to private chats
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  effect_id: isize,
  /// Non-persistent identifier, which will be returned back in messageSendingStatePending object and can be used to match sent messages and corresponding updateNewMessage updates
  sending_id: i64,
  /// Pass true to get a fake message instead of actually sending them
  only_preview: bool,
}

impl RObject for MessageSendOptions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageSendOptions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl MessageSendOptions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageSendOptionsBuilder {
    let inner = MessageSendOptions {
      td_name: "messageSendOptions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageSendOptionsBuilder { inner }
  }

  pub fn suggested_post_info(&self) -> &InputSuggestedPostInfo {
    &self.suggested_post_info
  }

  pub fn disable_notification(&self) -> bool {
    self.disable_notification
  }

  pub fn from_background(&self) -> bool {
    self.from_background
  }

  pub fn protect_content(&self) -> bool {
    self.protect_content
  }

  pub fn allow_paid_broadcast(&self) -> bool {
    self.allow_paid_broadcast
  }

  pub fn paid_message_star_count(&self) -> i64 {
    self.paid_message_star_count
  }

  pub fn update_order_of_installed_sticker_sets(&self) -> bool {
    self.update_order_of_installed_sticker_sets
  }

  pub fn scheduling_state(&self) -> &MessageSchedulingState {
    &self.scheduling_state
  }

  pub fn effect_id(&self) -> isize {
    self.effect_id
  }

  pub fn sending_id(&self) -> i64 {
    self.sending_id
  }

  pub fn only_preview(&self) -> bool {
    self.only_preview
  }
}

#[doc(hidden)]
pub struct RTDMessageSendOptionsBuilder {
  inner: MessageSendOptions,
}

impl RTDMessageSendOptionsBuilder {
  pub fn build(&self) -> MessageSendOptions {
    self.inner.clone()
  }

  pub fn suggested_post_info<T: AsRef<InputSuggestedPostInfo>>(
    &mut self,
    suggested_post_info: T,
  ) -> &mut Self {
    self.inner.suggested_post_info = suggested_post_info.as_ref().clone();
    self
  }

  pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self {
    self.inner.disable_notification = disable_notification;
    self
  }

  pub fn from_background(&mut self, from_background: bool) -> &mut Self {
    self.inner.from_background = from_background;
    self
  }

  pub fn protect_content(&mut self, protect_content: bool) -> &mut Self {
    self.inner.protect_content = protect_content;
    self
  }

  pub fn allow_paid_broadcast(&mut self, allow_paid_broadcast: bool) -> &mut Self {
    self.inner.allow_paid_broadcast = allow_paid_broadcast;
    self
  }

  pub fn paid_message_star_count(&mut self, paid_message_star_count: i64) -> &mut Self {
    self.inner.paid_message_star_count = paid_message_star_count;
    self
  }

  pub fn update_order_of_installed_sticker_sets(
    &mut self,
    update_order_of_installed_sticker_sets: bool,
  ) -> &mut Self {
    self.inner.update_order_of_installed_sticker_sets = update_order_of_installed_sticker_sets;
    self
  }

  pub fn scheduling_state<T: AsRef<MessageSchedulingState>>(
    &mut self,
    scheduling_state: T,
  ) -> &mut Self {
    self.inner.scheduling_state = scheduling_state.as_ref().clone();
    self
  }

  pub fn effect_id(&mut self, effect_id: isize) -> &mut Self {
    self.inner.effect_id = effect_id;
    self
  }

  pub fn sending_id(&mut self, sending_id: i64) -> &mut Self {
    self.inner.sending_id = sending_id;
    self
  }

  pub fn only_preview(&mut self, only_preview: bool) -> &mut Self {
    self.inner.only_preview = only_preview;
    self
  }
}

impl AsRef<MessageSendOptions> for MessageSendOptions {
  fn as_ref(&self) -> &MessageSendOptions {
    self
  }
}

impl AsRef<MessageSendOptions> for RTDMessageSendOptionsBuilder {
  fn as_ref(&self) -> &MessageSendOptions {
    &self.inner
  }
}
