use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents the type of file
pub trait TDFileType: Debug + RObject {}

/// Represents the type of file
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum FileType {
  #[doc(hidden)]
  _Default(()),
  /// The file is an animation
  Animation(FileTypeAnimation),
  /// The file is an audio file
  Audio(FileTypeAudio),
  /// The file is a document
  Document(FileTypeDocument),
  /// The file is a video for a live photo
  LivePhotoVideo(FileTypeLivePhotoVideo),
  /// The data is not a file
  None(FileTypeNone),
  /// The file is a notification sound
  NotificationSound(FileTypeNotificationSound),
  /// The file is a photo
  Photo(FileTypePhoto),
  /// The file is a photo published as a story
  PhotoStory(FileTypePhotoStory),
  /// The file is a profile photo
  ProfilePhoto(FileTypeProfilePhoto),
  /// The file was sent to a secret chat (the file type is not known to the server)
  Secret(FileTypeSecret),
  /// The file is a thumbnail of a file from a secret chat
  SecretThumbnail(FileTypeSecretThumbnail),
  /// The file is a file from Secure storage used for storing Telegram Passport files
  Secure(FileTypeSecure),
  /// The file is a self-destructing video for a live photo in a private chat
  SelfDestructingLivePhotoVideo(FileTypeSelfDestructingLivePhotoVideo),
  /// The file is a self-destructing photo in a private chat
  SelfDestructingPhoto(FileTypeSelfDestructingPhoto),
  /// The file is a self-destructing video in a private chat
  SelfDestructingVideo(FileTypeSelfDestructingVideo),
  /// The file is a self-destructing video note in a private chat
  SelfDestructingVideoNote(FileTypeSelfDestructingVideoNote),
  /// The file is a self-destructing voice note in a private chat
  SelfDestructingVoiceNote(FileTypeSelfDestructingVoiceNote),
  /// The file is a sticker
  Sticker(FileTypeSticker),
  /// The file is a thumbnail of another file
  Thumbnail(FileTypeThumbnail),
  /// The file type is not yet known
  Unknown(FileTypeUnknown),
  /// The file is a video
  Video(FileTypeVideo),
  /// The file is a video note
  VideoNote(FileTypeVideoNote),
  /// The file is a video published as a story
  VideoStory(FileTypeVideoStory),
  /// The file is a voice note
  VoiceNote(FileTypeVoiceNote),
  /// The file is a wallpaper or a background pattern
  Wallpaper(FileTypeWallpaper),
}

impl Default for FileType {
  fn default() -> Self {
    FileType::_Default(())
  }
}

impl<'de> Deserialize<'de> for FileType {
  fn deserialize<D>(deserializer: D) -> Result<FileType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      FileType,
      (fileTypeAnimation, Animation);
      (fileTypeAudio, Audio);
      (fileTypeDocument, Document);
      (fileTypeLivePhotoVideo, LivePhotoVideo);
      (fileTypeNone, None);
      (fileTypeNotificationSound, NotificationSound);
      (fileTypePhoto, Photo);
      (fileTypePhotoStory, PhotoStory);
      (fileTypeProfilePhoto, ProfilePhoto);
      (fileTypeSecret, Secret);
      (fileTypeSecretThumbnail, SecretThumbnail);
      (fileTypeSecure, Secure);
      (fileTypeSelfDestructingLivePhotoVideo, SelfDestructingLivePhotoVideo);
      (fileTypeSelfDestructingPhoto, SelfDestructingPhoto);
      (fileTypeSelfDestructingVideo, SelfDestructingVideo);
      (fileTypeSelfDestructingVideoNote, SelfDestructingVideoNote);
      (fileTypeSelfDestructingVoiceNote, SelfDestructingVoiceNote);
      (fileTypeSticker, Sticker);
      (fileTypeThumbnail, Thumbnail);
      (fileTypeUnknown, Unknown);
      (fileTypeVideo, Video);
      (fileTypeVideoNote, VideoNote);
      (fileTypeVideoStory, VideoStory);
      (fileTypeVoiceNote, VoiceNote);
      (fileTypeWallpaper, Wallpaper);

    )(deserializer)
  }
}

