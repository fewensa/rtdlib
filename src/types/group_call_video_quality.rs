
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes the quality of a group call video
pub trait TDGroupCallVideoQuality: Debug + RObject {}

/// Describes the quality of a group call video
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GroupCallVideoQuality {
  #[doc(hidden)] _Default(()),
  /// The best available video quality
  Full(GroupCallVideoQualityFull),
  /// The medium video quality
  Medium(GroupCallVideoQualityMedium),
  /// The worst available video quality
  Thumbnail(GroupCallVideoQualityThumbnail),

}

impl Default for GroupCallVideoQuality {
  fn default() -> Self { GroupCallVideoQuality::_Default(()) }
}

impl<'de> Deserialize<'de> for GroupCallVideoQuality {
  fn deserialize<D>(deserializer: D) -> Result<GroupCallVideoQuality, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      GroupCallVideoQuality,
      (groupCallVideoQualityFull, Full);
      (groupCallVideoQualityMedium, Medium);
      (groupCallVideoQualityThumbnail, Thumbnail);

    )(deserializer)
  }
}

impl RObject for GroupCallVideoQuality {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      GroupCallVideoQuality::Full(t) => t.td_name(),
      GroupCallVideoQuality::Medium(t) => t.td_name(),
      GroupCallVideoQuality::Thumbnail(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      GroupCallVideoQuality::Full(t) => t.extra(),
      GroupCallVideoQuality::Medium(t) => t.extra(),
      GroupCallVideoQuality::Thumbnail(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl GroupCallVideoQuality {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let GroupCallVideoQuality::_Default(_) = self { true } else { false } }

  pub fn is_full(&self) -> bool { if let GroupCallVideoQuality::Full(_) = self { true } else { false } }
  pub fn is_medium(&self) -> bool { if let GroupCallVideoQuality::Medium(_) = self { true } else { false } }
  pub fn is_thumbnail(&self) -> bool { if let GroupCallVideoQuality::Thumbnail(_) = self { true } else { false } }

  pub fn on_full<F: FnOnce(&GroupCallVideoQualityFull)>(&self, fnc: F) -> &Self { if let GroupCallVideoQuality::Full(t) = self { fnc(t) }; self }
  pub fn on_medium<F: FnOnce(&GroupCallVideoQualityMedium)>(&self, fnc: F) -> &Self { if let GroupCallVideoQuality::Medium(t) = self { fnc(t) }; self }
  pub fn on_thumbnail<F: FnOnce(&GroupCallVideoQualityThumbnail)>(&self, fnc: F) -> &Self { if let GroupCallVideoQuality::Thumbnail(t) = self { fnc(t) }; self }

  pub fn as_full(&self) -> Option<&GroupCallVideoQualityFull> { if let GroupCallVideoQuality::Full(t) = self { return Some(t) } None }
  pub fn as_medium(&self) -> Option<&GroupCallVideoQualityMedium> { if let GroupCallVideoQuality::Medium(t) = self { return Some(t) } None }
  pub fn as_thumbnail(&self) -> Option<&GroupCallVideoQualityThumbnail> { if let GroupCallVideoQuality::Thumbnail(t) = self { return Some(t) } None }



  pub fn full<T: AsRef<GroupCallVideoQualityFull>>(t: T) -> Self { GroupCallVideoQuality::Full(t.as_ref().clone()) }

  pub fn medium<T: AsRef<GroupCallVideoQualityMedium>>(t: T) -> Self { GroupCallVideoQuality::Medium(t.as_ref().clone()) }

  pub fn thumbnail<T: AsRef<GroupCallVideoQualityThumbnail>>(t: T) -> Self { GroupCallVideoQuality::Thumbnail(t.as_ref().clone()) }

}

impl AsRef<GroupCallVideoQuality> for GroupCallVideoQuality {
  fn as_ref(&self) -> &GroupCallVideoQuality { self }
}







/// The best available video quality
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallVideoQualityFull {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GroupCallVideoQualityFull {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "groupCallVideoQualityFull" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDGroupCallVideoQuality for GroupCallVideoQualityFull {}



impl GroupCallVideoQualityFull {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGroupCallVideoQualityFullBuilder {
    let mut inner = GroupCallVideoQualityFull::default();
    inner.td_name = "groupCallVideoQualityFull".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGroupCallVideoQualityFullBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGroupCallVideoQualityFullBuilder {
  inner: GroupCallVideoQualityFull
}

impl RTDGroupCallVideoQualityFullBuilder {
  pub fn build(&self) -> GroupCallVideoQualityFull { self.inner.clone() }

}

impl AsRef<GroupCallVideoQualityFull> for GroupCallVideoQualityFull {
  fn as_ref(&self) -> &GroupCallVideoQualityFull { self }
}

impl AsRef<GroupCallVideoQualityFull> for RTDGroupCallVideoQualityFullBuilder {
  fn as_ref(&self) -> &GroupCallVideoQualityFull { &self.inner }
}







/// The medium video quality
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallVideoQualityMedium {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GroupCallVideoQualityMedium {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "groupCallVideoQualityMedium" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDGroupCallVideoQuality for GroupCallVideoQualityMedium {}



impl GroupCallVideoQualityMedium {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGroupCallVideoQualityMediumBuilder {
    let mut inner = GroupCallVideoQualityMedium::default();
    inner.td_name = "groupCallVideoQualityMedium".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGroupCallVideoQualityMediumBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGroupCallVideoQualityMediumBuilder {
  inner: GroupCallVideoQualityMedium
}

impl RTDGroupCallVideoQualityMediumBuilder {
  pub fn build(&self) -> GroupCallVideoQualityMedium { self.inner.clone() }

}

impl AsRef<GroupCallVideoQualityMedium> for GroupCallVideoQualityMedium {
  fn as_ref(&self) -> &GroupCallVideoQualityMedium { self }
}

impl AsRef<GroupCallVideoQualityMedium> for RTDGroupCallVideoQualityMediumBuilder {
  fn as_ref(&self) -> &GroupCallVideoQualityMedium { &self.inner }
}







/// The worst available video quality
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallVideoQualityThumbnail {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GroupCallVideoQualityThumbnail {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "groupCallVideoQualityThumbnail" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDGroupCallVideoQuality for GroupCallVideoQualityThumbnail {}



impl GroupCallVideoQualityThumbnail {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGroupCallVideoQualityThumbnailBuilder {
    let mut inner = GroupCallVideoQualityThumbnail::default();
    inner.td_name = "groupCallVideoQualityThumbnail".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGroupCallVideoQualityThumbnailBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGroupCallVideoQualityThumbnailBuilder {
  inner: GroupCallVideoQualityThumbnail
}

impl RTDGroupCallVideoQualityThumbnailBuilder {
  pub fn build(&self) -> GroupCallVideoQualityThumbnail { self.inner.clone() }

}

impl AsRef<GroupCallVideoQualityThumbnail> for GroupCallVideoQualityThumbnail {
  fn as_ref(&self) -> &GroupCallVideoQualityThumbnail { self }
}

impl AsRef<GroupCallVideoQualityThumbnail> for RTDGroupCallVideoQualityThumbnailBuilder {
  fn as_ref(&self) -> &GroupCallVideoQualityThumbnail { &self.inner }
}



