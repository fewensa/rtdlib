use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a tab shown in a user or a chat profile
pub trait TDProfileTab: Debug + RObject {}

/// Describes a tab shown in a user or a chat profile
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ProfileTab {
  #[doc(hidden)]
  _Default(()),
  /// A tab with documents posted by the channel
  Files(ProfileTabFiles),
  /// A tab with animations posted by the channel
  Gifs(ProfileTabGifs),
  /// A tab with gifts received by the user or the channel chat
  Gifts(ProfileTabGifts),
  /// A tab with messages posted by the channel and containing links
  Links(ProfileTabLinks),
  /// A tab with photos and videos posted by the channel
  Media(ProfileTabMedia),
  /// A tab with audio messages posted by the channel
  Music(ProfileTabMusic),
  /// A tab with stories posted by the user or the channel chat and saved to profile
  Posts(ProfileTabPosts),
  /// A tab with voice notes posted by the channel
  Voice(ProfileTabVoice),
}

impl Default for ProfileTab {
  fn default() -> Self {
    ProfileTab::_Default(())
  }
}

impl<'de> Deserialize<'de> for ProfileTab {
  fn deserialize<D>(deserializer: D) -> Result<ProfileTab, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ProfileTab,
      (profileTabFiles, Files);
      (profileTabGifs, Gifs);
      (profileTabGifts, Gifts);
      (profileTabLinks, Links);
      (profileTabMedia, Media);
      (profileTabMusic, Music);
      (profileTabPosts, Posts);
      (profileTabVoice, Voice);

    )(deserializer)
  }
}

