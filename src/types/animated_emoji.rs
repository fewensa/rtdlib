
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes an animated representation of an emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnimatedEmoji {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Animated sticker for the emoji
  sticker: Sticker,
  /// Emoji modifier fitzpatrick type; 0-6; 0 if none
  fitzpatrick_type: i64,
  /// File containing the sound to be played when the animated emoji is clicked if any; may be null. The sound is encoded with the Opus codec, and stored inside an OGG container
  sound: Option<File>,
  
}

impl RObject for AnimatedEmoji {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "animatedEmoji" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl AnimatedEmoji {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAnimatedEmojiBuilder {
    let mut inner = AnimatedEmoji::default();
    inner.td_name = "animatedEmoji".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAnimatedEmojiBuilder { inner }
  }

  pub fn sticker(&self) -> &Sticker { &self.sticker }

  pub fn fitzpatrick_type(&self) -> i64 { self.fitzpatrick_type }

  pub fn sound(&self) -> &Option<File> { &self.sound }

}

#[doc(hidden)]
pub struct RTDAnimatedEmojiBuilder {
  inner: AnimatedEmoji
}

impl RTDAnimatedEmojiBuilder {
  pub fn build(&self) -> AnimatedEmoji { self.inner.clone() }

   
  pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
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

}

impl AsRef<AnimatedEmoji> for AnimatedEmoji {
  fn as_ref(&self) -> &AnimatedEmoji { self }
}

impl AsRef<AnimatedEmoji> for RTDAnimatedEmojiBuilder {
  fn as_ref(&self) -> &AnimatedEmoji { &self.inner }
}



