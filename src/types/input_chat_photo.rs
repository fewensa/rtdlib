
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes a photo to be set as a user profile or chat photo
pub trait TDInputChatPhoto: Debug + RObject {}

/// Describes a photo to be set as a user profile or chat photo
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputChatPhoto {
  #[doc(hidden)] _Default(()),
  /// An animation in MPEG4 format; must be square, shorter than 10 seconds, have width between 160 and 800 and be at most 2MB in size
  Animation(InputChatPhotoAnimation),
  /// A previously used profile photo of the current user
  Previous(InputChatPhotoPrevious),
  /// A static photo in JPEG format
  Static(InputChatPhotoStatic),

}

impl Default for InputChatPhoto {
  fn default() -> Self { InputChatPhoto::_Default(()) }
}

impl<'de> Deserialize<'de> for InputChatPhoto {
  fn deserialize<D>(deserializer: D) -> Result<InputChatPhoto, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InputChatPhoto,
      (inputChatPhotoAnimation, Animation);
      (inputChatPhotoPrevious, Previous);
      (inputChatPhotoStatic, Static);

    )(deserializer)
  }
}

impl RObject for InputChatPhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      InputChatPhoto::Animation(t) => t.td_name(),
      InputChatPhoto::Previous(t) => t.td_name(),
      InputChatPhoto::Static(t) => t.td_name(),

      _ => "-1",
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl InputChatPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let InputChatPhoto::_Default(_) = self { true } else { false } }

  pub fn is_animation(&self) -> bool { if let InputChatPhoto::Animation(_) = self { true } else { false } }
  pub fn is_previous(&self) -> bool { if let InputChatPhoto::Previous(_) = self { true } else { false } }
  pub fn is_static(&self) -> bool { if let InputChatPhoto::Static(_) = self { true } else { false } }

  pub fn on_animation<F: FnOnce(&InputChatPhotoAnimation)>(&self, fnc: F) -> &Self { if let InputChatPhoto::Animation(t) = self { fnc(t) }; self }
  pub fn on_previous<F: FnOnce(&InputChatPhotoPrevious)>(&self, fnc: F) -> &Self { if let InputChatPhoto::Previous(t) = self { fnc(t) }; self }
  pub fn on_static<F: FnOnce(&InputChatPhotoStatic)>(&self, fnc: F) -> &Self { if let InputChatPhoto::Static(t) = self { fnc(t) }; self }

  pub fn as_animation(&self) -> Option<&InputChatPhotoAnimation> { if let InputChatPhoto::Animation(t) = self { return Some(t) } None }
  pub fn as_previous(&self) -> Option<&InputChatPhotoPrevious> { if let InputChatPhoto::Previous(t) = self { return Some(t) } None }
  pub fn as_static(&self) -> Option<&InputChatPhotoStatic> { if let InputChatPhoto::Static(t) = self { return Some(t) } None }



  pub fn animation<T: AsRef<InputChatPhotoAnimation>>(t: T) -> Self { InputChatPhoto::Animation(t.as_ref().clone()) }

  pub fn previous<T: AsRef<InputChatPhotoPrevious>>(t: T) -> Self { InputChatPhoto::Previous(t.as_ref().clone()) }

  pub fn static_<T: AsRef<InputChatPhotoStatic>>(t: T) -> Self { InputChatPhoto::Static(t.as_ref().clone()) }

}

impl AsRef<InputChatPhoto> for InputChatPhoto {
  fn as_ref(&self) -> &InputChatPhoto { self }
}







/// An animation in MPEG4 format; must be square, shorter than 10 seconds, have width between 160 and 800 and be at most 2MB in size
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputChatPhotoAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Animation to be set as profile photo. Only inputFileLocal and inputFileGenerated are allowed
  animation: InputFile,
  /// Timestamp of the frame, which will be used as static chat photo
  main_frame_timestamp: f32,
  
}

impl RObject for InputChatPhotoAnimation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputChatPhotoAnimation" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputChatPhoto for InputChatPhotoAnimation {}



