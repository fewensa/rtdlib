use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Information about the sticker, which was used to create the chat photo. The sticker is shown at the center of the photo and occupies at most 67% of it
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatPhotoSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of the sticker
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: ChatPhotoStickerType,
  /// The fill to be used as background for the sticker; rotation angle in backgroundFillGradient isn't supported
  background_fill: BackgroundFill,
}

impl RObject for ChatPhotoSticker {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatPhotoSticker"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatPhotoSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatPhotoStickerBuilder {
    let inner = ChatPhotoSticker {
      td_name: "chatPhotoSticker".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatPhotoStickerBuilder { inner }
  }

  pub fn type_(&self) -> &ChatPhotoStickerType {
    &self.type_
  }

  pub fn background_fill(&self) -> &BackgroundFill {
    &self.background_fill
  }
}

#[doc(hidden)]
pub struct RTDChatPhotoStickerBuilder {
  inner: ChatPhotoSticker,
}

impl RTDChatPhotoStickerBuilder {
  pub fn build(&self) -> ChatPhotoSticker {
    self.inner.clone()
  }

  pub fn type_<T: AsRef<ChatPhotoStickerType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

  pub fn background_fill<T: AsRef<BackgroundFill>>(&mut self, background_fill: T) -> &mut Self {
    self.inner.background_fill = background_fill.as_ref().clone();
    self
  }
}

impl AsRef<ChatPhotoSticker> for ChatPhotoSticker {
  fn as_ref(&self) -> &ChatPhotoSticker {
    self
  }
}

impl AsRef<ChatPhotoSticker> for RTDChatPhotoStickerBuilder {
  fn as_ref(&self) -> &ChatPhotoSticker {
    &self.inner
  }
}