impl RObject for FileType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      FileType::Animation(t) => t.td_name(),
      FileType::Audio(t) => t.td_name(),
      FileType::Document(t) => t.td_name(),
      FileType::LivePhotoVideo(t) => t.td_name(),
      FileType::None(t) => t.td_name(),
      FileType::NotificationSound(t) => t.td_name(),
      FileType::Photo(t) => t.td_name(),
      FileType::PhotoStory(t) => t.td_name(),
      FileType::ProfilePhoto(t) => t.td_name(),
      FileType::Secret(t) => t.td_name(),
      FileType::SecretThumbnail(t) => t.td_name(),
      FileType::Secure(t) => t.td_name(),
      FileType::SelfDestructingLivePhotoVideo(t) => t.td_name(),
      FileType::SelfDestructingPhoto(t) => t.td_name(),
      FileType::SelfDestructingVideo(t) => t.td_name(),
      FileType::SelfDestructingVideoNote(t) => t.td_name(),
      FileType::SelfDestructingVoiceNote(t) => t.td_name(),
      FileType::Sticker(t) => t.td_name(),
      FileType::Thumbnail(t) => t.td_name(),
      FileType::Unknown(t) => t.td_name(),
      FileType::Video(t) => t.td_name(),
      FileType::VideoNote(t) => t.td_name(),
      FileType::VideoStory(t) => t.td_name(),
      FileType::VoiceNote(t) => t.td_name(),
      FileType::Wallpaper(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      FileType::Animation(t) => t.extra(),
      FileType::Audio(t) => t.extra(),
      FileType::Document(t) => t.extra(),
      FileType::LivePhotoVideo(t) => t.extra(),
      FileType::None(t) => t.extra(),
      FileType::NotificationSound(t) => t.extra(),
      FileType::Photo(t) => t.extra(),
      FileType::PhotoStory(t) => t.extra(),
      FileType::ProfilePhoto(t) => t.extra(),
      FileType::Secret(t) => t.extra(),
      FileType::SecretThumbnail(t) => t.extra(),
      FileType::Secure(t) => t.extra(),
      FileType::SelfDestructingLivePhotoVideo(t) => t.extra(),
      FileType::SelfDestructingPhoto(t) => t.extra(),
      FileType::SelfDestructingVideo(t) => t.extra(),
      FileType::SelfDestructingVideoNote(t) => t.extra(),
      FileType::SelfDestructingVoiceNote(t) => t.extra(),
      FileType::Sticker(t) => t.extra(),
      FileType::Thumbnail(t) => t.extra(),
      FileType::Unknown(t) => t.extra(),
      FileType::Video(t) => t.extra(),
      FileType::VideoNote(t) => t.extra(),
      FileType::VideoStory(t) => t.extra(),
      FileType::VoiceNote(t) => t.extra(),
      FileType::Wallpaper(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl FileType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, FileType::_Default(_))
  }

  pub fn is_animation(&self) -> bool {
    matches!(self, FileType::Animation(_))
  }
  pub fn is_audio(&self) -> bool {
    matches!(self, FileType::Audio(_))
  }
  pub fn is_document(&self) -> bool {
    matches!(self, FileType::Document(_))
  }
  pub fn is_live_photo_video(&self) -> bool {
    matches!(self, FileType::LivePhotoVideo(_))
  }
  pub fn is_none(&self) -> bool {
    matches!(self, FileType::None(_))
  }
  pub fn is_notification_sound(&self) -> bool {
    matches!(self, FileType::NotificationSound(_))
  }
  pub fn is_photo(&self) -> bool {
    matches!(self, FileType::Photo(_))
  }
  pub fn is_photo_story(&self) -> bool {
    matches!(self, FileType::PhotoStory(_))
  }
  pub fn is_profile_photo(&self) -> bool {
    matches!(self, FileType::ProfilePhoto(_))
  }
  pub fn is_secret(&self) -> bool {
    matches!(self, FileType::Secret(_))
  }
  pub fn is_secret_thumbnail(&self) -> bool {
    matches!(self, FileType::SecretThumbnail(_))
  }
  pub fn is_secure(&self) -> bool {
    matches!(self, FileType::Secure(_))
  }
  pub fn is_self_destructing_live_photo_video(&self) -> bool {
    matches!(self, FileType::SelfDestructingLivePhotoVideo(_))
  }
  pub fn is_self_destructing_photo(&self) -> bool {
    matches!(self, FileType::SelfDestructingPhoto(_))
  }
  pub fn is_self_destructing_video(&self) -> bool {
    matches!(self, FileType::SelfDestructingVideo(_))
  }
  pub fn is_self_destructing_video_note(&self) -> bool {
    matches!(self, FileType::SelfDestructingVideoNote(_))
  }
  pub fn is_self_destructing_voice_note(&self) -> bool {
    matches!(self, FileType::SelfDestructingVoiceNote(_))
  }
  pub fn is_sticker(&self) -> bool {
    matches!(self, FileType::Sticker(_))
  }
  pub fn is_thumbnail(&self) -> bool {
    matches!(self, FileType::Thumbnail(_))
  }
  pub fn is_unknown(&self) -> bool {
    matches!(self, FileType::Unknown(_))
  }
  pub fn is_video(&self) -> bool {
    matches!(self, FileType::Video(_))
  }
  pub fn is_video_note(&self) -> bool {
    matches!(self, FileType::VideoNote(_))
  }
  pub fn is_video_story(&self) -> bool {
    matches!(self, FileType::VideoStory(_))
  }
  pub fn is_voice_note(&self) -> bool {
    matches!(self, FileType::VoiceNote(_))
  }
  pub fn is_wallpaper(&self) -> bool {
    matches!(self, FileType::Wallpaper(_))
  }

  pub fn on_animation<F: FnOnce(&FileTypeAnimation)>(&self, fnc: F) -> &Self {
    if let FileType::Animation(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_audio<F: FnOnce(&FileTypeAudio)>(&self, fnc: F) -> &Self {
    if let FileType::Audio(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_document<F: FnOnce(&FileTypeDocument)>(&self, fnc: F) -> &Self {
    if let FileType::Document(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_live_photo_video<F: FnOnce(&FileTypeLivePhotoVideo)>(&self, fnc: F) -> &Self {
    if let FileType::LivePhotoVideo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_none<F: FnOnce(&FileTypeNone)>(&self, fnc: F) -> &Self {
    if let FileType::None(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_notification_sound<F: FnOnce(&FileTypeNotificationSound)>(&self, fnc: F) -> &Self {
    if let FileType::NotificationSound(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_photo<F: FnOnce(&FileTypePhoto)>(&self, fnc: F) -> &Self {
    if let FileType::Photo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_photo_story<F: FnOnce(&FileTypePhotoStory)>(&self, fnc: F) -> &Self {
    if let FileType::PhotoStory(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_profile_photo<F: FnOnce(&FileTypeProfilePhoto)>(&self, fnc: F) -> &Self {
    if let FileType::ProfilePhoto(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_secret<F: FnOnce(&FileTypeSecret)>(&self, fnc: F) -> &Self {
    if let FileType::Secret(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_secret_thumbnail<F: FnOnce(&FileTypeSecretThumbnail)>(&self, fnc: F) -> &Self {
    if let FileType::SecretThumbnail(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_secure<F: FnOnce(&FileTypeSecure)>(&self, fnc: F) -> &Self {
    if let FileType::Secure(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_self_destructing_live_photo_video<F: FnOnce(&FileTypeSelfDestructingLivePhotoVideo)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let FileType::SelfDestructingLivePhotoVideo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_self_destructing_photo<F: FnOnce(&FileTypeSelfDestructingPhoto)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let FileType::SelfDestructingPhoto(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_self_destructing_video<F: FnOnce(&FileTypeSelfDestructingVideo)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let FileType::SelfDestructingVideo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_self_destructing_video_note<F: FnOnce(&FileTypeSelfDestructingVideoNote)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let FileType::SelfDestructingVideoNote(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_self_destructing_voice_note<F: FnOnce(&FileTypeSelfDestructingVoiceNote)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let FileType::SelfDestructingVoiceNote(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_sticker<F: FnOnce(&FileTypeSticker)>(&self, fnc: F) -> &Self {
    if let FileType::Sticker(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_thumbnail<F: FnOnce(&FileTypeThumbnail)>(&self, fnc: F) -> &Self {
    if let FileType::Thumbnail(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_unknown<F: FnOnce(&FileTypeUnknown)>(&self, fnc: F) -> &Self {
    if let FileType::Unknown(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video<F: FnOnce(&FileTypeVideo)>(&self, fnc: F) -> &Self {
    if let FileType::Video(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video_note<F: FnOnce(&FileTypeVideoNote)>(&self, fnc: F) -> &Self {
    if let FileType::VideoNote(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video_story<F: FnOnce(&FileTypeVideoStory)>(&self, fnc: F) -> &Self {
    if let FileType::VideoStory(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_voice_note<F: FnOnce(&FileTypeVoiceNote)>(&self, fnc: F) -> &Self {
    if let FileType::VoiceNote(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_wallpaper<F: FnOnce(&FileTypeWallpaper)>(&self, fnc: F) -> &Self {
    if let FileType::Wallpaper(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_animation(&self) -> Option<&FileTypeAnimation> {
    if let FileType::Animation(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_audio(&self) -> Option<&FileTypeAudio> {
    if let FileType::Audio(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_document(&self) -> Option<&FileTypeDocument> {
    if let FileType::Document(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_live_photo_video(&self) -> Option<&FileTypeLivePhotoVideo> {
    if let FileType::LivePhotoVideo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_none(&self) -> Option<&FileTypeNone> {
    if let FileType::None(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_notification_sound(&self) -> Option<&FileTypeNotificationSound> {
    if let FileType::NotificationSound(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_photo(&self) -> Option<&FileTypePhoto> {
    if let FileType::Photo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_photo_story(&self) -> Option<&FileTypePhotoStory> {
    if let FileType::PhotoStory(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_profile_photo(&self) -> Option<&FileTypeProfilePhoto> {
    if let FileType::ProfilePhoto(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_secret(&self) -> Option<&FileTypeSecret> {
    if let FileType::Secret(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_secret_thumbnail(&self) -> Option<&FileTypeSecretThumbnail> {
    if let FileType::SecretThumbnail(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_secure(&self) -> Option<&FileTypeSecure> {
    if let FileType::Secure(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_self_destructing_live_photo_video(
    &self,
  ) -> Option<&FileTypeSelfDestructingLivePhotoVideo> {
    if let FileType::SelfDestructingLivePhotoVideo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_self_destructing_photo(&self) -> Option<&FileTypeSelfDestructingPhoto> {
    if let FileType::SelfDestructingPhoto(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_self_destructing_video(&self) -> Option<&FileTypeSelfDestructingVideo> {
    if let FileType::SelfDestructingVideo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_self_destructing_video_note(&self) -> Option<&FileTypeSelfDestructingVideoNote> {
    if let FileType::SelfDestructingVideoNote(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_self_destructing_voice_note(&self) -> Option<&FileTypeSelfDestructingVoiceNote> {
    if let FileType::SelfDestructingVoiceNote(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_sticker(&self) -> Option<&FileTypeSticker> {
    if let FileType::Sticker(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_thumbnail(&self) -> Option<&FileTypeThumbnail> {
    if let FileType::Thumbnail(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_unknown(&self) -> Option<&FileTypeUnknown> {
    if let FileType::Unknown(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video(&self) -> Option<&FileTypeVideo> {
    if let FileType::Video(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video_note(&self) -> Option<&FileTypeVideoNote> {
    if let FileType::VideoNote(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video_story(&self) -> Option<&FileTypeVideoStory> {
    if let FileType::VideoStory(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_voice_note(&self) -> Option<&FileTypeVoiceNote> {
    if let FileType::VoiceNote(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_wallpaper(&self) -> Option<&FileTypeWallpaper> {
    if let FileType::Wallpaper(t) = self {
      return Some(t);
    }
    None
  }

  pub fn animation<T: AsRef<FileTypeAnimation>>(t: T) -> Self {
    FileType::Animation(t.as_ref().clone())
  }

  pub fn audio<T: AsRef<FileTypeAudio>>(t: T) -> Self {
    FileType::Audio(t.as_ref().clone())
  }

  pub fn document<T: AsRef<FileTypeDocument>>(t: T) -> Self {
    FileType::Document(t.as_ref().clone())
  }

  pub fn live_photo_video<T: AsRef<FileTypeLivePhotoVideo>>(t: T) -> Self {
    FileType::LivePhotoVideo(t.as_ref().clone())
  }

  pub fn none<T: AsRef<FileTypeNone>>(t: T) -> Self {
    FileType::None(t.as_ref().clone())
  }

  pub fn notification_sound<T: AsRef<FileTypeNotificationSound>>(t: T) -> Self {
    FileType::NotificationSound(t.as_ref().clone())
  }

  pub fn photo<T: AsRef<FileTypePhoto>>(t: T) -> Self {
    FileType::Photo(t.as_ref().clone())
  }

  pub fn photo_story<T: AsRef<FileTypePhotoStory>>(t: T) -> Self {
    FileType::PhotoStory(t.as_ref().clone())
  }

  pub fn profile_photo<T: AsRef<FileTypeProfilePhoto>>(t: T) -> Self {
    FileType::ProfilePhoto(t.as_ref().clone())
  }

  pub fn secret<T: AsRef<FileTypeSecret>>(t: T) -> Self {
    FileType::Secret(t.as_ref().clone())
  }

  pub fn secret_thumbnail<T: AsRef<FileTypeSecretThumbnail>>(t: T) -> Self {
    FileType::SecretThumbnail(t.as_ref().clone())
  }

  pub fn secure<T: AsRef<FileTypeSecure>>(t: T) -> Self {
    FileType::Secure(t.as_ref().clone())
  }

  pub fn self_destructing_live_photo_video<T: AsRef<FileTypeSelfDestructingLivePhotoVideo>>(
    t: T,
  ) -> Self {
    FileType::SelfDestructingLivePhotoVideo(t.as_ref().clone())
  }

  pub fn self_destructing_photo<T: AsRef<FileTypeSelfDestructingPhoto>>(t: T) -> Self {
    FileType::SelfDestructingPhoto(t.as_ref().clone())
  }

  pub fn self_destructing_video<T: AsRef<FileTypeSelfDestructingVideo>>(t: T) -> Self {
    FileType::SelfDestructingVideo(t.as_ref().clone())
  }

  pub fn self_destructing_video_note<T: AsRef<FileTypeSelfDestructingVideoNote>>(t: T) -> Self {
    FileType::SelfDestructingVideoNote(t.as_ref().clone())
  }

  pub fn self_destructing_voice_note<T: AsRef<FileTypeSelfDestructingVoiceNote>>(t: T) -> Self {
    FileType::SelfDestructingVoiceNote(t.as_ref().clone())
  }

  pub fn sticker<T: AsRef<FileTypeSticker>>(t: T) -> Self {
    FileType::Sticker(t.as_ref().clone())
  }

  pub fn thumbnail<T: AsRef<FileTypeThumbnail>>(t: T) -> Self {
    FileType::Thumbnail(t.as_ref().clone())
  }

  pub fn unknown<T: AsRef<FileTypeUnknown>>(t: T) -> Self {
    FileType::Unknown(t.as_ref().clone())
  }

  pub fn video<T: AsRef<FileTypeVideo>>(t: T) -> Self {
    FileType::Video(t.as_ref().clone())
  }

  pub fn video_note<T: AsRef<FileTypeVideoNote>>(t: T) -> Self {
    FileType::VideoNote(t.as_ref().clone())
  }

  pub fn video_story<T: AsRef<FileTypeVideoStory>>(t: T) -> Self {
    FileType::VideoStory(t.as_ref().clone())
  }

  pub fn voice_note<T: AsRef<FileTypeVoiceNote>>(t: T) -> Self {
    FileType::VoiceNote(t.as_ref().clone())
  }

  pub fn wallpaper<T: AsRef<FileTypeWallpaper>>(t: T) -> Self {
    FileType::Wallpaper(t.as_ref().clone())
  }
}

impl AsRef<FileType> for FileType {
  fn as_ref(&self) -> &FileType {
    self
  }
}

/// The file is an animation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeAnimation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeAnimation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeAnimation {}

impl FileTypeAnimation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeAnimationBuilder {
    let inner = FileTypeAnimation {
      td_name: "fileTypeAnimation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeAnimationBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeAnimationBuilder {
  inner: FileTypeAnimation,
}

impl RTDFileTypeAnimationBuilder {
  pub fn build(&self) -> FileTypeAnimation {
    self.inner.clone()
  }
}

impl AsRef<FileTypeAnimation> for FileTypeAnimation {
  fn as_ref(&self) -> &FileTypeAnimation {
    self
  }
}

impl AsRef<FileTypeAnimation> for RTDFileTypeAnimationBuilder {
  fn as_ref(&self) -> &FileTypeAnimation {
    &self.inner
  }
}

/// The file is an audio file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeAudio {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeAudio"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeAudio {}

impl FileTypeAudio {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeAudioBuilder {
    let inner = FileTypeAudio {
      td_name: "fileTypeAudio".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeAudioBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeAudioBuilder {
  inner: FileTypeAudio,
}

impl RTDFileTypeAudioBuilder {
  pub fn build(&self) -> FileTypeAudio {
    self.inner.clone()
  }
}

impl AsRef<FileTypeAudio> for FileTypeAudio {
  fn as_ref(&self) -> &FileTypeAudio {
    self
  }
}

impl AsRef<FileTypeAudio> for RTDFileTypeAudioBuilder {
  fn as_ref(&self) -> &FileTypeAudio {
    &self.inner
  }
}

/// The file is a document
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeDocument {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeDocument"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeDocument {}

impl FileTypeDocument {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeDocumentBuilder {
    let inner = FileTypeDocument {
      td_name: "fileTypeDocument".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeDocumentBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeDocumentBuilder {
  inner: FileTypeDocument,
}

impl RTDFileTypeDocumentBuilder {
  pub fn build(&self) -> FileTypeDocument {
    self.inner.clone()
  }
}

impl AsRef<FileTypeDocument> for FileTypeDocument {
  fn as_ref(&self) -> &FileTypeDocument {
    self
  }
}

impl AsRef<FileTypeDocument> for RTDFileTypeDocumentBuilder {
  fn as_ref(&self) -> &FileTypeDocument {
    &self.inner
  }
}

/// The file is a video for a live photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeLivePhotoVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeLivePhotoVideo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeLivePhotoVideo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeLivePhotoVideo {}

impl FileTypeLivePhotoVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeLivePhotoVideoBuilder {
    let inner = FileTypeLivePhotoVideo {
      td_name: "fileTypeLivePhotoVideo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeLivePhotoVideoBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeLivePhotoVideoBuilder {
  inner: FileTypeLivePhotoVideo,
}

impl RTDFileTypeLivePhotoVideoBuilder {
  pub fn build(&self) -> FileTypeLivePhotoVideo {
    self.inner.clone()
  }
}

impl AsRef<FileTypeLivePhotoVideo> for FileTypeLivePhotoVideo {
  fn as_ref(&self) -> &FileTypeLivePhotoVideo {
    self
  }
}

impl AsRef<FileTypeLivePhotoVideo> for RTDFileTypeLivePhotoVideoBuilder {
  fn as_ref(&self) -> &FileTypeLivePhotoVideo {
    &self.inner
  }
}

/// The data is not a file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeNone {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeNone {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeNone"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeNone {}

impl FileTypeNone {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeNoneBuilder {
    let inner = FileTypeNone {
      td_name: "fileTypeNone".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeNoneBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeNoneBuilder {
  inner: FileTypeNone,
}

impl RTDFileTypeNoneBuilder {
  pub fn build(&self) -> FileTypeNone {
    self.inner.clone()
  }
}

impl AsRef<FileTypeNone> for FileTypeNone {
  fn as_ref(&self) -> &FileTypeNone {
    self
  }
}

impl AsRef<FileTypeNone> for RTDFileTypeNoneBuilder {
  fn as_ref(&self) -> &FileTypeNone {
    &self.inner
  }
}

/// The file is a notification sound
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeNotificationSound {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeNotificationSound {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeNotificationSound"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeNotificationSound {}

impl FileTypeNotificationSound {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeNotificationSoundBuilder {
    let inner = FileTypeNotificationSound {
      td_name: "fileTypeNotificationSound".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeNotificationSoundBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeNotificationSoundBuilder {
  inner: FileTypeNotificationSound,
}

impl RTDFileTypeNotificationSoundBuilder {
  pub fn build(&self) -> FileTypeNotificationSound {
    self.inner.clone()
  }
}

impl AsRef<FileTypeNotificationSound> for FileTypeNotificationSound {
  fn as_ref(&self) -> &FileTypeNotificationSound {
    self
  }
}

impl AsRef<FileTypeNotificationSound> for RTDFileTypeNotificationSoundBuilder {
  fn as_ref(&self) -> &FileTypeNotificationSound {
    &self.inner
  }
}

/// The file is a photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypePhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypePhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypePhoto {}

impl FileTypePhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypePhotoBuilder {
    let inner = FileTypePhoto {
      td_name: "fileTypePhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypePhotoBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypePhotoBuilder {
  inner: FileTypePhoto,
}

impl RTDFileTypePhotoBuilder {
  pub fn build(&self) -> FileTypePhoto {
    self.inner.clone()
  }
}

impl AsRef<FileTypePhoto> for FileTypePhoto {
  fn as_ref(&self) -> &FileTypePhoto {
    self
  }
}

impl AsRef<FileTypePhoto> for RTDFileTypePhotoBuilder {
  fn as_ref(&self) -> &FileTypePhoto {
    &self.inner
  }
}

/// The file is a photo published as a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypePhotoStory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypePhotoStory {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypePhotoStory"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypePhotoStory {}

impl FileTypePhotoStory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypePhotoStoryBuilder {
    let inner = FileTypePhotoStory {
      td_name: "fileTypePhotoStory".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypePhotoStoryBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypePhotoStoryBuilder {
  inner: FileTypePhotoStory,
}

impl RTDFileTypePhotoStoryBuilder {
  pub fn build(&self) -> FileTypePhotoStory {
    self.inner.clone()
  }
}

impl AsRef<FileTypePhotoStory> for FileTypePhotoStory {
  fn as_ref(&self) -> &FileTypePhotoStory {
    self
  }
}

impl AsRef<FileTypePhotoStory> for RTDFileTypePhotoStoryBuilder {
  fn as_ref(&self) -> &FileTypePhotoStory {
    &self.inner
  }
}

/// The file is a profile photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeProfilePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeProfilePhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeProfilePhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeProfilePhoto {}

impl FileTypeProfilePhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeProfilePhotoBuilder {
    let inner = FileTypeProfilePhoto {
      td_name: "fileTypeProfilePhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeProfilePhotoBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeProfilePhotoBuilder {
  inner: FileTypeProfilePhoto,
}

impl RTDFileTypeProfilePhotoBuilder {
  pub fn build(&self) -> FileTypeProfilePhoto {
    self.inner.clone()
  }
}

impl AsRef<FileTypeProfilePhoto> for FileTypeProfilePhoto {
  fn as_ref(&self) -> &FileTypeProfilePhoto {
    self
  }
}

impl AsRef<FileTypeProfilePhoto> for RTDFileTypeProfilePhotoBuilder {
  fn as_ref(&self) -> &FileTypeProfilePhoto {
    &self.inner
  }
}

/// The file was sent to a secret chat (the file type is not known to the server)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeSecret {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeSecret {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeSecret"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeSecret {}

impl FileTypeSecret {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeSecretBuilder {
    let inner = FileTypeSecret {
      td_name: "fileTypeSecret".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeSecretBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeSecretBuilder {
  inner: FileTypeSecret,
}

impl RTDFileTypeSecretBuilder {
  pub fn build(&self) -> FileTypeSecret {
    self.inner.clone()
  }
}

impl AsRef<FileTypeSecret> for FileTypeSecret {
  fn as_ref(&self) -> &FileTypeSecret {
    self
  }
}

impl AsRef<FileTypeSecret> for RTDFileTypeSecretBuilder {
  fn as_ref(&self) -> &FileTypeSecret {
    &self.inner
  }
}

/// The file is a thumbnail of a file from a secret chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeSecretThumbnail {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeSecretThumbnail {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeSecretThumbnail"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeSecretThumbnail {}

impl FileTypeSecretThumbnail {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeSecretThumbnailBuilder {
    let inner = FileTypeSecretThumbnail {
      td_name: "fileTypeSecretThumbnail".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeSecretThumbnailBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeSecretThumbnailBuilder {
  inner: FileTypeSecretThumbnail,
}

impl RTDFileTypeSecretThumbnailBuilder {
  pub fn build(&self) -> FileTypeSecretThumbnail {
    self.inner.clone()
  }
}

impl AsRef<FileTypeSecretThumbnail> for FileTypeSecretThumbnail {
  fn as_ref(&self) -> &FileTypeSecretThumbnail {
    self
  }
}

impl AsRef<FileTypeSecretThumbnail> for RTDFileTypeSecretThumbnailBuilder {
  fn as_ref(&self) -> &FileTypeSecretThumbnail {
    &self.inner
  }
}

/// The file is a file from Secure storage used for storing Telegram Passport files
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeSecure {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeSecure {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeSecure"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeSecure {}

impl FileTypeSecure {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeSecureBuilder {
    let inner = FileTypeSecure {
      td_name: "fileTypeSecure".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeSecureBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeSecureBuilder {
  inner: FileTypeSecure,
}

impl RTDFileTypeSecureBuilder {
  pub fn build(&self) -> FileTypeSecure {
    self.inner.clone()
  }
}

impl AsRef<FileTypeSecure> for FileTypeSecure {
  fn as_ref(&self) -> &FileTypeSecure {
    self
  }
}

impl AsRef<FileTypeSecure> for RTDFileTypeSecureBuilder {
  fn as_ref(&self) -> &FileTypeSecure {
    &self.inner
  }
}

/// The file is a self-destructing video for a live photo in a private chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeSelfDestructingLivePhotoVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeSelfDestructingLivePhotoVideo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeSelfDestructingLivePhotoVideo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeSelfDestructingLivePhotoVideo {}

impl FileTypeSelfDestructingLivePhotoVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeSelfDestructingLivePhotoVideoBuilder {
    let inner = FileTypeSelfDestructingLivePhotoVideo {
      td_name: "fileTypeSelfDestructingLivePhotoVideo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeSelfDestructingLivePhotoVideoBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeSelfDestructingLivePhotoVideoBuilder {
  inner: FileTypeSelfDestructingLivePhotoVideo,
}

impl RTDFileTypeSelfDestructingLivePhotoVideoBuilder {
  pub fn build(&self) -> FileTypeSelfDestructingLivePhotoVideo {
    self.inner.clone()
  }
}

impl AsRef<FileTypeSelfDestructingLivePhotoVideo> for FileTypeSelfDestructingLivePhotoVideo {
  fn as_ref(&self) -> &FileTypeSelfDestructingLivePhotoVideo {
    self
  }
}

impl AsRef<FileTypeSelfDestructingLivePhotoVideo>
  for RTDFileTypeSelfDestructingLivePhotoVideoBuilder
{
  fn as_ref(&self) -> &FileTypeSelfDestructingLivePhotoVideo {
    &self.inner
  }
}

/// The file is a self-destructing photo in a private chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeSelfDestructingPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeSelfDestructingPhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeSelfDestructingPhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeSelfDestructingPhoto {}

impl FileTypeSelfDestructingPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeSelfDestructingPhotoBuilder {
    let inner = FileTypeSelfDestructingPhoto {
      td_name: "fileTypeSelfDestructingPhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeSelfDestructingPhotoBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeSelfDestructingPhotoBuilder {
  inner: FileTypeSelfDestructingPhoto,
}

impl RTDFileTypeSelfDestructingPhotoBuilder {
  pub fn build(&self) -> FileTypeSelfDestructingPhoto {
    self.inner.clone()
  }
}

impl AsRef<FileTypeSelfDestructingPhoto> for FileTypeSelfDestructingPhoto {
  fn as_ref(&self) -> &FileTypeSelfDestructingPhoto {
    self
  }
}

impl AsRef<FileTypeSelfDestructingPhoto> for RTDFileTypeSelfDestructingPhotoBuilder {
  fn as_ref(&self) -> &FileTypeSelfDestructingPhoto {
    &self.inner
  }
}

/// The file is a self-destructing video in a private chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeSelfDestructingVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeSelfDestructingVideo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeSelfDestructingVideo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeSelfDestructingVideo {}

impl FileTypeSelfDestructingVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeSelfDestructingVideoBuilder {
    let inner = FileTypeSelfDestructingVideo {
      td_name: "fileTypeSelfDestructingVideo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeSelfDestructingVideoBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeSelfDestructingVideoBuilder {
  inner: FileTypeSelfDestructingVideo,
}

impl RTDFileTypeSelfDestructingVideoBuilder {
  pub fn build(&self) -> FileTypeSelfDestructingVideo {
    self.inner.clone()
  }
}

impl AsRef<FileTypeSelfDestructingVideo> for FileTypeSelfDestructingVideo {
  fn as_ref(&self) -> &FileTypeSelfDestructingVideo {
    self
  }
}

impl AsRef<FileTypeSelfDestructingVideo> for RTDFileTypeSelfDestructingVideoBuilder {
  fn as_ref(&self) -> &FileTypeSelfDestructingVideo {
    &self.inner
  }
}

/// The file is a self-destructing video note in a private chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeSelfDestructingVideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeSelfDestructingVideoNote {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeSelfDestructingVideoNote"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeSelfDestructingVideoNote {}

impl FileTypeSelfDestructingVideoNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeSelfDestructingVideoNoteBuilder {
    let inner = FileTypeSelfDestructingVideoNote {
      td_name: "fileTypeSelfDestructingVideoNote".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeSelfDestructingVideoNoteBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeSelfDestructingVideoNoteBuilder {
  inner: FileTypeSelfDestructingVideoNote,
}

impl RTDFileTypeSelfDestructingVideoNoteBuilder {
  pub fn build(&self) -> FileTypeSelfDestructingVideoNote {
    self.inner.clone()
  }
}

impl AsRef<FileTypeSelfDestructingVideoNote> for FileTypeSelfDestructingVideoNote {
  fn as_ref(&self) -> &FileTypeSelfDestructingVideoNote {
    self
  }
}

impl AsRef<FileTypeSelfDestructingVideoNote> for RTDFileTypeSelfDestructingVideoNoteBuilder {
  fn as_ref(&self) -> &FileTypeSelfDestructingVideoNote {
    &self.inner
  }
}

/// The file is a self-destructing voice note in a private chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeSelfDestructingVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeSelfDestructingVoiceNote {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeSelfDestructingVoiceNote"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeSelfDestructingVoiceNote {}

impl FileTypeSelfDestructingVoiceNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeSelfDestructingVoiceNoteBuilder {
    let inner = FileTypeSelfDestructingVoiceNote {
      td_name: "fileTypeSelfDestructingVoiceNote".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeSelfDestructingVoiceNoteBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeSelfDestructingVoiceNoteBuilder {
  inner: FileTypeSelfDestructingVoiceNote,
}

impl RTDFileTypeSelfDestructingVoiceNoteBuilder {
  pub fn build(&self) -> FileTypeSelfDestructingVoiceNote {
    self.inner.clone()
  }
}

impl AsRef<FileTypeSelfDestructingVoiceNote> for FileTypeSelfDestructingVoiceNote {
  fn as_ref(&self) -> &FileTypeSelfDestructingVoiceNote {
    self
  }
}

impl AsRef<FileTypeSelfDestructingVoiceNote> for RTDFileTypeSelfDestructingVoiceNoteBuilder {
  fn as_ref(&self) -> &FileTypeSelfDestructingVoiceNote {
    &self.inner
  }
}

/// The file is a sticker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeSticker {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeSticker"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeSticker {}

impl FileTypeSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeStickerBuilder {
    let inner = FileTypeSticker {
      td_name: "fileTypeSticker".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeStickerBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeStickerBuilder {
  inner: FileTypeSticker,
}

impl RTDFileTypeStickerBuilder {
  pub fn build(&self) -> FileTypeSticker {
    self.inner.clone()
  }
}

impl AsRef<FileTypeSticker> for FileTypeSticker {
  fn as_ref(&self) -> &FileTypeSticker {
    self
  }
}

impl AsRef<FileTypeSticker> for RTDFileTypeStickerBuilder {
  fn as_ref(&self) -> &FileTypeSticker {
    &self.inner
  }
}

/// The file is a thumbnail of another file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeThumbnail {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeThumbnail {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeThumbnail"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeThumbnail {}

impl FileTypeThumbnail {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeThumbnailBuilder {
    let inner = FileTypeThumbnail {
      td_name: "fileTypeThumbnail".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeThumbnailBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeThumbnailBuilder {
  inner: FileTypeThumbnail,
}

impl RTDFileTypeThumbnailBuilder {
  pub fn build(&self) -> FileTypeThumbnail {
    self.inner.clone()
  }
}

impl AsRef<FileTypeThumbnail> for FileTypeThumbnail {
  fn as_ref(&self) -> &FileTypeThumbnail {
    self
  }
}

impl AsRef<FileTypeThumbnail> for RTDFileTypeThumbnailBuilder {
  fn as_ref(&self) -> &FileTypeThumbnail {
    &self.inner
  }
}

/// The file type is not yet known
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeUnknown {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeUnknown {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeUnknown"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeUnknown {}

impl FileTypeUnknown {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeUnknownBuilder {
    let inner = FileTypeUnknown {
      td_name: "fileTypeUnknown".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeUnknownBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeUnknownBuilder {
  inner: FileTypeUnknown,
}

impl RTDFileTypeUnknownBuilder {
  pub fn build(&self) -> FileTypeUnknown {
    self.inner.clone()
  }
}

impl AsRef<FileTypeUnknown> for FileTypeUnknown {
  fn as_ref(&self) -> &FileTypeUnknown {
    self
  }
}

impl AsRef<FileTypeUnknown> for RTDFileTypeUnknownBuilder {
  fn as_ref(&self) -> &FileTypeUnknown {
    &self.inner
  }
}

/// The file is a video
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeVideo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeVideo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeVideo {}

impl FileTypeVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeVideoBuilder {
    let inner = FileTypeVideo {
      td_name: "fileTypeVideo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeVideoBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeVideoBuilder {
  inner: FileTypeVideo,
}

impl RTDFileTypeVideoBuilder {
  pub fn build(&self) -> FileTypeVideo {
    self.inner.clone()
  }
}

impl AsRef<FileTypeVideo> for FileTypeVideo {
  fn as_ref(&self) -> &FileTypeVideo {
    self
  }
}

impl AsRef<FileTypeVideo> for RTDFileTypeVideoBuilder {
  fn as_ref(&self) -> &FileTypeVideo {
    &self.inner
  }
}

/// The file is a video note
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeVideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeVideoNote {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeVideoNote"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeVideoNote {}

impl FileTypeVideoNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeVideoNoteBuilder {
    let inner = FileTypeVideoNote {
      td_name: "fileTypeVideoNote".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeVideoNoteBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeVideoNoteBuilder {
  inner: FileTypeVideoNote,
}

impl RTDFileTypeVideoNoteBuilder {
  pub fn build(&self) -> FileTypeVideoNote {
    self.inner.clone()
  }
}

impl AsRef<FileTypeVideoNote> for FileTypeVideoNote {
  fn as_ref(&self) -> &FileTypeVideoNote {
    self
  }
}

impl AsRef<FileTypeVideoNote> for RTDFileTypeVideoNoteBuilder {
  fn as_ref(&self) -> &FileTypeVideoNote {
    &self.inner
  }
}

/// The file is a video published as a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeVideoStory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeVideoStory {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeVideoStory"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeVideoStory {}

impl FileTypeVideoStory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeVideoStoryBuilder {
    let inner = FileTypeVideoStory {
      td_name: "fileTypeVideoStory".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeVideoStoryBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeVideoStoryBuilder {
  inner: FileTypeVideoStory,
}

impl RTDFileTypeVideoStoryBuilder {
  pub fn build(&self) -> FileTypeVideoStory {
    self.inner.clone()
  }
}

impl AsRef<FileTypeVideoStory> for FileTypeVideoStory {
  fn as_ref(&self) -> &FileTypeVideoStory {
    self
  }
}

impl AsRef<FileTypeVideoStory> for RTDFileTypeVideoStoryBuilder {
  fn as_ref(&self) -> &FileTypeVideoStory {
    &self.inner
  }
}

/// The file is a voice note
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeVoiceNote {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeVoiceNote"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeVoiceNote {}

impl FileTypeVoiceNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeVoiceNoteBuilder {
    let inner = FileTypeVoiceNote {
      td_name: "fileTypeVoiceNote".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeVoiceNoteBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeVoiceNoteBuilder {
  inner: FileTypeVoiceNote,
}

impl RTDFileTypeVoiceNoteBuilder {
  pub fn build(&self) -> FileTypeVoiceNote {
    self.inner.clone()
  }
}

impl AsRef<FileTypeVoiceNote> for FileTypeVoiceNote {
  fn as_ref(&self) -> &FileTypeVoiceNote {
    self
  }
}

impl AsRef<FileTypeVoiceNote> for RTDFileTypeVoiceNoteBuilder {
  fn as_ref(&self) -> &FileTypeVoiceNote {
    &self.inner
  }
}

/// The file is a wallpaper or a background pattern
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeWallpaper {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FileTypeWallpaper {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileTypeWallpaper"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFileType for FileTypeWallpaper {}

impl FileTypeWallpaper {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileTypeWallpaperBuilder {
    let inner = FileTypeWallpaper {
      td_name: "fileTypeWallpaper".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFileTypeWallpaperBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFileTypeWallpaperBuilder {
  inner: FileTypeWallpaper,
}

impl RTDFileTypeWallpaperBuilder {
  pub fn build(&self) -> FileTypeWallpaper {
    self.inner.clone()
  }
}

impl AsRef<FileTypeWallpaper> for FileTypeWallpaper {
  fn as_ref(&self) -> &FileTypeWallpaper {
    self
  }
}

impl AsRef<FileTypeWallpaper> for RTDFileTypeWallpaperBuilder {
  fn as_ref(&self) -> &FileTypeWallpaper {
    &self.inner
  }
}
