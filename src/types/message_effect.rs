use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about an effect added to a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageEffect {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the effect
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  id: isize,
  /// Static icon for the effect in WEBP format; may be null if none
  static_icon: Option<Sticker>,
  /// Emoji corresponding to the effect that can be used if static icon isn't available
  emoji: String,
  /// True, if Telegram Premium subscription is required to use the effect
  is_premium: bool,
  /// Type of the effect
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: MessageEffectType,
}

impl RObject for MessageEffect {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageEffect"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl MessageEffect {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageEffectBuilder {
    let inner = MessageEffect {
      td_name: "messageEffect".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageEffectBuilder { inner }
  }

  pub fn id(&self) -> isize {
    self.id
  }

  pub fn static_icon(&self) -> &Option<Sticker> {
    &self.static_icon
  }

  pub fn emoji(&self) -> &String {
    &self.emoji
  }

  pub fn is_premium(&self) -> bool {
    self.is_premium
  }

  pub fn type_(&self) -> &MessageEffectType {
    &self.type_
  }
}

#[doc(hidden)]
pub struct RTDMessageEffectBuilder {
  inner: MessageEffect,
}

impl RTDMessageEffectBuilder {
  pub fn build(&self) -> MessageEffect {
    self.inner.clone()
  }

  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn static_icon<T: AsRef<Sticker>>(&mut self, static_icon: T) -> &mut Self {
    self.inner.static_icon = Some(static_icon.as_ref().clone());
    self
  }

  pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
    self.inner.emoji = emoji.as_ref().to_string();
    self
  }

  pub fn is_premium(&mut self, is_premium: bool) -> &mut Self {
    self.inner.is_premium = is_premium;
    self
  }

  pub fn type_<T: AsRef<MessageEffectType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }
}

impl AsRef<MessageEffect> for MessageEffect {
  fn as_ref(&self) -> &MessageEffect {
    self
  }
}

impl AsRef<MessageEffect> for RTDMessageEffectBuilder {
  fn as_ref(&self) -> &MessageEffect {
    &self.inner
  }
}
