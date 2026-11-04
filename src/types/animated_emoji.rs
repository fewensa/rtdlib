use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an animated or custom representation of an emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnimatedEmoji {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Sticker for the emoji; may be null if yet unknown for a custom emoji. If the sticker is a custom emoji, then it can have arbitrary format
  sticker: Option<Sticker>,
  /// Expected width of the sticker, which can be used if the sticker is null
  sticker_width: i64,
  /// Expected height of the sticker, which can be used if the sticker is null
  sticker_height: i64,
  /// Emoji modifier fitzpatrick type; 0-6; 0 if none
  fitzpatrick_type: i64,
  /// File containing the sound to be played when the sticker is clicked; may be null. The sound is encoded with the Opus codec, and stored inside an OGG container
  sound: Option<File>,
  /// True, if the emoji is a regular emoji
  is_regular_emoji: bool,
}

impl RObject for AnimatedEmoji {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "animatedEmoji"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AnimatedEmoji {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAnimatedEmojiBuilder {
    let inner = AnimatedEmoji {
      td_name: "animatedEmoji".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAnimatedEmojiBuilder { inner }
  }

  pub fn sticker(&self) -> &Option<Sticker> {
    &self.sticker
  }

  pub fn sticker_width(&self) -> i64 {
    self.sticker_width
  }

  pub fn sticker_height(&self) -> i64 {
    self.sticker_height
  }

  pub fn fitzpatrick_type(&self) -> i64 {
    self.fitzpatrick_type
  }

  pub fn sound(&self) -> &Option<File> {
    &self.sound
  }

  pub fn is_regular_emoji(&self) -> bool {
    self.is_regular_emoji
  }
}

#[doc(hidden)]
pub struct RTDAnimatedEmojiBuilder {
  inner: AnimatedEmoji,
}

impl RTDAnimatedEmojiBuilder {
  pub fn build(&self) -> AnimatedEmoji {
    self.inner.clone()
  }

  pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = Some(sticker.as_ref().clone());
    self
  }

  pub fn sticker_width(&mut self, sticker_width: i64) -> &mut Self {
    self.inner.sticker_width = sticker_width;
    self
  }

  pub fn sticker_height(&mut self, sticker_height: i64) -> &mut Self {
    self.inner.sticker_height = sticker_height;
    self
  }

  pub fn fitzpatrick_type(&mut self, fitzpatrick_type: i64) -> &mut Self {
    self.inner.fitzpatrick_type = fitzpatrick_type;
    self
  }

  pub fn sound<T: AsRef<File>>(&mut self, sound: T) -> &mut Self {
    self.inner.sound = Some(sound.as_ref().clone());
    self
  }

  pub fn is_regular_emoji(&mut self, is_regular_emoji: bool) -> &mut Self {
    self.inner.is_regular_emoji = is_regular_emoji;
    self
  }
}

impl AsRef<AnimatedEmoji> for AnimatedEmoji {
  fn as_ref(&self) -> &AnimatedEmoji {
    self
  }
}

impl AsRef<AnimatedEmoji> for RTDAnimatedEmojiBuilder {
  fn as_ref(&self) -> &AnimatedEmoji {
    &self.inner
  }
}
