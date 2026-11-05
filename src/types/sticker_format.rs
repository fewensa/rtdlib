use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes format of a sticker
pub trait TDStickerFormat: Debug + RObject {}

/// Describes format of a sticker
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum StickerFormat {
  #[doc(hidden)]
  _Default(()),
  /// The sticker is an animation in TGS format
  Tgs(StickerFormatTgs),
  /// The sticker is a video in WEBM format
  Webm(StickerFormatWebm),
  /// The sticker is an image in WEBP format
  Webp(StickerFormatWebp),
}

impl Default for StickerFormat {
  fn default() -> Self {
    StickerFormat::_Default(())
  }
}

impl<'de> Deserialize<'de> for StickerFormat {
  fn deserialize<D>(deserializer: D) -> Result<StickerFormat, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      StickerFormat,
      (stickerFormatTgs, Tgs);
      (stickerFormatWebm, Webm);
      (stickerFormatWebp, Webp);

    )(deserializer)
  }
}

impl RObject for StickerFormat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      StickerFormat::Tgs(t) => t.td_name(),
      StickerFormat::Webm(t) => t.td_name(),
      StickerFormat::Webp(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      StickerFormat::Tgs(t) => t.extra(),
      StickerFormat::Webm(t) => t.extra(),
      StickerFormat::Webp(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StickerFormat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, StickerFormat::_Default(_))
  }

  pub fn is_tgs(&self) -> bool {
    matches!(self, StickerFormat::Tgs(_))
  }
  pub fn is_webm(&self) -> bool {
    matches!(self, StickerFormat::Webm(_))
  }
  pub fn is_webp(&self) -> bool {
    matches!(self, StickerFormat::Webp(_))
  }

  pub fn on_tgs<F: FnOnce(&StickerFormatTgs)>(&self, fnc: F) -> &Self {
    if let StickerFormat::Tgs(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_webm<F: FnOnce(&StickerFormatWebm)>(&self, fnc: F) -> &Self {
    if let StickerFormat::Webm(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_webp<F: FnOnce(&StickerFormatWebp)>(&self, fnc: F) -> &Self {
    if let StickerFormat::Webp(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_tgs(&self) -> Option<&StickerFormatTgs> {
    if let StickerFormat::Tgs(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_webm(&self) -> Option<&StickerFormatWebm> {
    if let StickerFormat::Webm(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_webp(&self) -> Option<&StickerFormatWebp> {
    if let StickerFormat::Webp(t) = self {
      return Some(t);
    }
    None
  }

  pub fn tgs<T: AsRef<StickerFormatTgs>>(t: T) -> Self {
    StickerFormat::Tgs(t.as_ref().clone())
  }

  pub fn webm<T: AsRef<StickerFormatWebm>>(t: T) -> Self {
    StickerFormat::Webm(t.as_ref().clone())
  }

  pub fn webp<T: AsRef<StickerFormatWebp>>(t: T) -> Self {
    StickerFormat::Webp(t.as_ref().clone())
  }
}

impl AsRef<StickerFormat> for StickerFormat {
  fn as_ref(&self) -> &StickerFormat {
    self
  }
}

/// The sticker is an animation in TGS format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerFormatTgs {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for StickerFormatTgs {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "stickerFormatTgs"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStickerFormat for StickerFormatTgs {}

impl StickerFormatTgs {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStickerFormatTgsBuilder {
    let inner = StickerFormatTgs {
      td_name: "stickerFormatTgs".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDStickerFormatTgsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDStickerFormatTgsBuilder {
  inner: StickerFormatTgs,
}

impl RTDStickerFormatTgsBuilder {
  pub fn build(&self) -> StickerFormatTgs {
    self.inner.clone()
  }
}

impl AsRef<StickerFormatTgs> for StickerFormatTgs {
  fn as_ref(&self) -> &StickerFormatTgs {
    self
  }
}

impl AsRef<StickerFormatTgs> for RTDStickerFormatTgsBuilder {
  fn as_ref(&self) -> &StickerFormatTgs {
    &self.inner
  }
}

/// The sticker is a video in WEBM format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerFormatWebm {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for StickerFormatWebm {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "stickerFormatWebm"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStickerFormat for StickerFormatWebm {}

impl StickerFormatWebm {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStickerFormatWebmBuilder {
    let inner = StickerFormatWebm {
      td_name: "stickerFormatWebm".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDStickerFormatWebmBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDStickerFormatWebmBuilder {
  inner: StickerFormatWebm,
}

impl RTDStickerFormatWebmBuilder {
  pub fn build(&self) -> StickerFormatWebm {
    self.inner.clone()
  }
}

impl AsRef<StickerFormatWebm> for StickerFormatWebm {
  fn as_ref(&self) -> &StickerFormatWebm {
    self
  }
}

impl AsRef<StickerFormatWebm> for RTDStickerFormatWebmBuilder {
  fn as_ref(&self) -> &StickerFormatWebm {
    &self.inner
  }
}

/// The sticker is an image in WEBP format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerFormatWebp {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for StickerFormatWebp {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "stickerFormatWebp"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStickerFormat for StickerFormatWebp {}

impl StickerFormatWebp {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStickerFormatWebpBuilder {
    let inner = StickerFormatWebp {
      td_name: "stickerFormatWebp".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDStickerFormatWebpBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDStickerFormatWebpBuilder {
  inner: StickerFormatWebp,
}

impl RTDStickerFormatWebpBuilder {
  pub fn build(&self) -> StickerFormatWebp {
    self.inner.clone()
  }
}

impl AsRef<StickerFormatWebp> for StickerFormatWebp {
  fn as_ref(&self) -> &StickerFormatWebp {
    self
  }
}

impl AsRef<StickerFormatWebp> for RTDStickerFormatWebpBuilder {
  fn as_ref(&self) -> &StickerFormatWebp {
    &self.inner
  }
}