impl InputChatPhotoAnimation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputChatPhotoAnimationBuilder {
    let mut inner = InputChatPhotoAnimation::default();
    inner.td_name = "inputChatPhotoAnimation".to_string();
    RTDInputChatPhotoAnimationBuilder { inner }
  }

  pub fn animation(&self) -> &InputFile { &self.animation }

  pub fn main_frame_timestamp(&self) -> f32 { self.main_frame_timestamp }

}

#[doc(hidden)]
pub struct RTDInputChatPhotoAnimationBuilder {
  inner: InputChatPhotoAnimation
}

impl RTDInputChatPhotoAnimationBuilder {
  pub fn build(&self) -> InputChatPhotoAnimation { self.inner.clone() }

   
  pub fn animation<T: AsRef<InputFile>>(&mut self, animation: T) -> &mut Self {
    self.inner.animation = animation.as_ref().clone();
    self
  }

   
  pub fn main_frame_timestamp(&mut self, main_frame_timestamp: f32) -> &mut Self {
    self.inner.main_frame_timestamp = main_frame_timestamp;
    self
  }

}

impl AsRef<InputChatPhotoAnimation> for InputChatPhotoAnimation {
  fn as_ref(&self) -> &InputChatPhotoAnimation { self }
}

impl AsRef<InputChatPhotoAnimation> for RTDInputChatPhotoAnimationBuilder {
  fn as_ref(&self) -> &InputChatPhotoAnimation { &self.inner }
}







/// A previously used profile photo of the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputChatPhotoPrevious {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Identifier of the profile photo to reuse
  chat_photo_id: isize,
  
}

impl RObject for InputChatPhotoPrevious {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputChatPhotoPrevious" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputChatPhoto for InputChatPhotoPrevious {}



impl InputChatPhotoPrevious {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputChatPhotoPreviousBuilder {
    let mut inner = InputChatPhotoPrevious::default();
    inner.td_name = "inputChatPhotoPrevious".to_string();
    RTDInputChatPhotoPreviousBuilder { inner }
  }

  pub fn chat_photo_id(&self) -> isize { self.chat_photo_id }

}

#[doc(hidden)]
pub struct RTDInputChatPhotoPreviousBuilder {
  inner: InputChatPhotoPrevious
}

impl RTDInputChatPhotoPreviousBuilder {
  pub fn build(&self) -> InputChatPhotoPrevious { self.inner.clone() }

   
  pub fn chat_photo_id(&mut self, chat_photo_id: isize) -> &mut Self {
    self.inner.chat_photo_id = chat_photo_id;
    self
  }

}

impl AsRef<InputChatPhotoPrevious> for InputChatPhotoPrevious {
  fn as_ref(&self) -> &InputChatPhotoPrevious { self }
}

impl AsRef<InputChatPhotoPrevious> for RTDInputChatPhotoPreviousBuilder {
  fn as_ref(&self) -> &InputChatPhotoPrevious { &self.inner }
}







/// A static photo in JPEG format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputChatPhotoStatic {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Photo to be set as profile photo. Only inputFileLocal and inputFileGenerated are allowed
  photo: InputFile,
  
}

impl RObject for InputChatPhotoStatic {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputChatPhotoStatic" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputChatPhoto for InputChatPhotoStatic {}



impl InputChatPhotoStatic {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputChatPhotoStaticBuilder {
    let mut inner = InputChatPhotoStatic::default();
    inner.td_name = "inputChatPhotoStatic".to_string();
    RTDInputChatPhotoStaticBuilder { inner }
  }

  pub fn photo(&self) -> &InputFile { &self.photo }

}

#[doc(hidden)]
pub struct RTDInputChatPhotoStaticBuilder {
  inner: InputChatPhotoStatic
}

impl RTDInputChatPhotoStaticBuilder {
  pub fn build(&self) -> InputChatPhotoStatic { self.inner.clone() }

   
  pub fn photo<T: AsRef<InputFile>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = photo.as_ref().clone();
    self
  }

}

impl AsRef<InputChatPhotoStatic> for InputChatPhotoStatic {
  fn as_ref(&self) -> &InputChatPhotoStatic { self }
}

impl AsRef<InputChatPhotoStatic> for RTDInputChatPhotoStaticBuilder {
  fn as_ref(&self) -> &InputChatPhotoStatic { &self.inner }
}



