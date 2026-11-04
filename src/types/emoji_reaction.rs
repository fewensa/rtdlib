use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about an emoji reaction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiReaction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text representation of the reaction
  emoji: String,
  /// Reaction title
  title: String,
  /// True, if the reaction can be added to new messages and enabled in chats
  is_active: bool,
  /// Static icon for the reaction
  static_icon: Sticker,
  /// Appear animation for the reaction
  appear_animation: Sticker,
  /// Select animation for the reaction
  select_animation: Sticker,
  /// Activate animation for the reaction
  activate_animation: Sticker,
  /// Effect animation for the reaction
  effect_animation: Sticker,
  /// Around animation for the reaction; may be null
  around_animation: Option<Sticker>,
  /// Center animation for the reaction; may be null
  center_animation: Option<Sticker>,
}

impl RObject for EmojiReaction {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emojiReaction"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl EmojiReaction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmojiReactionBuilder {
    let inner = EmojiReaction {
      td_name: "emojiReaction".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDEmojiReactionBuilder { inner }
  }

  pub fn emoji(&self) -> &String {
    &self.emoji
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn is_active(&self) -> bool {
    self.is_active
  }

  pub fn static_icon(&self) -> &Sticker {
    &self.static_icon
  }

  pub fn appear_animation(&self) -> &Sticker {
    &self.appear_animation
  }

  pub fn select_animation(&self) -> &Sticker {
    &self.select_animation
  }

  pub fn activate_animation(&self) -> &Sticker {
    &self.activate_animation
  }

  pub fn effect_animation(&self) -> &Sticker {
    &self.effect_animation
  }

  pub fn around_animation(&self) -> &Option<Sticker> {
    &self.around_animation
  }

  pub fn center_animation(&self) -> &Option<Sticker> {
    &self.center_animation
  }
}

#[doc(hidden)]
pub struct RTDEmojiReactionBuilder {
  inner: EmojiReaction,
}

impl RTDEmojiReactionBuilder {
  pub fn build(&self) -> EmojiReaction {
    self.inner.clone()
  }

  pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
    self.inner.emoji = emoji.as_ref().to_string();
    self
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn is_active(&mut self, is_active: bool) -> &mut Self {
    self.inner.is_active = is_active;
    self
  }

  pub fn static_icon<T: AsRef<Sticker>>(&mut self, static_icon: T) -> &mut Self {
    self.inner.static_icon = static_icon.as_ref().clone();
    self
  }

  pub fn appear_animation<T: AsRef<Sticker>>(&mut self, appear_animation: T) -> &mut Self {
    self.inner.appear_animation = appear_animation.as_ref().clone();
    self
  }

  pub fn select_animation<T: AsRef<Sticker>>(&mut self, select_animation: T) -> &mut Self {
    self.inner.select_animation = select_animation.as_ref().clone();
    self
  }

  pub fn activate_animation<T: AsRef<Sticker>>(&mut self, activate_animation: T) -> &mut Self {
    self.inner.activate_animation = activate_animation.as_ref().clone();
    self
  }

  pub fn effect_animation<T: AsRef<Sticker>>(&mut self, effect_animation: T) -> &mut Self {
    self.inner.effect_animation = effect_animation.as_ref().clone();
    self
  }

  pub fn around_animation<T: AsRef<Sticker>>(&mut self, around_animation: T) -> &mut Self {
    self.inner.around_animation = Some(around_animation.as_ref().clone());
    self
  }

  pub fn center_animation<T: AsRef<Sticker>>(&mut self, center_animation: T) -> &mut Self {
    self.inner.center_animation = Some(center_animation.as_ref().clone());
    self
  }
}

impl AsRef<EmojiReaction> for EmojiReaction {
  fn as_ref(&self) -> &EmojiReaction {
    self
  }
}

impl AsRef<EmojiReaction> for RTDEmojiReactionBuilder {
  fn as_ref(&self) -> &EmojiReaction {
    &self.inner
  }
}