impl RObject for ProfileTab {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      ProfileTab::Files(t) => t.td_name(),
      ProfileTab::Gifs(t) => t.td_name(),
      ProfileTab::Gifts(t) => t.td_name(),
      ProfileTab::Links(t) => t.td_name(),
      ProfileTab::Media(t) => t.td_name(),
      ProfileTab::Music(t) => t.td_name(),
      ProfileTab::Posts(t) => t.td_name(),
      ProfileTab::Voice(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      ProfileTab::Files(t) => t.extra(),
      ProfileTab::Gifs(t) => t.extra(),
      ProfileTab::Gifts(t) => t.extra(),
      ProfileTab::Links(t) => t.extra(),
      ProfileTab::Media(t) => t.extra(),
      ProfileTab::Music(t) => t.extra(),
      ProfileTab::Posts(t) => t.extra(),
      ProfileTab::Voice(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ProfileTab {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, ProfileTab::_Default(_))
  }

  pub fn is_files(&self) -> bool {
    matches!(self, ProfileTab::Files(_))
  }
  pub fn is_gifs(&self) -> bool {
    matches!(self, ProfileTab::Gifs(_))
  }
  pub fn is_gifts(&self) -> bool {
    matches!(self, ProfileTab::Gifts(_))
  }
  pub fn is_links(&self) -> bool {
    matches!(self, ProfileTab::Links(_))
  }
  pub fn is_media(&self) -> bool {
    matches!(self, ProfileTab::Media(_))
  }
  pub fn is_music(&self) -> bool {
    matches!(self, ProfileTab::Music(_))
  }
  pub fn is_posts(&self) -> bool {
    matches!(self, ProfileTab::Posts(_))
  }
  pub fn is_voice(&self) -> bool {
    matches!(self, ProfileTab::Voice(_))
  }

  pub fn on_files<F: FnOnce(&ProfileTabFiles)>(&self, fnc: F) -> &Self {
    if let ProfileTab::Files(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_gifs<F: FnOnce(&ProfileTabGifs)>(&self, fnc: F) -> &Self {
    if let ProfileTab::Gifs(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_gifts<F: FnOnce(&ProfileTabGifts)>(&self, fnc: F) -> &Self {
    if let ProfileTab::Gifts(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_links<F: FnOnce(&ProfileTabLinks)>(&self, fnc: F) -> &Self {
    if let ProfileTab::Links(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_media<F: FnOnce(&ProfileTabMedia)>(&self, fnc: F) -> &Self {
    if let ProfileTab::Media(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_music<F: FnOnce(&ProfileTabMusic)>(&self, fnc: F) -> &Self {
    if let ProfileTab::Music(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_posts<F: FnOnce(&ProfileTabPosts)>(&self, fnc: F) -> &Self {
    if let ProfileTab::Posts(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_voice<F: FnOnce(&ProfileTabVoice)>(&self, fnc: F) -> &Self {
    if let ProfileTab::Voice(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_files(&self) -> Option<&ProfileTabFiles> {
    if let ProfileTab::Files(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_gifs(&self) -> Option<&ProfileTabGifs> {
    if let ProfileTab::Gifs(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_gifts(&self) -> Option<&ProfileTabGifts> {
    if let ProfileTab::Gifts(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_links(&self) -> Option<&ProfileTabLinks> {
    if let ProfileTab::Links(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_media(&self) -> Option<&ProfileTabMedia> {
    if let ProfileTab::Media(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_music(&self) -> Option<&ProfileTabMusic> {
    if let ProfileTab::Music(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_posts(&self) -> Option<&ProfileTabPosts> {
    if let ProfileTab::Posts(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_voice(&self) -> Option<&ProfileTabVoice> {
    if let ProfileTab::Voice(t) = self {
      return Some(t);
    }
    None
  }

  pub fn files<T: AsRef<ProfileTabFiles>>(t: T) -> Self {
    ProfileTab::Files(t.as_ref().clone())
  }

  pub fn gifs<T: AsRef<ProfileTabGifs>>(t: T) -> Self {
    ProfileTab::Gifs(t.as_ref().clone())
  }

  pub fn gifts<T: AsRef<ProfileTabGifts>>(t: T) -> Self {
    ProfileTab::Gifts(t.as_ref().clone())
  }

  pub fn links<T: AsRef<ProfileTabLinks>>(t: T) -> Self {
    ProfileTab::Links(t.as_ref().clone())
  }

  pub fn media<T: AsRef<ProfileTabMedia>>(t: T) -> Self {
    ProfileTab::Media(t.as_ref().clone())
  }

  pub fn music<T: AsRef<ProfileTabMusic>>(t: T) -> Self {
    ProfileTab::Music(t.as_ref().clone())
  }

  pub fn posts<T: AsRef<ProfileTabPosts>>(t: T) -> Self {
    ProfileTab::Posts(t.as_ref().clone())
  }

  pub fn voice<T: AsRef<ProfileTabVoice>>(t: T) -> Self {
    ProfileTab::Voice(t.as_ref().clone())
  }
}

impl AsRef<ProfileTab> for ProfileTab {
  fn as_ref(&self) -> &ProfileTab {
    self
  }
}

/// A tab with documents posted by the channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileTabFiles {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ProfileTabFiles {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "profileTabFiles"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDProfileTab for ProfileTabFiles {}

impl ProfileTabFiles {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDProfileTabFilesBuilder {
    let inner = ProfileTabFiles {
      td_name: "profileTabFiles".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDProfileTabFilesBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDProfileTabFilesBuilder {
  inner: ProfileTabFiles,
}

impl RTDProfileTabFilesBuilder {
  pub fn build(&self) -> ProfileTabFiles {
    self.inner.clone()
  }
}

impl AsRef<ProfileTabFiles> for ProfileTabFiles {
  fn as_ref(&self) -> &ProfileTabFiles {
    self
  }
}

impl AsRef<ProfileTabFiles> for RTDProfileTabFilesBuilder {
  fn as_ref(&self) -> &ProfileTabFiles {
    &self.inner
  }
}

/// A tab with animations posted by the channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileTabGifs {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ProfileTabGifs {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "profileTabGifs"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDProfileTab for ProfileTabGifs {}

impl ProfileTabGifs {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDProfileTabGifsBuilder {
    let inner = ProfileTabGifs {
      td_name: "profileTabGifs".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDProfileTabGifsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDProfileTabGifsBuilder {
  inner: ProfileTabGifs,
}

impl RTDProfileTabGifsBuilder {
  pub fn build(&self) -> ProfileTabGifs {
    self.inner.clone()
  }
}

impl AsRef<ProfileTabGifs> for ProfileTabGifs {
  fn as_ref(&self) -> &ProfileTabGifs {
    self
  }
}

impl AsRef<ProfileTabGifs> for RTDProfileTabGifsBuilder {
  fn as_ref(&self) -> &ProfileTabGifs {
    &self.inner
  }
}

/// A tab with gifts received by the user or the channel chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileTabGifts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ProfileTabGifts {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "profileTabGifts"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDProfileTab for ProfileTabGifts {}

impl ProfileTabGifts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDProfileTabGiftsBuilder {
    let inner = ProfileTabGifts {
      td_name: "profileTabGifts".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDProfileTabGiftsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDProfileTabGiftsBuilder {
  inner: ProfileTabGifts,
}

impl RTDProfileTabGiftsBuilder {
  pub fn build(&self) -> ProfileTabGifts {
    self.inner.clone()
  }
}

impl AsRef<ProfileTabGifts> for ProfileTabGifts {
  fn as_ref(&self) -> &ProfileTabGifts {
    self
  }
}

impl AsRef<ProfileTabGifts> for RTDProfileTabGiftsBuilder {
  fn as_ref(&self) -> &ProfileTabGifts {
    &self.inner
  }
}

/// A tab with messages posted by the channel and containing links
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileTabLinks {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ProfileTabLinks {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "profileTabLinks"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDProfileTab for ProfileTabLinks {}

impl ProfileTabLinks {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDProfileTabLinksBuilder {
    let inner = ProfileTabLinks {
      td_name: "profileTabLinks".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDProfileTabLinksBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDProfileTabLinksBuilder {
  inner: ProfileTabLinks,
}

impl RTDProfileTabLinksBuilder {
  pub fn build(&self) -> ProfileTabLinks {
    self.inner.clone()
  }
}

impl AsRef<ProfileTabLinks> for ProfileTabLinks {
  fn as_ref(&self) -> &ProfileTabLinks {
    self
  }
}

impl AsRef<ProfileTabLinks> for RTDProfileTabLinksBuilder {
  fn as_ref(&self) -> &ProfileTabLinks {
    &self.inner
  }
}

/// A tab with photos and videos posted by the channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileTabMedia {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ProfileTabMedia {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "profileTabMedia"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDProfileTab for ProfileTabMedia {}

impl ProfileTabMedia {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDProfileTabMediaBuilder {
    let inner = ProfileTabMedia {
      td_name: "profileTabMedia".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDProfileTabMediaBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDProfileTabMediaBuilder {
  inner: ProfileTabMedia,
}

impl RTDProfileTabMediaBuilder {
  pub fn build(&self) -> ProfileTabMedia {
    self.inner.clone()
  }
}

impl AsRef<ProfileTabMedia> for ProfileTabMedia {
  fn as_ref(&self) -> &ProfileTabMedia {
    self
  }
}

impl AsRef<ProfileTabMedia> for RTDProfileTabMediaBuilder {
  fn as_ref(&self) -> &ProfileTabMedia {
    &self.inner
  }
}

/// A tab with audio messages posted by the channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileTabMusic {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ProfileTabMusic {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "profileTabMusic"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDProfileTab for ProfileTabMusic {}

impl ProfileTabMusic {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDProfileTabMusicBuilder {
    let inner = ProfileTabMusic {
      td_name: "profileTabMusic".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDProfileTabMusicBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDProfileTabMusicBuilder {
  inner: ProfileTabMusic,
}

impl RTDProfileTabMusicBuilder {
  pub fn build(&self) -> ProfileTabMusic {
    self.inner.clone()
  }
}

impl AsRef<ProfileTabMusic> for ProfileTabMusic {
  fn as_ref(&self) -> &ProfileTabMusic {
    self
  }
}

impl AsRef<ProfileTabMusic> for RTDProfileTabMusicBuilder {
  fn as_ref(&self) -> &ProfileTabMusic {
    &self.inner
  }
}

/// A tab with stories posted by the user or the channel chat and saved to profile
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileTabPosts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ProfileTabPosts {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "profileTabPosts"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDProfileTab for ProfileTabPosts {}

impl ProfileTabPosts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDProfileTabPostsBuilder {
    let inner = ProfileTabPosts {
      td_name: "profileTabPosts".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDProfileTabPostsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDProfileTabPostsBuilder {
  inner: ProfileTabPosts,
}

impl RTDProfileTabPostsBuilder {
  pub fn build(&self) -> ProfileTabPosts {
    self.inner.clone()
  }
}

impl AsRef<ProfileTabPosts> for ProfileTabPosts {
  fn as_ref(&self) -> &ProfileTabPosts {
    self
  }
}

impl AsRef<ProfileTabPosts> for RTDProfileTabPostsBuilder {
  fn as_ref(&self) -> &ProfileTabPosts {
    &self.inner
  }
}

/// A tab with voice notes posted by the channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileTabVoice {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ProfileTabVoice {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "profileTabVoice"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDProfileTab for ProfileTabVoice {}

impl ProfileTabVoice {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDProfileTabVoiceBuilder {
    let inner = ProfileTabVoice {
      td_name: "profileTabVoice".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDProfileTabVoiceBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDProfileTabVoiceBuilder {
  inner: ProfileTabVoice,
}

impl RTDProfileTabVoiceBuilder {
  pub fn build(&self) -> ProfileTabVoice {
    self.inner.clone()
  }
}

impl AsRef<ProfileTabVoice> for ProfileTabVoice {
  fn as_ref(&self) -> &ProfileTabVoice {
    self
  }
}

impl AsRef<ProfileTabVoice> for RTDProfileTabVoiceBuilder {
  fn as_ref(&self) -> &ProfileTabVoice {
    &self.inner
  }
}
