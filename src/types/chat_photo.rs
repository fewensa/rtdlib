use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a chat or user profile photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique photo identifier
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  id: isize,
  /// Point in time (Unix timestamp) when the photo has been added
  added_date: i64,
  /// Photo minithumbnail; may be null
  minithumbnail: Option<Minithumbnail>,
  /// Available variants of the photo in JPEG format, in different sizes
  sizes: Vec<PhotoSize>,
  /// A big (up to 1280x1280) animated variant of the photo in MPEG4 format; may be null
  animation: Option<AnimatedChatPhoto>,
  /// A small (160x160) animated variant of the photo in MPEG4 format; may be null even if the big animation is available
  small_animation: Option<AnimatedChatPhoto>,
  /// Sticker-based version of the chat photo; may be null
  sticker: Option<ChatPhotoSticker>,
}

impl RObject for ChatPhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatPhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatPhotoBuilder {
    let inner = ChatPhoto {
      td_name: "chatPhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatPhotoBuilder { inner }
  }

  pub fn id(&self) -> isize {
    self.id
  }

  pub fn added_date(&self) -> i64 {
    self.added_date
  }

  pub fn minithumbnail(&self) -> &Option<Minithumbnail> {
    &self.minithumbnail
  }

  pub fn sizes(&self) -> &Vec<PhotoSize> {
    &self.sizes
  }

  pub fn animation(&self) -> &Option<AnimatedChatPhoto> {
    &self.animation
  }

  pub fn small_animation(&self) -> &Option<AnimatedChatPhoto> {
    &self.small_animation
  }

  pub fn sticker(&self) -> &Option<ChatPhotoSticker> {
    &self.sticker
  }
}

#[doc(hidden)]
pub struct RTDChatPhotoBuilder {
  inner: ChatPhoto,
}

impl RTDChatPhotoBuilder {
  pub fn build(&self) -> ChatPhoto {
    self.inner.clone()
  }

  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn added_date(&mut self, added_date: i64) -> &mut Self {
    self.inner.added_date = added_date;
    self
  }

  pub fn minithumbnail<T: AsRef<Minithumbnail>>(&mut self, minithumbnail: T) -> &mut Self {
    self.inner.minithumbnail = Some(minithumbnail.as_ref().clone());
    self
  }

  pub fn sizes(&mut self, sizes: Vec<PhotoSize>) -> &mut Self {
    self.inner.sizes = sizes;
    self
  }

  pub fn animation<T: AsRef<AnimatedChatPhoto>>(&mut self, animation: T) -> &mut Self {
    self.inner.animation = Some(animation.as_ref().clone());
    self
  }

  pub fn small_animation<T: AsRef<AnimatedChatPhoto>>(&mut self, small_animation: T) -> &mut Self {
    self.inner.small_animation = Some(small_animation.as_ref().clone());
    self
  }

  pub fn sticker<T: AsRef<ChatPhotoSticker>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = Some(sticker.as_ref().clone());
    self
  }
}

impl AsRef<ChatPhoto> for ChatPhoto {
  fn as_ref(&self) -> &ChatPhoto {
    self
  }
}

impl AsRef<ChatPhoto> for RTDChatPhotoBuilder {
  fn as_ref(&self) -> &ChatPhoto {
    &self.inner
  }
}
