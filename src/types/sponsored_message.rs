use crate::errors::*;
use crate::types::*;
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
  /// True, if the message needs to be labeled as "recommended" instead of "sponsored"
  is_recommended: bool,
  /// True, if the message can be reported to Telegram moderators through reportChatSponsoredMessage
  can_be_reported: bool,
  /// Content of the message. Currently, can be only of the types messageText, messageAnimation, messagePhoto, or messageVideo. Video messages can be viewed fullscreen. The content must be fully downloaded before the message is shown
  content: MessageContent,
  /// Information about the sponsor of the message
  sponsor: AdvertisementSponsor,
  /// Title of the sponsored message
  title: String,
  /// Text for the message action button
  button_text: String,
  /// Identifier of the accent color for title, button text and message background
  accent_color_id: i64,
  /// Identifier of a custom emoji to be shown on the message background; 0 if none
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  background_custom_emoji_id: isize,
  /// If non-empty, additional information about the sponsored message to be shown along with the message
  additional_info: String,
}

impl RObject for SponsoredMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sponsoredMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SponsoredMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSponsoredMessageBuilder {
    let inner = SponsoredMessage {
      td_name: "sponsoredMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSponsoredMessageBuilder { inner }
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn is_recommended(&self) -> bool {
    self.is_recommended
  }

  pub fn can_be_reported(&self) -> bool {
    self.can_be_reported
  }

  pub fn content(&self) -> &MessageContent {
    &self.content
  }

  pub fn sponsor(&self) -> &AdvertisementSponsor {
    &self.sponsor
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn button_text(&self) -> &String {
    &self.button_text
  }

  pub fn accent_color_id(&self) -> i64 {
    self.accent_color_id
  }

  pub fn background_custom_emoji_id(&self) -> isize {
    self.background_custom_emoji_id
  }

  pub fn additional_info(&self) -> &String {
    &self.additional_info
  }
}

#[doc(hidden)]
pub struct RTDSponsoredMessageBuilder {
  inner: SponsoredMessage,
}

impl RTDSponsoredMessageBuilder {
  pub fn build(&self) -> SponsoredMessage {
    self.inner.clone()
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

  pub fn is_recommended(&mut self, is_recommended: bool) -> &mut Self {
    self.inner.is_recommended = is_recommended;
    self
  }

  pub fn can_be_reported(&mut self, can_be_reported: bool) -> &mut Self {
    self.inner.can_be_reported = can_be_reported;
    self
  }

  pub fn content<T: AsRef<MessageContent>>(&mut self, content: T) -> &mut Self {
    self.inner.content = content.as_ref().clone();
    self
  }

  pub fn sponsor<T: AsRef<AdvertisementSponsor>>(&mut self, sponsor: T) -> &mut Self {
    self.inner.sponsor = sponsor.as_ref().clone();
    self
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn button_text<T: AsRef<str>>(&mut self, button_text: T) -> &mut Self {
    self.inner.button_text = button_text.as_ref().to_string();
    self
  }

  pub fn accent_color_id(&mut self, accent_color_id: i64) -> &mut Self {
    self.inner.accent_color_id = accent_color_id;
    self
  }

  pub fn background_custom_emoji_id(&mut self, background_custom_emoji_id: isize) -> &mut Self {
    self.inner.background_custom_emoji_id = background_custom_emoji_id;
    self
  }

  pub fn additional_info<T: AsRef<str>>(&mut self, additional_info: T) -> &mut Self {
    self.inner.additional_info = additional_info.as_ref().to_string();
    self
  }
}

impl AsRef<SponsoredMessage> for SponsoredMessage {
  fn as_ref(&self) -> &SponsoredMessage {
    self
  }
}

impl AsRef<SponsoredMessage> for RTDSponsoredMessageBuilder {
  fn as_ref(&self) -> &SponsoredMessage {
    &self.inner
  }
}
